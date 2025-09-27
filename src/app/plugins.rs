//! Game Plugins Aggregator

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import all game plugins
use crate::{
    camera::CameraPlugin,
    content_creation::ContentCreationPlugin,
    diagnostics::DiagnosticsPlugin,
    loading::LoadingScreenPlugin,
    menus::MenusPlugin,
    modding::ModdingPlugin,
    nations::{DramaEnginePlugin, NationPlugin},
    parallel::ParallelPlugin,
    performance::PerformanceMonitoringPlugin,
    relationships::RelationshipsPlugin,
    save_load::SaveLoadPlugin,
    settings::SettingsUIPlugin,
    simulation::SimulationPlugin,
    states::StatesPlugin,
    ui::UIPlugin,
    world::{NoiseComputePlugin, ProvinceEventsPlugin, WorldPlugin},
};

define_plugin!(GamePlugins {
    plugins: [
        // === Core Systems (required by most other plugins) ===
        StatesPlugin,
        ParallelPlugin,
        RelationshipsPlugin,

        // === Foundation Systems ===
        ModdingPlugin,
        ProvinceEventsPlugin,

        // === UI Foundation (depend on States) ===
        MenusPlugin,
        LoadingScreenPlugin,
        SettingsUIPlugin,

        // === World and Simulation ===
        WorldPlugin,
        NoiseComputePlugin,
        NationPlugin,
        DramaEnginePlugin,
        SimulationPlugin,
        SaveLoadPlugin,

        // === Interface and Controls ===
        UIPlugin,
        CameraPlugin,
        ContentCreationPlugin,

        // === Monitoring and Diagnostics ===
        DiagnosticsPlugin,
        PerformanceMonitoringPlugin
    ],

    // Custom initialization for conditional plugins
    custom_init: |app: &mut bevy::app::App| {
        // Add parallel safety validation in debug builds
        #[cfg(debug_assertions)]
        {
            info!("Debug mode: Adding parallel safety validation");
            app.add_plugins(crate::safety::ParallelSafetyPlugin);
        }
    }
});