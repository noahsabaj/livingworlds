//! World plugin implementation - PLUGIN AGGREGATION AUTOMATION!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import from sibling modules through super (gateway pattern)
use super::{BorderPlugin, CloudPlugin, OverlayPlugin, TerrainPlugin, WorldConfigPlugin};
use super::{ProvincesSpatialIndex};
use super::events::{WorldGeneratedEvent, ProvinceSelectedEvent};
use super::systems::{handle_province_selection, initialize_world_systems, update_world_bounds_camera, WorldState};

/// Main world plugin using REVOLUTIONARY plugin aggregation automation!
///
/// **AUTOMATION ACHIEVEMENT**: 92 lines of manual coordination → ~50 lines declarative!
define_plugin!(WorldPlugin {
    plugins: [
        CloudPlugin,
        TerrainPlugin,
        BorderPlugin,
        OverlayPlugin,
        WorldConfigPlugin
    ],

    resources: [ProvincesSpatialIndex, WorldState],

    events: [WorldGeneratedEvent, ProvinceSelectedEvent],

    startup: [initialize_world_systems],

    update: [(handle_province_selection, update_world_bounds_camera).chain()]
});