//! Living Worlds - Simulation Core

use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod phases;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        // Register simulation systems and phases here
    }
}
