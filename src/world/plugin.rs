//! World plugin implementation - PLUGIN AGGREGATION AUTOMATION!

use bevy_plugin_builder::define_plugin;

// Import from sibling modules through super (gateway pattern)
use super::{BorderPlugin, CloudPlugin, OverlayPlugin, TerrainPlugin, WorldConfigPlugin};
use super::{ProvincesSpatialIndex, CoastalProvinceCache};
use super::events::{WorldGeneratedEvent, ProvinceSelectedEvent};

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

    resources: [ProvincesSpatialIndex, CoastalProvinceCache],

    messages: [WorldGeneratedEvent, ProvinceSelectedEvent]
});