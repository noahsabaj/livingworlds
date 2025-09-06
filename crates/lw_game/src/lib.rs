//! Living Worlds Game Logic
//! 
//! All game-specific components, systems, and logic consolidated here.
//! Uses Bevy for ECS, rendering, UI, and input.

pub mod components;
pub mod systems;
pub mod types;

use bevy::prelude::*;

// Re-export key types
pub use components::*;
pub use types::*;

/// Main game plugin that adds all systems
pub struct LivingWorldsPlugin;

impl Plugin for LivingWorldsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add game systems
            .add_systems(Update, (
                systems::update_army_morale_system,
                systems::army_movement_system,
                systems::combat_resolution_system,
                systems::economy_system,
                systems::trade_route_system,
            ));
    }
}
