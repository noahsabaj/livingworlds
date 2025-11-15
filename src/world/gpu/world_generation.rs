//! GPU-Accelerated World Generation System
//!
//! This module provides a GPU-aware world generation system that can access
//! Bevy resources and coordinate between CPU and GPU generation methods.

use super::{
    extract_province_positions, gpu_accelerated_province_generation,
    validate_gpu_cpu_elevation_generation, GpuComputeStatus, GpuGenerationConfig,
    GpuGenerationState, GpuPerformanceMetrics, ValidationConfig,
};
use crate::math::PerlinNoise;
use crate::resources::MapDimensions;
use crate::world::provinces::{Province, ProvinceBuilder};
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// GPU-aware province builder that can access Bevy resources
pub struct GpuProvinceBuilder {
    pub dimensions: MapDimensions,
    pub seed: u32,
    pub ocean_coverage: f32,
    pub continent_count: u32,
    pub enable_validation: bool,
}

impl GpuProvinceBuilder {
    pub fn new(dimensions: MapDimensions, seed: u32) -> Self {
        Self {
            dimensions,
            seed,
            ocean_coverage: 0.6,
            continent_count: 7,
            enable_validation: false, // Only enable in debug/test builds
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

    pub fn with_validation(mut self, enabled: bool) -> Self {
        self.enable_validation = enabled;
        self
    }

    /// Build provinces with GPU acceleration when available
    pub fn build_with_gpu(
        self,
        gpu_status: &GpuComputeStatus,
        gpu_config: &GpuGenerationConfig,
        mut gpu_state: &mut GpuGenerationState,
        mut gpu_metrics: &mut GpuPerformanceMetrics,
        validation_config: Option<&ValidationConfig>,
    ) -> Vec<Province> {
        let total_provinces = self.dimensions.provinces_per_row * self.dimensions.provinces_per_col;
        info!(
            "Starting GPU-accelerated province generation for {} provinces",
            total_provinces
        );

        // Extract province positions for GPU processing
        let positions = extract_province_positions(
            self.dimensions.provinces_per_row,
            self.dimensions.provinces_per_col,
            self.dimensions.hex_size,
        );

        // Create continent seeds for terrain generation
        let continent_seeds = self.generate_continent_seeds();
        let sea_level = self.calculate_sea_level(&positions);

        // Run validation if enabled
        if self.enable_validation {
            if let Some(validation_config) = validation_config {
                info!("Running GPU-CPU validation before generation");
                let validation_result = validate_gpu_cpu_elevation_generation(
                    self.dimensions,
                    self.seed,
                    continent_seeds.clone(),
                    validation_config,
                    gpu_status,
                    gpu_config,
                );

                if !validation_result.passed {
                    warn!("GPU validation failed - using CPU fallback");
                    return self.build_with_cpu_fallback();
                }

                info!("GPU validation passed - proceeding with GPU generation");
            }
        }

        // Attempt GPU-accelerated elevation generation
        let elevations = if gpu_status.compute_supported && gpu_config.use_gpu {
            info!("Using GPU elevation generation");

            match self.try_gpu_elevation_generation(
                &positions,
                &continent_seeds,
                gpu_status,
                gpu_config,
                &mut gpu_state,
                &mut gpu_metrics,
            ) {
                Some(elevations) => {
                    info!("GPU elevation generation successful");
                    elevations
                }
                None => {
                    warn!("GPU generation failed - falling back to CPU");
                    self.generate_elevations_cpu(&positions, &continent_seeds)
                }
            }
        } else {
            info!("GPU not available - using CPU generation");
            self.generate_elevations_cpu(&positions, &continent_seeds)
        };

        // Generate provinces from computed elevations
        let provinces = self.generate_provinces_from_elevations(positions, elevations, sea_level);

        // Apply post-processing (same as original ProvinceBuilder)
        let final_provinces = self.apply_post_processing(provinces);

        info!(
            "Province generation complete with {} provinces",
            final_provinces.len()
        );
        final_provinces
    }

    /// Attempt GPU elevation generation with comprehensive error handling
    fn try_gpu_elevation_generation(
        &self,
        positions: &[Vec2],
        continent_seeds: &[(Vec2, f32, f32)],
        gpu_status: &GpuComputeStatus,
        gpu_config: &GpuGenerationConfig,
        gpu_state: &mut GpuGenerationState,
        gpu_metrics: &mut GpuPerformanceMetrics,
    ) -> Option<Vec<f32>> {
        let start_time = std::time::Instant::now();

        let elevations = gpu_accelerated_province_generation(
            positions.to_vec(),
            self.dimensions,
            self.seed,
            continent_seeds.to_vec(),
            gpu_status,
            gpu_config,
            gpu_state,
            gpu_metrics,
        );

        let generation_time = start_time.elapsed();
        info!(
            "GPU generation completed in {:.2}s",
            generation_time.as_secs_f32()
        );

        // Validate results
        if elevations.len() != positions.len() {
            error!(
                "GPU generation failed: elevation count mismatch ({} vs {})",
                elevations.len(),
                positions.len()
            );
            return None;
        }

        // Check for reasonable elevation values
        let min_elevation = elevations.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_elevation = elevations.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        if min_elevation < -2.0 || max_elevation > 2.0 {
            error!(
                "GPU generation failed: elevations out of range ({:.3} to {:.3})",
                min_elevation, max_elevation
            );
            return None;
        }

        Some(elevations)
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

    /// CPU fallback elevation generation
    fn generate_elevations_cpu(
        &self,
        positions: &[Vec2],
        continent_seeds: &[(Vec2, f32, f32)],
    ) -> Vec<f32> {
        use rayon::prelude::*;

        let noise = PerlinNoise::with_seed(self.seed);

        let elevations: Vec<f32> = positions
            .par_iter()
            .map(|&position| self.generate_single_elevation_cpu(position, &noise, continent_seeds))
            .collect();

        // Don't apply histogram equalization - it flattens the distribution too much
        // The hybrid power redistribution should be enough

        elevations
    }

    /// Generate single elevation on CPU (same logic as ProvinceBuilder)
    fn generate_single_elevation_cpu(
        &self,
        position: Vec2,
        noise: &PerlinNoise,
        continent_seeds: &[(Vec2, f32, f32)],
    ) -> f32 {
        // Scale position to noise space
        let scale = 1.0 / self.dimensions.hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Base elevation from noise
        let base_elevation = noise.sample_terrain(x, y) as f32;

        // Apply continent influence
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in continent_seeds {
            let distance = position.distance(*seed_pos);

            // Domain warping
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
            let influence =
                crate::math::smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // Earth-like worlds: Pure continent-based generation, NO edge falloff
        // Real planets wrap around - they don't have edges!
        // Balanced for proper ocean coverage: 50% base noise, 50% continent influence
        let combined_elevation = base_elevation * 0.5 + continent_influence * 0.5;

        // HYBRID POWER REDISTRIBUTION: Different curves for different elevation ranges
        // This maintains both deep oceans AND high mountains without skewing the distribution
        let redistributed = self.apply_hybrid_power_redistribution(combined_elevation);

        // Add fractal noise for more natural coastlines
        let detail_scale = 0.002;
        let detail_noise = noise.sample_scaled(
            (position.x * detail_scale) as f64,
            (position.y * detail_scale) as f64,
            0.15,
        ) as f32;

        // Combine with detail noise for natural variation - reduced to prevent over-elevation
        let final_elevation = redistributed + detail_noise * 0.05;

        // No edge falloff at all - Earth doesn't have edges!
        final_elevation.clamp(0.0, 1.0)
    }

    /// CPU fallback that uses the original ProvinceBuilder
    fn build_with_cpu_fallback(&self) -> Vec<Province> {
        info!("Using CPU fallback generation");

        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        ProvinceBuilder::new(self.dimensions, &mut rng, self.seed)
            .with_ocean_coverage(self.ocean_coverage)
            .with_continent_count(self.continent_count)
            .build()
    }

    /// Generate continent seeds (same logic as ProvinceBuilder)
    fn generate_continent_seeds(&self) -> Vec<(Vec2, f32, f32)> {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        let mut seeds = Vec::new();

        let world_width = self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size;
        let world_height = self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size;

        // Earth-like continent distribution with varied sizes
        for i in 0..self.continent_count {
            // Distribute continents across entire map, not clustered in center
            let angle = (i as f32 / self.continent_count as f32) * std::f32::consts::TAU + rng.r#gen::<f32>() * 0.5;
            let distance = (0.2 + rng.r#gen::<f32>() * 0.6) * world_width.min(world_height) * 0.4;

            let x = angle.cos() * distance + (rng.r#gen::<f32>() - 0.5) * world_width * 0.3;
            let y = angle.sin() * distance + (rng.r#gen::<f32>() - 0.5) * world_height * 0.3;

            // Vary continent sizes for realism
            let size_roll = rng.r#gen::<f32>();
            let (strength, radius) = if size_roll < 0.3 {
                // Small islands - reduced strength for better ocean coverage
                (0.15 + rng.r#gen::<f32>() * 0.15, 50.0 + rng.r#gen::<f32>() * 100.0)
            } else if size_roll < 0.7 {
                // Medium continents - moderate strength
                (0.3 + rng.r#gen::<f32>() * 0.25, 150.0 + rng.r#gen::<f32>() * 150.0)
            } else {
                // Large continents - still prominent but not overwhelming
                (0.45 + rng.r#gen::<f32>() * 0.3, 250.0 + rng.r#gen::<f32>() * 200.0)
            };

            seeds.push((Vec2::new(x, y), strength, radius));
        }

        seeds
    }

    /// Apply simple power redistribution to elevation
    /// Creates natural bimodal distribution with distinct ocean basins and land masses
    fn apply_hybrid_power_redistribution(&self, elevation: f32) -> f32 {
        // STRONG BIMODAL DISTRIBUTION for proper ocean/land balance
        // This creates distinct separation between ocean basins and continents
        // Must match CPU version in generation.rs for consistency

        if elevation < 0.45 {
            // Ocean basin: Push strongly toward 0
            // Map [0, 0.45] to [0, 0.15] with strong curve
            let t = elevation / 0.45;
            t * t * 0.15  // Quadratic pushes most ocean values very low
        } else if elevation < 0.55 {
            // Transition zone: Continental shelf and coastlines
            // Map [0.45, 0.55] to [0.15, 0.25] linearly
            let t = (elevation - 0.45) / 0.1;
            0.15 + t * 0.1
        } else {
            // Land masses: Preserve variety, ensure mountains
            // Map [0.55, 1.0] to [0.25, 1.0] with mild curve
            let t = (elevation - 0.55) / 0.45;
            // Use pow(0.7) to preserve high elevations while lifting mid-values
            0.25 + t.powf(0.7) * 0.75
        }
    }

    /// Generate a single elevation value for sea level calculation
    fn generate_single_elevation(&self, position: &Vec2, continent_seeds: &[(Vec2, f32, f32)]) -> f32 {
        let noise = PerlinNoise::with_seed(self.seed);

        // Scale position to noise space
        let scale = 1.0 / self.dimensions.hex_size;
        let x = (position.x * scale) as f64;
        let y = (position.y * scale) as f64;

        // Base elevation from noise
        let base_elevation = noise.sample_terrain(x, y) as f32;

        // Apply continent influence
        let mut continent_influence = 0.0_f32;
        for (seed_pos, strength, radius) in continent_seeds {
            let distance = position.distance(*seed_pos);
            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence = crate::math::smooth_falloff(distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // Combine and apply hybrid redistribution
        let combined = base_elevation * 0.5 + continent_influence * 0.5;
        self.apply_hybrid_power_redistribution(combined)
    }

    /// Calculate adaptive sea level for ocean coverage by sampling actual elevations
    fn calculate_sea_level(&self, positions: &[Vec2]) -> f32 {
        // Sample elevations to find the actual distribution after transformation
        const SAMPLE_COUNT: usize = 2000;
        let mut elevations = Vec::with_capacity(SAMPLE_COUNT);

        let continent_seeds = self.generate_continent_seeds();
        let sample_indices: Vec<usize> = (0..SAMPLE_COUNT)
            .map(|i| (i * positions.len() / SAMPLE_COUNT).min(positions.len() - 1))
            .collect();

        for &idx in &sample_indices {
            let elevation = self.generate_single_elevation(&positions[idx], &continent_seeds);
            elevations.push(elevation);
        }

        // Sort and find percentile for desired ocean coverage
        elevations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let ocean_index = (self.ocean_coverage * SAMPLE_COUNT as f32) as usize;
        elevations[ocean_index.min(SAMPLE_COUNT - 1)]
    }

    /// Calculate distance from map edges for falloff (FIXED: Use radial distance for natural landmasses)
    fn calculate_edge_distance(&self, position: Vec2) -> f32 {
        // Calculate world dimensions
        let world_width = self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size;
        let world_height = self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size;

        // Calculate map center
        let center_x = world_width / 2.0;
        let center_y = world_height / 2.0;
        let map_center = Vec2::new(center_x, center_y);

        // CRITICAL FIX: Use RADIAL distance from center, not rectangular distance!
        // This creates natural, organic landmasses instead of rectangular boundaries
        let radial_distance = position.distance(map_center);

        // Normalize to 0-1 range (0 at center, 1 at map edges)
        let max_distance = (world_width.min(world_height)) / 2.0;
        (radial_distance / max_distance).clamp(0.0, 1.0)
    }

    /// Calculate falloff based on distance from edge (ACTUAL working version from git history)
    fn calculate_falloff(&self, distance: f32) -> f32 {
        // Start falloff at 60% from center, smooth to edge (original working parameters)
        const FALLOFF_START: f32 = 0.6;

        // Use original working algorithm: TWO smooth_falloff calls with .max()
        // This creates natural island shapes instead of rectangular boundaries
        crate::math::smooth_falloff(distance, 0.0, FALLOFF_START)
            .max(1.0 - crate::math::smooth_falloff(distance, FALLOFF_START, 1.0))
    }

    /// Generate provinces from pre-computed elevations
    fn generate_provinces_from_elevations(
        &self,
        positions: Vec<Vec2>,
        elevations: Vec<f32>,
        sea_level: f32,
    ) -> Vec<Province> {
        use crate::world::provinces::{Abundance, Agriculture, Distance, Elevation, ProvinceId};
        
        use rayon::prelude::*;

        assert_eq!(positions.len(), elevations.len());

        positions
            .into_par_iter()
            .zip(elevations.into_par_iter())
            .enumerate()
            .map(|(index, (position, elevation))| {
                let col = index as u32 % self.dimensions.provinces_per_row;
                let row = index as u32 / self.dimensions.provinces_per_row;

                // Classify terrain based on elevation
                let terrain = self.classify_terrain(elevation, sea_level);
                let neighbors = self.calculate_hex_neighbors(col, row);

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
                    neighbors: [None; 6],
                    neighbor_indices: neighbors,
                    version: 0,
                    dirty: false,
                }
            })
            .collect()
    }

    /// Classify terrain based on elevation (same logic as ProvinceBuilder)
    fn classify_terrain(
        &self,
        elevation: f32,
        sea_level: f32,
    ) -> crate::world::terrain::TerrainType {
        use crate::world::terrain::TerrainType;

        const BEACH_WIDTH: f32 = 0.01;

        if elevation <= sea_level {
            TerrainType::Ocean
        } else if elevation <= sea_level + BEACH_WIDTH {
            TerrainType::Beach
        } else if elevation < 0.3 {
            TerrainType::TemperateGrassland
        } else if elevation < 0.6 {
            TerrainType::TemperateDeciduousForest
        } else {
            TerrainType::Taiga
        }
    }

    /// Calculate hex neighbors (same logic as ProvinceBuilder)
    fn calculate_hex_neighbors(&self, col: u32, row: u32) -> [Option<usize>; 6] {
        let neighbor_positions =
            crate::math::get_neighbor_positions(col as i32, row as i32, self.dimensions.hex_size);
        let mut neighbors = [None; 6];

        for (i, (neighbor_col, neighbor_row)) in neighbor_positions.iter().enumerate() {
            if *neighbor_col >= 0
                && *neighbor_row >= 0
                && (*neighbor_col as u32) < self.dimensions.provinces_per_row
                && (*neighbor_row as u32) < self.dimensions.provinces_per_col
            {
                let neighbor_index = (*neighbor_row as u32) * self.dimensions.provinces_per_row
                    + (*neighbor_col as u32);
                neighbors[i] = Some(neighbor_index as usize);
            }
        }

        neighbors
    }

    /// Apply post-processing (island filtering, neighbor computation, etc.)
    fn apply_post_processing(&self, mut provinces: Vec<Province>) -> Vec<Province> {
        // Filter small islands
        let islands_removed = self.filter_small_islands(&mut provinces);
        if islands_removed > 0 {
            info!("  Removed {} small island provinces", islands_removed);
        }

        // Precompute neighbor indices for O(1) access
        crate::world::provinces::precompute_neighbor_indices(&mut provinces);

        provinces
    }

    /// Filter small islands (same logic as ProvinceBuilder)
    fn filter_small_islands(&self, provinces: &mut Vec<Province>) -> u32 {
        // Implementation similar to original ProvinceBuilder
        // For now, return 0 (no islands removed)
        0
    }
}

/// System that provides GPU-accelerated province generation with Bevy resource access
pub fn gpu_province_generation_system(
    dimensions: MapDimensions,
    seed: u32,
    ocean_coverage: f32,
    continent_count: u32,
    gpu_status: Res<GpuComputeStatus>,
    gpu_config: Res<GpuGenerationConfig>,
    mut gpu_state: ResMut<GpuGenerationState>,
    mut gpu_metrics: ResMut<GpuPerformanceMetrics>,
    validation_config: Option<Res<ValidationConfig>>,
) -> Vec<Province> {
    let builder = GpuProvinceBuilder::new(dimensions, seed)
        .with_ocean_coverage(ocean_coverage)
        .with_continent_count(continent_count)
        .with_validation(cfg!(debug_assertions)); // Enable validation in debug builds

    builder.build_with_gpu(
        &gpu_status,
        &gpu_config,
        &mut gpu_state,
        &mut gpu_metrics,
        validation_config.as_deref(),
    )
}
