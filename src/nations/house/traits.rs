//! House traits and archetype system
//!
//! This module handles the inheritable characteristics that define a house's
//! strengths, weaknesses, and behavioral tendencies across generations.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Inheritable traits of a noble house
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct HouseTraits {
    /// Military prowess runs in the family
    pub martial: f32,

    /// Administrative and economic acumen
    pub stewardship: f32,

    /// Diplomatic skill and charisma
    pub diplomacy: f32,

    /// Learning and technological advancement
    pub learning: f32,

    /// Tendency toward treachery and schemes
    pub intrigue: f32,

    /// Religious devotion
    pub piety: f32,
}

impl HouseTraits {
    /// Create random house traits
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self {
            martial: rng.gen_range(0.0..1.0),
            stewardship: rng.gen_range(0.0..1.0),
            diplomacy: rng.gen_range(0.0..1.0),
            learning: rng.gen_range(0.0..1.0),
            intrigue: rng.gen_range(0.0..1.0),
            piety: rng.gen_range(0.0..1.0),
        }
    }

    /// Create traits weighted toward a specific archetype
    pub fn archetype(archetype: HouseArchetype, rng: &mut impl rand::Rng) -> Self {
        match archetype {
            HouseArchetype::Warrior => Self {
                martial: 0.6 + rng.gen_range(0.0..0.4),
                stewardship: rng.gen_range(0.0..0.6),
                diplomacy: rng.gen_range(0.0..0.6),
                learning: rng.gen_range(0.0..0.5),
                intrigue: rng.gen_range(0.0..0.5),
                piety: rng.gen_range(0.0..0.6),
            },
            HouseArchetype::Merchant => Self {
                martial: rng.gen_range(0.0..0.5),
                stewardship: 0.6 + rng.gen_range(0.0..0.4),
                diplomacy: 0.4 + rng.gen_range(0.0..0.4),
                learning: rng.gen_range(0.2..0.7),
                intrigue: rng.gen_range(0.1..0.6),
                piety: rng.gen_range(0.0..0.4),
            },
            HouseArchetype::Scholar => Self {
                martial: rng.gen_range(0.0..0.4),
                stewardship: rng.gen_range(0.2..0.7),
                diplomacy: rng.gen_range(0.2..0.6),
                learning: 0.6 + rng.gen_range(0.0..0.4),
                intrigue: rng.gen_range(0.1..0.5),
                piety: rng.gen_range(0.2..0.8),
            },
            HouseArchetype::Zealot => Self {
                martial: rng.gen_range(0.3..0.8),
                stewardship: rng.gen_range(0.0..0.5),
                diplomacy: rng.gen_range(0.0..0.4),
                learning: rng.gen_range(0.1..0.6),
                intrigue: rng.gen_range(0.0..0.5),
                piety: 0.7 + rng.gen_range(0.0..0.3),
            },
            HouseArchetype::Schemer => Self {
                martial: rng.gen_range(0.1..0.6),
                stewardship: rng.gen_range(0.2..0.7),
                diplomacy: rng.gen_range(0.3..0.7),
                learning: rng.gen_range(0.2..0.6),
                intrigue: 0.6 + rng.gen_range(0.0..0.4),
                piety: rng.gen_range(0.0..0.4),
            },
        }
    }

    /// Get the dominant trait for this house
    pub fn dominant_trait(&self) -> DominantTrait {
        let max_trait = self
            .martial
            .max(self.stewardship)
            .max(self.diplomacy)
            .max(self.learning)
            .max(self.intrigue)
            .max(self.piety);

        if max_trait == self.martial {
            DominantTrait::Martial
        } else if max_trait == self.stewardship {
            DominantTrait::Stewardship
        } else if max_trait == self.diplomacy {
            DominantTrait::Diplomacy
        } else if max_trait == self.learning {
            DominantTrait::Learning
        } else if max_trait == self.intrigue {
            DominantTrait::Intrigue
        } else {
            DominantTrait::Piety
        }
    }
}

/// Archetypes for house generation
#[derive(Debug, Clone, Copy)]
pub enum HouseArchetype {
    Warrior,
    Merchant,
    Scholar,
    Zealot,
    Schemer,
}

/// The dominant trait that defines a house's character
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DominantTrait {
    Martial,
    Stewardship,
    Diplomacy,
    Learning,
    Intrigue,
    Piety,
}
