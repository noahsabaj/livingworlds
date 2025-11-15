//! App Initialization - Bevy Core Plugin Configuration
//!
//! This module handles the low-level initialization of Bevy's core systems.
//! It contains pure Bevy framework configuration without game-specific logic.

use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::config::AppConfig;

/// Configure Bevy's default plugins with Living Worlds settings
///
/// This sets up the window configuration and conditionally disables
/// audio based on the application configuration.
pub fn configure_default_plugins(config: &AppConfig) -> impl PluginGroup {
    let mut default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: config.window.title.clone(),
            resolution: WindowResolution::new(config.window.width as u32, config.window.height as u32),
            resizable: config.window.resizable,
            ..default()
        }),
        ..default()
    });

    // Conditionally disable audio based on configuration
    if config.enable_audio {
        info!("Audio enabled");
    } else {
        info!("Audio disabled in configuration");
        default_plugins = default_plugins.disable::<AudioPlugin>();
    }

    default_plugins
}
