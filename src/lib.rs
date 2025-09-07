//! Living Worlds - Core Game Library
//! 
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// Module declarations - these are our game systems
pub mod camera;
pub mod clouds;
pub mod terrain;
pub mod ui;

// Re-export commonly used items for convenient access
pub mod prelude {
    pub use crate::camera::CameraPlugin;
    pub use crate::clouds::{CloudPlugin, spawn_clouds};
    pub use crate::terrain::{
        TerrainPlugin, TerrainType, ClimateZone,
        classify_terrain_with_climate, get_terrain_color_gradient,
        generate_elevation, generate_continent_centers,
        get_terrain_population_multiplier, EDGE_BUFFER, SQRT3,
    };
    pub use crate::ui::{UIPlugin, FpsText};
}

use bevy::prelude::*;
use bevy::audio::AudioPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

// Import our plugins
use crate::camera::CameraPlugin;
use crate::clouds::CloudPlugin;
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
            // Disable audio to avoid ALSA underrun errors during development
            .disable::<AudioPlugin>()
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default());
    
    // Add all Living Worlds game plugins
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(CameraPlugin);
    
    app
}

/// Configuration for world generation
#[derive(Resource)]
pub struct WorldSeed(pub u32);

/// World size configuration
#[derive(Resource, Clone, Copy)]
pub enum WorldSize {
    Small,   // 150x100 provinces
    Medium,  // 300x200 provinces (default)
    Large,   // 450x300 provinces
}

impl WorldSize {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" => WorldSize::Small,
            "large" => WorldSize::Large,
            _ => WorldSize::Medium,
        }
    }
    
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            WorldSize::Small => (150, 100),
            WorldSize::Medium => (300, 200),
            WorldSize::Large => (450, 300),
        }
    }
}

/// Whether to show FPS counter
#[derive(Resource)]
pub struct ShowFps(pub bool);

/// Current game time and speed
#[derive(Resource)]
pub struct GameTime {
    pub current_date: f32, // Days since start
    pub speed: f32,        // Time multiplier
    pub paused: bool,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            current_date: 0.0,
            speed: 1.0,
            paused: false,
        }
    }
}

// Game constants - these should eventually move to a constants.rs file
pub const HEX_SIZE_PIXELS: f32 = 50.0;
pub const PROVINCES_PER_ROW: u32 = 300;
pub const PROVINCES_PER_COL: u32 = 200;