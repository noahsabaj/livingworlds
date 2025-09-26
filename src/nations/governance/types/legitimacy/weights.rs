//! Legitimacy weight system
//!
//! This module contains the weight system that determines how different
//! government types value different legitimacy factors.

use serde::{Deserialize, Serialize};

use crate::nations::governance::{GovernmentCategory, GovernmentType};

/// Weights for different legitimacy factors based on government type
#[derive(Debug, Clone)]
pub struct LegitimacyWeights {
    // Primary legitimacy sources
    pub electoral: f32,
    pub divine: f32,
    pub revolutionary: f32,

    // Universal factors
    pub prosperity: f32,
    pub efficiency: f32,
    pub popularity: f32,
    pub tradition: f32,
    pub institutional: f32,
    pub diplomatic: f32,
    pub military: f32,

    // Penalty sensitivities
    pub crisis_sensitivity: f32,
    pub succession: f32,
    pub corruption: f32,
    pub trust: f32,
    pub unity: f32,
    pub minority_unrest: f32,
    pub foreign_perception: f32,
}

impl LegitimacyWeights {
    /// Get appropriate weights for a government type
    pub fn for_government_type(gov_type: GovernmentType) -> Self {
        match gov_type.category() {
            GovernmentCategory::Democratic => Self {
                electoral: 0.4,
                divine: 0.0,
                revolutionary: 0.0,
                prosperity: 0.25,
                efficiency: 0.2,
                popularity: 0.3,
                tradition: 0.05,
                institutional: 0.15,
                diplomatic: 0.2,
                military: 0.1,
                crisis_sensitivity: 1.0,
                succession: 0.05,
                corruption: 0.35,
                trust: 0.4,
                unity: 0.25,
                minority_unrest: 0.3,
                foreign_perception: 0.25,
            },

            GovernmentCategory::Autocratic => Self {
                electoral: 0.0,
                divine: 0.0,
                revolutionary: 0.0,
                prosperity: 0.15,
                efficiency: 0.25,
                popularity: 0.1,
                tradition: 0.15,
                institutional: 0.35,
                diplomatic: 0.1,
                military: 0.3,
                crisis_sensitivity: 0.7,
                succession: 0.3,
                corruption: 0.1,
                trust: 0.05,
                unity: 0.35,
                minority_unrest: 0.15,
                foreign_perception: 0.1,
            },

            // Simplified - full implementation would have all categories
            _ => Self {
                electoral: 0.0,
                divine: 0.0,
                revolutionary: 0.0,
                prosperity: 0.2,
                efficiency: 0.2,
                popularity: 0.2,
                tradition: 0.1,
                institutional: 0.2,
                diplomatic: 0.15,
                military: 0.15,
                crisis_sensitivity: 0.8,
                succession: 0.2,
                corruption: 0.2,
                trust: 0.2,
                unity: 0.2,
                minority_unrest: 0.2,
                foreign_perception: 0.15,
            },
        }
    }
}