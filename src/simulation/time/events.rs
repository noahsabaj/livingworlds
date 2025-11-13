//! Time-related events

use bevy::prelude::*;

/// Event sent when simulation speed changes
#[derive(Message)]
pub struct SimulationSpeedChanged {
    pub new_speed: f32,
    pub is_paused: bool,
}

/// Event sent when a new year begins
#[derive(Message)]
pub struct NewYearEvent {
    pub year: u32,
}
