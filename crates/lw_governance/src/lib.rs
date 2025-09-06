//! Living Worlds - Governance Domain

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct GovernancePlugin;

impl Plugin for GovernancePlugin {
    fn build(&self, app: &mut App) {
        // Register governance systems here
    }
}
