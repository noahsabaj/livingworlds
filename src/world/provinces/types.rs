//! Province data structure and related types
//!
//! This module contains the core Province struct and its type-safe wrappers.
//! Provinces represent individual hexagonal tiles in the game world.

use super::super::terrain::TerrainType;
use crate::world::MineralType;
use crate::constants::PROVINCE_MIN_POPULATION;
use crate::name_generator::Culture;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};
use std::fmt;

// TYPE-SAFE WRAPPERS - Zero-cost abstractions for compile-time validation

/// Province entity marker component
/// Links an entity to its data in ProvinceStorage
#[derive(Component, Debug, Clone, Copy)]
pub struct ProvinceEntity {
    /// Index into ProvinceStorage.provinces array
    pub storage_index: usize,
    /// Province ID for quick lookups
    pub id: ProvinceId,
}

/// Type-safe province identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct ProvinceId(pub u32);

impl ProvinceId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ProvinceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Province#{}", self.0)
    }
}

impl Default for ProvinceId {
    fn default() -> Self {
        Self(0)
    }
}

/// Type-safe elevation with automatic clamping to [0.0, 1.0]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Elevation(f32);

impl Elevation {
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    /// Check if this is sea level
    pub fn is_sea_level(&self) -> bool {
        self.0 < 0.1
    }

    /// Check if this is mountain height
    pub fn is_mountain(&self) -> bool {
        self.0 > 0.65
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Self(0.5)
    }
}

impl fmt::Display for Elevation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// Type-safe agriculture value with validation [0.0, 3.0]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Agriculture(f32);

impl Agriculture {
    /// Minimum agriculture value (barren land)
    pub const MIN: f32 = 0.0;

    /// Maximum agriculture value (most fertile land)
    pub const MAX: f32 = 3.0;

    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    /// Check if land is barren (= 0.0)
    pub fn is_barren(&self) -> bool {
        self.0 == Self::MIN
    }

    /// Check if land is arable (>= 0.5)
    pub fn is_arable(&self) -> bool {
        self.0 >= 0.5
    }

    /// Check if land is fertile (>= 1.5)
    pub fn is_fertile(&self) -> bool {
        self.0 >= 1.5
    }

    /// Check if land is very fertile (>= 2.5)
    pub fn is_very_fertile(&self) -> bool {
        self.0 >= 2.5
    }
}

impl Default for Agriculture {
    fn default() -> Self {
        Self(0.5)
    }
}

impl fmt::Display for Agriculture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

/// Type-safe distance measurement with special infinite value
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Distance(f32);

impl Distance {
    /// Maximum valid distance
    pub const MAX: f32 = 10000.0;

    pub fn new(value: f32) -> Self {
        if value < 0.0 {
            Self(0.0)
        } else if value > Self::MAX {
            Self(f32::INFINITY)
        } else {
            Self(value)
        }
    }

    pub fn infinite() -> Self {
        Self(f32::INFINITY)
    }

    pub fn value(&self) -> f32 {
        self.0
    }

    /// Check if distance is infinite
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    /// Check if within range
    pub fn within(&self, range: f32) -> bool {
        self.0 <= range
    }
}

impl Default for Distance {
    fn default() -> Self {
        Self(0.0)
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_infinite() {
            write!(f, "∞")
        } else {
            write!(f, "{:.1}", self.0)
        }
    }
}

/// Type-safe mineral abundance percentage [0-100]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Reflect, Serialize, Deserialize)]
pub struct Abundance(u8);

impl Abundance {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    /// Check if there's any abundance
    pub fn has_any(&self) -> bool {
        self.0 > 0
    }

    /// Check if this is rich (> 75)
    pub fn is_rich(&self) -> bool {
        self.0 > 75
    }

    /// Get as normalized float [0.0, 1.0]
    pub fn normalized(&self) -> f32 {
        self.0 as f32 / 100.0
    }
}

impl Default for Abundance {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for Abundance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl From<u8> for Abundance {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

/// The 6 directions for hexagonal neighbors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HexDirection {
    NorthEast = 0,
    East = 1,
    SouthEast = 2,
    SouthWest = 3,
    West = 4,
    NorthWest = 5,
}

/// Province represents a single hexagonal tile in the world
///
/// Provinces are NOT entities in the mega-mesh architecture.
/// They are stored in the ProvinceStorage resource as a Vec.
#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Province {
    // === Identity & Location ===
    /// Unique identifier for this province
    pub id: ProvinceId,

