//! Diagnostics Configuration - Performance Monitoring Settings
//!
//! This module provides diagnostics-specific configuration for Living Worlds,
//! including FPS monitoring, console output, and performance tracking settings.

use bevy::prelude::*;

// Import constants that will be needed
use crate::FPS_DISPLAY_INTERVAL_SECS;

/// Configuration for diagnostics display
#[derive(Debug, Clone, Resource)]
pub struct DiagnosticsConfig {
    pub show_fps: bool,
    pub fps_interval: f32,
    pub use_console: bool,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            show_fps: cfg!(debug_assertions), // Only show in debug builds by default
            fps_interval: FPS_DISPLAY_INTERVAL_SECS,
            use_console: false, // Use UI display by default, not console
        }
    }
}
