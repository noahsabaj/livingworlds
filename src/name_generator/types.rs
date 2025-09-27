//! Type definitions for the Living Worlds name generation system
//!
//! This module contains all the enums and types that define the parameters
//! for name generation, including cultures, roles, regions, and relationships.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// The type of name to generate, with context-specific parameters
#[derive(Debug, Clone)]
pub enum NameType {
    World,
    Nation {
        culture: Culture,
    },
    House {
        culture: Culture,
    },
    Province {
        region: Region,
        culture: Culture,
    },
    City {
        size: CitySize,
        culture: Culture,
    },
    Person {
        gender: Gender,
        culture: Culture,
        role: PersonRole,
    },
    River,
    Mountain,
    Ocean,
    Desert,
    Forest,
}

/// Cultural/linguistic styles for name generation
///
/// Each culture has its own naming patterns, titles, and linguistic characteristics
/// that affect how names are constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum Culture {
    /// European-inspired (knights, kingdoms, classical)
    Western,
    /// Asian-inspired (dynasties, honor, harmony)
    Eastern,
    /// Nordic/Slavic-inspired (clans, harsh winters, warriors)
    Northern,
    /// Mediterranean/African-inspired (trade, sun, ancient civilizations)
    Southern,
    /// Middle Eastern-inspired (oases, nomads, ancient empires)
    Desert,
    /// Polynesian/Caribbean-inspired (seafaring, tropical, island chains)
    Island,
    /// Lost civilization style (mysterious, advanced, forgotten)
    Ancient,
    /// Fantasy/magical names (arcane, otherworldly, mystical)
    Mystical,
}

/// Gender for person name generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    /// For cultures or roles without gender distinction
    Neutral,
}

/// Role/occupation for person name generation
///
/// Affects titles and sometimes name patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonRole {
    /// Kings, Queens, Emperors, etc.
    Ruler,
    /// Military commanders
    General,
    /// Ambassadors and negotiators
    Diplomat,
    /// Traders and business owners
    Merchant,
    /// Academics and researchers
    Scholar,
    /// Religious leaders
    Priest,
    /// Adventurers and discoverers
    Explorer,
    /// Court advisors and counselors
    Advisor,
    /// Nobility and aristocrats
    Noble,
    /// Regular citizens
    Commoner,
}

/// Geographical region types for place names
///
/// Affects prefixes and descriptors in generated names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Coastal,
    Mountain,
    Desert,
    Forest,
    Plains,
    River,
    Arctic,
    Tropical,
    Valley,
    Island,
}

/// City size categories affecting name generation
///
/// Larger settlements tend to have grander names
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitySize {
    /// < 100 people
    Hamlet,
    /// 100-1,000 people
    Village,
    /// 1,000-10,000 people
    Town,
    /// 10,000-100,000 people
    City,
    /// > 100,000 people
    Metropolis,
}

/// Relationship between names for derivative generation
///
/// Used when creating settlements related to existing ones
#[derive(Debug, Clone, Copy)]
pub enum NameRelation {
    /// "New X" - a colony or refounded settlement
    NewSettlement,
    /// "Old X" - the original when a "New X" exists
    OldSettlement,
    /// Derived from parent city
    ChildCity,
    /// Sister city with close ties
    TwinCity,
    /// Competing or rival settlement
    RivalCity,
}

impl Culture {
    /// Get a human-readable name for the culture
    pub fn name(&self) -> &'static str {
        match self {
            Culture::Western => "Western",
            Culture::Eastern => "Eastern",
            Culture::Northern => "Northern",
            Culture::Southern => "Southern",
            Culture::Desert => "Desert",
            Culture::Island => "Island",
            Culture::Ancient => "Ancient",
            Culture::Mystical => "Mystical",
        }
    }

    /// Get all available cultures
    pub fn all() -> &'static [Culture] {
        &[
            Culture::Western,
            Culture::Eastern,
            Culture::Northern,
            Culture::Southern,
            Culture::Desert,
            Culture::Island,
            Culture::Ancient,
            Culture::Mystical,
        ]
    }
}

impl Default for Culture {
    fn default() -> Self {
        Culture::Western
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Neutral
    }
}

impl Default for PersonRole {
    fn default() -> Self {
        PersonRole::Commoner
    }
}

impl Default for Region {
    fn default() -> Self {
        Region::Plains
    }
}

impl Default for CitySize {
    fn default() -> Self {
        CitySize::Village
    }
}
