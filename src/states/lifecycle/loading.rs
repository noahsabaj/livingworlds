//! Loading State Lifecycle Management
//!
//! This module handles the enter/exit lifecycle for loading-related states,
//! including the initial Loading state and LoadingWorld state for world generation.

use crate::states::definitions::*;
use bevy::prelude::*;

/// System that runs when entering the Loading state
pub fn enter_loading(_commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    #[cfg(feature = "debug-states")]
    debug!("Entering Loading state");

    // Immediately transition to MainMenu since the game has no external assets to load
    // All content is procedurally generated at runtime
    next_state.set(GameState::MainMenu);
}

/// Cleanup when exiting the Loading state
pub fn exit_loading(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting Loading state");
}

/// System that runs when entering the LoadingWorld state
pub fn enter_loading_world(
    _commands: Commands,
    _next_state: ResMut<NextState<GameState>>,
    pending_load: Option<Res<crate::save_load::PendingLoadData>>,
) {
    #[cfg(feature = "debug-states")]
    debug!("Entering LoadingWorld state");

    // If we have a save to load, transition to InGame after loading
    // Otherwise, wait for world generation to complete
    if pending_load.is_some() {
    } else {
        // World generation will handle the transition
    }
}

/// Cleanup when exiting the LoadingWorld state
pub fn exit_loading_world(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting LoadingWorld state");
    // Cleanup any loading UI
}
