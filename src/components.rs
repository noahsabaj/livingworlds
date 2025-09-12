//! Core game components with type-safe wrappers
//! 
//! This module contains all the ECS components used throughout the game.
//! Components are data attached to entities. For global singleton data,
//! see the resources module.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};
use crate::terrain::TerrainType;
use std::fmt;

// ============================================================================
// TYPE-SAFE WRAPPERS - Zero-cost abstractions for compile-time validation
// ============================================================================

/// Type-safe province identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub struct ProvinceId(pub u32);

impl ProvinceId {
    /// Create a new province ID
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    /// Get the raw ID value
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
    /// Create a new elevation, automatically clamped to valid range
    pub fn new(value: f32) -> Self {
        Self(value.clamp(0.0, 1.0))
    }
    
    /// Get the raw elevation value
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
    
    /// Create a new agriculture value, automatically clamped
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }
    
    /// Get the raw agriculture value
    pub fn value(&self) -> f32 {
        self.0
    }
    
    /// Get as a multiplier (0.0 to 1.0)
    pub fn multiplier(&self) -> f32 {
        self.0 / Self::MAX
    }
    
    /// Check if land is barren
    pub fn is_barren(&self) -> bool {
        self.0 < 0.5
    }
    
    /// Check if land is fertile
    pub fn is_fertile(&self) -> bool {
        self.0 > 2.0
    }
}

impl Default for Agriculture {
    fn default() -> Self {
        Self(1.0)
    }
}

impl fmt::Display for Agriculture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

/// Type-safe distance measurement in hexagon units
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Reflect, Serialize, Deserialize)]
pub struct Distance(f32);

impl Distance {
    /// Create a new distance value
    pub fn new(hexagons: f32) -> Self {
        Self(hexagons.max(0.0))
    }
    
    /// Create infinite distance
    pub fn infinite() -> Self {
        Self(f32::INFINITY)
    }
    
    /// Get the raw distance value
    pub fn value(&self) -> f32 {
        self.0
    }
    
    /// Check if distance is infinite
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }
    
    /// Check if within range
    pub fn within(&self, max_distance: f32) -> bool {
        self.0 <= max_distance
    }
}

impl Default for Distance {
    fn default() -> Self {
        Self::infinite()
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_infinite() {
            write!(f, "∞")
        } else {
            write!(f, "{:.1}", self.0)
        }
    }
}

/// Type-safe mineral abundance with [0, 100] validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Reflect, Serialize, Deserialize)]
pub struct Abundance(u8);

impl Abundance {
    /// Create new abundance, automatically clamped to [0, 100]
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }
    
    /// Create zero abundance
    pub fn zero() -> Self {
        Self(0)
    }
    
    /// Create maximum abundance
    pub fn max() -> Self {
        Self(100)
    }
    
    /// Get the raw value
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

// ============================================================================
// HEXAGON DIRECTIONS
// ============================================================================

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

// ============================================================================
// VALIDATION CONSTANTS
// ============================================================================

/// Maximum distance from fresh water source (in hexagon units)
pub const FRESH_WATER_MAX_DISTANCE: f32 = 10.0;

/// Default starting population for new provinces
pub const DEFAULT_POPULATION: u32 = 1000;

// ============================================================================
// COMPONENTS - Data attached to entities
// ============================================================================

/// Province represents a single hexagonal tile in the world
/// 
/// Note: Provinces are NOT entities in the mega-mesh architecture.
/// They are stored in the ProvinceStorage resource as a Vec.
/// The Component derive is kept for backwards compatibility but will be removed.
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Province {
    // === Identity & Location (16 bytes) ===
    /// Unique identifier for this province
    pub id: ProvinceId,
    
    /// World position in 2D space
    pub position: Vec2,
    
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
    
    // === Spatial Relationships (48 bytes) ===
    /// IDs of the 6 neighboring hexagons (NE, E, SE, SW, W, NW)
    /// None if neighbor is off-map or doesn't exist
    pub neighbors: [Option<ProvinceId>; 6],
    
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
            population: DEFAULT_POPULATION,
            max_population: DEFAULT_POPULATION * 10,
            terrain: TerrainType::Plains,
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
            version: 0,
            dirty: false,
        }
    }
}

