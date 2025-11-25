//! States System Plugin
//!
//! This module provides the Bevy plugin that integrates all state management
//! systems into the application, coordinating between definitions, transitions,
//! lifecycle, and utilities.

use super::{
    definitions::*,
    lifecycle::*,
    transitions::{handle_menu_events, handle_state_transitions},
};
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Plugin that manages all state-related systems using declarative syntax
define_plugin!(StatesPlugin {
    states: [GameState],
    sub_states: [MenuState],

    resources: [
        CurrentSettingsTab,
        SavedWorldExists,
        WorldGenerationInProgress,
        PendingWorldGeneration
    ],

    messages: [
        RequestStateTransition,
        MenuEvent,
        StartWorldGeneration
    ],

    update: [
        handle_state_transitions,
        handle_menu_events.run_if(in_state(GameState::MainMenu)),
        (
            check_and_trigger_world_generation,
            crate::world::poll_async_world_generation,
            crate::world::spawn_province_entities,
            crate::world::setup_province_neighbors,
            crate::world::handle_world_generation_transition_delay
        ).run_if(in_state(GameState::LoadingWorld)),
        handle_error_dialog_buttons.run_if(in_state(GameState::WorldGenerationFailed)),
        log_state_changes.run_if(state_changed::<GameState>)
    ],

    on_enter: {
        GameState::Loading => [enter_loading],
        GameState::MainMenu => [enter_main_menu],
        GameState::WorldConfiguration => [enter_world_configuration],
        GameState::WorldGeneration => [enter_world_generation],
        GameState::LoadingWorld => [enter_loading_world],
        GameState::InGame => [enter_in_game],
        GameState::Paused => [enter_paused],
        GameState::WorldGenerationFailed => [enter_world_generation_failed]
    },

    on_exit: {
        GameState::Loading => [exit_loading],
        GameState::MainMenu => [exit_main_menu],
        GameState::WorldConfiguration => [exit_world_configuration],
        GameState::WorldGeneration => [exit_world_generation],
        GameState::LoadingWorld => [exit_loading_world],
        GameState::InGame => [exit_in_game],
        GameState::Paused => [exit_paused],
        GameState::WorldGenerationFailed => [exit_world_generation_failed]
    }
});
