//! FPS Display System - Real-time Performance Monitoring
//!
//! This module provides FPS display functionality for Living Worlds,
//! including real-time frame rate monitoring and display systems.

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

// Import configuration types and constants
use crate::{DiagnosticsConfig, MS_PER_SECOND};

/// Display FPS information based on configuration
///
/// This system monitors frame rate and displays it according to the
/// DiagnosticsConfig settings. It can output to console or be used
/// for UI-based display in the future.
///
/// The system respects the fps_interval setting to avoid spamming
/// the output with updates.
pub fn display_fps(
    diagnostics: Res<DiagnosticsStore>,
    config: Res<DiagnosticsConfig>,
    mut last_print: Local<f32>,
    time: Res<Time>,
) {
    if time.elapsed_secs() - *last_print < config.fps_interval {
        return;
    }

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let frame_time_ms = MS_PER_SECOND / value as f32;

            if config.use_console {
                // Console output (only in debug builds or when explicitly enabled)
                info!("FPS: {:.1} | Frame Time: {:.2}ms", value, frame_time_ms);
            } else {
                // In production, this would update a UI element instead
                // This is a placeholder for future UI-based FPS display
                #[cfg(debug_assertions)]
                trace!("FPS: {:.1} | Frame Time: {:.2}ms", value, frame_time_ms);
            }

            *last_print = time.elapsed_secs();
        }
    }
}
