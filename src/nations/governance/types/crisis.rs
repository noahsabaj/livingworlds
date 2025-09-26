//! Crisis tracking for government legitimacy
//!
//! This module contains types for tracking various crises that impact
//! government legitimacy and stability.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Tracks various crises that impact government legitimacy
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct CrisisFactors {
    pub famine: bool,
    pub plague: bool,
    pub civil_unrest: bool,
    pub lost_capital: bool,
    pub economic_collapse: bool,
    pub military_coup_attempt: bool,
    pub recent_defeat: bool,
    pub succession_crisis: bool,
}