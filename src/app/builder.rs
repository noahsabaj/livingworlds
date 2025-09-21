//! Application Builder - Core Bevy App Construction
//!
//! This module contains the main application building logic for Living Worlds,
//! handling the construction of the complete Bevy application with all required
//! plugins and configuration.

use bevy::audio::AudioPlugin;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_pkv::PkvStore;

// Import configuration types from the config module (once it's created)
use crate::{AppConfig, DiagnosticsConfig};

// Import all plugins consistently
use crate::{
    camera::CameraPlugin,
    loading::LoadingScreenPlugin,
    menus::MenusPlugin,
    modding::ModdingPlugin,
    nations::NationPlugin,
    relationships::RelationshipsPlugin,
    save_load::SaveLoadPlugin,
    settings::SettingsUIPlugin,
    simulation::SimulationPlugin,
    states::StatesPlugin,
    ui::UIPlugin,
    // World module plugins
    world::{
        BorderPlugin, CloudPlugin, NoiseComputePlugin, OverlayPlugin, ProvinceEventsPlugin,
        TerrainPlugin, WorldConfigPlugin,
    },
};

// Import from sibling modules
use super::initialization;
use super::plugin_order;

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
/// # Plugin Initialization Order
///
/// The plugins are initialized in a specific order to ensure proper dependencies:
/// 1. **Steam** (if enabled) - Must be before rendering plugins
/// 2. **States** - Core state management, required by most other plugins
/// 3. **Modding** - Loads mod configurations early
/// 4. **Province Events** - Event system for province changes
/// 5. **Menus** - UI menus (depends on States)
/// 6. **World Config** - World generation configuration UI
/// 7. **Loading Screen** - Unified loading UI
/// 8. **Settings** - Game settings management
/// 9. **Simulation & Game Systems** - Core gameplay plugins
/// 10. **UI & Camera** - User interface and camera controls
/// 11. **Borders** - GPU-instanced border rendering (visual layer)
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

    // Add all Living Worlds game plugins in dependency order
    info!("Initializing game plugins");
    plugin_order::register_all_plugins(&mut app);

    // Performance monitoring (conditional - only in debug builds)
    #[cfg(debug_assertions)]
    app.add_plugins(crate::performance::PerformanceMonitoringPlugin);

    // Parallel safety validation (conditional - only in debug builds)
    #[cfg(debug_assertions)]
    app.add_plugins(crate::safety::ParallelSafetyPlugin);

    Ok(app)
}
