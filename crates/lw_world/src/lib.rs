//! Living Worlds - World Domain
//! 
//! Contains all components and systems related to the physical world:
//! geography, terrain, climate, resources, and natural disasters.

pub mod components;

// Re-export key types at crate root
pub use components::*;
use bevy::prelude::*;

pub mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Register world systems here
    }
}