    /// World position in 2D space
    pub position: Vec2,

    /// Nation entity that owns/controls this province
    pub owner_entity: Option<Entity>,

    /// Cultural identity of this province (assigned based on geography)
    pub culture: Option<Culture>,

    // === Population (8 bytes) ===
    /// Current population (now properly an integer)
    pub population: u32,

    /// Maximum population this province can support
    pub max_population: u32,

    // === Terrain & Geography (aligned) ===
    /// Terrain type determining base characteristics
    pub terrain: TerrainType,

    /// Elevation from 0.0 (sea level) to 1.0 (highest peaks)
    pub elevation: Elevation,

    /// Food production capacity
    pub agriculture: Agriculture,

    /// Distance to nearest river/delta in hexagon units
    pub fresh_water_distance: Distance,

    // === Mineral Resources (7 bytes, will be padded to 8) ===
    /// Iron abundance - Common, used for tools and weapons
    pub iron: Abundance,

    /// Copper abundance - Common, used for bronze
    pub copper: Abundance,

    /// Tin abundance - Rare, essential for bronze
    pub tin: Abundance,

    /// Gold abundance - Rare, used for currency
    pub gold: Abundance,

    /// Coal abundance - Common in lowlands, fuel source
    pub coal: Abundance,

    /// Stone abundance - Ubiquitous, building material
    pub stone: Abundance,

    /// Gems abundance - Very rare, luxury goods
    pub gems: Abundance,

    // === Spatial Relationships (96 bytes total) ===
    /// IDs of the 6 neighboring hexagons (NE, E, SE, SW, W, NW)
    /// None if neighbor is off-map or doesn't exist
    pub neighbors: [Option<ProvinceId>; 6],

    /// Direct indices into the provinces array for O(1) neighbor access
    /// Precomputed during generation to avoid HashMap lookups
    pub neighbor_indices: [Option<usize>; 6],

    // === Change Tracking (8 bytes) ===
    /// Version number incremented on each change
    pub version: u32,

    /// Dirty flag for systems that need to track changes
    pub dirty: bool,
}

impl Default for Province {
    fn default() -> Self {
        Self {
            id: ProvinceId::default(),
            position: Vec2::ZERO,
            owner_entity: None,
            culture: None,
            population: PROVINCE_MIN_POPULATION,
            max_population: PROVINCE_MIN_POPULATION * 10,
            terrain: TerrainType::TemperateGrassland,
            elevation: Elevation::default(),
            agriculture: Agriculture::default(),
            fresh_water_distance: Distance::infinite(),
            iron: Abundance::default(),
            copper: Abundance::default(),
            tin: Abundance::default(),
            gold: Abundance::default(),
            coal: Abundance::default(),
            stone: Abundance::default(),
            gems: Abundance::default(),
            neighbors: [None; 6],
            neighbor_indices: [None; 6],
            version: 0,
            dirty: false,
        }
    }
}

impl Province {
    pub fn new(id: ProvinceId, position: Vec2) -> Self {
        Self {
            id,
            position,
            ..Default::default()
        }
    }

    /// Mark this province as modified
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
        self.version = self.version.wrapping_add(1);
    }

    /// Clear the dirty flag
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    /// Set population with validation and change tracking
    /// Returns Some with old and new values if changed, None otherwise
    pub fn set_population(&mut self, population: u32) -> Option<(u32, u32)> {
        if self.population != population {
            let old_population = self.population;
            self.population = population.min(self.max_population);
            self.mark_dirty();
            Some((old_population, self.population))
        } else {
            None
        }
    }

    /// Check if this province is habitable
    pub fn is_habitable(&self) -> bool {
        self.terrain != TerrainType::Ocean
    }

    /// Check if this province has fresh water access
    pub fn has_fresh_water(&self) -> bool {
        self.terrain == TerrainType::River || self.fresh_water_distance.within(2.0)
    }

    /// Calculate population growth multiplier based on terrain and resources
    pub fn growth_multiplier(&self) -> f32 {
        let base = match self.terrain {
            TerrainType::River => 2.5,
            TerrainType::TropicalRainforest | TerrainType::TemperateRainforest => 1.5,
            TerrainType::TemperateGrassland | TerrainType::Savanna => 1.2,
            TerrainType::Ocean => 0.0,
            TerrainType::PolarDesert | TerrainType::TropicalDesert => 0.3,
            _ => 1.0,
        };

        // Modify by agriculture
        base * (0.5 + self.agriculture.value() / 3.0)
    }

    /// Get mineral abundance for a specific mineral type
    pub fn get_mineral_abundance(&self, mineral_type: MineralType) -> Option<u8> {
        let abundance = match mineral_type {
            MineralType::Iron => self.iron.value(),
            MineralType::Copper => self.copper.value(),
            MineralType::Tin => self.tin.value(),
            MineralType::Gold => self.gold.value(),
            MineralType::Coal => self.coal.value(),
            MineralType::Stone => self.stone.value(),
            MineralType::Gems => self.gems.value(),
            _ => 0,
        };

        if abundance > 0 {
            Some(abundance)
        } else {
            None
        }
    }
}

