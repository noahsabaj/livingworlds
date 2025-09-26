//! Governance configuration and settings
//!
//! This module contains types for configuring governance generation
//! and simulation parameters.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Settings for governance generation
#[derive(Resource, Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct GovernanceSettings {
    pub starting_government_weights: GovernmentWeights,
    pub allow_revolutions: bool,
    pub revolution_threshold: f32,
    pub peaceful_transition_chance: f32,
}

/// Weights for initial government type selection
#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct GovernmentWeights {
    pub anarchist: f32,
    pub socialist: f32,
    pub fascist: f32,
    pub democratic: f32,
    pub economic: f32,
    pub religious: f32,
    pub traditional: f32,
    pub special: f32,
}

impl Default for GovernanceSettings {
    fn default() -> Self {
        Self {
            starting_government_weights: GovernmentWeights {
                anarchist: 0.02,
                socialist: 0.05,
                fascist: 0.03,
                democratic: 0.1,
                economic: 0.1,
                religious: 0.2,
                traditional: 0.5, // Most common historically
                special: 0.0,
            },
            allow_revolutions: true,
            revolution_threshold: 0.8,
            peaceful_transition_chance: 0.3,
        }
    }
}