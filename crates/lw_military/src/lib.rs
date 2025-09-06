//! Living Worlds - Military Domain

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct MilitaryPlugin;

impl Plugin for MilitaryPlugin {
    fn build(&self, app: &mut App) {
        // Register military systems here
    }
}
