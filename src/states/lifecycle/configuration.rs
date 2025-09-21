//! Configuration State Lifecycle Management
//!
//! This module handles the enter/exit lifecycle for world configuration-related states,
//! including WorldConfiguration and WorldGeneration states.

use crate::states::definitions::*;
use bevy::prelude::*;

/// System that runs when entering the WorldConfiguration state
pub fn enter_world_configuration(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    debug!("Entering WorldConfiguration state");
    // The world_config module handles spawning the configuration UI
}

/// Cleanup when exiting the WorldConfiguration state
pub fn exit_world_configuration(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting WorldConfiguration state");
    // The world_config module handles cleanup
}

/// System that runs when entering the WorldGeneration state
pub fn enter_world_generation(
    _commands: Commands,
    mut world_gen: ResMut<WorldGenerationInProgress>,
) {
    #[cfg(feature = "debug-states")]
    debug!("Entering WorldGeneration state");
    world_gen.0 = true; // Mark as in progress
                        // Note: This state is now mostly unused - we go directly to LoadingWorld
}

/// Cleanup when exiting the WorldGeneration state
pub fn exit_world_generation(
    _commands: Commands,
    mut world_gen: ResMut<WorldGenerationInProgress>,
) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting WorldGeneration state");
    world_gen.0 = false; // CRITICAL FIX: Reset the flag so generation can happen again
                         // Cleanup world generation UI
}
