//! World configuration plugin
//!
//! This plugin manages the world configuration UI state and systems.

use bevy::prelude::*;
use crate::states::GameState;
use super::types::WorldGenerationSettings;
use super::layout;
use super::handlers;

pub struct WorldConfigPlugin;

impl Plugin for WorldConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<WorldGenerationSettings>()

            // State enter/exit systems
            .add_systems(OnEnter(GameState::WorldConfiguration), (
                handlers::init_default_settings,
                layout::spawn_world_config_ui,
            ))
            .add_systems(OnExit(GameState::WorldConfiguration),
                layout::despawn_world_config_ui
            )

            // Update systems
            .add_systems(Update, (
                // Input handlers
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
                handlers::handle_back_button,
            ).run_if(in_state(GameState::WorldConfiguration)));
    }
}