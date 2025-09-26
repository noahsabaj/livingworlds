//! Political pressure tracking
//!
//! This module contains types for tracking various political pressures
//! that can lead to government transitions or reforms.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component tracking political pressure sources
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, Reflect)]
pub struct PoliticalPressure {
    pub economic_crisis: f32,
    pub military_defeat: f32,
    pub cultural_shift: f32,
    pub external_influence: f32,
    pub technological_change: f32,
    pub religious_fervor: f32,
    pub revolutionary_ideas: f32,
}