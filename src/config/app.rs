//! Application Configuration - Complete App Configuration Management
//!
//! This module provides the main application configuration that combines
//! all other configuration types for a unified configuration interface.

// Import sibling configuration modules
use super::{DiagnosticsConfig, WindowConfig};

/// Complete application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub diagnostics: DiagnosticsConfig,
    pub enable_audio: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            diagnostics: DiagnosticsConfig::default(),
            enable_audio: false, // Permanently disabled - prevents ALSA underrun errors on Linux
        }
    }
}

impl AppConfig {
    /// Create a custom application configuration
    ///
    /// This provides a starting point for customizing the default configuration
    /// while maintaining sensible defaults for unspecified settings.
    pub fn custom() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for development
    ///
    /// Enables diagnostics and debugging features that are useful during development.
    pub fn development() -> Self {
        Self {
            window: WindowConfig::default(),
            diagnostics: DiagnosticsConfig {
                show_fps: true,
                fps_interval: 0.5, // More frequent updates for development
                use_console: true, // Console output for development
            },
            enable_audio: false, // Still disabled for stability
        }
    }

    /// Create a configuration optimized for production/release
    ///
    /// Disables debugging features and optimizes for performance and user experience.
    pub fn production() -> Self {
        Self {
            window: WindowConfig::default(),
            diagnostics: DiagnosticsConfig {
                show_fps: false,
                fps_interval: 1.0,
                use_console: false,
            },
            enable_audio: false, // Disabled until audio issues are resolved
        }
    }
}
