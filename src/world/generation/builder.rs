//! World builder implementation
//!
//! This module contains the main WorldBuilder that orchestrates all generation steps
//! to create a complete World data structure.

use bevy::log::info;
use rand::{rngs::StdRng, SeedableRng};

use super::errors::{WorldGenerationError, WorldGenerationErrorType};
use crate::resources::{MapDimensions, WorldSize};
use crate::world::World;

// Import utilities

/// World builder that orchestrates all generation steps
///
/// This is the main entry point for world generation. It coordinates
/// all the individual builders to create a complete World.
pub struct WorldBuilder {
    seed: u32,
    size: WorldSize,
    rng: StdRng,
    dimensions: MapDimensions,
    continent_count: u32,
    ocean_coverage: f32,
    river_density: f32,
}

impl WorldBuilder {
    pub fn new(
        seed: u32,
        size: WorldSize,
        continent_count: u32,
        ocean_coverage: f32,
        river_density: f32,
    ) -> Self {
        let rng = StdRng::seed_from_u64(seed as u64);
        let dimensions = MapDimensions::from_world_size(&size);

        Self {
            seed,
            size,
            rng,
            dimensions,
            continent_count,
            ocean_coverage,
            river_density,
        }
    }

    pub fn build(mut self) -> Result<World, WorldGenerationError> {
        self.build_with_progress(None::<fn(&str, f32)>)
    }

    pub fn build_with_progress(
        mut self,
        progress_callback: Option<impl Fn(&str, f32)>,
    ) -> Result<World, WorldGenerationError> {
        let _start = std::time::Instant::now();

        // Helper to report progress
        let report_progress = |step: &str, progress: f32| {
            if let Some(ref callback) = progress_callback {
                info!("WorldBuilder: Sending progress: {} - {:.1}%", step, progress * 100.0);
                callback(step, progress);
            }
        };

        // Step 1: Generate provinces with Perlin noise elevation
        report_progress("Generating provinces...", 0.1);
        let mut provinces =
            crate::world::ProvinceBuilder::new(self.dimensions, &mut self.rng, self.seed)
                .with_ocean_coverage(self.ocean_coverage)
                .with_continent_count(self.continent_count)
                .build();

        // Step 1b: Precompute neighbor indices for O(1) lookups
        // (This is already done in ProvinceBuilder::build() now)

        // Step 2: Apply erosion simulation for realistic terrain
        report_progress("Applying erosion...", 0.2);
        let erosion_iterations =
            match self.dimensions.provinces_per_row * self.dimensions.provinces_per_col {
                n if n < 400_000 => 3_000,
                n if n < 700_000 => 5_000,
                _ => 8_000,
            };
        crate::world::apply_erosion_to_provinces(
            &mut provinces,
            self.dimensions,
            &mut self.rng,
            erosion_iterations,
        );

        // Step 3: Calculate ocean depths
        report_progress("Calculating ocean depths...", 0.3);
        crate::world::calculate_ocean_depths(&mut provinces, self.dimensions);

        // Step 4: Generate climate zones
        report_progress("Generating climate zones...", 0.4);
        crate::world::apply_climate_to_provinces(&mut provinces, self.dimensions);

        // Step 5: Generate river systems
        report_progress("Creating river systems...", 0.5);
        let river_system =
            crate::world::RiverBuilder::new(&mut provinces, self.dimensions, &mut self.rng)
                .with_density(self.river_density)
                .build()
                .map_err(|e| WorldGenerationError {
                    error_message: format!("Failed to generate rivers: {}", e),
                    error_type: WorldGenerationErrorType::GenerationFailed,
                })?;

        // Step 6: Calculate agriculture values
        report_progress("Calculating agriculture...", 0.6);
        crate::world::calculate_agriculture_values(&mut provinces, &river_system, self.dimensions)
            .map_err(|e| WorldGenerationError {
                error_message: format!("Failed to calculate agriculture: {}", e),
                error_type: WorldGenerationErrorType::GenerationFailed,
            })?;

        // Step 7: Generate cloud system
        report_progress("Generating clouds...", 0.7);
        let cloud_system = crate::world::CloudBuilder::new(&mut self.rng, &self.dimensions).build();

        // Final step
        report_progress("Finalizing world...", 0.9);

        Ok(World {
            provinces,
            rivers: river_system,
            clouds: cloud_system,
            seed: self.seed,
        })
    }
}
