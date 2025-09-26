//! GPU-accelerated elevation generation
//!
//! Provides GPU acceleration for elevation generation when available,
//! falling back to CPU processing when GPU is not available.

use bevy::prelude::Vec2;
use log::info;

use crate::math::smooth_falloff;
use crate::parallel::parallel_map;
use crate::resources::MapDimensions;
use crate::world::generation::GenerationUtils;
use super::continents::ContinentGenerator;
use super::elevation_processor::ElevationProcessor;

/// Start falloff at 60% from center for natural island shapes
const FALLOFF_START: f32 = 0.6;

/// Handles GPU-accelerated generation operations
pub struct GpuAccelerator {
    dimensions: MapDimensions,
    seed: u32,
}

impl GpuAccelerator {
    pub fn new(dimensions: MapDimensions, seed: u32) -> Self {
        Self { dimensions, seed }
    }

    /// Try to generate elevations using parallel CPU processing
    /// (Full GPU path is handled by GpuProvinceBuilder in world/gpu module)
    pub fn try_gpu_elevation_generation(
        &self,
        positions: &[Vec2],
        continent_count: u32,
    ) -> Vec<f32> {
        info!("  Using parallel CPU elevation generation (full GPU path available via GpuProvinceBuilder)");

        // Generate continent seeds
        let continent_gen = ContinentGenerator::new(self.dimensions, continent_count, self.seed);
        let continent_seeds = continent_gen.generate_seeds_for_gpu();

        // Create elevation processor
        let elevation_processor = ElevationProcessor::new(self.seed, self.dimensions);

        // Generate elevations in parallel
        let mut elevations: Vec<f32> = parallel_map(
            positions,
            |&position| elevation_processor.generate_elevation(position, &continent_seeds),
            "elevation_generation",
        );

        // Apply adaptive redistribution
        elevation_processor.apply_adaptive_redistribution(&mut elevations);

        elevations
    }

    /// Prepare data for GPU processing
    pub fn prepare_gpu_data(positions: &[Vec2]) -> Vec<[f32; 2]> {
        positions.iter()
            .map(|pos| [pos.x, pos.y])
            .collect()
    }

    /// Calculate edge distance for GPU using radial distance
    pub fn calculate_edge_distance_gpu(&self, position: Vec2) -> f32 {
        let world_width = self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size;
        let world_height = self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size;

        // Calculate map center (assumes centered at origin)
        let map_center = Vec2::ZERO;

        // Use radial distance from center for natural, circular falloff
        position.distance(map_center)
    }

    /// Calculate falloff curve for edges
    pub fn calculate_falloff_gpu(&self, distance_to_edge: f32) -> f32 {
        // Normalize distance to 0-1 range
        let max_distance = (self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size)
            .min(self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size) / 2.0;
        let normalized_distance = (distance_to_edge / max_distance).clamp(0.0, 1.0);

        // Use TWO smooth_falloff calls with .max() for natural island shapes
        smooth_falloff(normalized_distance, 0.0, FALLOFF_START)
            .max(1.0 - smooth_falloff(normalized_distance, FALLOFF_START, 1.0))
    }
}