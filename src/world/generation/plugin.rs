//! World generation plugin
//!
//! This plugin registers world generation systems and startup logic.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// Plugin that registers world generation systems using the revolutionary automation system
define_plugin!(GenerationPlugin {
    startup: [|| info!("World generation module ready")]
});