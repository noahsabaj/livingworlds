//! GPU-CPU Coordination System
//!
//! This module orchestrates the complex async communication between CPU world
//! generation and GPU compute, managing the state transitions and data flow.

use super::{
    get_completed_elevations, request_elevation_readback,
    resources::{ComputeBufferHandles, NoiseComputeSettings},
    upload_province_positions, GpuComputeStatus, GpuElevationData,
};
use crate::world::provinces::Province;
use bevy::prelude::*;

/// Represents the current state of GPU world generation
#[derive(Debug, Clone, PartialEq, Resource)]
pub enum GpuGenerationState {
    /// GPU compute not available - using CPU fallback
    CpuFallback,
    /// Ready to start GPU generation
    Ready,
    /// Uploading province positions to GPU
    UploadingPositions,
    /// GPU compute shader executing
    Computing,
    /// Waiting for GPU results to be ready
    AwaitingResults,
    /// Reading back computed data from GPU
    ReadingBack,
    /// GPU generation complete - data available
    Complete(Vec<f32>),
    /// GPU generation failed - fallback to CPU
    Failed(String),
}

impl Default for GpuGenerationState {
    fn default() -> Self {
        Self::Ready
    }
}

/// Configuration for GPU world generation
#[derive(Resource, Debug, Clone)]
pub struct GpuGenerationConfig {
    pub use_gpu: bool,
    pub fallback_on_failure: bool,
    pub timeout_seconds: f32,
    pub max_retries: u32,
}

impl Default for GpuGenerationConfig {
    fn default() -> Self {
        Self {
            use_gpu: true,
            fallback_on_failure: true,
            timeout_seconds: 30.0,
            max_retries: 3,
        }
    }
}

/// Tracks timing and performance metrics for GPU operations
#[derive(Resource, Debug, Default)]
pub struct GpuPerformanceMetrics {
    pub last_generation_time: Option<std::time::Duration>,
    pub total_operations: u32,
    pub successful_operations: u32,
    pub gpu_speedup_factor: Option<f32>,
}

/// System to coordinate GPU world generation - main orchestrator
pub fn coordinate_gpu_generation(
    mut state: ResMut<GpuGenerationState>,
    gpu_status: Res<GpuComputeStatus>,
    config: Res<GpuGenerationConfig>,
    elevation_data: Res<GpuElevationData>,
    mut metrics: ResMut<GpuPerformanceMetrics>,
    time: Res<Time>,
) {
    // Check if GPU compute is available
    if !gpu_status.compute_supported || !config.use_gpu {
        if *state != GpuGenerationState::CpuFallback {
            info!("GPU compute not available - using CPU fallback");
            *state = GpuGenerationState::CpuFallback;
        }
        return;
    }

    match state.as_ref() {
        GpuGenerationState::Ready => {
            // Ready to start - this will be triggered by world generation
            debug!("GPU generation system ready");
        }

        GpuGenerationState::UploadingPositions => {
            // Upload is handled by separate system, transition to computing
            *state = GpuGenerationState::Computing;
            info!("Transitioning to GPU compute phase");
        }

        GpuGenerationState::Computing => {
            // GPU shaders are executing, transition to waiting for results
            *state = GpuGenerationState::AwaitingResults;
            info!("GPU compute dispatched, awaiting results");
        }

        GpuGenerationState::AwaitingResults => {
            // Transition to reading back state
            // The actual readback request happens in the render app
            *state = GpuGenerationState::ReadingBack;
            info!("Waiting for GPU readback");
        }

        GpuGenerationState::ReadingBack => {
            // Check if results are available from the shared resource
            if elevation_data.ready && elevation_data.elevations.is_some() {
                let elevations = elevation_data.elevations.as_ref().unwrap().clone();
                let elevation_count = elevations.len();
                *state = GpuGenerationState::Complete(elevations);
                metrics.successful_operations += 1;
                info!(
                    "GPU generation completed successfully with {} elevations",
                    elevation_count
                );
            }
        }

        GpuGenerationState::Complete(_) => {
            // Results available - world generation can consume them
            debug!("GPU results ready for consumption");
        }

        GpuGenerationState::Failed(error) => {
            warn!("GPU generation failed: {}", error);
            if config.fallback_on_failure {
                *state = GpuGenerationState::CpuFallback;
                info!("Falling back to CPU generation");
            }
        }

        GpuGenerationState::CpuFallback => {
            // Using CPU - no GPU coordination needed
        }
    }

    metrics.total_operations += 1;
}

/// System to initiate GPU elevation generation for a set of provinces
pub fn request_gpu_elevation_generation(
    provinces: Vec<Vec2>,
    mut state: ResMut<GpuGenerationState>,
    gpu_status: Res<GpuComputeStatus>,
    config: Res<GpuGenerationConfig>,
) -> bool {
    // Check if GPU generation is possible
    if !gpu_status.compute_supported || !config.use_gpu {
        return false;
    }

    if *state != GpuGenerationState::Ready {
        warn!("GPU generation not ready, current state: {:?}", *state);
        return false;
    }

    // Store province positions for upload
    *state = GpuGenerationState::UploadingPositions;
    info!(
        "Starting GPU elevation generation for {} provinces",
        provinces.len()
    );

    // The actual upload will be handled by render world systems
    true
}

/// System to get completed GPU elevation data
pub fn get_gpu_elevation_results(mut state: ResMut<GpuGenerationState>) -> Option<Vec<f32>> {
    match state.as_ref() {
        GpuGenerationState::Complete(elevations) => {
            let results = elevations.clone();
            *state = GpuGenerationState::Ready; // Reset for next generation
            Some(results)
        }
        _ => None,
    }
}

/// System to handle GPU generation failures and implement retry logic
pub fn handle_gpu_failures(
    mut state: ResMut<GpuGenerationState>,
    config: Res<GpuGenerationConfig>,
    mut retry_count: Local<u32>,
) {
    match state.as_ref() {
        GpuGenerationState::Failed(_) => {
            if *retry_count < config.max_retries {
                *retry_count += 1;
                warn!(
                    "GPU generation failed, retrying ({}/{})",
                    *retry_count, config.max_retries
                );
                *state = GpuGenerationState::Ready;
            } else {
                error!(
                    "GPU generation failed after {} retries, falling back to CPU",
                    config.max_retries
                );
                *state = GpuGenerationState::CpuFallback;
                *retry_count = 0;
            }
        }
        GpuGenerationState::Complete(_) => {
            // Reset retry count on success
            *retry_count = 0;
        }
        _ => {}
    }
}

/// System to monitor GPU generation timeouts
pub fn monitor_gpu_timeouts(
    mut state: ResMut<GpuGenerationState>,
    config: Res<GpuGenerationConfig>,
    time: Res<Time>,
    mut timeout_timer: Local<Option<std::time::Instant>>,
) {
    match state.as_ref() {
        GpuGenerationState::Computing
        | GpuGenerationState::AwaitingResults
        | GpuGenerationState::ReadingBack => {
            // Start timeout timer if not already started
            if timeout_timer.is_none() {
                *timeout_timer = Some(std::time::Instant::now());
            }

            // Check if timeout exceeded
            if let Some(start_time) = *timeout_timer {
                let elapsed = start_time.elapsed();
                if elapsed.as_secs_f32() > config.timeout_seconds {
                    error!(
                        "GPU generation timed out after {:.2}s",
                        elapsed.as_secs_f32()
                    );
                    *state = GpuGenerationState::Failed("Timeout".to_string());
                    *timeout_timer = None;
                }
            }
        }
        _ => {
            // Reset timeout timer for other states
            *timeout_timer = None;
        }
    }
}