impl Province {
    /// Create a new province with the given ID and position
    pub fn new(id: ProvinceId, position: Vec2) -> Self {
        Self {
            id,
            position,
            ..Default::default()
        }
    }
    
    /// Create using the builder pattern
    pub fn builder(id: ProvinceId) -> ProvinceBuilder {
        ProvinceBuilder::new(id)
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
    pub fn set_population(&mut self, population: u32) {
        if self.population != population {
            self.population = population.min(self.max_population);
            self.mark_dirty();
        }
    }
    
    /// Update max population based on current conditions
    pub fn update_max_population(&mut self) {
        // Base capacity from terrain
        let terrain_capacity = match self.terrain {
            TerrainType::Ocean => 0,
            TerrainType::River | TerrainType::Delta => 50_000,
            TerrainType::Plains => 30_000,
            TerrainType::Forest => 20_000,
            TerrainType::Hills => 15_000,
            TerrainType::Mountains => 5_000,
            TerrainType::Desert => 3_000,
            TerrainType::Tundra => 2_000,
            TerrainType::Beach => 10_000,
            TerrainType::Ice => 500,
            TerrainType::Jungle => 25_000,
        };
        
        // Modifiers
        let agriculture_multiplier = 1.0 + self.agriculture.value();
        let water_multiplier = if self.fresh_water_distance.value() <= 1.0 {
            2.0
        } else if self.fresh_water_distance.value() <= 3.0 {
            1.5
        } else {
            1.0
        };
        
        let new_max = (terrain_capacity as f32 * agriculture_multiplier * water_multiplier) as u32;
        if self.max_population != new_max {
            self.max_population = new_max;
            self.mark_dirty();
        }
    }
    
    /// Get the neighbor in a specific direction
    pub fn get_neighbor(&self, direction: HexDirection) -> Option<ProvinceId> {
        self.neighbors[direction as usize]
    }
    
    /// Set a neighbor in a specific direction
    pub fn set_neighbor(&mut self, direction: HexDirection, neighbor: Option<ProvinceId>) {
        self.neighbors[direction as usize] = neighbor;
    }
    
    /// Check if this province can support more population
    pub fn can_grow(&self) -> bool {
        self.population < self.max_population && match self.terrain {
            TerrainType::Ocean => false,
            _ => true,
        }
    }
    
    /// Check if this province has access to fresh water
    pub fn has_fresh_water(&self) -> bool {
        self.fresh_water_distance.within(FRESH_WATER_MAX_DISTANCE)
    }
    
    /// Get the agriculture multiplier for population growth
    pub fn agriculture_multiplier(&self) -> f32 {
        self.agriculture.multiplier()
    }
    
    /// Calculate population growth rate
    pub fn growth_rate(&self) -> f32 {
        if !self.can_grow() {
            return 0.0;
        }
        
        let base_rate = 0.02; // 2% base growth
        let agriculture_bonus = self.agriculture.multiplier();
        let water_bonus = if self.has_fresh_water() { 1.5 } else { 1.0 };
        let terrain_modifier = self.terrain.population_multiplier();
        
        // Crowding penalty as we approach max population
        let crowding = self.population as f32 / self.max_population as f32;
        let crowding_modifier = 1.0 - (crowding * crowding * 0.5);
        
        base_rate * agriculture_bonus * water_bonus * terrain_modifier * crowding_modifier
    }
    
    /// Calculate total mineral richness
    pub fn total_mineral_richness(&self) -> f32 {
        // Weighted sum of all minerals
        let iron_value = self.iron.value() as f32 * 1.0;
        let copper_value = self.copper.value() as f32 * 1.5;
        let tin_value = self.tin.value() as f32 * 3.0;  // Rare
        let gold_value = self.gold.value() as f32 * 10.0;  // Very valuable
        let coal_value = self.coal.value() as f32 * 0.8;
        let stone_value = self.stone.value() as f32 * 0.2;  // Common
        let gem_value = self.gems.value() as f32 * 20.0;  // Extremely valuable
        
        (iron_value + copper_value + tin_value + gold_value + coal_value + stone_value + gem_value) / 100.0
    }
}

/// Builder pattern for safe Province construction
pub struct ProvinceBuilder {
    province: Province,
}

impl ProvinceBuilder {
    /// Create a new builder with the given ID
    pub fn new(id: ProvinceId) -> Self {
        Self {
            province: Province {
                id,
                ..Default::default()
            }
        }
    }
    
