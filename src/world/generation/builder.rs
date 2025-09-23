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
    climate_type: crate::world::ClimateType,
}

impl WorldBuilder {
    pub fn new(
        seed: u32,
        size: WorldSize,
        continent_count: u32,
        ocean_coverage: f32,
        river_density: f32,
        climate_type: crate::world::ClimateType,
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
            climate_type,
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
        let province_count = self.dimensions.provinces_per_row * self.dimensions.provinces_per_col;
        report_progress(&format!("Generating {} provinces with Perlin noise elevation...", province_count), 0.1);
        let mut provinces =
            crate::world::ProvinceBuilder::new(self.dimensions, &mut self.rng, self.seed)
                .with_ocean_coverage(self.ocean_coverage)
                .with_continent_count(self.continent_count)
                .build();

        // Step 1b: Precompute neighbor indices for O(1) lookups
        // (This is already done in ProvinceBuilder::build() now)

        // Step 2: Apply erosion simulation for realistic terrain
        let erosion_iterations =
            match self.dimensions.provinces_per_row * self.dimensions.provinces_per_col {
                n if n < 400_000 => 3_000,
                n if n < 700_000 => 5_000,
                _ => 8_000,
            };
        report_progress(&format!("Applying erosion simulation ({} iterations)...", erosion_iterations), 0.2);
        crate::world::apply_erosion_to_provinces(
            &mut provinces,
            self.dimensions,
            &mut self.rng,
            erosion_iterations,
        );

        // Step 3: Calculate ocean depths
        let ocean_count = provinces.iter().filter(|p| p.elevation.value() <= 0.0).count();
        report_progress(&format!("Calculating depths for {} ocean provinces...", ocean_count), 0.3);
        crate::world::calculate_ocean_depths(&mut provinces, self.dimensions);

        // Step 4: Generate climate zones
        report_progress(&format!("Generating climate zones across {} provinces...", provinces.len()), 0.4);
        crate::world::apply_climate_to_provinces(&mut provinces, self.dimensions, self.climate_type);

        // Step 5: Generate river systems
        let target_rivers = (provinces.len() as f32 * self.river_density * 0.001) as usize;
        report_progress(&format!("Creating river systems (targeting ~{} rivers)...", target_rivers), 0.5);
        let river_system =
            crate::world::RiverBuilder::new(&mut provinces, self.dimensions, &mut self.rng)
                .with_density(self.river_density)
                .build()
                .map_err(|e| WorldGenerationError {
                    error_message: format!("Failed to generate rivers: {}", e),
                    error_type: WorldGenerationErrorType::GenerationFailed,
                })?;

        // Step 6: Calculate agriculture values
        report_progress(&format!("Calculating agriculture values for {} provinces...", provinces.len()), 0.6);
        crate::world::calculate_agriculture_values(&mut provinces, &river_system, self.dimensions)
            .map_err(|e| WorldGenerationError {
                error_message: format!("Failed to calculate agriculture: {}", e),
                error_type: WorldGenerationErrorType::GenerationFailed,
            })?;

        // Step 7: Generate cloud system
        let cloud_count = 90; // Default cloud count
        report_progress(&format!("Generating {} procedural clouds...", cloud_count), 0.7);
        let cloud_system = crate::world::CloudBuilder::new(&mut self.rng, &self.dimensions).build();

        // Step 8: Final validation and packaging
        report_progress("Finalizing world data structures...", 0.85);

        // Step 9: Complete!
        report_progress("World generation complete!", 0.95);

        Ok(World {
            provinces,
            rivers: river_system,
            clouds: cloud_system,
            seed: self.seed,
        })
    }
}
