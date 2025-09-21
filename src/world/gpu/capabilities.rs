//! GPU capability detection and validation
//!
//! This module checks GPU compute capabilities and determines whether
//! GPU acceleration can be used for world generation.

use super::types::GpuComputeStatus;
use bevy::render::{render_resource::WgpuFeatures, renderer::RenderDevice};

/// Check GPU compute capabilities and determine if acceleration is available
pub fn check_gpu_compute_support(device: &RenderDevice) -> GpuComputeStatus {
    let limits = device.limits();

    // Check minimum requirements for our compute shaders
    let min_workgroup_size = 64;
    let min_buffer_size = 268_435_456; // 256MB minimum

    let mut status = GpuComputeStatus {
        available: true,
        compute_supported: true,
        max_workgroup_size: limits.max_compute_invocations_per_workgroup,
        max_buffer_size: limits.max_buffer_size as u64,
        fallback_reason: None,
    };

    // Validate capabilities
    if limits.max_compute_invocations_per_workgroup < min_workgroup_size {
        status.compute_supported = false;
        status.fallback_reason = Some(format!(
            "Insufficient workgroup size: {} < {}",
            limits.max_compute_invocations_per_workgroup, min_workgroup_size
        ));
    }

    if limits.max_buffer_size < min_buffer_size as u64 {
        status.compute_supported = false;
        status.fallback_reason = Some(format!(
            "Insufficient buffer size: {} < {}",
            limits.max_buffer_size, min_buffer_size
        ));
    }

    // Check for required features
    let features = device.features();
    if !features.contains(WgpuFeatures::BUFFER_BINDING_ARRAY) {
        status.compute_supported = false;
        status.fallback_reason = Some("Missing BUFFER_BINDING_ARRAY feature".to_string());
    }

    status
}
