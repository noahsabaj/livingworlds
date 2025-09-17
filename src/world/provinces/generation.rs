//! Province generation with Perlin noise-based terrain
//!
//! This module generates individual hexagonal provinces with detailed
//! terrain features using our centralized Perlin noise module.

use bevy::prelude::Vec2;
use log::info;
use rand::rngs::StdRng;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::math::{
    calculate_grid_position, euclidean_vec2, get_neighbor_positions, normalized_edge_distance,
    random_range, sin_cos, smooth_falloff, smoothstep, PerlinNoise, TAU,
};
use crate::resources::MapDimensions;
use super::super::terrain::TerrainType;
use super::{Abundance, Agriculture, Distance, Elevation, Province, ProvinceId};

/// Default ocean coverage percentage (0.0 to 1.0)
const DEFAULT_OCEAN_COVERAGE: f32 = 0.6;

/// Beach/coast width in elevation units
const BEACH_WIDTH: f32 = 0.01;  // Reduced from 0.02 for thinner, more realistic beaches

/// Province builder that creates hexagonal provinces with Perlin noise elevation
pub struct ProvinceBuilder<'a> {
    dimensions: MapDimensions,
    noise: PerlinNoise,
    rng: &'a mut StdRng,
    seed: u32,
    ocean_coverage: f32,
    continent_count: u32,
    continent_seeds: Vec<(Vec2, f32, f32)>, // (position, strength, radius)
}

impl<'a> ProvinceBuilder<'a> {
    pub fn new(dimensions: MapDimensions, rng: &'a mut StdRng, seed: u32) -> Self {
        // Create our centralized noise generator - works out of the box!
        let noise = PerlinNoise::with_seed(seed);

        Self {
            dimensions,
            noise,
            rng,
            seed,
            ocean_coverage: DEFAULT_OCEAN_COVERAGE,
            continent_count: 7,
            continent_seeds: Vec::new(), // Will be generated in build()
        }
    }

    pub fn with_ocean_coverage(mut self, coverage: f32) -> Self {
        self.ocean_coverage = coverage.clamp(0.1, 0.9);
        self
    }

    pub fn with_continent_count(mut self, count: u32) -> Self {
        self.continent_count = count.max(1);
        self
    }

    pub fn build(mut self) -> Vec<Province> {
        let total_provinces = self.dimensions.provinces_per_row * self.dimensions.provinces_per_col;
        info!("  Generating {} hexagonal provinces", total_provinces);

        // Generate continent seeds for more natural landmass distribution
        self.generate_continent_seeds();
        info!("  Generated {} continent seeds", self.continent_seeds.len());

        let sea_level = self.calculate_sea_level();
        info!(
            "  Sea level set to {:.3} for {:.0}% ocean coverage",
            sea_level,
            self.ocean_coverage * 100.0
        );

        // Generate provinces in parallel for performance
        let mut provinces: Vec<Province> = (0..total_provinces)
            .into_par_iter()
            .map(|index| self.generate_province(index, sea_level))
            .collect();

        // Filter out small islands to prevent "spaghetti islands"
        let islands_removed = self.filter_small_islands(&mut provinces);
        if islands_removed > 0 {
            info!("  Removed {} small island provinces", islands_removed);
        }

        // Precompute neighbor indices for O(1) neighbor access BEFORE rain shadow
        precompute_neighbor_indices(&mut provinces);

        // Apply rain shadow effect for realistic desert placement - now uses indices!
        self.apply_rain_shadow(&mut provinces);
        info!("  Applied rain shadow effect for desert placement");

        let ocean_count = provinces
            .iter()
            .filter(|p| p.terrain == TerrainType::Ocean)
            .count();
        let land_count = provinces.len() - ocean_count;
        info!(
            "  Generated {} land provinces, {} ocean provinces",
            land_count, ocean_count
        );

        provinces
    }

