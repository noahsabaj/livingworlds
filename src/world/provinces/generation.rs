//! Province generation with Perlin noise-based terrain
//!
//! This module generates individual hexagonal provinces with detailed
//! terrain features using our centralized Perlin noise module.

use bevy::prelude::Vec2;
use log::info;
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use std::collections::HashMap;

use super::super::terrain::TerrainType;
use super::{Abundance, Agriculture, Distance, Elevation, Province, ProvinceId};
use crate::math::{
    calculate_grid_position, get_neighbor_positions, normalized_edge_distance, smooth_falloff,
    smoothstep, PerlinNoise,
};
use crate::resources::MapDimensions;
use crate::world::generation::GenerationUtils;
use crate::world::gpu::{
    extract_province_positions, gpu_accelerated_province_generation, GpuComputeStatus,
    GpuGenerationConfig, GpuGenerationState, GpuPerformanceMetrics,
};
use std::f32::consts::TAU;

/// Default ocean coverage percentage (0.0 to 1.0)
const DEFAULT_OCEAN_COVERAGE: f32 = 0.6;

/// Beach/coast width in elevation units
const BEACH_WIDTH: f32 = 0.01; // Reduced from 0.02 for thinner, more realistic beaches

/// Province builder that creates hexagonal provinces with Perlin noise elevation
pub struct ProvinceBuilder<'a> {
    utils: GenerationUtils,
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

        // Create generation utilities for shared operations
        let utils = GenerationUtils::new(dimensions);

        Self {
            utils,
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
        let total_provinces = self.utils.total_provinces();
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

        // Generate provinces using GPU acceleration when available
        let mut provinces = self.generate_provinces_accelerated(total_provinces, sea_level);

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

    /// Generate provinces with GPU acceleration when available
    fn generate_provinces_accelerated(
        &self,
        total_provinces: u32,
        sea_level: f32,
    ) -> Vec<Province> {
        // Try GPU acceleration if available (this will be called from a system with access to resources)
        // For now, we'll implement a hybrid approach that can be called from anywhere

        // Extract all province positions for GPU processing
        let positions = extract_province_positions(
            self.utils.dimensions().provinces_per_row,
            self.utils.dimensions().provinces_per_col,
            self.utils.dimensions().hex_size,
        );

        // Use GPU acceleration for elevation generation if we can access it
        let elevations = self.try_gpu_elevation_generation(&positions);

        // Generate provinces with the computed elevations
        self.generate_provinces_from_elevations(positions, elevations, sea_level)
    }

    /// Attempt GPU elevation generation with fallback to CPU
    fn try_gpu_elevation_generation(&self, positions: &[Vec2]) -> Vec<f32> {
        // Create continent seeds for terrain generation
        let continent_seeds = self.generate_continent_seeds_for_gpu();

        // For now, we use CPU since GPU requires full Bevy resource context
        // The GPU path is handled by generate_world_with_gpu_acceleration() in setup.rs
        // which uses GpuProvinceBuilder for full GPU acceleration
        info!("  Using CPU elevation generation (full GPU path available via GpuProvinceBuilder)");

        // Use existing CPU generation logic
        let mut elevations: Vec<f32> = positions
            .par_iter()
            .map(|&position| {
                // Generate raw elevation without redistribution
                self.generate_single_elevation_cpu_raw(position, &continent_seeds)
            })
            .collect();

        // Apply adaptive redistribution to the entire elevation set
        // This ensures proper ocean/land balance and guarantees river sources
        self.apply_adaptive_redistribution(&mut elevations);

        elevations
    }

    /// Generate provinces from pre-computed positions and elevations
    fn generate_provinces_from_elevations(
        &self,
        positions: Vec<Vec2>,
        elevations: Vec<f32>,
        sea_level: f32,
    ) -> Vec<Province> {
        assert_eq!(
            positions.len(),
            elevations.len(),
            "Positions and elevations must have same length"
        );

        // Generate provinces in parallel using pre-computed elevations
        positions
            .into_par_iter()
            .zip(elevations.into_par_iter())
            .enumerate()
            .map(|(index, (position, elevation))| {
                let province_id = ProvinceId::new(index as u32);
                let (col, row) = self.utils.id_to_grid_coords(province_id);

                // Determine terrain type based on elevation
                let terrain = self.classify_terrain(elevation, sea_level);
                let neighbors = self.calculate_hex_neighbors(col as u32, row as u32);
                let neighbor_indices = self.utils.get_neighbor_indices(col, row);

                Province {
                    id: ProvinceId::new(index as u32),
                    position,
                    owner: None,
                    culture: None,
                    elevation: Elevation::new(elevation),
                    terrain,
                    population: 0,
                    max_population: 1000,
                    agriculture: Agriculture::default(),
                    fresh_water_distance: Distance::default(),
                    iron: Abundance::default(),
                    copper: Abundance::default(),
                    tin: Abundance::default(),
                    gold: Abundance::default(),
                    coal: Abundance::default(),
                    stone: Abundance::default(),
                    gems: Abundance::default(),
                    neighbors,
                    neighbor_indices,
                    version: 0,
                    dirty: false,
                }
            })
            .collect()
    }

    /// Generate raw elevation without redistribution
    fn generate_single_elevation_cpu_raw(
        &self,
        position: Vec2,
        continent_seeds: &[(Vec2, f32, f32)],
    ) -> f32 {
        // Scale position to noise space
        let scale = 1.0 / self.utils.dimensions().hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Use centralized noise module with preset
        let base_elevation = self.noise.sample_terrain(x, y) as f32;

        // Apply continent influence
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in continent_seeds {
            let distance = position.distance(*seed_pos);

            // Add domain warping for organic shapes
            let warp_x = self.noise.sample_scaled(
                (position.x * 0.005) as f64,
                (position.y * 0.005) as f64,
                0.01,
            ) as f32;
            let warp_y = self.noise.sample_scaled(
                (position.x * 0.005 + 100.0) as f64,
                (position.y * 0.005 + 100.0) as f64,
                0.01,
            ) as f32;

            let warp_strength = radius * 0.3;
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence = smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // Combine base noise and continent influence WITHOUT redistribution
        // Just return the raw combined value
        // NO EDGE FALLOFF - let continents and noise create natural boundaries
        let combined = base_elevation * 0.5 + continent_influence * 0.5;
        combined.clamp(0.0, 1.0)
    }

    /// CPU fallback for single elevation generation (extracted from generate_elevation)
    fn generate_single_elevation_cpu(&self, position: Vec2) -> f32 {
        // Scale position to noise space (important for proper sampling)
        let scale = 1.0 / self.utils.dimensions().hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Use centralized noise module with preset
        let base_elevation = self.noise.sample_terrain(x, y) as f32;

        // Apply continent influence with noise-warped distance for organic shapes
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in &self.continent_seeds {
            let distance = position.distance(*seed_pos);

            // Add domain warping using noise to create irregular continent shapes
            let warp_x = self.noise.sample_scaled(
                (position.x * 0.005) as f64,
                (position.y * 0.005) as f64,
                0.01,
            ) as f32;
            let warp_y = self.noise.sample_scaled(
                (position.x * 0.005 + 100.0) as f64,
                (position.y * 0.005 + 100.0) as f64,
                0.01,
            ) as f32;

            // Apply warping to distance - creates irregular, organic continent shapes
            let warp_strength = radius * 0.3;
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            // Use smooth falloff with inner and outer radius for better control
            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence = smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // For Earth-like worlds, we use continent-based generation without aggressive edge falloff
        // Edge falloff creates unrealistic ring-shaped worlds
        // Instead, let continents and noise create natural land distribution

        // Combine base noise and continent influence
        // Weight: 50% base noise, 50% continent influence (proven working balance)
        // This replaces the old 40/40/20 split after removing edge falloff
        let combined_elevation = base_elevation * 0.5 + continent_influence * 0.5;

        // HYBRID POWER REDISTRIBUTION: Different curves for different elevation ranges
        // This maintains both deep oceans AND high mountains without skewing the distribution
        let redistributed = self.apply_hybrid_power_redistribution(combined_elevation);

        // NO EDGE FALLOFF - creates unnatural circular patterns
        // Let continent seeds and noise create natural boundaries
        redistributed.clamp(0.0, 1.0)
    }

    fn generate_province(&self, index: u32, sea_level: f32) -> Province {
        let province_id = ProvinceId::new(index);
        let (col, row) = self.utils.id_to_grid_coords(province_id);

        // Use the single source of truth for hexagon positioning
        let position = calculate_grid_position(
            col as u32,
            row as u32,
            self.utils.dimensions().hex_size,
            self.utils.dimensions().provinces_per_row,
            self.utils.dimensions().provinces_per_col,
        );

        // Generate elevation using multi-octave Perlin noise
        let elevation = self.generate_elevation(position);

        // Determine terrain type based on elevation
        let terrain = self.classify_terrain(elevation, sea_level);

        let neighbors = self.calculate_hex_neighbors(col as u32, row as u32);
        let neighbor_indices = self.utils.get_neighbor_indices(col, row);

        Province {
            id: ProvinceId::new(index),
            position,
            owner: None,   // Nations are assigned later
            culture: None, // Culture is assigned later
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
            neighbor_indices,
            version: 0,
            dirty: false,
        }
    }

    /// Generate continent seeds for natural landmass distribution
    fn generate_continent_seeds(&mut self) {
        self.continent_seeds.clear();

        // Vary continent count for more diverse worlds
        // Handle presets with fewer than 3 continents (e.g., Pangaea with 1)
        let min_continents = 3.min(self.continent_count);
        let num_continents = if min_continents == self.continent_count {
            // If continent_count < 3, use exact count (no variation)
            self.continent_count
        } else {
            // Otherwise, vary between 3 and the specified count
            self.rng.gen_range(min_continents..=self.continent_count)
        };

        let map_width = self.utils.dimensions().bounds.x_max - self.utils.dimensions().bounds.x_min;
        let map_height =
            self.utils.dimensions().bounds.y_max - self.utils.dimensions().bounds.y_min;
        let center_x = self.utils.dimensions().bounds.x_min + map_width / 2.0;
        let center_y = self.utils.dimensions().bounds.y_min + map_height / 2.0;

        for i in 0..num_continents {
            // Place some continents near center, others more randomly
            let (x, y) = if i < 2 && num_continents > 4 {
                // First couple continents closer to center for larger worlds
                let angle = self.rng.gen_range(0.0..TAU);
                let dist = self.rng.gen_range(0.2..0.5) * map_width.min(map_height) / 2.0;
                let (sin_angle, cos_angle) = angle.sin_cos();
                (center_x + cos_angle * dist, center_y + sin_angle * dist)
            } else {
                // Others more randomly distributed
                {
                    let pos = self.utils.random_position(self.rng);
                    (pos.x, pos.y)
                }
            };

            let position = Vec2::new(x, y);
            let strength = self.rng.gen_range(0.6..1.0);
            let radius = self.rng.gen_range(0.15..0.35) * map_width.min(map_height);

            self.continent_seeds.push((position, strength, radius));
        }
    }

    /// Generate elevation using our centralized Perlin noise module with continent seeds
    fn generate_elevation(&self, position: Vec2) -> f32 {
        // Scale position to noise space (important for proper sampling)
        let scale = 1.0 / self.utils.dimensions().hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Use centralized noise module with preset
        let base_elevation = self.noise.sample_terrain(x, y) as f32;

        // Apply continent influence with noise-warped distance for organic shapes
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in &self.continent_seeds {
            let distance = position.distance(*seed_pos);

            // Add domain warping using noise to create irregular continent shapes
            // Sample noise at a scale that creates interesting perturbations
            let warp_x = self.noise.sample_scaled(
                (position.x * 0.005) as f64,
                (position.y * 0.005) as f64,
                0.01,
            ) as f32;
            let warp_y = self.noise.sample_scaled(
                (position.x * 0.005 + 100.0) as f64,
                (position.y * 0.005 + 100.0) as f64,
                0.01,
            ) as f32;

            // Apply warping to distance - creates irregular, organic continent shapes
            let warp_strength = radius * 0.3; // Warp up to 30% of radius
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            // Use smooth falloff with inner and outer radius for better control
            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence =
                crate::math::smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // For Earth-like worlds, minimize edge effects to avoid ring-shaped continents
        // Combine base noise and continent influence with proper balance for ocean coverage
        // Weight: 50% base noise, 50% continent influence (proven working balance)
        let combined_elevation = base_elevation * 0.5 + continent_influence * 0.5;

        // HYBRID POWER REDISTRIBUTION: Preserves ocean basins while creating mountain peaks
        let redistributed = self.apply_hybrid_power_redistribution(combined_elevation);

        // NO EDGE FALLOFF - let continent seeds and noise create natural boundaries
        redistributed.clamp(0.0, 1.0)
    }

    /// Calculate distance from map edges for falloff (FIXED: Use radial distance)
    fn calculate_edge_distance(&self, position: Vec2) -> f32 {
        let width = self.utils.dimensions().bounds.x_max - self.utils.dimensions().bounds.x_min;
        let height = self.utils.dimensions().bounds.y_max - self.utils.dimensions().bounds.y_min;

        // Calculate center of the map
        let center_x = self.utils.dimensions().bounds.x_min + width / 2.0;
        let center_y = self.utils.dimensions().bounds.y_min + height / 2.0;
        let map_center = Vec2::new(center_x, center_y);

        // Use RADIAL distance from center, not rectangular distance!
        // This creates natural, circular falloff instead of rectangular boundaries
        let distance_from_center = position.distance(map_center);
        let max_distance = (width.min(height)) / 2.0;

        // Normalize to 0-1 range (0 at center, 1 at edges)
        (distance_from_center / max_distance).clamp(0.0, 1.0)
    }

    /// Calculate falloff based on distance from edge (WORKING version from git history)
    fn calculate_falloff(&self, distance: f32) -> f32 {
        // Start falloff at 60% from center, smooth to edge (original working parameters)
        const FALLOFF_START: f32 = 0.6;

        // Use original working algorithm from git history: TWO smooth_falloff calls with .max()
        // This creates natural island shapes instead of rectangular boundaries
        // Discovery: This exact algorithm from commit 7622ee8 creates organic landmasses
        smooth_falloff(distance, 0.0, FALLOFF_START)
            .max(1.0 - smooth_falloff(distance, FALLOFF_START, 1.0))
    }

    /// Apply adaptive redistribution based on actual elevation distribution
    /// This creates a robust bimodal distribution that adapts to any noise pattern
    fn apply_adaptive_redistribution(&self, elevations: &mut [f32]) {
        // ADAPTIVE PERCENTILE-BASED REDISTRIBUTION
        // No more magic numbers - adapt to the actual distribution!

        // Step 1: Sort to find percentiles
        let mut sorted = elevations.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Step 2: Find key percentiles in the distribution
        let len = sorted.len();
        let p10 = sorted[len * 10 / 100];  // 10th percentile
        let p40 = sorted[len * 40 / 100];  // 40th percentile
        let p60 = sorted[len * 60 / 100];  // 60th percentile (target sea level)
        let p70 = sorted[len * 70 / 100];  // 70th percentile
        let p90 = sorted[len * 90 / 100];  // 90th percentile
        let p95 = sorted[len * 95 / 100];  // 95th percentile
        let p99 = sorted[len * 99 / 100];  // 99th percentile

        // Step 3: Build mapping function based on percentiles
        // This guarantees the desired distribution regardless of input
        for elevation in elevations.iter_mut() {
            let val = *elevation;

            *elevation = if val <= p60 {
                // Bottom 60% → Ocean (0.0 to 0.15)
                // Use smooth curve for natural depth variation
                let t = (val - sorted[0]) / (p60 - sorted[0]).max(0.001);
                t * t * 0.15  // Quadratic for deep ocean basins

            } else if val <= p70 {
                // 60-70% → Coastal transition (0.15 to 0.25)
                let t = (val - p60) / (p70 - p60).max(0.001);
                0.15 + t * 0.10

            } else if val <= p90 {
                // 70-90% → Lowlands and hills (0.25 to 0.45)
                let t = (val - p70) / (p90 - p70).max(0.001);
                0.25 + t * 0.20

            } else if val <= p95 {
                // 90-95% → Mountains (0.45 to 0.65)
                let t = (val - p90) / (p95 - p90).max(0.001);
                0.45 + t * 0.20

            } else {
                // Top 5% → High peaks (0.65 to 1.0)
                // GUARANTEE peaks above 0.65 for river sources!
                let t = (val - p95) / (p99 - p95).max(0.001);
                0.65 + t.powf(0.7) * 0.35
            };
        }

        // Log the redistribution for debugging
        info!("  Adaptive redistribution: p60={:.3} (sea), p90={:.3} (mountains), p95={:.3} (peaks)",
              p60, p90, p95);
    }

    /// Apply simple redistribution to a single elevation value
    /// Used for sea level calculation sampling
    fn apply_hybrid_power_redistribution(&self, elevation: f32) -> f32 {
        // For single-value redistribution, use a simple curve
        // This is only used during sea level sampling, not final generation
        elevation.powf(1.2)
    }

    /// Apply histogram equalization to normalize elevation distribution
    fn histogram_equalization(&self, elevations: &mut [f32]) {
        const BINS: usize = 256;

        // Build histogram
        let mut histogram = vec![0u32; BINS];
        for &elev in elevations.iter() {
            let bin = ((elev.clamp(0.0, 1.0) * (BINS - 1) as f32) as usize).min(BINS - 1);
            histogram[bin] += 1;
        }

        // Compute cumulative distribution function (CDF)
        let mut cdf = vec![0.0f32; BINS];
        let total = elevations.len() as f32;
        let mut sum = 0u32;
        for i in 0..BINS {
            sum += histogram[i];
            cdf[i] = sum as f32 / total;
        }

        // Apply equalization
        for elev in elevations.iter_mut() {
            let bin = ((elev.clamp(0.0, 1.0) * (BINS - 1) as f32) as usize).min(BINS - 1);
            *elev = cdf[bin];
        }
    }

    /// Calculate sea level for desired ocean coverage
    fn calculate_sea_level(&mut self) -> f32 {
        // ADAPTIVE SEA LEVEL CALCULATION
        // Sample the actual redistributed elevations to find the right sea level

        const SAMPLE_COUNT: usize = 4000; // Increased for better accuracy
        let continent_seeds = self.generate_continent_seeds_for_gpu();

        // Generate sample elevations
        let mut elevations = Vec::with_capacity(SAMPLE_COUNT);
        for _ in 0..SAMPLE_COUNT {
            let position = self.utils.random_position(self.rng);
            // Generate raw elevation
            let raw = self.generate_single_elevation_cpu_raw(position, &continent_seeds);
            elevations.push(raw);
        }

        // Apply the same adaptive redistribution that will be used for actual generation
        self.apply_adaptive_redistribution(&mut elevations);

        // Now find the percentile for ocean coverage
        elevations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let ocean_index = (self.ocean_coverage * SAMPLE_COUNT as f32) as usize;

        // The sea level is simply the elevation at the desired percentile
        // No magic clamps needed - the adaptive redistribution ensures proper values
        elevations[ocean_index.min(SAMPLE_COUNT - 1)]
    }

    /// Classify terrain based on elevation
    fn classify_terrain(&self, elevation: f32, sea_level: f32) -> TerrainType {
        // Apply smoothstep near sea level for cleaner coastlines
        // Expanded range to create cleaner land/ocean boundaries
        let smoothed_elevation = if (elevation - sea_level).abs() < 0.04 {
            // Near sea level, apply smoothstep to reduce noise
            let t = (elevation - (sea_level - 0.04)) / 0.08; // Normalize to 0-1 range
            let smooth_t = smoothstep(0.0, 1.0, t);
            (sea_level - 0.04) + smooth_t * 0.08
        } else {
            elevation
        };

        if smoothed_elevation < sea_level {
            TerrainType::Ocean
        } else if smoothed_elevation < sea_level + BEACH_WIDTH {
            TerrainType::Beach
        } else if smoothed_elevation < sea_level + 0.05 {
            // Reduced from 0.1 - coastal lowlands
            TerrainType::TemperateGrassland
        } else if smoothed_elevation < sea_level + 0.15 {
            // Reduced from 0.2 - foothills
            TerrainType::TemperateDeciduousForest
        } else if smoothed_elevation < sea_level + 0.25 {
            // Reduced from 0.35 - mid-elevation
            TerrainType::Chaparral // This will become desert with rain shadow
        } else if smoothed_elevation < sea_level + 0.4 {
            // Reduced from 0.5 - highlands
            TerrainType::Alpine
        } else {
            TerrainType::Tundra // Mountain peaks above 0.4
        }
    }

    /// Calculate hexagonal neighbors using the shared utilities
    fn calculate_hex_neighbors(&self, col: u32, row: u32) -> [Option<ProvinceId>; 6] {
        // Use shared utilities for neighbor coordinate calculation
        let neighbor_coords = self.utils.get_neighbor_coords(col as i32, row as i32);
        let mut neighbors = [None; 6];

        // Convert coordinates to ProvinceId using shared utilities
        for (i, (neighbor_col, neighbor_row)) in neighbor_coords.iter().enumerate() {
            if let Some(province_id) = self.utils.grid_coords_to_id(*neighbor_col, *neighbor_row) {
                neighbors[i] = Some(province_id);
            }
        }

        neighbors
    }

    /// Filter out small islands using flood-fill algorithm
    ///
    /// This prevents "spaghetti islands" by removing land masses smaller than a threshold.
    /// Uses connected component analysis to identify and remove small isolated land provinces.
    fn filter_small_islands(&self, provinces: &mut Vec<Province>) -> usize {
        const MIN_ISLAND_SIZE: usize = 50; // Increased from 10 - removes smallest fragments but keeps archipelagos

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
                            if !visited[neighbor_idx]
                                && provinces[neighbor_idx].terrain != TerrainType::Ocean
                            {
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
                    if neighbor.terrain == TerrainType::Alpine
                        || neighbor.terrain == TerrainType::Tundra
                    {
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

    /// Generate continent seeds for GPU-compatible terrain generation
    fn generate_continent_seeds_for_gpu(&self) -> Vec<(Vec2, f32, f32)> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed as u64);
        let mut seeds = Vec::new();

        let world_width =
            self.utils.dimensions().provinces_per_row as f32 * self.utils.dimensions().hex_size;
        let world_height =
            self.utils.dimensions().provinces_per_col as f32 * self.utils.dimensions().hex_size;

        // Earth-like continent distribution with varied sizes
        for i in 0..self.continent_count {
            // Distribute continents across entire map, not clustered in center
            let angle = (i as f32 / self.continent_count as f32) * std::f32::consts::TAU + rng.gen::<f32>() * 0.5;
            let distance = (0.2 + rng.gen::<f32>() * 0.6) * world_width.min(world_height) * 0.4;

            let x = angle.cos() * distance + (rng.gen::<f32>() - 0.5) * world_width * 0.3;
            let y = angle.sin() * distance + (rng.gen::<f32>() - 0.5) * world_height * 0.3;

            // Vary continent sizes for realism
            let size_roll = rng.gen::<f32>();
            let (strength, radius) = if size_roll < 0.3 {
                // Small islands - reduced strength for better ocean coverage
                (0.15 + rng.gen::<f32>() * 0.15, 50.0 + rng.gen::<f32>() * 100.0)
            } else if size_roll < 0.7 {
                // Medium continents - moderate strength
                (0.3 + rng.gen::<f32>() * 0.25, 150.0 + rng.gen::<f32>() * 150.0)
            } else {
                // Large continents - still prominent but not overwhelming
                (0.45 + rng.gen::<f32>() * 0.3, 250.0 + rng.gen::<f32>() * 200.0)
            };

            seeds.push((Vec2::new(x, y), strength, radius));
        }

        seeds
    }

    /// Generate single elevation with continent influence (for GPU integration)
    fn generate_single_elevation_cpu_with_continents(
        &self,
        position: Vec2,
        continent_seeds: &[(Vec2, f32, f32)],
    ) -> f32 {
        let noise = PerlinNoise::with_seed(self.seed);

        // Scale position to noise space
        let scale = 1.0 / self.utils.dimensions().hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Base elevation from noise
        let base_elevation = noise.sample_terrain(x, y) as f32;

        // Apply continent influence
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in continent_seeds {
            let distance = position.distance(*seed_pos);

            // Domain warping for more natural coastlines
            let warp_x = noise.sample_scaled(
                (position.x * 0.005) as f64,
                (position.y * 0.005) as f64,
                0.01,
            ) as f32;
            let warp_y = noise.sample_scaled(
                (position.x * 0.005 + 100.0) as f64,
                (position.y * 0.005 + 100.0) as f64,
                0.01,
            ) as f32;

            let warp_strength = radius * 0.3;
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence = smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // CRITICAL FIX: Earth-like worlds do NOT have edge falloff!
        // Real planets wrap around - they don't have edges
        // Balance base elevation and continent influence for proper ocean coverage
        // Use consistent 50/50 balance across all generation functions
        let combined = base_elevation * 0.5 + continent_influence * 0.5;

        // HYBRID POWER REDISTRIBUTION: Balance ocean depths and mountain heights
        let redistributed = self.apply_hybrid_power_redistribution(combined);

        // Add fractal detail for natural variation
        let detail_scale = 0.002;
        let detail_noise = noise.sample_scaled(
            (position.x * detail_scale) as f64,
            (position.y * detail_scale) as f64,
            0.15,
        ) as f32;

        // Final elevation with subtle detail - reduced to prevent over-elevation
        (redistributed + detail_noise * 0.05).clamp(0.0, 1.0)
    }

    /// Calculate edge distance for GPU integration (FIXED: Use radial distance)
    fn calculate_edge_distance_gpu(&self, position: Vec2) -> f32 {
        let world_width =
            self.utils.dimensions().provinces_per_row as f32 * self.utils.dimensions().hex_size;
        let world_height =
            self.utils.dimensions().provinces_per_col as f32 * self.utils.dimensions().hex_size;

        // Calculate map center (assumes centered at origin)
        let map_center = Vec2::ZERO;

        // Use RADIAL distance from center for natural, circular falloff
        // This eliminates the rectangular boundaries that were plaguing the terrain
        position.distance(map_center)
    }

    /// Calculate falloff curve for edges (non-mutable version for GPU)
    fn calculate_falloff_gpu(&self, distance_to_edge: f32) -> f32 {
        // Normalize distance to 0-1 range for consistent behavior
        let max_distance = (self.utils.dimensions().provinces_per_row as f32 * self.utils.dimensions().hex_size).min(
            self.utils.dimensions().provinces_per_col as f32 * self.utils.dimensions().hex_size) / 2.0;
        let normalized_distance = (distance_to_edge / max_distance).clamp(0.0, 1.0);

        // Start falloff at 60% from center, smooth to edge (working parameters from git history)
        const FALLOFF_START: f32 = 0.6;

        // Use original working algorithm: TWO smooth_falloff calls with .max()
        // This creates natural island shapes instead of rectangular boundaries
        smooth_falloff(normalized_distance, 0.0, FALLOFF_START)
            .max(1.0 - smooth_falloff(normalized_distance, FALLOFF_START, 1.0))
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
