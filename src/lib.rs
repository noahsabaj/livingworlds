//! Living Worlds - Core Game Library
//! 
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// Module declarations - these are our game systems
pub mod borders;
pub mod camera;
pub mod clouds;
pub mod colors;
pub mod components;
pub mod constants;
pub mod minerals;
pub mod music;
pub mod overlay;
pub mod resources;
pub mod simulation;
pub mod setup;
pub mod terrain;
pub mod ui;

// Re-export commonly used items for convenient access
pub mod prelude {
    pub use crate::borders::BorderPlugin;
    pub use crate::camera::CameraPlugin;
    pub use crate::clouds::{CloudPlugin, spawn_clouds};
    pub use crate::components::{
        Province, Nation, SelectedProvince,
        TileInfoPanel, TileInfoText,
        ProvinceResources, ProvinceInfrastructure, NationStockpile,
        NationTechnology, MineralType, TechnologyAge, EquipmentTier,
    };
    pub use crate::constants::*;
    pub use crate::resources::{
        WorldSeed, WorldSize, GameTime,
        SelectedProvinceInfo, ProvincesSpatialIndex,
    };
    pub use crate::setup::{setup_world, ProvinceStorage};
    pub use crate::terrain::{
        TerrainPlugin, TerrainType, ClimateZone,
        classify_terrain_with_climate,
        generate_elevation_with_edges,
        get_terrain_population_multiplier,
    };
    pub use crate::colors::{
        get_terrain_color_gradient,
        mineral_abundance_color, stone_abundance_color,
        combined_richness_color, infrastructure_level_color,
        get_mineral_color,
    };
    pub use crate::ui::{UIPlugin};
    pub use crate::music::{ProceduralMusicPlugin, MusicState};
    pub use crate::minerals::{MineralPlugin, generate_ore_veins, calculate_province_resources};
    pub use crate::overlay::OverlayPlugin;
    pub use crate::simulation::SimulationPlugin;
}

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};
use bevy::audio::AudioPlugin;

// Import our plugins
use crate::borders::BorderPlugin;
use crate::camera::CameraPlugin;
use crate::clouds::CloudPlugin;
use crate::minerals::MineralPlugin;
// use crate::music::ProceduralMusicPlugin;  // Temporarily disabled
use crate::overlay::OverlayPlugin;
use crate::simulation::SimulationPlugin;
use crate::terrain::TerrainPlugin;
use crate::ui::UIPlugin;

/// Builds the core Bevy application with all Living Worlds plugins.
/// This sets up the engine, window, and all game systems but doesn't
/// include game-specific resources or startup systems.
pub fn build_app() -> App {
    let mut app = App::new();
    
    // Configure Bevy's default plugins with our settings
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Living Worlds".into(),
                    resolution: (1920.0, 1080.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .disable::<AudioPlugin>()  // Disable audio to prevent ALSA underruns
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_systems(Update, display_fps);
    
    // Add all Living Worlds game plugins
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(MineralPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BorderPlugin);  // GPU-instanced border rendering
        // Audio disabled temporarily to prevent ALSA underruns
        // .add_plugins(ProceduralMusicPlugin);
    
    app
}

// Re-export resources for backward compatibility
pub use resources::{WorldSeed, WorldSize, GameTime};

// Re-export commonly used constants for backward compatibility
pub use constants::{HEX_SIZE_PIXELS, PROVINCES_PER_ROW, PROVINCES_PER_COL};

/// Display FPS in console every second
fn display_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut last_print: Local<f32>,
    time: Res<Time>,
) {
    // Only print once per second to avoid console spam
    if time.elapsed_secs() - *last_print > 1.0 {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                println!("FPS: {:.1} | Frame Time: {:.2}ms", value, 1000.0 / value);
                *last_print = time.elapsed_secs();
            }
        }
    }
}