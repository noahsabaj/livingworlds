//! Mod Browser UI Plugin
//!
//! This plugin provides the complete mod browser interface, orchestrating
//! all UI spawning, event handling, and state management systems.

use super::handlers;
use super::state::{ModBrowserState, WorkshopCache};
use super::types::{
    ApplyModChangesEvent, CloseModBrowserEvent, OpenModBrowserEvent, SwitchModTabEvent,
};
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use bevy_simple_text_input::TextInputPlugin;

define_plugin!(ModBrowserUIPlugin {
    resources: [ModBrowserState, WorkshopCache],

    events: [
        OpenModBrowserEvent,
        CloseModBrowserEvent,
        ApplyModChangesEvent,
        SwitchModTabEvent
    ],

    plugins: [TextInputPlugin],

    update: [
        // Browser management
        handlers::handle_open_mod_browser,
        handlers::handle_close_mod_browser,
        handlers::handle_close_button_clicks,

        // Tab switching
        handlers::handle_tab_button_clicks,
        handlers::handle_tab_switching,
        handlers::update_tab_buttons,

        // Interactions
        handlers::handle_mod_toggles,
        handlers::handle_confirm_modset_clicks,
        handlers::handle_apply_changes,

        // Search
        handlers::handle_search_input_changes,
        handlers::handle_search_submit
    ]
});