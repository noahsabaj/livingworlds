//! Living Worlds - Economics Domain

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct EconomicsPlugin;

impl Plugin for EconomicsPlugin {
    fn build(&self, app: &mut App) {
        // Register economic systems here
    }
}
