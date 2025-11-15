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
            enable_audio: false,
        }
    }
}
