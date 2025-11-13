//! App Initialization - Steam Integration and Core Plugin Configuration
//!
//! This module handles the low-level initialization of Bevy's core systems
//! including Steam integration, audio configuration, and diagnostics setup.

use bevy::audio::AudioPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::*;
use bevy::window::WindowResolution;

// Import configuration types from the config module
use crate::config::{AppConfig, DiagnosticsConfig};

// Import diagnostics from the diagnostics module
use crate::diagnostics::display_fps;

/// Setup Steam integration if the steam feature is enabled
///
/// This must be called before adding DefaultPlugins to ensure proper
/// Steam integration initialization order.
pub fn setup_steam_integration(app: &mut App) {
    #[cfg(feature = "steam")]
    {
        info!("Initializing Steam integration");
        app.add_plugins(crate::steam::SteamPlugin);
    }

    #[cfg(not(feature = "steam"))]
    {
        trace!("Steam integration disabled (feature not enabled)");
    }
}

/// Configure Bevy's default plugins with Living Worlds settings
///
/// This sets up the window configuration and conditionally disables
/// audio based on the application configuration.
pub fn configure_default_plugins(config: &AppConfig) -> impl PluginGroup + use<> {
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
    if !config.enable_audio {
        info!("Audio disabled in configuration");
        default_plugins = default_plugins.disable::<AudioPlugin>();
    }

    default_plugins
}

/// Setup diagnostics systems if enabled in configuration
///
/// This adds FPS monitoring and other diagnostic capabilities
/// based on the configuration settings.
pub fn setup_diagnostics(app: &mut App, config: &AppConfig) {
    if config.diagnostics.show_fps {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(config.diagnostics.clone())
            .add_systems(
                Update,
                display_fps.run_if(resource_exists::<DiagnosticsConfig>),
            );

        info!("Diagnostics enabled - FPS monitoring active");
    } else {
        trace!("Diagnostics disabled in configuration");
    }
}

/// Initialize audio configuration
///
/// Handles audio-specific initialization beyond the basic plugin setup.
/// Currently this is minimal but provides a hook for future audio configuration.
pub fn setup_audio_configuration(_config: &AppConfig) {
    // Future: Audio device configuration, volume settings, etc.
    debug!("Audio configuration completed");
}
