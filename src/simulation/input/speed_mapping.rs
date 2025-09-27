//! Speed mapping helpers to eliminate duplication

use bevy::prelude::*;
use crate::simulation::SimulationSpeed;

/// Speed level mappings for direct key selection
pub const SPEED_LEVELS: &[(KeyCode, SimulationSpeed)] = &[
    (KeyCode::Digit1, SimulationSpeed::Paused),
    (KeyCode::Digit2, SimulationSpeed::Normal),
    (KeyCode::Digit3, SimulationSpeed::Fast),
    (KeyCode::Digit4, SimulationSpeed::Faster),
    (KeyCode::Digit5, SimulationSpeed::Fastest),
];

pub fn handle_speed_keys(keyboard: &ButtonInput<KeyCode>) -> Option<SimulationSpeed> {
    SPEED_LEVELS
        .iter()
        .find(|(key, _)| keyboard.just_pressed(*key))
        .map(|(_, speed)| *speed)
}

/// Get the next faster speed level
pub fn get_next_speed_level(current_speed: SimulationSpeed) -> SimulationSpeed {
    current_speed.faster()
}

/// Get the next slower speed level
pub fn get_previous_speed_level(current_speed: SimulationSpeed) -> SimulationSpeed {
    current_speed.slower()
}
