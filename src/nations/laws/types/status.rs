//! Law status and tracking types
//!
//! Contains types for tracking the status of laws including
//! proposals, voting, and popularity.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Current status of a law in a nation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum LawStatus {
    /// Law is not enacted
    Inactive,
    /// Law is being debated/voted on
    Proposed {
        support: f32,
        days_remaining: f32,
    },
    /// Law is being implemented
    Implementing {
        progress: f32,
        days_remaining: f32,
    },
    /// Law is fully active
    Active {
        enacted_date: i32,
        popularity: f32,
    },
    /// Law is being repealed
    Repealing {
        progress: f32,
        days_remaining: f32,
    },
}

/// Complexity of implementing a law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum LawComplexity {
    Trivial,    // Easy to pass (e.g., declaring holidays)
    Simple,     // Minor bureaucratic effort
    Moderate,   // Significant reform needed
    Complex,    // Major societal change
    Revolutionary, // Complete system overhaul
}

impl LawComplexity {
    /// Time in days to implement this law
    pub fn implementation_time(&self) -> f32 {
        match self {
            Self::Trivial => 30.0,
            Self::Simple => 90.0,
            Self::Moderate => 180.0,
            Self::Complex => 365.0,
            Self::Revolutionary => 730.0,
        }
    }

    /// Difficulty modifier for passing this law
    pub fn difficulty_modifier(&self) -> f32 {
        match self {
            Self::Trivial => 0.9,
            Self::Simple => 0.7,
            Self::Moderate => 0.5,
            Self::Complex => 0.3,
            Self::Revolutionary => 0.1,
        }
    }
}

/// How popular a law is with different groups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawPopularity {
    /// Support from the general population
    pub popular_support: f32,
    /// Support from the nobility/elite
    pub elite_support: f32,
    /// Support from the military
    pub military_support: f32,
    /// Support from religious authorities
    pub religious_support: f32,
    /// Support from merchants/traders
    pub merchant_support: f32,
}

impl LawPopularity {
    /// Calculate overall support weighted by government type
    pub fn weighted_support(&self, weights: &super::effects::PopularityWeights) -> f32 {
        self.popular_support * weights.popular_weight
            + self.elite_support * weights.elite_weight
            + self.military_support * weights.military_weight
            + self.religious_support * weights.religious_weight
            + self.merchant_support * weights.merchant_weight
    }
}