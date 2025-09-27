//! Plugin for the dropdown system

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use super::systems::*;

define_plugin!(DropdownPlugin {
    update: [
        handle_dropdown_interactions::<String>,
        update_dropdown_display::<String>,
        handle_dropdown_keyboard::<String>
    ]
});