// ================================================================================================
// ECS PROVINCE COMPONENTS - For full entity-based province system
// ================================================================================================

/// Marker component for province entities
///
/// Used to identify province entities in queries without loading data.
/// All province entities must have this component.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
pub struct ProvinceMarker;

/// ALL province data in a single component (cache-optimal for 3M entities)
///
/// This consolidates all province data into a single component to minimize
/// archetype fragmentation and maximize cache efficiency during iteration.
/// Minerals are embedded directly rather than as separate components.
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ProvinceData {
    /// Unique identifier for this province
    pub id: ProvinceId,

    /// World position in 2D space
    pub position: Vec2,

    /// Terrain type determining base characteristics
    pub terrain: TerrainType,

    /// Elevation from 0.0 (sea level) to 1.0 (highest peaks)
    pub elevation: Elevation,

    /// Food production capacity
    pub agriculture: Agriculture,

    /// Distance to nearest river/delta in hexagon units
    pub fresh_water_distance: Distance,

    /// Current population
    pub population: u32,

    /// Maximum population this province can support
    pub max_population: u32,

    /// Cultural identity of this province
    pub culture: Option<Culture>,

    // === Mineral Resources (embedded for cache efficiency) ===
    /// Iron abundance - Common, used for tools and weapons
    pub iron: Abundance,

    /// Copper abundance - Common, used for bronze
    pub copper: Abundance,

    /// Tin abundance - Rare, essential for bronze
    pub tin: Abundance,

    /// Gold abundance - Rare, used for currency
    pub gold: Abundance,

    /// Coal abundance - Common in lowlands, fuel source
    pub coal: Abundance,

    /// Stone abundance - Ubiquitous, building material
    pub stone: Abundance,

    /// Gems abundance - Very rare, luxury goods
    pub gems: Abundance,
}

impl ProvinceData {
    /// Create new province data from a Province struct
    pub fn from_province(province: &Province) -> Self {
        Self {
            id: province.id,
            position: province.position,
            terrain: province.terrain,
            elevation: province.elevation,
            agriculture: province.agriculture,
            fresh_water_distance: province.fresh_water_distance,
            population: province.population,
            max_population: province.max_population,
            culture: province.culture,
            iron: province.iron,
            copper: province.copper,
            tin: province.tin,
            gold: province.gold,
            coal: province.coal,
            stone: province.stone,
            gems: province.gems,
        }
    }

    /// Check if this province is habitable
    pub fn is_habitable(&self) -> bool {
        self.terrain != TerrainType::Ocean
    }

    /// Check if this province has fresh water access
    pub fn has_fresh_water(&self) -> bool {
        self.terrain == TerrainType::River || self.fresh_water_distance.within(2.0)
    }

    /// Calculate population growth multiplier based on terrain and resources
    pub fn growth_multiplier(&self) -> f32 {
        let base = match self.terrain {
            TerrainType::River => 2.5,
            TerrainType::TropicalRainforest | TerrainType::TemperateRainforest => 1.5,
            TerrainType::TemperateGrassland | TerrainType::Savanna => 1.2,
            TerrainType::Ocean => 0.0,
            TerrainType::PolarDesert | TerrainType::TropicalDesert => 0.3,
            _ => 1.0,
        };

        // Modify by agriculture
        base * (0.5 + self.agriculture.value() / 3.0)
    }