    fn generate_province(&self, index: u32, sea_level: f32) -> Province {
        let col = index % self.dimensions.provinces_per_row;
        let row = index / self.dimensions.provinces_per_row;

        // Use the single source of truth for hexagon positioning
        let position = calculate_grid_position(
            col,
            row,
            self.dimensions.hex_size,
            self.dimensions.provinces_per_row,
            self.dimensions.provinces_per_col,
        );

        // Generate elevation using multi-octave Perlin noise
        let elevation = self.generate_elevation(position);

        // Determine terrain type based on elevation
        let terrain = self.classify_terrain(elevation, sea_level);

        let neighbors = self.calculate_hex_neighbors(col, row);

        Province {
            id: ProvinceId::new(index),
            position,
            owner: None, // Nations are assigned later
            elevation: Elevation::new(elevation),
            terrain,
            population: 0,
            max_population: 1000,
            agriculture: Agriculture::default(),
            fresh_water_distance: Distance::infinite(),
            iron: Abundance::default(),
            copper: Abundance::default(),
            tin: Abundance::default(),
            gold: Abundance::default(),
            coal: Abundance::default(),
            stone: Abundance::default(),
            gems: Abundance::default(),
            neighbors,
            neighbor_indices: [None; 6], // Will be populated in a second pass
            version: 0,
            dirty: false,
        }
    }

    /// Generate continent seeds for natural landmass distribution
    fn generate_continent_seeds(&mut self) {
        self.continent_seeds.clear();

        // Vary continent count for more diverse worlds
        let num_continents = random_range(self.rng, 3, self.continent_count + 1);

        let map_width = self.dimensions.bounds.x_max - self.dimensions.bounds.x_min;
        let map_height = self.dimensions.bounds.y_max - self.dimensions.bounds.y_min;
        let center_x = self.dimensions.bounds.x_min + map_width / 2.0;
        let center_y = self.dimensions.bounds.y_min + map_height / 2.0;

        for i in 0..num_continents {
            // Place some continents near center, others more randomly
            let (x, y) = if i < 2 && num_continents > 4 {
                // First couple continents closer to center for larger worlds
                let angle = random_range(self.rng, 0.0, TAU);
                let dist = random_range(self.rng, 0.2, 0.5) * map_width.min(map_height) / 2.0;
                let (cos_angle, sin_angle) = sin_cos(angle);
                (center_x + cos_angle * dist, center_y + sin_angle * dist)
            } else {
                // Others more randomly distributed
                (
                    random_range(
                        self.rng,
                        self.dimensions.bounds.x_min,
                        self.dimensions.bounds.x_max,
                    ),
                    random_range(
                        self.rng,
                        self.dimensions.bounds.y_min,
                        self.dimensions.bounds.y_max,
                    ),
                )
            };

            let position = Vec2::new(x, y);
            let strength = random_range(self.rng, 0.6, 1.0);
            let radius = random_range(self.rng, 0.15, 0.35) * map_width.min(map_height);

            self.continent_seeds.push((position, strength, radius));
        }
    }

