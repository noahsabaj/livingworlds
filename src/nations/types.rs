//! Core nation and dynasty types
//!
//! This module defines the fundamental structures for nations and dynasties,
//! representing the political entities that control provinces in the world.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Unique identifier for a nation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Component, Reflect)]
pub struct NationId(pub u32);

impl NationId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// A political entity that controls territory and has a ruling dynasty
#[derive(Debug, Clone, Component, Serialize, Deserialize, Reflect)]
pub struct Nation {
    pub id: NationId,
    pub name: String,
    pub adjective: String,  // "French" for "France"
    pub color: Color,
    pub capital_province: u32,
    // NOTE: Province ownership is stored in Province.owner, not here
    // Use ProvinceOwnershipCache resource for efficient queries

    // Economic and military strength
    pub treasury: f32,
    pub military_strength: f32,
    pub stability: f32,  // 0.0 to 1.0

    // Personality for AI decisions
    pub personality: NationPersonality,
}

/// Personality traits that drive nation behavior
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NationPersonality {
    pub aggression: f32,     // -1.0 (pacifist) to 1.0 (warmonger)
    pub expansionism: f32,   // -1.0 (isolationist) to 1.0 (imperialist)
    pub diplomacy: f32,      // -1.0 (hostile) to 1.0 (friendly)
    pub mercantilism: f32,   // -1.0 (closed) to 1.0 (free trade)
}

impl NationPersonality {
    /// Create a random personality
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self {
            aggression: rng.gen_range(-1.0..1.0),
            expansionism: rng.gen_range(-1.0..1.0),
            diplomacy: rng.gen_range(-1.0..1.0),
            mercantilism: rng.gen_range(-1.0..1.0),
        }
    }

    /// Create a balanced personality
    pub fn balanced() -> Self {
        Self {
            aggression: 0.0,
            expansionism: 0.0,
            diplomacy: 0.0,
            mercantilism: 0.0,
        }
    }
}

/// Bundle for spawning a nation entity
#[derive(Bundle)]
pub struct NationBundle {
    pub nation: Nation,
    pub transform: Transform,
    pub visibility: Visibility,
}

/// Resource tracking all nations
#[derive(Resource, Default)]
pub struct NationRegistry {
    pub nations: Vec<Nation>,
    pub nation_id_counter: u32,
}

impl NationRegistry {
    pub fn create_nation_id(&mut self) -> NationId {
        let id = NationId::new(self.nation_id_counter);
        self.nation_id_counter += 1;
        id
    }
}

/// Cached index of province ownership for efficient queries
/// This is rebuilt from Province.owner when territory changes
#[derive(Resource, Default)]
pub struct ProvinceOwnershipCache {
    /// Map from nation ID to set of owned province IDs
    pub by_nation: std::collections::HashMap<NationId, std::collections::HashSet<u32>>,
    /// Version counter to track when cache needs rebuilding
    pub version: u32,
}

impl ProvinceOwnershipCache {
    /// Get all provinces owned by a nation
    pub fn get_nation_provinces(&self, nation_id: NationId) -> Option<&std::collections::HashSet<u32>> {
        self.by_nation.get(&nation_id)
    }

    /// Count provinces owned by a nation
    pub fn count_nation_provinces(&self, nation_id: NationId) -> usize {
        self.by_nation.get(&nation_id).map_or(0, |set| set.len())
    }

    /// Rebuild cache from province data
    pub fn rebuild(&mut self, provinces: &[crate::world::Province]) {
        self.by_nation.clear();

        for province in provinces {
            if let Some(owner) = province.owner {
                self.by_nation
                    .entry(owner)
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(province.id.value());
            }
        }

        self.version += 1;
    }
}

/// Settings for nation generation
#[derive(Resource, Clone)]
pub struct NationGenerationSettings {
    pub nation_count: u32,
    pub nation_density: NationDensity,
    pub starting_development: StartingDevelopment,
    pub aggression_level: f32,
}

impl Default for NationGenerationSettings {
    fn default() -> Self {
        Self {
            nation_count: 20,
            nation_density: NationDensity::Balanced,
            starting_development: StartingDevelopment::Medieval,
            aggression_level: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NationDensity {
    Sparse,      // Large empires
    Balanced,    // Mix of sizes
    Fragmented,  // Many small states
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartingDevelopment {
    Primitive,
    Medieval,
    Renaissance,
    Mixed,
}