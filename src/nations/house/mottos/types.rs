//! Core types for the motto generation system
//!
//! This module defines the fundamental data structures used throughout the motto system.
//! All other modules depend on these types, making this the foundation of the architecture.

use super::super::traits::DominantTrait;
use crate::name_generator::Culture;

/// A motto variation with rarity and optional requirements
///
/// This is the core data structure for all motto content. Each variation represents
/// a specific motto text with metadata about when it should be used.
#[derive(Debug, Clone)]
pub struct MottoVariation {
    pub text: &'static str,
    pub rarity: MottoRarity,
    pub min_trait: Option<f32>, // Minimum trait value (0.0-1.0) required
    pub min_prestige: Option<f32>, // Minimum prestige required
}

/// Rarity tiers for mottos with their selection weights
///
/// The rarity system creates a hierarchical selection mechanism where more prestigious
/// houses are more likely to get rare mottos, while common mottos are available to all.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MottoRarity {
    Common,    // 60% chance when selected - available to all houses
    Uncommon,  // 30% chance when selected - slightly distinctive
    Rare,      // 8% chance when selected - marks notable houses
    Legendary, // 2% chance when selected - reserved for truly exceptional houses
}

impl MottoRarity {
    /// Get the selection weight for this rarity tier
    ///
    /// These weights determine the probability distribution when selecting
    /// from eligible mottos. Higher weights = higher chance of selection.
    pub fn selection_weight(self) -> f32 {
        match self {
            MottoRarity::Common => 60.0,
            MottoRarity::Uncommon => 30.0,
            MottoRarity::Rare => 8.0,
            MottoRarity::Legendary => 2.0,
        }
    }

    /// Get a human-readable description of this rarity tier
    pub fn description(self) -> &'static str {
        match self {
            MottoRarity::Common => "Common mottos available to all houses",
            MottoRarity::Uncommon => "Distinctive mottos for notable houses",
            MottoRarity::Rare => "Prestigious mottos for exceptional houses",
            MottoRarity::Legendary => "Legendary mottos for the most elite houses",
        }
    }
}

impl MottoVariation {
    /// Create a common motto with no requirements
    ///
    /// This is the most basic constructor - creates a motto available to any house
    /// regardless of their traits or prestige level.
    pub const fn common(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Common,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create an uncommon motto with no special requirements
    ///
    /// Uncommon mottos are less likely to be selected but don't have strict requirements,
    /// making them accessible while still being distinctive.
    pub const fn uncommon(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Uncommon,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create a rare motto with no special requirements
    ///
    /// Rare mottos have low selection probability, making them naturally exclusive
    /// without requiring specific trait thresholds.
    pub const fn rare(text: &'static str) -> Self {
        Self {
            text,
            rarity: MottoRarity::Rare,
            min_trait: None,
            min_prestige: None,
        }
    }

    /// Create a legendary motto with strict requirements
    ///
    /// Legendary mottos require both high trait values AND high prestige,
    /// ensuring they're reserved for truly exceptional houses.
    pub const fn legendary(text: &'static str, min_trait: f32, min_prestige: f32) -> Self {
        Self {
            text,
            rarity: MottoRarity::Legendary,
            min_trait: Some(min_trait),
            min_prestige: Some(min_prestige),
        }
    }

    /// Check if this variation is eligible for a house with given traits and prestige
    ///
    /// This is the core eligibility check - a motto can only be selected if the house
    /// meets all the minimum requirements.
    pub fn is_eligible(&self, trait_value: f32, prestige: f32) -> bool {
        let trait_ok = self.min_trait.map_or(true, |min| trait_value >= min);
        let prestige_ok = self.min_prestige.map_or(true, |min| prestige >= min);
        trait_ok && prestige_ok
    }
}

/// Configuration for compound motto generation
///
/// Compound mottos combine two traits for houses that excel in multiple areas.
/// This represents the combination rules and thresholds.
#[derive(Debug, Clone)]
pub struct CompoundMottoConfig {
    /// Minimum number of strong traits (>0.7) required for compound consideration
    pub min_strong_traits: usize,
    /// Probability of generating compound motto when eligible
    pub compound_probability: f32,
    /// Threshold for considering a trait "strong"
    pub strong_trait_threshold: f32,
}

impl Default for CompoundMottoConfig {
    fn default() -> Self {
        Self {
            min_strong_traits: 2,
            compound_probability: 0.3, // 30% chance
            strong_trait_threshold: 0.7,
        }
    }
}

/// Trait combination for compound mottos
///
/// Represents a specific pairing of traits that can create unique compound mottos.
/// Some combinations have special cultural variations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitCombination {
    pub primary: DominantTrait,
    pub secondary: DominantTrait,
}

impl TraitCombination {
    /// Create a new trait combination, automatically ordering traits for consistency
    ///
    /// This ensures that (Martial, Piety) and (Piety, Martial) are treated as the same combination.
    pub fn new(trait_a: DominantTrait, trait_b: DominantTrait) -> Self {
        // Order traits consistently to avoid duplicate combinations
        if (trait_a as u8) <= (trait_b as u8) {
            Self {
                primary: trait_a,
                secondary: trait_b,
            }
        } else {
            Self {
                primary: trait_b,
                secondary: trait_a,
            }
        }
    }

    /// Check if this combination has special cultural variations
    ///
    /// Some trait combinations (like Martial+Piety) have unique mottos that vary by culture,
    /// while others use generic compound templates.
    pub fn has_cultural_variations(self) -> bool {
        use DominantTrait::*;
        matches!(
            self,
            TraitCombination {
                primary: Martial,
                secondary: Piety
            } | TraitCombination {
                primary: Stewardship,
                secondary: Intrigue
            } | TraitCombination {
                primary: Learning,
                secondary: Piety
            } | TraitCombination {
                primary: Martial,
                secondary: Stewardship
            } | TraitCombination {
                primary: Diplomacy,
                secondary: Intrigue
            }
        )
    }
}

/// Error types for motto generation system
#[derive(Debug, Clone)]
pub enum MottoError {
    /// No eligible variations found for the given requirements
    NoEligibleVariations {
        trait_type: DominantTrait,
        culture: Culture,
    },
    /// Invalid trait value (outside 0.0-1.0 range)
    InvalidTraitValue { value: f32 },
    /// Invalid prestige value (outside 0.0-1.0 range)
    InvalidPrestige { value: f32 },
    /// Data consistency error in motto variations
    DataInconsistency { message: String },
}

impl std::fmt::Display for MottoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MottoError::NoEligibleVariations {
                trait_type,
                culture,
            } => {
                write!(
                    f,
                    "No eligible motto variations found for {:?} trait in {:?} culture",
                    trait_type, culture
                )
            }
            MottoError::InvalidTraitValue { value } => {
                write!(
                    f,
                    "Invalid trait value: {} (must be between 0.0 and 1.0)",
                    value
                )
            }
            MottoError::InvalidPrestige { value } => {
                write!(
                    f,
                    "Invalid prestige value: {} (must be between 0.0 and 1.0)",
                    value
                )
            }
            MottoError::DataInconsistency { message } => {
                write!(f, "Data consistency error: {}", message)
            }
        }
    }
}

impl std::error::Error for MottoError {}
