//! World configuration plugin - MASSIVE SYSTEM AUTOMATION!
//!
//! This plugin demonstrates ULTIMATE system scheduling automation!
//! 58 lines with 14 systems → 35 lines declarative paradise!

use super::handlers;
use super::layout;
use super::types::WorldGenerationSettings;
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// World configuration plugin using MASSIVE SYSTEM AUTOMATION!
///
// **AUTOMATION ACHIEVEMENT**: 58 lines with 14 systems → 35 lines declarative!
define_plugin!(WorldConfigPlugin {
    resources: [WorldGenerationSettings],

    update: [
        // All 14 world config systems beautifully organized!
        (// Input handlers
         handlers::handle_text_input_changes,
         handlers::handle_random_buttons,
         // Selection handlers
         handlers::handle_preset_selection,
         handlers::handle_size_selection,
         handlers::handle_climate_selection,
         handlers::handle_island_selection,
         handlers::handle_aggression_selection,
         handlers::handle_resource_selection,
         // UI interactions
         handlers::handle_preset_hover,
         handlers::handle_advanced_toggle,
         handlers::handle_slider_interactions,
         // Display updates
         handlers::update_seed_display,
         handlers::update_slider_displays,
         // Navigation
         handlers::handle_generate_button,
         handlers::handle_back_button).run_if(in_state(GameState::WorldConfiguration))
    ],

    on_enter: {
        GameState::WorldConfiguration => [
            handlers::init_default_settings,
            layout::spawn_world_config_ui
        ]
    },

    on_exit: {
        GameState::WorldConfiguration => [layout::despawn_world_config_ui]
    }
});
