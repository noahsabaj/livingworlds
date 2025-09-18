//! Loading screen plugin - Bevy integration for the loading system

use bevy_plugin_builder::define_plugin;
use crate::states::GameState;
use crate::ui::despawn_ui_entities;
use super::{
    state::LoadingState,
    events::CancelWorldGeneration,
    ui::{setup_loading_screen, LoadingScreenRoot},
    progress::{update_loading_progress, update_loading_text},
    events::{handle_cancel_button, handle_cancel_generation},
};

/// Plugin for the loading screen system
///
/// Manages all loading screen functionality including:
/// - UI setup and cleanup
/// - Progress tracking
/// - Event handling
/// - State transitions
///
/// Uses the revolutionary declarative plugin system for zero boilerplate.
define_plugin!(LoadingScreenPlugin {
    resources: [LoadingState],

    events: [CancelWorldGeneration],

    update: [
        (
            update_loading_progress,
            update_loading_text,
            handle_cancel_button,
            handle_cancel_generation,
        ).run_if(in_state(GameState::LoadingWorld))
    ],

    on_enter: {
        GameState::LoadingWorld => [setup_loading_screen]
    },

    on_exit: {
        GameState::LoadingWorld => [despawn_ui_entities::<LoadingScreenRoot>]
    }
});