    /// Get mineral abundance for a specific mineral type
    pub fn get_mineral_abundance(&self, mineral_type: MineralType) -> Option<u8> {
        let abundance = match mineral_type {
            MineralType::Iron => self.iron.value(),
            MineralType::Copper => self.copper.value(),
            MineralType::Tin => self.tin.value(),
            MineralType::Gold => self.gold.value(),
            MineralType::Coal => self.coal.value(),
            MineralType::Stone => self.stone.value(),
            MineralType::Gems => self.gems.value(),
            _ => 0,
        };

        if abundance > 0 {
            Some(abundance)
        } else {
            None
        }
    }
}

impl Default for ProvinceData {
    fn default() -> Self {
        Self {
            id: ProvinceId::default(),
            position: Vec2::ZERO,
            terrain: TerrainType::TemperateGrassland,
            elevation: Elevation::default(),
            agriculture: Agriculture::default(),
            fresh_water_distance: Distance::infinite(),
            population: PROVINCE_MIN_POPULATION,
            max_population: PROVINCE_MIN_POPULATION * 10,
            culture: None,
            iron: Abundance::default(),
            copper: Abundance::default(),
            tin: Abundance::default(),
            gold: Abundance::default(),
            coal: Abundance::default(),
            stone: Abundance::default(),
            gems: Abundance::default(),
        }
    }
}

/// Neighbor entity references for a province
///
/// Stores direct Entity references to neighboring provinces for O(1) lookups.
/// Uses the standard hexagonal direction order: NE, E, SE, SW, W, NW.
#[derive(Component, Debug, Clone, Default, Reflect)]
pub struct ProvinceNeighbors {
    /// Entity references to the 6 neighboring provinces
    /// Order: NE, E, SE, SW, W, NW (matches HexDirection enum)
    /// None if neighbor is off-map or ocean
    pub neighbors: [Option<Entity>; 6],
}

impl ProvinceNeighbors {
    /// Create new neighbor component from entity array
    pub fn new(neighbors: [Option<Entity>; 6]) -> Self {
        Self { neighbors }
    }

    /// Get neighbor in a specific direction
    pub fn get(&self, direction: HexDirection) -> Option<Entity> {
        self.neighbors[direction as usize]
    }

    /// Get all valid neighbors as an iterator
    pub fn iter_valid(&self) -> impl Iterator<Item = Entity> + '_ {
        self.neighbors.iter().filter_map(|&n| n)
    }

    /// Count of valid neighbors
    pub fn count(&self) -> usize {
        self.neighbors.iter().filter(|n| n.is_some()).count()
    }
}

/// Bundle for spawning a province entity
///
/// Contains all components needed for a province entity.
/// Use with `spawn_batch` for efficient bulk spawning of 3M provinces.
#[derive(Bundle, Default)]
pub struct ProvinceBundle {
    /// Marker for province queries
    pub marker: ProvinceMarker,

    /// All province data consolidated
    pub data: ProvinceData,

    /// Neighbor entity references (populated in second pass after all entities spawned)
    pub neighbors: ProvinceNeighbors,
}

impl ProvinceBundle {
    /// Create a bundle from a Province struct
    /// Note: neighbors must be set separately after all provinces are spawned
    pub fn from_province(province: &Province) -> Self {
        Self {
            marker: ProvinceMarker,
            data: ProvinceData::from_province(province),
            neighbors: ProvinceNeighbors::default(), // Set in second pass
        }
    }
}

/// Resource maintaining spawn order of province entities
///
/// Critical for mesh vertex alignment - vertex N corresponds to entity N.
/// Inserted after spawn_batch completes.
#[derive(Resource, Debug, Clone)]
pub struct ProvinceEntityOrder {
    /// Ordered list of province entities matching spawn order
    pub entities: Vec<Entity>,
}

impl ProvinceEntityOrder {
    /// Create from entity list
    pub fn new(entities: Vec<Entity>) -> Self {
        Self { entities }
    }

    /// Get entity by province index (vertex index)
    pub fn get(&self, index: usize) -> Option<Entity> {
        self.entities.get(index).copied()
    }

    /// Get index by entity (for reverse lookups)
    pub fn index_of(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|&e| e == entity)
    }

    /// Total count of provinces
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}
