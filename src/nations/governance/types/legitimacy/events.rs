//! Legitimacy event types
//!
//! This module contains types for tracking events that affect legitimacy.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Legitimacy event that occurred
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct LegitimacyEvent {
    pub event_type: LegitimacyEventType,
    pub impact: f32,
    pub days_ago: u32,
    pub description: String,
}

/// Types of legitimacy events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum LegitimacyEventType {
    // Positive events
    MilitaryVictory,
    EconomicBoom,
    DiplomaticSuccess,
    PopularReform,
    ReligiousBlessing,
    TechnologicalBreakthrough,

    // Negative events
    MilitaryDefeat,
    EconomicCrisis,
    DiplomaticHumiliation,
    UnpopularPolicy,
    ReligiousCensure,
    NaturalDisaster,

    // Neutral/Mixed events
    GovernmentReform,
    SuccessionChange,
    InternationalIntervention,
}