    /// Set the position
    pub fn position(mut self, pos: Vec2) -> Self {
        self.province.position = pos;
        self
    }
    
    /// Set the population
    pub fn population(mut self, pop: u32) -> Self {
        self.province.population = pop;
        self
    }
    
    /// Set the terrain type
    pub fn terrain(mut self, terrain: TerrainType) -> Self {
        self.province.terrain = terrain;
        self
    }
    
    /// Set the elevation
    pub fn elevation(mut self, elevation: f32) -> Self {
        self.province.elevation = Elevation::new(elevation);
        self
    }
    
    /// Set the agriculture value
    pub fn agriculture(mut self, agriculture: f32) -> Self {
        self.province.agriculture = Agriculture::new(agriculture);
        self
    }
    
    /// Set the fresh water distance
    pub fn fresh_water_distance(mut self, distance: f32) -> Self {
        self.province.fresh_water_distance = Distance::new(distance);
        self
    }
    
    /// Set mineral resources
    pub fn minerals(mut self, iron: u8, copper: u8, tin: u8, gold: u8, coal: u8, stone: u8, gems: u8) -> Self {
        self.province.iron = Abundance::new(iron);
        self.province.copper = Abundance::new(copper);
        self.province.tin = Abundance::new(tin);
        self.province.gold = Abundance::new(gold);
        self.province.coal = Abundance::new(coal);
        self.province.stone = Abundance::new(stone);
        self.province.gems = Abundance::new(gems);
        self
    }
    
    /// Build the province and calculate max population
    pub fn build(mut self) -> Province {
        self.province.update_max_population();
        self.province
    }
}

// Note: SelectedProvince marker component was removed (dead code - never used)
// The codebase uses SelectedProvinceInfo resource instead

/// Marker component for the tile info UI panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text display
#[derive(Component)]
pub struct TileInfoText;

/// Marker component for all game world entities that should be cleaned up when leaving the game
/// 
/// TODO: Replace with specific markers like TerrainEntity, CloudEntity, etc.
#[derive(Component)]
pub struct GameWorld;

// ============================================================================
// RESOURCE COMPONENTS - Mineral wealth and infrastructure
// ============================================================================

/// Types of minerals in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum MineralType {
    Iron,
    Copper,
    Tin,
    Gold,
    Coal,
    Stone,
    Gems,
}

impl MineralType {
    /// Iterate over all mineral types
    pub fn iter() -> impl Iterator<Item = MineralType> {
        use MineralType::*;
        [Iron, Copper, Tin, Gold, Coal, Stone, Gems].into_iter()
    }
    