    /// Generate elevation using our centralized Perlin noise module with continent seeds
    fn generate_elevation(&self, position: Vec2) -> f32 {
        // Scale position to noise space (important for proper sampling)
        let scale = 1.0 / self.dimensions.hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Use centralized noise module with preset
        let base_elevation = self.noise.sample_terrain(x, y) as f32;

        // Apply continent influence with noise-warped distance for organic shapes
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in &self.continent_seeds {
            let distance = euclidean_vec2(position, *seed_pos);

            // Add domain warping using noise to create irregular continent shapes
            // Sample noise at a scale that creates interesting perturbations
            let warp_x = self.noise.sample_scaled(
                (position.x * 0.005) as f64,
                (position.y * 0.005) as f64,
                0.01
            ) as f32;
            let warp_y = self.noise.sample_scaled(
                (position.x * 0.005 + 100.0) as f64,
                (position.y * 0.005 + 100.0) as f64,
                0.01
            ) as f32;

            // Apply warping to distance - creates irregular, organic continent shapes
            let warp_strength = radius * 0.3; // Warp up to 30% of radius
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            // Use smooth falloff with inner and outer radius for better control
            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence = crate::math::smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // Apply edge distance falloff for more natural coastlines
        let distance_to_edge = self.calculate_edge_distance(position);
        let edge_falloff = self.calculate_falloff(distance_to_edge);

        // Combine base noise, continent influence, and edge falloff
        // Weight: 40% base noise, 40% continent influence, 20% edge falloff
        let combined = base_elevation * 0.4 + continent_influence * 0.4 + edge_falloff * 0.2;

        combined.clamp(0.0, 1.0)
    }

    /// Calculate distance from map edges for falloff
    fn calculate_edge_distance(&self, position: Vec2) -> f32 {
        let width = self.dimensions.bounds.x_max - self.dimensions.bounds.x_min;
        let height = self.dimensions.bounds.y_max - self.dimensions.bounds.y_min;
        let center_x = self.dimensions.bounds.x_min + width / 2.0;
        let center_y = self.dimensions.bounds.y_min + height / 2.0;
        let map_center = Vec2::new(center_x, center_y);

        // Use centralized edge distance calculation
        normalized_edge_distance(position, map_center, width / 2.0, height / 2.0)
    }

    /// Calculate falloff based on distance from edge
    fn calculate_falloff(&self, distance: f32) -> f32 {
        // Start falloff at 70% from center for more land area
        const FALLOFF_START: f32 = 0.7;
        const FALLOFF_END: f32 = 1.0;

        // Use smooth falloff - full strength in center, gradual decrease to edges
        if distance <= FALLOFF_START {
            1.0
        } else if distance >= FALLOFF_END {
            0.0
        } else {
            // Smooth transition between FALLOFF_START and FALLOFF_END
            let t = (distance - FALLOFF_START) / (FALLOFF_END - FALLOFF_START);
            // Use smoothstep for natural transition
            1.0 - smoothstep(0.0, 1.0, t)
        }
    }

    /// Calculate sea level for desired ocean coverage
    fn calculate_sea_level(&mut self) -> f32 {
        // Reduced sampling for faster calculation (10000 -> 2000)
        const SAMPLE_COUNT: usize = 2000;
        let mut elevations = Vec::with_capacity(SAMPLE_COUNT);

        for _ in 0..SAMPLE_COUNT {
            let x = random_range(
                self.rng,
                self.dimensions.bounds.x_min,
                self.dimensions.bounds.x_max,
            );
            let y = random_range(
                self.rng,
                self.dimensions.bounds.y_min,
                self.dimensions.bounds.y_max,
            );
            let elevation = self.generate_elevation(Vec2::new(x, y));
            elevations.push(elevation);
        }

        // Sort and find percentile for desired ocean coverage
        elevations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let ocean_index = (self.ocean_coverage * SAMPLE_COUNT as f32) as usize;
        elevations[ocean_index.min(SAMPLE_COUNT - 1)]
    }

    /// Classify terrain based on elevation
    fn classify_terrain(&self, elevation: f32, sea_level: f32) -> TerrainType {
        // Apply smoothstep near sea level for cleaner coastlines
        // This reduces tiny islands created by noise right at sea level
        let smoothed_elevation = if (elevation - sea_level).abs() < 0.02 {
            // Near sea level, apply smoothstep to reduce noise
            let t = (elevation - (sea_level - 0.02)) / 0.04; // Normalize to 0-1 range
            let smooth_t = smoothstep(0.0, 1.0, t);
            (sea_level - 0.02) + smooth_t * 0.04
        } else {
            elevation
        };

        if smoothed_elevation < sea_level {
            TerrainType::Ocean
        } else if smoothed_elevation < sea_level + BEACH_WIDTH {
            TerrainType::Beach
        } else if smoothed_elevation < sea_level + 0.05 {  // Reduced from 0.1 - coastal lowlands
            TerrainType::TemperateGrassland
        } else if smoothed_elevation < sea_level + 0.15 {  // Reduced from 0.2 - foothills
            TerrainType::TemperateDeciduousForest
        } else if smoothed_elevation < sea_level + 0.25 {  // Reduced from 0.35 - mid-elevation
            TerrainType::Chaparral  // This will become desert with rain shadow
        } else if smoothed_elevation < sea_level + 0.4 {   // Reduced from 0.5 - highlands
            TerrainType::Alpine
        } else {
            TerrainType::Tundra // Mountain peaks above 0.4
        }
    }

    /// Calculate hexagonal neighbors using the single source of truth
    fn calculate_hex_neighbors(&self, col: u32, row: u32) -> [Option<ProvinceId>; 6] {
        let mut neighbors = [None; 6];

        let neighbor_positions =
            get_neighbor_positions(col as i32, row as i32, self.dimensions.hex_size);

        for (i, &(neighbor_col, neighbor_row)) in neighbor_positions.iter().enumerate() {
            if neighbor_col >= 0
                && neighbor_col < self.dimensions.provinces_per_row as i32
                && neighbor_row >= 0
                && neighbor_row < self.dimensions.provinces_per_col as i32
            {
                let neighbor_id =
                    neighbor_row as u32 * self.dimensions.provinces_per_row + neighbor_col as u32;
                neighbors[i] = Some(ProvinceId::new(neighbor_id));
            }
        }

        neighbors
    }

    /// Filter out small islands using flood-fill algorithm
    ///
    /// This prevents "spaghetti islands" by removing land masses smaller than a threshold.
    /// Uses connected component analysis to identify and remove small isolated land provinces.
    fn filter_small_islands(&self, provinces: &mut Vec<Province>) -> usize {
        const MIN_ISLAND_SIZE: usize = 10; // Minimum provinces for a valid landmass

        // Build a quick lookup map for province indices
        let province_map: HashMap<u32, usize> = provinces
            .iter()
            .enumerate()
            .map(|(idx, p)| (p.id.value(), idx))
            .collect();

        // Track which provinces have been visited
        let mut visited = vec![false; provinces.len()];
        let mut islands_to_remove = Vec::new();

        // Find all connected land components
        for (idx, province) in provinces.iter().enumerate() {
            // Skip if already visited or if it's ocean
            if visited[idx] || province.terrain == TerrainType::Ocean {
                continue;
            }

            // Flood fill to find connected component size
            let mut component = Vec::new();
            let mut stack = vec![idx];

            while let Some(current_idx) = stack.pop() {
                if visited[current_idx] {
                    continue;
                }

                visited[current_idx] = true;
                component.push(current_idx);

                // Check all neighbors
                let current_province = &provinces[current_idx];
                for neighbor_id_opt in &current_province.neighbors {
                    if let Some(neighbor_id) = neighbor_id_opt {
                        if let Some(&neighbor_idx) = province_map.get(&neighbor_id.value()) {
                            // Only add land neighbors that haven't been visited
                            if !visited[neighbor_idx] && provinces[neighbor_idx].terrain != TerrainType::Ocean {
                                stack.push(neighbor_idx);
                            }
                        }
                    }
                }
            }

            // If component is too small, mark for removal
            if component.len() < MIN_ISLAND_SIZE {
                islands_to_remove.extend(component);
            }
        }

        // Convert small islands to ocean
        let removed_count = islands_to_remove.len();
        for idx in islands_to_remove {
            provinces[idx].terrain = TerrainType::Ocean;
            provinces[idx].elevation = Elevation::new(0.1); // Shallow ocean
        }

        removed_count
    }

    /// Apply rain shadow effect to create realistic desert placement
    ///
    /// Converts mid-elevation terrain (Chaparral) to desert when it's in the
    /// "shadow" of mountains, simulating how mountains block moisture.
    fn apply_rain_shadow(&self, provinces: &mut Vec<Province>) {
        // No need for HashMap - we have neighbor indices now!

        // Find provinces that should become deserts (in rain shadow)
        let mut to_convert = Vec::new();

        for (idx, province) in provinces.iter().enumerate() {
            // Only consider Chaparral (mid-elevation dry areas)
            if province.terrain != TerrainType::Chaparral {
                continue;
            }

            // Check if there are mountains nearby (within 2 hex distance)
            let mut mountain_count = 0;
            let mut total_neighbors = 0;

            // Check immediate neighbors using precomputed indices!
            for &neighbor_idx_opt in &province.neighbor_indices {
                if let Some(neighbor_idx) = neighbor_idx_opt {
                    total_neighbors += 1;
                    let neighbor = &provinces[neighbor_idx];
                    if neighbor.terrain == TerrainType::Alpine || neighbor.terrain == TerrainType::Tundra {
                        mountain_count += 1;
                    }
                }
            }

            // If surrounded by mountains (rain shadow), convert to desert
            // Also convert if isolated mid-elevation area (likely arid)
            if mountain_count >= 2 || (mountain_count >= 1 && province.elevation.value() > 0.3) {
                to_convert.push(idx);
            }
        }

        // Convert appropriate provinces to desert
        for idx in to_convert {
            provinces[idx].terrain = TerrainType::SubtropicalDesert;
        }
    }
}

/// Calculate ocean depths based on distance from land
pub fn calculate_ocean_depths(provinces: &mut [Province], _dimensions: MapDimensions) {
    info!("  Calculating ocean depths...");

    // Use BFS to calculate distance from land
    let mut ocean_distances: HashMap<u32, u32> = HashMap::new();
    let mut queue = std::collections::VecDeque::new();

    // Initialize with coastal provinces - use precomputed indices!
    for province in provinces.iter() {
        if province.terrain != TerrainType::Ocean {
            // This is land, check neighbors for ocean using precomputed indices
            for &neighbor_idx_opt in &province.neighbor_indices {
                if let Some(neighbor_idx) = neighbor_idx_opt {
                    if provinces[neighbor_idx].terrain == TerrainType::Ocean {
                        let neighbor_id = provinces[neighbor_idx].id.value();
                        ocean_distances.insert(neighbor_id, 1);
                        queue.push_back((neighbor_idx, 1));
                    }
                }
            }
        }
    }

    // BFS to calculate distances - now using indices directly!
    while let Some((current_idx, distance)) = queue.pop_front() {
        for &neighbor_idx_opt in &provinces[current_idx].neighbor_indices {
            if let Some(neighbor_idx) = neighbor_idx_opt {
                let neighbor_id = provinces[neighbor_idx].id.value();
                if provinces[neighbor_idx].terrain == TerrainType::Ocean
                    && !ocean_distances.contains_key(&neighbor_id)
                {
                    ocean_distances.insert(neighbor_id, distance + 1);
                    queue.push_back((neighbor_idx, distance + 1));
                }
            }
        }
    }

    // Apply depth based on distance
    for province in provinces.iter_mut() {
        if province.terrain == TerrainType::Ocean {
            let distance = ocean_distances
                .get(&province.id.value())
                .copied()
                .unwrap_or(100);

            // Adjust elevation based on distance from land
            if distance <= 2 {
                province.elevation = Elevation::new(0.12); // Shallow water
            } else if distance <= 5 {
                province.elevation = Elevation::new(0.07); // Continental shelf
            } else {
                province.elevation = Elevation::new(0.02); // Deep ocean
            }
        }
    }

    info!("  Ocean depths calculated");
}

/// Precompute neighbor indices for O(1) neighbor access
/// This eliminates HashMap lookups during world generation
pub fn precompute_neighbor_indices(provinces: &mut Vec<Province>) {
    // Since province IDs are sequential (0, 1, 2, ...), we can use direct indexing
    // This eliminates the HashMap entirely for a huge speedup

    let province_count = provinces.len();

    // Populate neighbor indices for each province
    for province in provinces.iter_mut() {
        for (i, neighbor_id_opt) in province.neighbors.iter().enumerate() {
            if let Some(neighbor_id) = neighbor_id_opt {
                // Province IDs are sequential, so ID == index
                let neighbor_idx = neighbor_id.value() as usize;
                // Bounds check for safety
                if neighbor_idx < province_count {
                    province.neighbor_indices[i] = Some(neighbor_idx);
                }
            }
        }
    }
}
