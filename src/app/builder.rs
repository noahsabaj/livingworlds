//! Application Builder - Core Bevy App Construction
//!
//! This module contains the main application building logic for Living Worlds,
//! handling the construction of the complete Bevy application with all required
//! plugins and configuration.

use bevy::prelude::*;
use bevy_pkv::PkvStore;

// Import configuration types from the config module
use crate::config::AppConfig;

// Import all plugins consistently

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

/// Builds the core Bevy application with all Living Worlds plugins.
///
/// This sets up the engine, window, and all game systems but doesn't
/// include game-specific resources or startup systems.
///
/// # Plugin Initialization
///
/// All game plugins are registered through the declarative GamePlugins aggregator,
/// which uses bevy-plugin-builder to ensure proper dependency order and compile-time
/// validation. See GamePlugins in plugins.rs for the complete plugin list and ordering.
///
/// # Errors
///
/// Returns `AppBuildError` if:
/// - Storage initialization fails
/// - Critical plugin initialization fails
pub fn build_app() -> Result<App, AppBuildError> {
    build_app_with_config(AppConfig::default())
}

/// Builds the app with custom configuration
pub fn build_app_with_config(config: AppConfig) -> Result<App, AppBuildError> {
    let mut app = App::new();

    // Initialize Steam integration (delegate to initialization module)
    initialization::setup_steam_integration(&mut app);

    // Configure Bevy's default plugins with our settings
    let default_plugins = initialization::configure_default_plugins(&config);
    app.add_plugins(default_plugins);

    // Add diagnostics if enabled (delegate to initialization module)
    initialization::setup_diagnostics(&mut app, &config);

    // Initialize storage - PkvStore::new returns PkvStore directly, not Result
    app.insert_resource(PkvStore::new("LivingWorlds", "LivingWorlds"));

    // Add all Living Worlds game plugins using revolutionary aggregation!
    // GamePlugins handles ALL plugin registration including conditional debug plugins
    info!("Initializing game plugins with declarative automation");
    app.add_plugins(GamePlugins);

    Ok(app)
}
