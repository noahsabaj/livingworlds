//! Window Configuration - Display and Window Management Settings
//!
//! This module provides window-specific configuration for Living Worlds,
//! including window dimensions, title, and display properties.

// Import constants that will be needed
use crate::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

/// Configuration for the application window
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            title: format!("Living Worlds - {}", crate::version::version_number()),
            resizable: true,
        }
    }
}
