//! Time-related events

use bevy::prelude::*;

/// Event sent when simulation speed changes
#[derive(Event)]
pub struct SimulationSpeedChanged {
    pub new_speed: f32,
    pub is_paused: bool,
}

/// Event sent when a new year begins
#[derive(Event)]
pub struct NewYearEvent {
    pub year: u32,
}