    /// Get the display name for this mineral type
    pub fn display_name(&self) -> &'static str {
        match self {
            MineralType::Iron => "Iron",
            MineralType::Copper => "Copper",
            MineralType::Tin => "Tin",
            MineralType::Gold => "Gold",
            MineralType::Coal => "Coal",
            MineralType::Stone => "Stone",
            MineralType::Gems => "Gems",
        }
    }
    
    /// Parse a mineral type from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "iron" => Some(MineralType::Iron),
            "copper" => Some(MineralType::Copper),
            "tin" => Some(MineralType::Tin),
            "gold" => Some(MineralType::Gold),
            "coal" => Some(MineralType::Coal),
            "stone" => Some(MineralType::Stone),
            "gems" | "gem" => Some(MineralType::Gems),
            _ => None,
        }
    }
    
    /// Get a description of what this mineral is used for
    pub fn description(&self) -> &'static str {
        match self {
            MineralType::Iron => "Common metal used for tools and weapons",
            MineralType::Copper => "Soft metal used to make bronze when combined with tin",
            MineralType::Tin => "Rare metal essential for creating bronze alloy",
            MineralType::Gold => "Precious metal used for currency and luxury items",
            MineralType::Coal => "Fuel source found in lowland areas",
            MineralType::Stone => "Ubiquitous building material for construction",
            MineralType::Gems => "Very rare luxury items for trade and decoration",
        }
    }
    
    /// Get the rarity level of this mineral (0 = common, 3 = very rare)
    pub fn rarity(&self) -> u8 {
        match self {
            MineralType::Stone => 0,  // Ubiquitous
            MineralType::Iron | MineralType::Copper | MineralType::Coal => 1,  // Common
            MineralType::Tin | MineralType::Gold => 2,  // Rare
            MineralType::Gems => 3,  // Very rare
        }
    }
    
    /// Check if this mineral is considered precious
    pub fn is_precious(&self) -> bool {
        matches!(self, MineralType::Gold | MineralType::Gems)
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS FOR TERRAIN INTEGRATION
// ============================================================================

impl TerrainType {
    /// Get the population growth multiplier for this terrain
    pub fn population_multiplier(&self) -> f32 {
        match self {
            TerrainType::Ocean => 0.0,
            TerrainType::Beach => 0.8,
            TerrainType::Plains => 1.5,
            TerrainType::Hills => 1.0,
            TerrainType::Mountains => 0.3,
            TerrainType::Ice => 0.1,
            TerrainType::Tundra => 0.2,
            TerrainType::Desert => 0.4,
            TerrainType::Forest => 1.2,
            TerrainType::Jungle => 1.3,
            TerrainType::River => 2.0,
            TerrainType::Delta => 3.0,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_province_id() {
        let id = ProvinceId::new(42);
        assert_eq!(id.value(), 42);
        assert_eq!(format!("{}", id), "Province#42");
    }
    
    #[test]
    fn test_elevation_clamping() {
        let e1 = Elevation::new(-1.0);
        assert_eq!(e1.value(), 0.0);
        
        let e2 = Elevation::new(2.0);
        assert_eq!(e2.value(), 1.0);
        
        let e3 = Elevation::new(0.5);
        assert_eq!(e3.value(), 0.5);
    }
    
    #[test]
    fn test_agriculture_validation() {
        let a1 = Agriculture::new(-1.0);
        assert_eq!(a1.value(), 0.0);
        
        let a2 = Agriculture::new(5.0);
        assert_eq!(a2.value(), 3.0);
        
        let a3 = Agriculture::new(2.0);
        assert_eq!(a3.multiplier(), 2.0 / 3.0);
    }
    
    #[test]
    fn test_abundance_validation() {
        let a1 = Abundance::new(50);
        assert_eq!(a1.value(), 50);
        
        let a2 = Abundance::new(150);
        assert_eq!(a2.value(), 100);  // Clamped
        
        let a3 = Abundance::new(80);
        assert!(a3.is_rich());
    }
    
    #[test]
    fn test_province_builder() {
        let province = Province::builder(ProvinceId::new(1))
            .position(Vec2::new(100.0, 200.0))
            .population(5000)
            .terrain(TerrainType::Plains)
            .elevation(0.3)
            .agriculture(2.5)
            .fresh_water_distance(5.0)
            .build();
        
        assert_eq!(province.id.value(), 1);
        assert_eq!(province.population, 5000);
        assert_eq!(province.elevation.value(), 0.3);
        assert!(province.has_fresh_water());
    }
    
    #[test]
    fn test_province_resources_depletion() {
        let mut resources = ProvinceResources::new(100, 50, 30, 20, 80, 90, 10);
        
        let depleted = resources.deplete(MineralType::Gold, 15);
        assert_eq!(depleted, 15);
        assert_eq!(resources.gold.value(), 5);
        
        let depleted2 = resources.deplete(MineralType::Gold, 10);
        assert_eq!(depleted2, 5);  // Only 5 left
        assert_eq!(resources.gold.value(), 0);
    }
    
    #[test]
    fn test_distance_type() {
        let d1 = Distance::new(5.0);
        assert!(d1.within(10.0));
        
        let d2 = Distance::infinite();
        assert!(d2.is_infinite());
        assert!(!d2.within(100.0));
    }
}