//! Diagnostics Types - Common Types for Performance Monitoring
//!
//! This module provides common types and constants used throughout
//! the diagnostics system for performance monitoring and FPS display.

// Re-export diagnostics configuration from config module for convenience
pub use crate::DiagnosticsConfig;

/// Diagnostic metric identifiers
///
/// These constants identify different performance metrics that can be tracked
/// by the diagnostics system.
pub const FPS_METRIC: &str = "fps";
pub const FRAME_TIME_METRIC: &str = "frame_time";
pub const MEMORY_USAGE_METRIC: &str = "memory_usage";

/// Diagnostic display modes
///
/// Defines how diagnostic information should be displayed to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// No diagnostic display
    None,
    /// Console-based output
    Console,
    /// UI-based overlay display
    Overlay,
    /// Both console and overlay
    Both,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::None
    }
}
