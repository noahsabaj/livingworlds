//! Core game components
//! 
//! This module contains all the ECS components used throughout the game.
//! Components are data attached to entities. For global singleton data,
//! see the resources module.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};
use crate::terrain::TerrainType;

// ============================================================================
// VALIDATION CONSTANTS
// ============================================================================

/// Minimum agriculture value (barren land)
pub const AGRICULTURE_MIN: f32 = 0.0;

/// Maximum agriculture value (most fertile land)
pub const AGRICULTURE_MAX: f32 = 3.0;

/// Default agriculture value for average land
pub const AGRICULTURE_DEFAULT: f32 = 1.0;

/// Maximum distance from fresh water source (in hexagon units)
pub const FRESH_WATER_MAX_DISTANCE: f32 = 10.0;

/// Value representing infinite distance from water
pub const FRESH_WATER_INFINITE: f32 = f32::INFINITY;

/// Minimum mineral abundance
pub const MINERAL_ABUNDANCE_MIN: u8 = 0;

/// Maximum mineral abundance
pub const MINERAL_ABUNDANCE_MAX: u8 = 100;

/// Default starting population for new provinces
pub const DEFAULT_POPULATION: f32 = 1000.0;

/// Default elevation for average terrain
pub const DEFAULT_ELEVATION: f32 = 0.5;

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
    /// Unique identifier for this province
    pub id: u32,
    
    /// World position in 2D space
    pub position: Vec2,
    
    /// Current population (should be u32 in future refactor)
    pub population: f32,
    
    /// Terrain type determining base characteristics
    pub terrain: TerrainType,
    
    /// Elevation from 0.0 (sea level) to 1.0 (highest peaks)
    pub elevation: f32,
    
    /// Food production capacity from AGRICULTURE_MIN to AGRICULTURE_MAX
    pub agriculture: f32,
    
    /// Distance to nearest river/delta in hexagon units
    /// Use FRESH_WATER_INFINITE for no water access
    pub fresh_water_distance: f32,
}

impl Default for Province {
    fn default() -> Self {
        Self {
            id: 0,
            position: Vec2::ZERO,
            population: DEFAULT_POPULATION,
            terrain: TerrainType::Plains,
            elevation: DEFAULT_ELEVATION,
            agriculture: AGRICULTURE_DEFAULT,
            fresh_water_distance: FRESH_WATER_INFINITE,
        }
    }
}

impl Province {
    /// Create a new province with the given ID and position
    pub fn new(id: u32, position: Vec2) -> Self {
        Self {
            id,
            position,
            ..Default::default()
        }
    }
    
    /// Check if this province has access to fresh water
    pub fn has_fresh_water(&self) -> bool {
        self.fresh_water_distance < FRESH_WATER_MAX_DISTANCE
    }
    
    /// Get the agriculture multiplier for population growth
    pub fn agriculture_multiplier(&self) -> f32 {
        self.agriculture / AGRICULTURE_MAX
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

/// Mineral resources present in a province
/// 
/// All values range from MINERAL_ABUNDANCE_MIN to MINERAL_ABUNDANCE_MAX
/// Small struct that can be efficiently copied
#[derive(Component, Default, Clone, Copy, Debug, Reflect, Serialize, Deserialize)]
pub struct ProvinceResources {
    /// Iron abundance (0-100) - Common, used for tools and weapons
    pub iron: u8,
    
    /// Copper abundance (0-100) - Common, used for bronze
    pub copper: u8,
    
    /// Tin abundance (0-100) - Rare, essential for bronze
    pub tin: u8,
    
    /// Gold abundance (0-100) - Rare, used for currency
    pub gold: u8,
    
    /// Coal abundance (0-100) - Common in lowlands, fuel source
    pub coal: u8,
    
    /// Stone abundance (0-100) - Ubiquitous, building material
    pub stone: u8,
    
    /// Gems abundance (0-100) - Very rare, luxury goods
    pub gems: u8,
}

impl ProvinceResources {
    /// Create new resources with all minerals at zero
    pub fn empty() -> Self {
        Self::default()
    }
    
    /// Create resources with specified values, clamping to valid range
    pub fn new(iron: u8, copper: u8, tin: u8, gold: u8, coal: u8, stone: u8, gems: u8) -> Self {
        Self {
            iron: iron.min(MINERAL_ABUNDANCE_MAX),
            copper: copper.min(MINERAL_ABUNDANCE_MAX),
            tin: tin.min(MINERAL_ABUNDANCE_MAX),
            gold: gold.min(MINERAL_ABUNDANCE_MAX),
            coal: coal.min(MINERAL_ABUNDANCE_MAX),
            stone: stone.min(MINERAL_ABUNDANCE_MAX),
            gems: gems.min(MINERAL_ABUNDANCE_MAX),
        }
    }
    
    /// Get total mineral richness (sum of all minerals)
    pub fn total_richness(&self) -> u16 {
        self.iron as u16 
            + self.copper as u16 
            + self.tin as u16 
            + self.gold as u16 
            + self.coal as u16 
            + self.stone as u16 
            + self.gems as u16
    }
    
    /// Check if this province has any minerals
    pub fn has_minerals(&self) -> bool {
        self.total_richness() > 0
    }
    
    /// Get abundance of a specific mineral type
    pub fn get_abundance(&self, mineral: MineralType) -> u8 {
        match mineral {
            MineralType::Iron => self.iron,
            MineralType::Copper => self.copper,
            MineralType::Tin => self.tin,
            MineralType::Gold => self.gold,
            MineralType::Coal => self.coal,
            MineralType::Stone => self.stone,
            MineralType::Gems => self.gems,
        }
    }
}

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
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_province_default() {
        let province = Province::default();
        assert_eq!(province.id, 0);
        assert_eq!(province.population, DEFAULT_POPULATION);
        assert_eq!(province.agriculture, AGRICULTURE_DEFAULT);
        assert!(!province.has_fresh_water());
    }
    
    #[test]
    fn test_mineral_type_iteration() {
        let minerals: Vec<_> = MineralType::iter().collect();
        assert_eq!(minerals.len(), 7);
        assert_eq!(minerals[0], MineralType::Iron);
        assert_eq!(minerals[6], MineralType::Gems);
    }
    
    #[test]
    fn test_mineral_type_parsing() {
        assert_eq!(MineralType::from_str("iron"), Some(MineralType::Iron));
        assert_eq!(MineralType::from_str("GOLD"), Some(MineralType::Gold));
        assert_eq!(MineralType::from_str("invalid"), None);
    }
    
    #[test]
    fn test_province_resources_validation() {
        let resources = ProvinceResources::new(50, 200, 30, 150, 80, 90, 255);
        assert_eq!(resources.copper, 100);  // Clamped from 200
        assert_eq!(resources.gold, 100);     // Clamped from 150
        assert_eq!(resources.gems, 100);     // Clamped from 255
        assert_eq!(resources.iron, 50);      // Unchanged
    }
    
    #[test]
    fn test_province_resources_copy() {
        let r1 = ProvinceResources::new(10, 20, 30, 40, 50, 60, 70);
        let r2 = r1;  // Copy, not move
        assert_eq!(r1.iron, r2.iron);
        assert_eq!(r1.total_richness(), r2.total_richness());
    }
}