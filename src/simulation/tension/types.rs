//! World tension types and data structures

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};

/// World Tension - Global metric tracking conflict and instability
///
/// Tension ranges from 0.0 (perfect peace) to 1.0 (world war).
/// It rises quickly with conflicts but falls slowly during peace,
/// simulating how real-world tensions have momentum.
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct WorldTension {
    /// Current tension level (0.0 to 1.0)
    pub current: f32,
    /// Target tension based on world state
    pub target: f32,
    /// Rate of change
    pub velocity: f32,

    // Contributing factors (each 0.0 to 1.0)
    /// Percentage of nations at war
    pub war_factor: f32,
    /// Power imbalance (one nation too dominant)
    pub power_imbalance: f32,
    /// Economic disruption (trade routes broken)
    pub economic_stress: f32,
    /// Recent collapses or disasters
    pub instability_factor: f32,

    // Physics parameters
    /// How fast tension rises (default: 2.0)
    pub heating_rate: f32,
    /// How slowly tension falls (default: 0.3)
    pub cooling_rate: f32,
    /// Resistance to change (default: 0.8)
    pub inertia: f32,
}

impl Default for WorldTension {
    fn default() -> Self {
        Self {
            current: 0.0,  // Start at perfect peace
            target: 0.0,
            velocity: 0.0,

            war_factor: 0.0,
            power_imbalance: 0.0,
            economic_stress: 0.0,
            instability_factor: 0.0,

            heating_rate: 2.0,    // Wars escalate quickly
            cooling_rate: 0.3,    // Peace returns slowly
            inertia: 0.8,         // Smooth transitions
        }
    }
}

impl WorldTension {
    /// Calculate tension from war percentage using exponential curve
    ///
    /// This uses a power function to make tension rise exponentially:
    /// - 10% at war = ~18% tension (local conflicts)
    /// - 25% at war = ~40% tension (regional wars)
    /// - 50% at war = ~70% tension (world crisis)
    /// - 75% at war = ~90% tension (near apocalypse)
    /// - 100% at war = 100% tension (total war)
    pub fn calculate_from_war_percentage(war_percentage: f32) -> f32 {
        // Use square root for exponential growth
        // This makes small conflicts barely register but large wars escalate rapidly
        war_percentage.sqrt().clamp(0.0, 1.0)
    }
}