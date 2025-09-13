//! Province generation with Perlin noise-based terrain
//!
//! This module generates individual hexagonal provinces with detailed
//! terrain features using our centralized Perlin noise module.

use rayon::prelude::*;
use std::collections::HashMap;
use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::Rng;

use crate::components::{Province, ProvinceId, Elevation, Agriculture, Distance, Abundance};
use crate::world::terrain::TerrainType;
use crate::constants::*;
use crate::resources::MapDimensions;
use crate::math::hexagon::{calculate_grid_position, get_neighbor_positions};
use crate::math::perlin::{PerlinNoise, TerrainPreset};
use crate::math::distance::{normalized_edge_distance, smooth_falloff};

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Default ocean coverage percentage (0.0 to 1.0)
const DEFAULT_OCEAN_COVERAGE: f32 = 0.6;

/// Beach/coast width in elevation units
const BEACH_WIDTH: f32 = 0.02;

/// Province builder that creates hexagonal provinces with Perlin noise elevation
pub struct ProvinceBuilder<'a> {
    dimensions: MapDimensions,
    noise: PerlinNoise,
    rng: &'a mut StdRng,
    seed: u32,
    ocean_coverage: f32,
    continent_count: u32,
}

impl<'a> ProvinceBuilder<'a> {
    /// Create a new province builder
    pub fn new(
        dimensions: MapDimensions,
        rng: &'a mut StdRng,
        seed: u32,
    ) -> Self {
        // Create our centralized noise generator - works out of the box!
        let noise = PerlinNoise::with_seed(seed);

        Self {
            dimensions,
            noise,
            rng,
            seed,
            ocean_coverage: DEFAULT_OCEAN_COVERAGE,
            continent_count: 7,
        }
    }

    /// Set the ocean coverage percentage
    pub fn with_ocean_coverage(mut self, coverage: f32) -> Self {
        self.ocean_coverage = coverage.clamp(0.1, 0.9);
        self
    }

    /// Set the number of continents
    pub fn with_continent_count(mut self, count: u32) -> Self {
        self.continent_count = count.max(1);
        self
    }

    /// Build the provinces
    pub fn build(mut self) -> Vec<Province> {
        let total_provinces = self.dimensions.provinces_per_row * self.dimensions.provinces_per_col;
        println!("  Generating {} hexagonal provinces", total_provinces);

        // Calculate sea level based on desired ocean coverage
        let sea_level = self.calculate_sea_level();
        println!("  Sea level set to {:.3} for {:.0}% ocean coverage", sea_level, self.ocean_coverage * 100.0);

        // Generate provinces in parallel for performance
        let provinces: Vec<Province> = (0..total_provinces)
            .into_par_iter()
            .map(|index| self.generate_province(index, sea_level))
            .collect();

        // Calculate terrain statistics
        let ocean_count = provinces.iter().filter(|p| p.terrain == TerrainType::Ocean).count();
        let land_count = provinces.len() - ocean_count;
        println!("  Generated {} land provinces, {} ocean provinces", land_count, ocean_count);

        provinces
    }

    /// Generate a single province at the given index
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

        // Calculate neighbors (hexagonal pattern)
        let neighbors = self.calculate_hex_neighbors(col, row);

