//! State Management Utility Functions
//!
//! This module provides helper functions for checking state conditions
//! and requesting state transitions throughout the application.

use super::definitions::*;
use bevy::prelude::*;

/// Request a state transition (with validation)
pub fn request_transition(
    from: GameState,
    to: GameState,
    writer: &mut MessageWriter<RequestStateTransition>,
) {
    writer.write(RequestStateTransition { from, to });
}

/// Check if we're in a menu state
pub fn is_in_menu(state: &State<GameState>) -> bool {
    matches!(**state, GameState::MainMenu | GameState::Paused)
}

/// Check if gameplay is active
pub fn is_gameplay_active(state: &State<GameState>) -> bool {
    matches!(**state, GameState::InGame | GameState::Paused)
}

/// Helper to check if world generation can proceed
pub fn can_generate_world(world_gen: &WorldGenerationInProgress) -> bool {
    !world_gen.0 // Can generate if not already in progress
}
