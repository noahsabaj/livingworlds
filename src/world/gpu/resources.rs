//! GPU Compute Resources and Settings
//!
//! This module defines the resources used for GPU compute configuration and state management.

use super::types::ComputeMode;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_resource::Buffer;

/// Settings for GPU-accelerated noise generation
#[derive(Resource, Clone, ExtractResource, Reflect)]
#[reflect(Resource)]
pub struct NoiseComputeSettings {
    /// Whether to use GPU acceleration
    pub enabled: bool,
    /// Compute mode (GPU or CPU fallback)
    pub mode: ComputeMode,
    /// Workgroup size for compute shader (must be multiple of 32, max 256)
    pub workgroup_size: u32,
    /// Batch size for processing provinces
    pub batch_size: u32,
    /// Noise generation seed
    pub seed: u32,
    /// Number of octaves for fractal noise
    pub octaves: u32,
    /// Base frequency for noise
    pub frequency: f32,
    /// Persistence (amplitude decay per octave)
    pub persistence: f32,
    /// Lacunarity (frequency increase per octave)
    pub lacunarity: f32,
    /// Overall amplitude multiplier
    pub amplitude: f32,
    /// World scale factor
    pub scale: f32,
    /// X offset for noise sampling
    pub offset_x: f32,
    /// Y offset for noise sampling
    pub offset_y: f32,
}

impl Default for NoiseComputeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: ComputeMode::Gpu,
            workgroup_size: 256,
            batch_size: 1_000_000, // Process 1M provinces at a time
            seed: 42,
            octaves: 8,
            frequency: 0.01,
            persistence: 0.5,
            lacunarity: 2.0,
            amplitude: 1.0,
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Settings for GPU-accelerated erosion simulation
#[derive(Resource, Clone, ExtractResource, Reflect)]
#[reflect(Resource)]
pub struct ErosionComputeSettings {
    /// Whether to use GPU acceleration for erosion
    pub enabled: bool,
    /// Compute mode (GPU or CPU fallback)
    pub mode: ComputeMode,
    /// Workgroup size for erosion compute shader
    pub workgroup_size: u32,
    /// Number of water droplets to simulate
    pub num_droplets: u32,
    /// Droplets to process per dispatch
    pub droplets_per_batch: u32,
    /// Erosion simulation seed
    pub seed: u32,
    /// Initial water volume per droplet
    pub initial_water: f32,
    /// Water evaporation rate
    pub evaporation_rate: f32,
    /// Sediment carrying capacity
    pub sediment_capacity: f32,
    /// Minimum slope for erosion
    pub min_slope: f32,
    /// Erosion rate
    pub erosion_rate: f32,
    /// Deposition rate
    pub deposition_rate: f32,
    /// Gravity constant
    pub gravity: f32,
    /// Inertia factor
    pub inertia: f32,
    /// Maximum droplet lifetime
    pub max_lifetime: u32,
    /// Thermal erosion angle threshold
    pub thermal_angle_threshold: f32,
    /// Thermal erosion rate
    pub thermal_rate: f32,
}

impl Default for ErosionComputeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: ComputeMode::Gpu,
            workgroup_size: 64,
            num_droplets: 500_000,
            droplets_per_batch: 10_000,
            seed: 12345,
            initial_water: 1.0,
            evaporation_rate: 0.01,
            sediment_capacity: 4.0,
            min_slope: 0.01,
            erosion_rate: 0.3,
            deposition_rate: 0.3,
            gravity: 4.0,
            inertia: 0.3,
            max_lifetime: 100,
            thermal_angle_threshold: 0.6,
            thermal_rate: 0.1,
        }
    }
}

/// GPU buffer handles for compute operations
#[derive(Resource, Default)]
pub struct ComputeBufferHandles {
    pub positions_buffer: Option<Buffer>,
    pub elevations_buffer: Option<Buffer>,
    pub heightmap_buffer: Option<Buffer>,
    pub droplet_starts_buffer: Option<Buffer>,
    pub noise_params_buffer: Option<Buffer>,
    pub erosion_params_buffer: Option<Buffer>,
}

/// State tracking for GPU compute operations
#[derive(Resource, Default)]
pub struct ComputeState {
    pub noise_generated: bool,
    pub erosion_complete: bool,
    pub data_on_gpu: bool,
    pub readback_pending: bool,
    pub current_batch: u32,
    pub total_batches: u32,
}

/// Performance metrics for GPU compute
#[derive(Resource, Default, Debug)]
pub struct ComputeMetrics {
    pub noise_generation_time: f32,
    pub erosion_time: f32,
    pub readback_time: f32,
    pub total_gpu_time: f32,
    pub provinces_processed: u32,
    pub droplets_simulated: u32,
}
