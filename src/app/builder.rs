//! Application Builder

use bevy::prelude::*;
// TEMPORARILY DISABLED: bevy_pkv doesn't support Bevy 0.17 yet
// use bevy_pkv::PkvStore;

use crate::config::AppConfig;
use crate::states::GameState;

// Import from sibling modules
use super::initialization;
use super::plugins::GamePlugins;

// === Error Types ===
/// Errors that can occur during app building
#[derive(Debug, thiserror::Error)]
pub enum AppBuildError {
    #[error("Failed to initialize storage: {0}")]
    StorageInit(String),

    #[error("Failed to initialize plugin: {0}")]
    PluginInit(String),
}

pub fn build_app() -> Result<App, AppBuildError> {
    build_app_with_config(AppConfig::default())
}

/// Builds the app with custom configuration
pub fn build_app_with_config(config: AppConfig) -> Result<App, AppBuildError> {
    let mut app = App::new();

    // NOTE: StateScoped automatic cleanup is enabled by default in Bevy 0.17+
    // Entities marked with DespawnOnExit(GameState) are automatically despawned
    // when exiting that state, eliminating the need for manual cleanup systems

    // Initialize Steam integration (delegate to initialization module)
    initialization::setup_steam_integration(&mut app);

    // Configure Bevy's default plugins with our settings
    let default_plugins = initialization::configure_default_plugins(&config);
    app.add_plugins(default_plugins);

    // Add diagnostics if enabled (delegate to initialization module)
    initialization::setup_diagnostics(&mut app, &config);

    // TEMPORARILY DISABLED: bevy_pkv doesn't support Bevy 0.17 yet
    // Settings persistence will be restored when bevy_pkv 0.14+ is released
    // Initialize storage - PkvStore::new returns PkvStore directly, not Result
    // app.insert_resource(PkvStore::new("LivingWorlds", "LivingWorlds"));

    // Add all Living Worlds game plugins using aggregation!
    // GamePlugins handles ALL plugin registration including conditional debug plugins
    info!("Initializing game plugins with declarative automation");
    app.add_plugins(GamePlugins);

    Ok(app)
}
