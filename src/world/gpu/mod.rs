//! GPU Compute acceleration for world generation - Gateway Module
//!
//! This module provides GPU-accelerated terrain generation using compute shaders.
//! It replaces CPU-based noise generation and erosion with massively parallel GPU computation,
//! achieving 10-20x speedups for large worlds.

// Private submodules
mod benchmark;
mod buffers;
mod capabilities;
mod coordinator;
mod integration;
mod node;
mod plugin;
mod resources;
mod types;
mod validation;
mod world_generation;

// Public exports - Plugin
pub use plugin::NoiseComputePlugin;

// Public exports - Resources and settings
pub use resources::{
    ComputeMetrics, ErosionComputeSettings,
    NoiseComputeSettings, GpuGenerationRequest,
};

// Public exports - Types
pub use types::{ComputeLabel, ComputeMode, GpuComputeStatus, GpuResources};

// Public exports - Capabilities
pub use capabilities::check_gpu_compute_support;

// Public exports - Buffer types
pub use buffers::{
    get_completed_elevations,
    request_elevation_readback, upload_province_positions, GpuElevationData,
    GpuErosionParams, GpuNoiseParams, GpuProvinceData,
};

// Public exports - Coordination
pub use coordinator::{
    get_gpu_elevation_results, request_gpu_elevation_generation,
    GpuGenerationConfig, GpuGenerationState, GpuPerformanceMetrics,
};

// Public exports - Integration
pub use integration::{
    extract_province_positions, gpu_accelerated_province_generation, GpuElevationGenerator,
};

// Public exports - Validation
pub use validation::{
    validate_gpu_cpu_elevation_generation,
    ValidationConfig, ValidationResult,
};

// Public exports - Benchmarking

// Public exports - World Generation
pub use world_generation::GpuProvinceBuilder;
