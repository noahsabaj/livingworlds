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
    ComputeBufferHandles, ComputeMetrics, ComputeState, ErosionComputeSettings,
    NoiseComputeSettings,
};

// Public exports - Types
pub use types::{ComputeLabel, ComputeMode, GpuComputeStatus, GpuResources};

// Public exports - Capabilities
pub use capabilities::check_gpu_compute_support;

// Public exports - Buffer types
pub use buffers::{
    get_completed_elevations, init_compute_buffers, process_gpu_readbacks,
    request_elevation_readback, upload_province_positions, ComputeBindGroups, GpuElevationData,
    GpuErosionParams, GpuNoiseParams, GpuProvinceData, GpuReadbackManager, PendingReadback,
};

// Public exports - Coordination
pub use coordinator::{
    coordinate_gpu_generation, get_gpu_elevation_results, handle_gpu_failures,
    monitor_gpu_timeouts, request_gpu_elevation_generation, GpuGenerationConfig,
    GpuGenerationState, GpuPerformanceMetrics,
};

// Public exports - Integration
pub use integration::{
    extract_province_positions, gpu_accelerated_province_generation, GpuElevationGenerator,
};

// Public exports - Validation
pub use validation::{
    periodic_validation_system, run_validation_test, validate_gpu_cpu_elevation_generation,
    ValidationConfig, ValidationHistory, ValidationMismatch, ValidationResult,
};

// Public exports - Benchmarking
pub use benchmark::{
    benchmark_progress_system, run_gpu_benchmark, BenchmarkConfig, BenchmarkResults, BenchmarkRun,
    BenchmarkState,
};

// Public exports - World Generation
pub use world_generation::{gpu_province_generation_system, GpuProvinceBuilder};
