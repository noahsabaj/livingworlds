//! Diagnostics Module - Performance Monitoring and FPS Display
//!
//! This module provides diagnostic capabilities for Living Worlds,
//! following the gateway architecture pattern. It handles:
//! - FPS display and monitoring systems
//! - Performance metrics collection and reporting
//! - Diagnostic plugin registration and management
//! - Integration with Bevy's diagnostic systems
//!
//! # Gateway Architecture
//! This module controls access to diagnostic components through a clean API.
//! Internal implementation details are private, and external code can only
//! access functionality through the exported interfaces.
//!
//! # Usage
//! ```rust
//! use living_worlds::diagnostics::{DiagnosticsPlugin, display_fps};
//!
//! // Add diagnostics plugin to app
//! app.add_plugins(DiagnosticsPlugin);
//!
//! // Use FPS display system
//! app.add_systems(Update, display_fps.run_if(resource_exists::<DiagnosticsConfig>));
//! ```

// Private module declarations - implementation details hidden from external code
mod error_context;
mod fps;
mod plugin;
mod types;

// Public exports - controlled API surface following gateway pattern
pub use error_context::{
    capture_performance_metrics, collect_generation_metrics, ErrorContext, ErrorType,
    GenerationMetrics, GpuStatus, PerformanceMetrics,
};
pub use fps::display_fps;
pub use plugin::DiagnosticsPlugin;
