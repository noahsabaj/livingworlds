//! Bevy plugin for the keyboard shortcuts system

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::registry::ShortcutRegistry;
use super::systems::*;
use super::types::{ShortcutConfig, ShortcutEvent};

define_plugin!(ShortcutPlugin {
    resources: [
        ShortcutConfig,
        ShortcutRegistry
    ],

    events: [
        ShortcutEvent
    ],

    startup: [
        init_shortcuts
    ],

    update: [
        // Process input and trigger events
        process_shortcuts,

        // Update context based on game state
        update_shortcut_context,

        // Handle common shortcuts
        handle_common_shortcuts,

        // UI updates
        update_shortcut_hints.run_if(resource_changed::<ShortcutRegistry>),

        // Rebinding system (only when enabled)
        handle_rebinding.run_if(resource_exists_and_changed::<ShortcutConfig>)
    ]
});

/// Initialize default shortcuts on startup
fn init_shortcuts(mut registry: ResMut<ShortcutRegistry>) {
    registry.init_defaults();
    info!("Keyboard shortcuts initialized");
}