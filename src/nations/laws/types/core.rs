//! Core law type definitions
//!
//! Contains the fundamental types for the law system including
//! the law structure itself, identifiers, and categories.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::effects::LawEffects;
use super::status::LawComplexity;
use crate::nations::GovernmentCategory;

/// Unique identifier for a law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LawId(pub u16);

impl LawId {
    pub const fn new(id: u16) -> Self {
        Self(id)
    }
}

/// Major categories of laws for organization and UI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
pub enum LawCategory {
    Economic,
    Military,
    Social,
    Religious,
    Criminal,
    Property,
    Immigration,
    Environmental,
    Technology,
    Cultural,
    Administrative,
    Diplomatic,
}

impl LawCategory {
    /// Get all categories as a slice for iteration
    pub const ALL: &'static [Self] = &[
        Self::Economic,
        Self::Military,
        Self::Social,
        Self::Religious,
        Self::Criminal,
        Self::Property,
        Self::Immigration,
        Self::Environmental,
        Self::Technology,
        Self::Cultural,
        Self::Administrative,
        Self::Diplomatic,
    ];

    /// Get a descriptive name for the category
    pub fn name(&self) -> &str {
        match self {
            Self::Economic => "Economic Laws",
            Self::Military => "Military Laws",
            Self::Social => "Social Laws",
            Self::Religious => "Religious Laws",
            Self::Criminal => "Criminal Justice",
            Self::Property => "Property Laws",
            Self::Immigration => "Immigration Laws",
            Self::Environmental => "Environmental Laws",
            Self::Technology => "Technology Laws",
            Self::Cultural => "Cultural Laws",
            Self::Administrative => "Administrative Laws",
            Self::Diplomatic => "Diplomatic Laws",
        }
    }

    /// Get a symbol for UI display (using ASCII art or letters)
    pub fn symbol(&self) -> &str {
        match self {
            Self::Economic => "$",
            Self::Military => "M",
            Self::Social => "S",
            Self::Religious => "R",
            Self::Criminal => "C",
            Self::Property => "P",
            Self::Immigration => "I",
            Self::Environmental => "E",
            Self::Technology => "T",
            Self::Cultural => "U",
            Self::Administrative => "A",
            Self::Diplomatic => "D",
        }
    }
}

/// Represents a single law that can be enacted by a nation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Law {
    /// Unique identifier
    pub id: LawId,

    /// Category for organization
    pub category: LawCategory,

    /// Display name (e.g., "Universal Healthcare")
    pub name: String,

    /// Detailed description of what the law does
    pub description: String,

    /// Mechanical effects when enacted
    pub effects: LawEffects,

    /// Prerequisites that must be met to pass this law
    pub prerequisites: Vec<LawPrerequisite>,

    /// Laws that cannot coexist with this one
    pub conflicts_with: Vec<LawId>,

    /// How much each government type likes/dislikes this law (-1.0 to 1.0)
    pub government_affinity: HashMap<GovernmentCategory, f32>,

    /// Complexity level affecting passage difficulty
    pub complexity: LawComplexity,

    /// Base popularity with the population
    pub base_popularity: f32,

    /// Whether this is a foundational law that's hard to repeal
    pub is_constitutional: bool,

    /// Year this law becomes available (for tech progression)
    pub available_from_year: i32,
}

/// Prerequisites for passing a law
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LawPrerequisite {
    /// Requires a certain government category
    GovernmentCategory(GovernmentCategory),
    /// Requires another law to be active
    RequiresLaw(LawId),
    /// Requires a minimum technology level
    TechnologyLevel(u32),
    /// Requires minimum stability
    MinimumStability(f32),
    /// Requires minimum legitimacy
    MinimumLegitimacy(f32),
    /// Requires a specific year to have passed
    YearReached(i32),
    /// Requires control of a minimum number of provinces
    MinimumProvinces(u32),
    /// Custom prerequisite with description
    Custom(String),
}