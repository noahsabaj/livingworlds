//! Main governance component
//!
//! This module contains the primary Governance component that tracks
//! a nation's government type and political state.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::government::GovernmentType;
use super::legitimacy::LegitimacyFactors;

/// Component that tracks a nation's governance
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Governance {
    pub government_type: GovernmentType,
    pub stability: f32,                        // 0.0-1.0
    pub reform_pressure: f32,                  // Pressure to change government
    pub tradition_strength: f32,               // Resistance to change
    pub last_transition: Option<u32>,          // Game time of last change
    pub days_in_power: u32,                    // Days since last government change
    pub legitimacy: f32,                       // Cached legitimacy value (0.0-1.0)
    pub legitimacy_trend: f32,                 // Rate of change (-1.0 to 1.0)
    pub legitimacy_factors: LegitimacyFactors, // Comprehensive legitimacy tracking
}