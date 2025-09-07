//! Living Worlds - Core Game Library
//! 
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// Module declarations - these are our game systems
pub mod camera;
pub mod clouds;
pub mod components;
pub mod constants;
pub mod minerals;
pub mod music;
pub mod overlay;
pub mod resources;
pub mod setup;
pub mod terrain;
pub mod ui;

// Re-export commonly used items for convenient access
pub mod prelude {
    pub use crate::camera::CameraPlugin;
    pub use crate::clouds::{CloudPlugin, spawn_clouds};
    pub use crate::components::{
        Province, Nation, SelectedProvince, GhostProvince,
        TileInfoPanel, TileInfoText,
        ProvinceResources, ProvinceInfrastructure, NationStockpile,
        NationTechnology, MineralType, TechnologyAge, EquipmentTier,
    };
    pub use crate::constants::*;
    pub use crate::resources::{
        WorldSeed, WorldSize, GameTime,
        SelectedProvinceInfo, ProvincesSpatialIndex,
    };
    pub use crate::setup::setup_world;
    pub use crate::terrain::{
        TerrainPlugin, TerrainType, ClimateZone,
        classify_terrain_with_climate, get_terrain_color_gradient,
        generate_elevation, generate_continent_centers,
        get_terrain_population_multiplier,
    };
    pub use crate::ui::{UIPlugin};
    pub use crate::music::{ProceduralMusicPlugin, MusicState};
    pub use crate::minerals::{MineralPlugin, generate_ore_veins, calculate_province_resources};
    pub use crate::overlay::OverlayPlugin;
}

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

// Import our plugins
use crate::camera::CameraPlugin;
use crate::clouds::CloudPlugin;
use crate::minerals::MineralPlugin;
use crate::music::ProceduralMusicPlugin;
use crate::overlay::OverlayPlugin;
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
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default());
    
    // Add all Living Worlds game plugins
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(MineralPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ProceduralMusicPlugin);
    
    app
}

// Re-export resources for backward compatibility
pub use resources::{WorldSeed, WorldSize, GameTime};

// Re-export commonly used constants for backward compatibility
pub use constants::{HEX_SIZE_PIXELS, PROVINCES_PER_ROW, PROVINCES_PER_COL};