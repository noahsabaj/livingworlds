//! Core nation and house types
//!
//! This module defines the fundamental structures for nations and dynasties,
//! representing the political entities that control provinces in the world.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

/// Unique identifier for a nation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Component)]
pub struct NationId(pub u32);

impl NationId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for NationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A political entity that controls territory and has a ruling house
///
/// Uses Bevy 0.16 Component Hooks for automatic cache cleanup when removed
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Component)]
/// A nation in the world with territory, government, and economy.
///
/// ## Province Ownership vs Territory Grouping
///
/// Living Worlds uses a two-level ownership model:
///
/// - **Province.owner_entity**: Source of truth for "who owns this province"
///   - Direct field on each Province struct in ProvinceStorage
///   - Used for rendering (map colors), AI decisions, and game logic
///   - Query with `crate::nations::ownership` utilities
///
/// - **OwnedBy/OwnsTerritory**: For Territory component relationships
///   - Territory = contiguous group of provinces (uses Bevy relationships)
///   - One nation can have multiple territories (mainland + colonies)
///   - Used for pathfinding, naval connections, territorial integrity checks
///
/// This separation allows efficient queries (provinces by direct lookup)
/// while maintaining logical groupings (territories for game mechanics).
pub struct Nation {
    pub name: String,
    pub adjective: String, // "French" for "France"
    pub color: Color,
    pub capital_province: u32,

    // Economic and military strength
    pub treasury: f32,
    pub tax_rate: f32, // 0.0 to 1.0 (0% to 100%)
    pub military_strength: f32,
    pub stability: f32, // 0.0 to 1.0

    // Cultural identity from nation's capital province
    pub culture: crate::name_generator::Culture,

    // Technological advancement
    pub technology_level: u32,

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

/// Economy component for nations
///
/// Stores economic modifiers that affect tax collection, production,
/// and trade. These values are multipliers applied to base calculations.
/// Law effects modify these values to implement economic policies.
#[derive(Component, Debug, Clone, Reflect)]
pub struct Economy {
    /// Efficiency of tax collection (0.0 = no tax, 1.0 = base, 2.0 = double)
    pub tax_efficiency: f32,
    /// Industrial production multiplier
    pub industrial_multiplier: f32,
    /// Agricultural production multiplier
    pub agricultural_multiplier: f32,
    /// Trade income multiplier
    pub trade_multiplier: f32,
    /// Base maintenance cost per turn
    pub maintenance_cost: f32,
}

impl Default for Economy {
    fn default() -> Self {
        Self {
            tax_efficiency: 1.0,
            industrial_multiplier: 1.0,
            agricultural_multiplier: 1.0,
            trade_multiplier: 1.0,
            maintenance_cost: 100.0,
        }
    }
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
    pub economy: Economy,
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

