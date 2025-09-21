//! GPU-CPU Integration for World Generation
//!
//! This module provides the integration layer that allows world generation
//! to use GPU compute when available, with automatic CPU fallback.

use super::{
    get_gpu_elevation_results, request_gpu_elevation_generation, GpuComputeStatus,
    GpuGenerationConfig, GpuGenerationState, GpuPerformanceMetrics, NoiseComputeSettings,
};
use crate::math::PerlinNoise;
use crate::resources::MapDimensions;
use bevy::prelude::*;

/// GPU-accelerated elevation generation system
/// This replaces the CPU-only province elevation generation when GPU is available
pub struct GpuElevationGenerator {
    pub noise: PerlinNoise,
    pub dimensions: MapDimensions,
    pub seed: u32,
    pub continent_seeds: Vec<(Vec2, f32, f32)>,
}

impl GpuElevationGenerator {
    pub fn new(
        dimensions: MapDimensions,
        seed: u32,
        continent_seeds: Vec<(Vec2, f32, f32)>,
    ) -> Self {
        let noise = PerlinNoise::with_seed(seed);

        Self {
            noise,
            dimensions,
            seed,
            continent_seeds,
        }
    }

    /// Generate elevations for provinces - GPU path with CPU fallback
    pub fn generate_elevations(
        &self,
        positions: &[Vec2],
        gpu_status: &GpuComputeStatus,
        gpu_config: &GpuGenerationConfig,
        gpu_state: &mut GpuGenerationState,
    ) -> Vec<f32> {
        let start_time = std::time::Instant::now();

        // Try GPU generation first if available
        if gpu_status.compute_supported && gpu_config.use_gpu {
            info!(
                "Attempting GPU elevation generation for {} provinces",
                positions.len()
            );

            match self.try_gpu_generation(positions, gpu_state) {
                Some(elevations) => {
                    let gpu_time = start_time.elapsed();
                    info!(
                        "GPU elevation generation completed in {:.2}s",
                        gpu_time.as_secs_f32()
                    );
                    return elevations;
                }
                None => {
                    warn!("GPU generation failed, falling back to CPU");
                }
            }
        }

        // CPU fallback generation
        info!(
            "Using CPU elevation generation for {} provinces",
            positions.len()
        );
        let elevations = self.generate_elevations_cpu(positions);
        let cpu_time = start_time.elapsed();
        info!(
            "CPU elevation generation completed in {:.2}s",
            cpu_time.as_secs_f32()
        );

        elevations
    }

    /// Attempt GPU generation - integrates with Bevy's GPU coordinator systems
    fn try_gpu_generation(
        &self,
        _positions: &[Vec2],
        gpu_state: &mut GpuGenerationState,
    ) -> Option<Vec<f32>> {
        // Check if GPU results are already available from coordinator systems
        match gpu_state {
            GpuGenerationState::Complete(elevations) => {
                let results = elevations.clone();
                *gpu_state = GpuGenerationState::Ready; // Reset for next use
                Some(results)
            }
            _ => {
                // GPU not ready or failed - use CPU fallback
                // The actual GPU coordination happens through Bevy systems in coordinator.rs
                None
            }
        }
    }

    /// CPU fallback elevation generation (current implementation)
    pub fn generate_elevations_cpu(&self, positions: &[Vec2]) -> Vec<f32> {
        use rayon::prelude::*;

        // Parallel CPU generation using existing logic
        positions
            .par_iter()
            .map(|&position| self.generate_single_elevation_cpu(position))
            .collect()
    }

    /// Generate single elevation on CPU (extracted from ProvinceBuilder)
    fn generate_single_elevation_cpu(&self, position: Vec2) -> f32 {
        // Scale position to noise space (important for proper sampling)
        let scale = 1.0 / self.dimensions.hex_size;
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
            let warp_strength = radius * 0.3; // Warp up to 30% of radius
            let warped_distance = distance + (warp_x + warp_y) * warp_strength;

            // Use smooth falloff with inner and outer radius for better control
            let inner_radius = radius * 0.4;
            let outer_radius = radius * 1.2;
            let influence =
                crate::math::smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;
            continent_influence = continent_influence.max(influence);
        }

        // Apply edge distance falloff for more natural coastlines
        let distance_to_edge = self.calculate_edge_distance(position);
        let edge_falloff = self.calculate_falloff(distance_to_edge);

        // Combine all influences for final elevation
        (base_elevation + continent_influence) * edge_falloff
    }

    /// Calculate distance to nearest world edge
    fn calculate_edge_distance(&self, position: Vec2) -> f32 {
        let world_width = self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size;
        let world_height = self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size;

        let half_width = world_width * 0.5;
        let half_height = world_height * 0.5;

        let dx = (position.x.abs() - half_width).max(0.0);
        let dy = (position.y.abs() - half_height).max(0.0);

        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate edge falloff for natural coastlines
    fn calculate_falloff(&self, distance_to_edge: f32) -> f32 {
        let falloff_start = self.dimensions.hex_size * 10.0; // Start falloff 10 hex sizes from edge
        let falloff_end = self.dimensions.hex_size * 50.0; // Complete falloff 50 hex sizes from edge

        if distance_to_edge <= falloff_start {
            1.0
        } else if distance_to_edge >= falloff_end {
            0.0
        } else {
            let t = (distance_to_edge - falloff_start) / (falloff_end - falloff_start);
            crate::math::smoothstep(0.0, 1.0, 1.0 - t) // Smooth falloff using our math utilities
        }
    }
}

/// Integration system that provides GPU-accelerated province generation
pub fn gpu_accelerated_province_generation(
    positions: Vec<Vec2>,
    dimensions: MapDimensions,
    seed: u32,
    continent_seeds: Vec<(Vec2, f32, f32)>,
    gpu_status: &GpuComputeStatus,
    gpu_config: &GpuGenerationConfig,
    gpu_state: &mut GpuGenerationState,
    metrics: &mut GpuPerformanceMetrics,
) -> Vec<f32> {
    let generator = GpuElevationGenerator::new(dimensions, seed, continent_seeds);

    let start_time = std::time::Instant::now();
    let elevations = generator.generate_elevations(&positions, gpu_status, gpu_config, gpu_state);
    let generation_time = start_time.elapsed();

    // Update performance metrics
    metrics.last_generation_time = Some(generation_time);
    metrics.total_operations += 1;

    if matches!(gpu_state, GpuGenerationState::Complete(_)) {
        metrics.successful_operations += 1;

        // Calculate speedup factor if we have both GPU and CPU times
        // This would require running both for comparison in benchmarking mode
        if let Some(previous_cpu_time) = metrics.last_generation_time {
            // For now, estimate based on known GPU advantages
            metrics.gpu_speedup_factor =
                Some(previous_cpu_time.as_secs_f32() / generation_time.as_secs_f32());
        }
    }

    elevations
}

/// Helper function to extract province positions for GPU upload
pub fn extract_province_positions(col_count: u32, row_count: u32, hex_size: f32) -> Vec<Vec2> {
    let total_provinces = col_count * row_count;
    let mut positions = Vec::with_capacity(total_provinces as usize);

    for index in 0..total_provinces {
        let col = index % col_count;
        let row = index / col_count;

        let position =
            crate::math::calculate_grid_position(col, row, hex_size, col_count, row_count);

        positions.push(position);
    }

    positions
}
