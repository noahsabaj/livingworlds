//! Main plugin for the simulation module

use crate::resources::GameTime;
use crate::states::GameState;
use bevy::prelude::*;

// Import from sibling modules through super (gateway pattern)
use super::input::handle_time_controls;
use super::time::{
    advance_game_time, resume_from_pause_menu, track_year_changes, NewYearEvent,
    SimulationSpeedChanged,
};

/// Plugin that manages the simulation time system
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<GameTime>()
            // Add events
            .add_event::<SimulationSpeedChanged>()
            .add_event::<NewYearEvent>()
            // Time management systems - only run during gameplay
            .add_systems(
                Update,
                (handle_time_controls, advance_game_time, track_year_changes)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::InGame), resume_from_pause_menu);

        // Example population growth system with events (will be enabled when nations are added)
        // .add_systems(Update, simulate_population_growth
        //     .run_if(in_state(GameState::InGame)));
    }
}
