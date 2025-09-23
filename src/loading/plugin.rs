//! Loading screen plugin - Bevy integration for the loading system

use super::{
    events::CancelWorldGeneration,
    state::LoadingState,
    ui::LoadingScreenRoot,
};
use crate::states::GameState;
use crate::ui::despawn_ui_entities;
use bevy_plugin_builder::define_plugin;

// Plugin for the loading screen system
///
// Manages all loading screen functionality including:
// - UI setup and cleanup
// - Progress tracking
// - Event handling
// - State transitions
///
define_plugin!(LoadingScreenPlugin {
    resources: [LoadingState],

    events: [CancelWorldGeneration],

    update: [
        super::progress::update_loading_progress,
        super::progress::update_loading_text,
        super::events::handle_cancel_button,
        super::events::handle_cancel_generation
    ],

    on_enter: {
        GameState::LoadingWorld => [super::ui::setup_loading_screen]
    },

    on_exit: {
        GameState::LoadingWorld => [despawn_ui_entities::<LoadingScreenRoot>]
    }
});
