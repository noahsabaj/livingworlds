//! Async world generation
//!
//! Background world generation that runs on AsyncComputeTaskPool.

use async_channel::Sender;
use bevy::log::{error, info};

use super::gpu::generate_world_with_gpu_acceleration;
use super::progress::GenerationProgress;
use super::validation::validate_settings;
use super::super::{WorldBuilder, WorldGenerationSettings};

/// Background world generation function - runs on AsyncComputeTaskPool
///
/// This function orchestrates world generation on a background thread,
/// sending progress updates through the provided channel.
pub async fn generate_world_async(
    settings: WorldGenerationSettings,
    progress_sender: Sender<GenerationProgress>,
    gpu_resources: Option<crate::world::gpu::GpuResources>,
) {
    info!(
        "Starting async world generation with settings: {:?}",
        settings
    );

    // Helper to send progress updates
    let send_progress = |step: &str, progress: f32| {
        info!("Async task: Sending progress update: {} - {:.1}%", step, progress * 100.0);
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

    // Validate settings
    send_progress("Validating settings...", 0.0);
    if let Err(e) = validate_settings(&settings) {
        error!("World generation validation failed: {}", e);
        let _ = progress_sender.try_send(GenerationProgress {
            step: format!("Error: {}", e),
            progress: 0.0,
            completed: true,
            world_data: None,
            error_message: Some(e.to_string()),
            generation_metrics: None,
        });
        return;
    }

    // Generate world data with progress reporting
    let start_time = std::time::Instant::now();

    // Create a progress callback closure that sends updates through the channel
    let progress_callback = |step: &str, progress: f32| {
        send_progress(step, progress);
    };

    // Choose between GPU-accelerated and CPU-only generation
    let world_result = if let Some(gpu_res) = gpu_resources.as_ref() {
        if gpu_res.compute_supported && gpu_res.use_gpu {
            info!("Using GPU-accelerated world generation");
            generate_world_with_gpu_acceleration(
                settings.clone(),
                gpu_res.clone(),
                progress_sender.clone(),
            )
        } else {
            info!("GPU available but disabled - using CPU generation");
            WorldBuilder::new(
                settings.seed,
                settings.world_size,
                settings.continent_count,
                settings.ocean_coverage,
                settings.river_density,
                settings.climate_type,
            )
            .build_with_progress(Some(progress_callback))
        }
    } else {
        info!("GPU not available - using CPU generation");
        WorldBuilder::new(
            settings.seed,
            settings.world_size,
            settings.continent_count,
            settings.ocean_coverage,
            settings.river_density,
            settings.climate_type,
        )
        .build_with_progress(Some(progress_callback))
    };

    let generation_time = start_time.elapsed().as_millis() as f32;

    // Handle generation result
    match world_result {
        Ok(world) => {
            info!("World generation completed in {:.1}ms", generation_time);
            // Send completion
            let _ = progress_sender.try_send(GenerationProgress {
                step: "World generation completed".to_string(),
                progress: 1.0,
                completed: true,
                world_data: Some(world),
                error_message: None,
                generation_metrics: None,
            });
        }
        Err(e) => {
            error!("World generation failed: {}", e);
            let _ = progress_sender.try_send(GenerationProgress {
                step: format!("Error: {}", e),
                progress: 0.0,
                completed: true,
                world_data: None,
                error_message: Some(e.to_string()),
                generation_metrics: None,
            });
            return;
        }
    }

    info!("Async world generation finished");
}
