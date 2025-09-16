//! Main world plugin implementation
//!
//! This module contains the WorldPlugin that orchestrates all world functionality
//! and integrates it with the Bevy app.

use bevy::prelude::*;

// Import from sibling modules through super (gateway pattern)
use super::{BorderPlugin, CloudPlugin, OverlayPlugin, TerrainPlugin, WorldConfigPlugin};
use super::{ProvinceId, ProvincesSpatialIndex};

/// Event fired when world generation completes
#[derive(Event)]
pub struct WorldGeneratedEvent {
    pub world: super::World,
    pub generation_time: std::time::Duration,
}

/// Event fired when a province is selected
#[derive(Event)]
pub struct ProvinceSelectedEvent {
    pub province_id: Option<ProvinceId>,
    pub position: Vec2,
}

/// Internal world state resource
#[derive(Resource, Default)]
struct WorldState {
    initialized: bool,
    selected_province: Option<ProvinceId>,
}

/// Main world plugin that registers all world-related systems
///
/// This plugin aggregates all world functionality into Bevy.
/// It's the ONLY place where world systems are registered with the app.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add feature plugins
            .add_plugins(CloudPlugin)
            .add_plugins(TerrainPlugin)
            .add_plugins(BorderPlugin)
            .add_plugins(OverlayPlugin)
            .add_plugins(WorldConfigPlugin)
            // Register world resources
            .init_resource::<ProvincesSpatialIndex>()
            .init_resource::<WorldState>()
            // Register world events
            .add_event::<WorldGeneratedEvent>()
            .add_event::<ProvinceSelectedEvent>()
            // Add world systems
            .add_systems(Startup, initialize_world_systems)
            .add_systems(
                Update,
                (handle_province_selection, update_world_bounds_camera).chain(),
            );
    }
}

// === WORLD SYSTEMS - Internal Bevy systems ===

/// Initialize world systems on startup
fn initialize_world_systems(mut commands: Commands) {
    info!("World systems initialized");

    // Initialize any world-specific resources
    commands.insert_resource(WorldState::default());
}

/// Handle province selection from mouse input
fn handle_province_selection(
    _mouse_button: Res<ButtonInput<MouseButton>>,
    _camera_query: Query<(&Camera, &GlobalTransform)>,
    _windows: Query<&Window>,
    _spatial_index: Res<ProvincesSpatialIndex>,
    _selection_events: EventWriter<ProvinceSelectedEvent>,
) {
    // This is where mouse picking and province selection would be implemented
    // Keeping it internal to the plugin module as it's Bevy-specific
}

/// Update camera bounds based on world size
fn update_world_bounds_camera(
    _spatial_index: Res<ProvincesSpatialIndex>,
    _camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    // This would constrain camera to world bounds
    // Keeping it internal to the plugin module as it's Bevy-specific
}