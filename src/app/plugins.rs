//! Game Plugins Aggregator - Revolutionary Plugin Registration Automation!
//!
//! This module demonstrates the ULTIMATE power of bevy-plugin-builder:
//! 80+ lines of imperative plugin registration to ~30 lines of pure declarative beauty!
//!
//! Using the aggregation feature of define_plugin!, we eliminate an entire file
//! (plugin_order.rs) and transform manual registration into compile-time validated,
//! declarative plugin management.

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
    nations::{DramaEnginePlugin, GovernancePlugin, NationPlugin},
    parallel::ParallelPlugin,
    performance::PerformanceMonitoringPlugin,
    relationships::RelationshipsPlugin,
    save_load::SaveLoadPlugin,
    settings::SettingsUIPlugin,
    simulation::SimulationPlugin,
    states::StatesPlugin,
    ui::UIPlugin,
    world::{NoiseComputePlugin, ProvinceEventsPlugin, WorldConfigPlugin, WorldPlugin},
};

// Import external UI builders plugin - dogfooding our own crate!
use bevy_ui_builders::UiBuilderPlugin;

/// Master plugin that aggregates all Living Worlds game plugins in dependency order.
///
/// **AUTOMATION ACHIEVEMENT**: 80+ lines to ~30 lines (63% reduction!)
///
/// This plugin ensures proper initialization order:
/// 1. Core systems (States, Parallel, Relationships)
/// 2. Foundation (Modding, Events)
/// 3. UI systems (Menus, Settings, Loading)
/// 4. World and simulation
/// 5. Interface and controls
/// 6. Monitoring and diagnostics
define_plugin!(GamePlugins {
    plugins: [
        // === Core Systems (required by most other plugins) ===
        StatesPlugin,           // State management foundation
        ParallelPlugin,         // Rayon parallel processing
        RelationshipsPlugin,    // Entity relationship system

        // === Foundation Systems ===
        ModdingPlugin,          // Mod loading and management
        ProvinceEventsPlugin,   // Province change events

        // === UI Foundation (depend on States) ===
        MenusPlugin,            // Main and pause menus
        WorldConfigPlugin,      // World generation UI
        LoadingScreenPlugin,    // Loading UI
        SettingsUIPlugin,       // Settings interface

        // === World and Simulation ===
        WorldPlugin,            // Aggregates Cloud, Terrain, Border, Overlay
        NoiseComputePlugin,     // GPU compute acceleration
        GovernancePlugin,       // Political systems
        NationPlugin,           // Nation management
        DramaEnginePlugin,      // Character drama system
        SimulationPlugin,       // Core simulation
        SaveLoadPlugin,         // Save/load functionality

        // === Interface and Controls ===
        UiBuilderPlugin,        // External UI builders (dogfooding our own crate!)
        UIPlugin,               // Aggregates all UI sub-plugins
        CameraPlugin,           // Camera controls
        ContentCreationPlugin,  // Viral moment detection

        // === Monitoring and Diagnostics ===
        DiagnosticsPlugin,      // FPS and performance display
        PerformanceMonitoringPlugin  // Rayon metrics
    ],

    // Custom initialization for conditional plugins
    custom_init: |app| {
        // Add parallel safety validation in debug builds
        #[cfg(debug_assertions)]
        {
            info!("Debug mode: Adding parallel safety validation");
            app.add_plugins(crate::safety::ParallelSafetyPlugin);
        }
    }
});