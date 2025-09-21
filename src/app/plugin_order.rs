//! Plugin Order Management - Dependency-Safe Plugin Registration
//!
//! This module manages the registration order of all Living Worlds plugins
//! to ensure proper dependency resolution and initialization.

use bevy::prelude::*;

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

/// Register all Living Worlds plugins in the correct dependency order
///
/// The plugins are initialized in a specific order to ensure proper dependencies:
/// 1. **Core Systems** - State management, relationships, modding, province events
/// 2. **UI Systems** - Menus, world config, loading screen, settings (depend on States)
/// 3. **World & Simulation** - Core gameplay systems
/// 4. **Interface & Controls** - User interface and camera controls
///
/// This function should be called after Bevy's DefaultPlugins have been added.
pub fn register_all_plugins(app: &mut App) {
    register_core_systems(app);
    register_ui_systems(app);
    register_world_and_simulation_systems(app);
    register_interface_and_controls(app);
}

/// Register core system plugins (required by other plugins)
fn register_core_systems(app: &mut App) {
    app.add_plugins(StatesPlugin) // State management (required by menus, world_config, etc.)
        .add_plugins(RelationshipsPlugin) // Entity relationships (required by nations, world systems)
        .add_plugins(ModdingPlugin) // Mod system (loads configs early)
        .add_plugins(ProvinceEventsPlugin); // Province change events
}

/// Register UI system plugins (depend on States)
fn register_ui_systems(app: &mut App) {
    app.add_plugins(MenusPlugin) // Menu UI system
        .add_plugins(WorldConfigPlugin) // World configuration UI
        .add_plugins(LoadingScreenPlugin) // Unified loading screen
        .add_plugins(SettingsUIPlugin); // Settings menu system
}

/// Register world generation and simulation systems
fn register_world_and_simulation_systems(app: &mut App) {
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(NoiseComputePlugin) // GPU compute acceleration for world generation
        .add_plugins(OverlayPlugin)
        .add_plugins(NationPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(SaveLoadPlugin);
}

/// Register user interface and control systems
fn register_interface_and_controls(app: &mut App) {
    app.add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BorderPlugin); // GPU-instanced border rendering
}
