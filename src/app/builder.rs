//! Application Builder

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::config::{AppConfig, DiagnosticsConfig};

// Import from sibling modules
use super::initialization;
use super::plugins::GamePlugins;

// === Constants ===
/// Application name used for storage and identification
const APP_NAME: &str = "LivingWorlds";

// === Error Types ===
/// Errors that can occur during app building
#[derive(Debug, thiserror::Error)]
pub enum AppBuildError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to initialize storage: {0}")]
    StorageInit(String),

    #[error("Failed to initialize plugin: {0}")]
    PluginInit(String),
}

pub fn build_app() -> Result<App, AppBuildError> {
    build_app_with_config(AppConfig::default())
}

/// Validates application configuration
fn validate_config(config: &AppConfig) -> Result<(), AppBuildError> {
    // Validate window dimensions
    const MIN_WIDTH: f32 = 640.0;
    const MAX_WIDTH: f32 = 7680.0;
    const MIN_HEIGHT: f32 = 480.0;
    const MAX_HEIGHT: f32 = 4320.0;

    if config.window.width < MIN_WIDTH || config.window.width > MAX_WIDTH {
        return Err(AppBuildError::InvalidConfig(format!(
            "Window width {} is outside valid range {}-{}",
            config.window.width, MIN_WIDTH, MAX_WIDTH
        )));
    }

    if config.window.height < MIN_HEIGHT || config.window.height > MAX_HEIGHT {
        return Err(AppBuildError::InvalidConfig(format!(
            "Window height {} is outside valid range {}-{}",
            config.window.height, MIN_HEIGHT, MAX_HEIGHT
        )));
    }

    Ok(())
}

/// Builds the app with custom configuration
pub fn build_app_with_config(config: AppConfig) -> Result<App, AppBuildError> {
    // Validate configuration first
    validate_config(&config)?;
    let mut app = App::new();

    // NOTE: StateScoped automatic cleanup is enabled by default in Bevy 0.17+
    // Entities marked with DespawnOnExit(GameState) are automatically despawned
    // when exiting that state, eliminating the need for manual cleanup systems

    // Initialize Steam integration
    #[cfg(feature = "steam")]
    {
        info!("Steam integration enabled");
        app.add_plugins(crate::steam::SteamPlugin);
    }

    #[cfg(not(feature = "steam"))]
    {
        debug!("Steam integration disabled (feature not enabled)");
    }

    // Configure Bevy's default plugins with our settings
    let default_plugins = initialization::configure_default_plugins(&config);
    app.add_plugins(default_plugins);

    // Setup diagnostics if enabled
    if config.diagnostics.show_fps {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(config.diagnostics.clone())
            .add_systems(
                Update,
                crate::diagnostics::display_fps.run_if(resource_exists::<DiagnosticsConfig>),
            );
        info!("FPS diagnostics enabled");
    } else {
        debug!("Diagnostics disabled in configuration");
    }

    // Initialize storage
    app.insert_resource(PkvStore::new(APP_NAME, APP_NAME));

    // Add all Living Worlds game plugins using aggregation
    // GamePlugins handles ALL plugin registration including conditional debug plugins
    app.add_plugins(GamePlugins);

    Ok(app)
}
