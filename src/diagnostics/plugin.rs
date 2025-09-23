//! Diagnostics Plugin - Bevy Plugin for Performance Monitoring
//!
//! This module provides the Bevy plugin integration for the diagnostics
//! system, handling registration of FPS monitoring and other diagnostic systems.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import from sibling modules
use super::fps::display_fps;

// Import configuration type
use crate::DiagnosticsConfig;

// Plugin for diagnostics and performance monitoring systems
///
// This plugin automatically registers the FPS monitoring system and
// integrates with Bevy's diagnostic infrastructure. It only adds the
// FPS display system when DiagnosticsConfig is present as a resource.
define_plugin!(DiagnosticsPlugin {
    // Core diagnostic infrastructure
    plugins: [FrameTimeDiagnosticsPlugin::default()],

    // FPS monitoring system (conditional on config presence)
    update: [display_fps.run_if(resource_exists::<DiagnosticsConfig>)]
});
