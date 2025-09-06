//! Living Worlds - Main Integration Plugin
//! 
//! This crate now serves as the integration layer that combines
//! all domain plugins into a cohesive game experience.

use bevy::prelude::*;

// Import domain plugins
use lw_simulation::SimulationPlugin;
use lw_world::WorldPlugin;
use lw_economics::EconomicsPlugin;
use lw_military::MilitaryPlugin;
use lw_culture::CulturePlugin;
use lw_governance::GovernancePlugin;

// Re-export TimeState for main.rs compatibility
// TODO: TimeState was removed during refactoring - update main.rs
// pub use lw_simulation::components::TimeState;

/// Main game plugin that integrates all domain plugins
pub struct LivingWorldsPlugin;

impl Plugin for LivingWorldsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add all domain plugins
            .add_plugins((
                SimulationPlugin,  // Core simulation loop
                WorldPlugin,       // Geography, terrain, climate
                EconomicsPlugin,   // Markets, trade, production
                MilitaryPlugin,    // Armies, combat, logistics
                CulturePlugin,     // Cultural systems
                GovernancePlugin,  // Government, diplomacy, AI
            ));
        
        // Any cross-domain integration logic goes here
    }
}