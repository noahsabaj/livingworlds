//! State Transition Logic and Validation
//!
//! This module handles all state transition validation, event processing,
//! and menu navigation logic for the state management system.

use super::definitions::*;
use bevy::prelude::*;

// Constants for test transitions (if needed for debugging)
#[cfg(feature = "debug-states")]
const TEST_TRANSITION_DELAY: f32 = 0.5;
#[cfg(feature = "debug-states")]
const MENU_TRANSITION_DELAY: f32 = 1.0;

/// Validates and handles state transition requests
pub fn handle_state_transitions(
    mut events: EventReader<RequestStateTransition>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    world_gen_progress: Res<WorldGenerationInProgress>,
) {
    for event in events.read() {
        info!("Processing state transition request: {:?} → {:?}", event.from, event.to);
        info!("Current state: {:?}", **current_state);

        // Validate the transition is legal
        if is_valid_transition(event.from, event.to, &world_gen_progress) {
            info!("Transition is valid");
            if event.from == **current_state {
                info!("Current state matches expected, setting transition to {:?}", event.to);
                #[cfg(feature = "debug-states")]
                {
                    let start_time = std::time::Instant::now();
                    debug!(
                        "[TRANSITION START] State transition: {:?} → {:?}",
                        event.from, event.to
                    );
                    next_state.set(event.to);
                    debug!(
                        "[TRANSITION QUEUED] Transition queued in {:.1}ms",
                        start_time.elapsed().as_secs_f32() * 1000.0
                    );
                }
                #[cfg(not(feature = "debug-states"))]
                next_state.set(event.to);
                info!("NextState resource updated to {:?}", event.to);
            } else {
                warn!(
                    "Invalid transition request: current state {:?} doesn't match expected {:?}",
                    **current_state, event.from
                );
            }
        } else {
            warn!(
                "Illegal state transition from {:?} to {:?}",
                event.from, event.to
            );
        }
    }
}

/// Checks if a state transition is valid
pub fn is_valid_transition(
    from: GameState,
    to: GameState,
    _world_gen: &WorldGenerationInProgress,
) -> bool {
    use GameState::*;

    match (from, to) {
        // Loading can only go to MainMenu
        (Loading, MainMenu) => true,

        // MainMenu can go to WorldConfiguration or LoadingWorld
        (MainMenu, WorldConfiguration) => true,
        (MainMenu, LoadingWorld) => true,

        // WorldConfiguration can go to LoadingWorld, WorldGeneration, or back to MainMenu
        (WorldConfiguration, LoadingWorld) => true, // Direct to loading for new world
        (WorldConfiguration, WorldGeneration) => true,
        (WorldConfiguration, MainMenu) => true,

        // WorldGeneration can go to LoadingWorld or back to WorldConfiguration/MainMenu
        (WorldGeneration, LoadingWorld) => true,
        (WorldGeneration, WorldConfiguration) => true,
        (WorldGeneration, MainMenu) => true,

        // LoadingWorld can go to InGame, back to MainMenu (on error), or back to WorldConfiguration (cancel)
        (LoadingWorld, InGame) => true,
        (LoadingWorld, MainMenu) => true,
        (LoadingWorld, WorldConfiguration) => true, // Allow cancel back to world config

        // InGame can only be paused
        (InGame, Paused) => true,

        // Paused can resume or go to MainMenu
        (Paused, InGame) => true,
        (Paused, MainMenu) => true,
        (Paused, LoadingWorld) => true, // For loading a different save

        // All other transitions are invalid
        _ => false,
    }
}

/// Handles menu navigation events
pub fn handle_menu_events(
    mut events: EventReader<MenuEvent>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    current_menu: Res<State<MenuState>>,
) {
    for event in events.read() {
        match event {
            MenuEvent::Open(state) => {
                debug!("Opening menu: {:?}", state);
                next_menu_state.set(*state);
            }
            MenuEvent::Close => {
                debug!("Closing menu");
                next_menu_state.set(MenuState::None);
            }
            MenuEvent::Back => {
                // Navigate back based on current menu
                let back_to = match **current_menu {
                    MenuState::Settings => MenuState::Main,
                    MenuState::LoadBrowser => MenuState::Main,
                    MenuState::SaveDialog => MenuState::Main,
                    MenuState::Statistics => MenuState::Main,
                    MenuState::Help => MenuState::Main,
                    MenuState::Credits => MenuState::Main,
                    _ => MenuState::None,
                };
                debug!("Navigating back to: {:?}", back_to);
                next_menu_state.set(back_to);
            }
        }
    }
}
