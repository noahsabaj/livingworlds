//! Type definitions for GPU compute module
//!
//! This file contains all the type definitions and enums used throughout
//! the GPU compute acceleration system.

use bevy::prelude::*;
use bevy::render::render_graph::RenderLabel;

/// Mode for compute operations - GPU accelerated or CPU fallback
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ComputeMode {
    /// Use GPU compute shaders for acceleration
    Gpu,
    /// Fall back to CPU implementation
    Cpu,
}

/// Labels for the compute shader pipeline stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RenderLabel)]
pub enum ComputeLabel {
    /// Noise generation compute pass
    NoiseGeneration,
    /// Erosion simulation compute pass
    ErosionSimulation,
    /// Data readback from GPU to CPU
    DataReadback,
}

/// GPU acceleration status and capabilities
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct GpuComputeStatus {
    /// Whether GPU compute is available
    pub available: bool,
    /// Whether compute shaders are supported
    pub compute_supported: bool,
    /// Maximum workgroup size supported
    pub max_workgroup_size: u32,
    /// Maximum buffer size in bytes
    pub max_buffer_size: u64,
    /// Reason for fallback if not available
    pub fallback_reason: Option<String>,
}

impl Default for GpuComputeStatus {
    fn default() -> Self {
        Self {
            available: true,
            compute_supported: true,
            max_workgroup_size: 256,
            max_buffer_size: 2_147_483_648, // 2GB default
            fallback_reason: None,
        }
    }
}

/// GPU resource snapshot for async tasks
/// This is a lightweight copy of GPU state that can be passed to async tasks
/// without holding onto Bevy resources
#[derive(Debug, Clone)]
pub struct GpuResources {
    pub compute_supported: bool,
    pub use_gpu: bool,
    pub timeout_ms: u64,
    pub validation_enabled: bool,
}
