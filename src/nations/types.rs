//! Core nation and house types
//!
//! This module defines the fundamental structures for nations and dynasties,
//! representing the political entities that control provinces in the world.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// A political entity that controls territory and has a ruling house
///
/// Uses Bevy 0.16 Component Hooks for automatic cache cleanup when removed
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Component)]
pub struct Nation {
    pub name: String,
    pub adjective: String, // "French" for "France"
    pub color: Color,
    pub capital_province: u32,
    // NOTE: Province ownership is stored in Province.owner, not here
    // Use ProvinceOwnershipCache resource for efficient queries

    // Economic and military strength
    pub treasury: f32,
    pub tax_rate: f32, // 0.0 to 1.0 (0% to 100%)
    pub military_strength: f32,
    pub stability: f32, // 0.0 to 1.0

    // Cultural identity from nation's capital province
    pub culture: crate::name_generator::Culture,

    // Personality for AI decisions
    pub personality: NationPersonality,
}

/// Personality traits that drive nation behavior
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NationPersonality {
    pub aggression: f32,   // -1.0 (pacifist) to 1.0 (warmonger)
    pub expansionism: f32, // -1.0 (isolationist) to 1.0 (imperialist)
    pub diplomacy: f32,    // -1.0 (hostile) to 1.0 (friendly)
    pub mercantilism: f32, // -1.0 (closed) to 1.0 (free trade)
}

impl NationPersonality {
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self {
            aggression: rng.gen_range(-1.0..1.0),
            expansionism: rng.gen_range(-1.0..1.0),
            diplomacy: rng.gen_range(-1.0..1.0),
            mercantilism: rng.gen_range(-1.0..1.0),
        }
    }

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
    pub pressure_vector: crate::simulation::PressureVector,
    pub history: super::NationHistory,
    pub laws: super::laws::NationLaws,
}

/// Resource tracking all nations
#[derive(Resource)]
pub struct NationRegistry {
    pub nations: Vec<Nation>,
    pub nation_id_counter: std::sync::Arc<std::sync::atomic::AtomicU32>,
}

impl Default for NationRegistry {
    fn default() -> Self {
        Self {
            nations: Vec::new(),
            nation_id_counter: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }
}

impl NationRegistry {
    /// Thread-safe nation ID creation using atomic operations
    pub fn create_nation_id(&self) -> NationId {
        let id = self
            .nation_id_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        NationId::new(id)
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
    Sparse,     // Large empires
    Balanced,   // Mix of sizes
    Fragmented, // Many small states
}

impl fmt::Display for NationDensity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NationDensity::Sparse => write!(f, "Sparse"),
            NationDensity::Balanced => write!(f, "Balanced"),
            NationDensity::Fragmented => write!(f, "Fragmented"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartingDevelopment {
    Primitive,
    Medieval,
    Renaissance,
    Mixed,
}

/// A territory is a group of contiguous provinces owned by same nation
#[derive(Component, Debug, Clone)]
pub struct Territory {
    pub provinces: HashSet<u32>, // The province IDs in this territory
    pub nation_id: NationId,
    pub center: Vec2,  // Geographic center
    pub is_core: bool, // Core territory vs conquered
}

/// Entity Relationships for Territory ownership
/// Territory is owned by a Nation - uses Bevy 0.16 automatic bidirectional tracking
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = OwnsTerritory)] // THIS enables automatic tracking!
pub struct OwnedBy(pub Entity);

/// Nation owns territories - automatically maintained by Bevy!
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = OwnedBy, linked_spawn)] // THIS creates the magic!
pub struct OwnsTerritory(Vec<Entity>); // Private for safety - Bevy handles access

impl OwnsTerritory {
    /// Safe read-only access to territories
    pub fn territories(&self) -> &[Entity] {
        &self.0
    }

    /// Check if nation owns any territories
    pub fn has_territories(&self) -> bool {
        !self.0.is_empty()
    }

    /// Count of territories owned
    pub fn territory_count(&self) -> usize {
        self.0.len()
    }
}
