//! World builder implementation
//!
//! This module contains the main WorldBuilder that orchestrates all generation steps
//! to create a complete World data structure.

use bevy::log::{debug, info};
use rand::{rngs::StdRng, SeedableRng};

use super::errors::{WorldGenerationError, WorldGenerationErrorType};
use crate::diagnostics::{TimedOperation, log_world_gen_step, log_world_gen_progress, log_memory_usage};
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

    pub fn build(self) -> Result<World, WorldGenerationError> {
        self.build_with_progress(None::<fn(&str, f32)>)
    }

    pub fn build_with_progress(
        mut self,
        progress_callback: Option<impl Fn(&str, f32)>,
    ) -> Result<World, WorldGenerationError> {
        let total_timer = TimedOperation::start_with_level("World Generation", crate::diagnostics::LogLevel::Info);

        info!("Starting world generation - Seed: {}, Size: {:?}, Continents: {}, Ocean: {:.0}%",
              self.seed, self.size, self.continent_count, self.ocean_coverage * 100.0);

        // Helper to report progress
        let report_progress = |step: &str, progress: f32| {
            if let Some(ref callback) = progress_callback {
                log_world_gen_progress(step, progress, None);
                callback(step, progress);
            }
        };

        // Step 1: Generate provinces with Perlin noise elevation
        let province_count = self.dimensions.provinces_per_row * self.dimensions.provinces_per_col;
        report_progress(&format!("Generating {} provinces with Perlin noise elevation...", province_count), 0.1);

        let province_timer = TimedOperation::start("Province Generation");
        let mut provinces =
            crate::world::ProvinceBuilder::new(self.dimensions, &mut self.rng, self.seed)
                .with_ocean_coverage(self.ocean_coverage)
                .with_continent_count(self.continent_count)
                .build();
        let province_time = province_timer.complete_with_context(format!("{} provinces", province_count));
        log_world_gen_step("Province Generation", province_count as usize, province_time);

        // Log memory usage after province generation
        let province_memory = provinces.len() * std::mem::size_of::<crate::world::Province>();
        log_memory_usage("Province Storage", province_memory);

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

        let erosion_timer = TimedOperation::start("Erosion Simulation");
        debug!("Starting erosion with {} iterations for {} provinces",
               erosion_iterations, provinces.len());
        crate::world::apply_erosion_to_provinces(
            &mut provinces,
            self.dimensions,
            &mut self.rng,
            erosion_iterations,
        );
        let erosion_time = erosion_timer.complete_with_context(format!("{} iterations", erosion_iterations));
        log_world_gen_step("Erosion Simulation", erosion_iterations, erosion_time);

        // Step 3: Calculate ocean depths
        let ocean_count = provinces.iter().filter(|p| p.elevation.value() <= 0.0).count();
        report_progress(&format!("Calculating depths for {} ocean provinces...", ocean_count), 0.3);

        let ocean_timer = TimedOperation::start("Ocean Depth Calculation");
        crate::world::calculate_ocean_depths(&mut provinces, self.dimensions);
        let ocean_time = ocean_timer.complete_with_context(format!("{} ocean provinces", ocean_count));
        log_world_gen_step("Ocean Depth Calculation", ocean_count, ocean_time);

        // Step 4: Generate climate zones
        report_progress(&format!("Generating climate zones across {} provinces...", provinces.len()), 0.4);

        let climate_timer = TimedOperation::start("Climate Generation");
        let climate_storage = crate::world::apply_climate_to_provinces(&mut provinces, self.dimensions, self.climate_type);
        let climate_time = climate_timer.complete_with_context(format!("{:?} climate", self.climate_type));
        log_world_gen_step("Climate Generation", provinces.len(), climate_time);

        // Step 5: Generate river systems
        let target_rivers = (provinces.len() as f32 * self.river_density * 0.001) as usize;
        report_progress(&format!("Creating river systems (targeting ~{} rivers)...", target_rivers), 0.5);

        let river_timer = TimedOperation::start("River Generation");
        let river_system =
            crate::world::RiverBuilder::new(&mut provinces, self.dimensions, &mut self.rng)
                .with_density(self.river_density)
                .build()
                .map_err(|e| WorldGenerationError {
                    error_message: format!("Failed to generate rivers: {}", e),
                    error_type: WorldGenerationErrorType::GenerationFailed,
                })?;
        let actual_rivers = river_system.river_tiles.len();
        let river_time = river_timer.complete_with_context(format!("{} rivers generated", actual_rivers));
        log_world_gen_step("River Generation", actual_rivers, river_time);

        // Step 6: Calculate agriculture values
        report_progress(&format!("Calculating agriculture values for {} provinces...", provinces.len()), 0.6);

        let agriculture_timer = TimedOperation::start("Agriculture Calculation");
        crate::world::calculate_agriculture_values(&mut provinces, &river_system, self.dimensions)
            .map_err(|e| WorldGenerationError {
                error_message: format!("Failed to calculate agriculture: {}", e),
                error_type: WorldGenerationErrorType::GenerationFailed,
            })?;
        let agriculture_time = agriculture_timer.complete();
        log_world_gen_step("Agriculture Calculation", provinces.len(), agriculture_time);

        // Step 7: Generate cloud system and finalize
        let cloud_count = 90; // Default cloud count
        report_progress(&format!("Generating {} procedural clouds and finalizing world...", cloud_count), 0.75);

        let cloud_timer = TimedOperation::start("Cloud Generation");
        let cloud_system = crate::world::CloudBuilder::new(&mut self.rng, &self.dimensions).build();
        let cloud_time = cloud_timer.complete_with_context(format!("{} clouds", cloud_system.clouds.len()));
        log_world_gen_step("Cloud Generation", cloud_system.clouds.len(), cloud_time);

        // Add a small delay to ensure the UI can update
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Step 8: Complete!
        report_progress("World generation complete!", 0.95);

        let total_time_ms = total_timer.complete();

        // Log summary statistics
        info!("World Generation Summary:");
        info!("  Total provinces: {}", provinces.len());
        info!("  Ocean provinces: {}", ocean_count);
        info!("  Land provinces: {}", provinces.len() - ocean_count);
        info!("  Rivers generated: {}", actual_rivers);
        info!("  Clouds generated: {}", cloud_system.clouds.len());
        info!("  Total time: {:.2}ms", total_time_ms);

        // Log memory usage summary
        let world_memory = provinces.len() * std::mem::size_of::<crate::world::Province>()
            + river_system.river_tiles.len() * std::mem::size_of::<u32>()
            + cloud_system.clouds.len() * std::mem::size_of::<crate::world::CloudData>();
        log_memory_usage("Total World Data", world_memory);

        Ok(World {
            provinces,
            rivers: river_system,
            clouds: cloud_system,
            climate_storage,
            infrastructure_storage: super::super::InfrastructureStorage::new(),
            seed: self.seed,
        })
    }
}
