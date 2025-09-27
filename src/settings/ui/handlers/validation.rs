//! Settings Validation and Application Handlers
//!
//! Focused handlers for settings validation and applying changes to the game.

use crate::settings::types::*;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PresentMode, PrimaryWindow, VideoModeSelection, WindowMode};

/// Apply settings changes to the actual game systems (window, audio, etc.)
pub fn apply_settings_changes(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }

    info!("Applying settings changes");

    // Apply graphics settings
    if let Ok(mut window) = windows.single_mut() {
        // Apply window mode
        window.mode = match settings.graphics.window_mode {
            WindowModeOption::Windowed => WindowMode::Windowed,
            WindowModeOption::Borderless => {
                WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            }
            WindowModeOption::Fullscreen => {
                WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
            }
        };

        // Apply resolution (only in windowed mode)
        if matches!(window.mode, WindowMode::Windowed) {
            window.resolution.set(
                settings.graphics.resolution.width,
                settings.graphics.resolution.height,
            );
        }

        // Apply VSync
        window.present_mode = if settings.graphics.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
    }

    // Apply audio settings

    // Log the audio settings for now
    info!(
        "  Master Volume: {:.0}%",
        settings.audio.master_volume * 100.0
    );
    info!("  SFX Volume: {:.0}%", settings.audio.sfx_volume * 100.0);
}

/// Validate settings to ensure they're within hardware capabilities
pub fn validate_settings(
    mut temp_settings: ResMut<TempGameSettings>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let monitor_width = window.width();
    let monitor_height = window.height();

    // Clamp resolution to monitor size
    if temp_settings.0.graphics.resolution.width > monitor_width {
        temp_settings.0.graphics.resolution.width = monitor_width;
    }
    if temp_settings.0.graphics.resolution.height > monitor_height {
        temp_settings.0.graphics.resolution.height = monitor_height;
    }

    // Ensure minimum resolution
    if temp_settings.0.graphics.resolution.width < 800.0 {
        temp_settings.0.graphics.resolution.width = 800.0;
    }
    if temp_settings.0.graphics.resolution.height < 600.0 {
        temp_settings.0.graphics.resolution.height = 600.0;
    }

    // Clamp all values to sensible ranges
    temp_settings.0.graphics.render_scale = temp_settings.0.graphics.render_scale.clamp(0.5, 2.0);
    temp_settings.0.audio.master_volume = temp_settings.0.audio.master_volume.clamp(0.0, 1.0);
    temp_settings.0.audio.sfx_volume = temp_settings.0.audio.sfx_volume.clamp(0.0, 1.0);
    temp_settings.0.interface.ui_scale = temp_settings.0.interface.ui_scale.clamp(0.75, 2.0);
    temp_settings.0.controls.camera_speed = temp_settings.0.controls.camera_speed.clamp(0.1, 5.0);
    temp_settings.0.controls.zoom_speed = temp_settings.0.controls.zoom_speed.clamp(0.1, 5.0);
}
