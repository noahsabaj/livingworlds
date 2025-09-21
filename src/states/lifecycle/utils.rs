//! Lifecycle Utility Functions
//!
//! This module contains cross-cutting lifecycle utilities including world generation
//! triggering and state change logging that support the overall lifecycle system.

use crate::states::definitions::*;
use bevy::prelude::*;

/// Check and trigger world generation after a delay
pub fn check_and_trigger_world_generation(
    commands: Commands,
    mut pending_gen: ResMut<PendingWorldGeneration>,
    time: Res<Time>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<crate::world::WorldGenerationSettings>,
    state_events: EventWriter<RequestStateTransition>,
    loading_state: ResMut<crate::loading::LoadingState>,
) {
    if !pending_gen.pending {
        return;
    }

    // Wait a short delay to allow loading screen to render
    pending_gen.delay_timer -= time.delta_secs();
    if pending_gen.delay_timer > 0.0 {
        return;
    }

    // Clear the pending flag
    pending_gen.pending = false;

    info!("Starting world generation after loading screen renders");

    crate::world::start_async_world_generation(
        commands,
        meshes,
        materials,
        settings,
        state_events,
        loading_state,
        None, // gpu_status
        None, // gpu_config
        None, // gpu_state
        None, // gpu_metrics
        None, // validation_config
    );
}

/// Logs state changes for debugging (only runs when state actually changes)
pub fn log_state_changes(state: Res<State<GameState>>, _menu_state: Option<Res<State<MenuState>>>) {
    #[cfg(feature = "debug-states")]
    {
        debug!("STATE CHANGED: Now in {:?}", **state);
        if let Some(menu) = _menu_state {
            debug!("  Menu state: {:?}", **menu);
        }
    }

    // In production, could log to a file or metrics system
    #[cfg(not(feature = "debug-states"))]
    info!("Game state changed to: {:?}", **state);
}