        Province {
            id: ProvinceId::new(index),
            position,
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
            version: 0,
            dirty: false,
        }
    }

    /// Generate elevation using our centralized Perlin noise module
    fn generate_elevation(&self, position: Vec2) -> f32 {
        // Scale position to noise space (important for proper sampling)
        // Without this, adjacent hexagons might sample nearly identical noise values
        let scale = 1.0 / self.dimensions.hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Use our ready-made terrain sampling that combines:
        // - Continental shelf features
        // - Multi-octave detail
        // - Ridge noise for mountains
        // All complexity handled internally!
        let elevation = self.noise.sample_terrain(x, y) as f32;

        // Apply distance falloff from map edges for island-like worlds
        let distance_to_edge = self.calculate_edge_distance(position);
        let falloff = self.calculate_falloff(distance_to_edge);

        (elevation * falloff).clamp(0.0, 1.0)
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
        // Start falloff at 60% from center, smooth to edge
        const FALLOFF_START: f32 = 0.6;

        // Use centralized smooth falloff function
        // Returns 1.0 inside FALLOFF_START, smoothly transitions to 0.0 at edge (1.0)
        smooth_falloff(distance, 0.0, FALLOFF_START).max(1.0 - smooth_falloff(distance, FALLOFF_START, 1.0))
    }

    /// Calculate sea level for desired ocean coverage
    fn calculate_sea_level(&mut self) -> f32 {
        // Sample elevation at many points to determine distribution
        const SAMPLE_COUNT: usize = 10000;
        let mut elevations = Vec::with_capacity(SAMPLE_COUNT);

        for _ in 0..SAMPLE_COUNT {
            let x = self.rng.gen_range(self.dimensions.bounds.x_min..self.dimensions.bounds.x_max);
            let y = self.rng.gen_range(self.dimensions.bounds.y_min..self.dimensions.bounds.y_max);
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
        if elevation < sea_level {
            TerrainType::Ocean
        } else if elevation < sea_level + BEACH_WIDTH {
            TerrainType::Beach
        } else if elevation < sea_level + 0.1 {
            TerrainType::TemperateGrassland
        } else if elevation < sea_level + 0.2 {
            TerrainType::TemperateDeciduousForest
        } else if elevation < sea_level + 0.35 {
            TerrainType::Chaparral
        } else if elevation < sea_level + 0.5 {
            TerrainType::Alpine
        } else {
            TerrainType::Tundra // Mountain peaks
        }
    }

    /// Calculate hexagonal neighbors using the single source of truth
    fn calculate_hex_neighbors(&self, col: u32, row: u32) -> [Option<ProvinceId>; 6] {
        let mut neighbors = [None; 6];

        // Get neighbor positions from the geometry module
        let neighbor_positions = get_neighbor_positions(
            col as i32,
            row as i32,
            self.dimensions.hex_size,
        );

        for (i, &(neighbor_col, neighbor_row)) in neighbor_positions.iter().enumerate() {
            // Check bounds
            if neighbor_col >= 0 && neighbor_col < self.dimensions.provinces_per_row as i32 &&
               neighbor_row >= 0 && neighbor_row < self.dimensions.provinces_per_col as i32 {
                let neighbor_id = neighbor_row as u32 * self.dimensions.provinces_per_row + neighbor_col as u32;
                neighbors[i] = Some(ProvinceId::new(neighbor_id));
            }
        }

        neighbors
    }
}

/// Calculate ocean depths based on distance from land
pub fn calculate_ocean_depths(provinces: &mut [Province], dimensions: MapDimensions) {
    println!("  Calculating ocean depths...");

    // Build spatial index for fast lookups
    let mut province_by_id: HashMap<u32, usize> = HashMap::new();
    for (idx, province) in provinces.iter().enumerate() {
        province_by_id.insert(province.id.value(), idx);
    }

    // Use BFS to calculate distance from land
    let mut ocean_distances: HashMap<u32, u32> = HashMap::new();
    let mut queue = std::collections::VecDeque::new();

    // Initialize with coastal provinces (distance 0)
    for province in provinces.iter() {
        if province.terrain != TerrainType::Ocean {
            // This is land, check neighbors for ocean
            for neighbor_opt in &province.neighbors {
                if let Some(neighbor_id) = neighbor_opt {
                    if let Some(&neighbor_idx) = province_by_id.get(&neighbor_id.value()) {
                        if provinces[neighbor_idx].terrain == TerrainType::Ocean {
                            ocean_distances.insert(neighbor_id.value(), 1);
                            queue.push_back((neighbor_id.value(), 1));
                        }
                    }
                }
            }
        }
    }

    // BFS to calculate distances
    while let Some((current_id, distance)) = queue.pop_front() {
        if let Some(&current_idx) = province_by_id.get(&current_id) {
            for neighbor_opt in &provinces[current_idx].neighbors {
                if let Some(neighbor_id) = neighbor_opt {
                    if let Some(&neighbor_idx) = province_by_id.get(&neighbor_id.value()) {
                        if provinces[neighbor_idx].terrain == TerrainType::Ocean &&
                           !ocean_distances.contains_key(&neighbor_id.value()) {
                            ocean_distances.insert(neighbor_id.value(), distance + 1);
                            queue.push_back((neighbor_id.value(), distance + 1));
                        }
                    }
                }
            }
        }
    }

    // Apply depth based on distance
    for province in provinces.iter_mut() {
        if province.terrain == TerrainType::Ocean {
            let distance = ocean_distances.get(&province.id.value()).copied().unwrap_or(100);

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

    println!("  Ocean depths calculated");
}