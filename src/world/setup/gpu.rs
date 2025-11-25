//! GPU-accelerated world generation
//!
//! Provides hybrid GPU/CPU world generation for improved performance.

use async_channel::Sender;
use bevy::log::{error, info};
use rand::{rngs::StdRng, SeedableRng};

use super::progress::GenerationProgress;
use super::super::WorldGenerationSettings;
use crate::resources::MapDimensions;
use crate::world::gpu::GpuProvinceBuilder;

/// Generate world with GPU acceleration for province generation
///
/// This is a hybrid approach: GPU for provinces, CPU for everything else.
/// Falls back to CPU if GPU generation fails.
pub fn generate_world_with_gpu_acceleration(
    settings: WorldGenerationSettings,
    gpu_resources: crate::world::gpu::GpuResources,
    progress_sender: Sender<GenerationProgress>,
) -> Result<crate::world::World, crate::world::generation::WorldGenerationError> {
    let dimensions = MapDimensions::from_world_size(&settings.world_size);

    // Helper to send progress updates
    let send_progress = |step: &str, progress: f32| {
        info!("GPU gen: {} - {:.1}%", step, progress * 100.0);
        let result = progress_sender.try_send(GenerationProgress {
            step: step.to_string(),
            progress,
            completed: false,
            world_data: None,
            error_message: None,
            generation_metrics: None,
        });
        if let Err(e) = result {
            error!("Failed to send progress update: {:?}", e);
        }
    };

    // Step 1: Generate provinces with GPU acceleration
    send_progress("Generating provinces with GPU acceleration...", 0.1);
    info!("GPU-accelerating province generation...");

    // Create a simplified GPU status for the async context
    let gpu_status = crate::world::gpu::GpuComputeStatus {
        available: true,
        compute_supported: gpu_resources.compute_supported,
        max_workgroup_size: 256,
        max_buffer_size: 2_147_483_648,
        fallback_reason: None,
    };

    let gpu_config = crate::world::gpu::GpuGenerationConfig {
        use_gpu: gpu_resources.use_gpu,
        fallback_on_failure: true,
        timeout_seconds: 30.0,
        max_retries: 3,
    };

    let mut gpu_state = crate::world::gpu::GpuGenerationState::default();
    let mut gpu_metrics = crate::world::gpu::GpuPerformanceMetrics::default();

    // Use GpuProvinceBuilder for accelerated province generation
    let mut provinces = GpuProvinceBuilder::new(dimensions, settings.seed)
        .with_ocean_coverage(settings.ocean_coverage)
        .with_continent_count(settings.continent_count)
        .with_validation(gpu_resources.validation_enabled)
        .build_with_gpu(
            &gpu_status,
            &gpu_config,
            &mut gpu_state,
            &mut gpu_metrics,
            None, // No validation config in async context
        );

    info!("GPU province generation completed");

    // Step 2-6: Continue with CPU-based processing for other world features
    let mut rng = StdRng::seed_from_u64(settings.seed as u64);

    // Step 2: Apply erosion simulation for realistic terrain
    send_progress("Applying erosion simulation...", 0.2);
    let erosion_iterations = match dimensions.provinces_per_row * dimensions.provinces_per_col {
        n if n < 400_000 => 3_000,
        n if n < 700_000 => 5_000,
        _ => 8_000,
    };
    crate::world::apply_erosion_to_provinces(
        &mut provinces,
        dimensions,
        &mut rng,
        erosion_iterations,
    );

    // Step 3: Calculate ocean depths
    send_progress("Calculating ocean depths...", 0.3);
    crate::world::calculate_ocean_depths(&mut provinces, dimensions);

    // Step 4: Generate climate zones
    send_progress("Generating climate zones...", 0.4);
    let climate_storage = crate::world::apply_climate_to_provinces(
        &mut provinces,
        dimensions,
        settings.climate_type,
    );

    // Step 5: Generate river systems
    send_progress("Creating river systems...", 0.5);
    let river_system = crate::world::RiverBuilder::new(&mut provinces, dimensions, &mut rng)
        .with_density(settings.river_density)
        .build()
        .map_err(|e| crate::world::generation::WorldGenerationError {
            error_message: format!("Failed to generate rivers: {}", e),
            error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
        })?;

    // Step 6: Calculate agriculture values
    send_progress("Calculating agriculture values...", 0.55);
    crate::world::calculate_agriculture_values(&mut provinces, &river_system, dimensions).map_err(
        |e| crate::world::generation::WorldGenerationError {
            error_message: format!("Failed to calculate agriculture: {}", e),
            error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
        },
    )?;

    // Step 7: Generate mineral resources
    send_progress("Generating mineral deposits...", 0.6);
    crate::world::generate_world_minerals(settings.seed, &mut provinces);

    // Step 8: Initialize population
    send_progress("Calculating initial population...", 0.65);
    crate::world::generation::initialize_province_populations(&mut provinces);

    // Step 9: Generate cloud system
    send_progress("Generating clouds...", 0.7);
    let cloud_system = crate::world::CloudBuilder::new(&mut rng, &dimensions).build();

    // Final step
    send_progress("Finalizing world...", 0.9);

    // Return the complete world
    Ok(crate::world::World {
        provinces,
        rivers: river_system,
        clouds: cloud_system,
        climate_storage,
        infrastructure_storage: crate::world::InfrastructureStorage::new(),
        seed: settings.seed,
    })
}
