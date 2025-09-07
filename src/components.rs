//! Core game components
//! 
//! This module contains all the ECS components used throughout the game.
//! Components are data attached to entities. For global singleton data,
//! see the resources module.

use bevy::prelude::*;
use crate::terrain::TerrainType;

// ============================================================================
// COMPONENTS - Data attached to entities
// ============================================================================

/// Province represents a single hexagonal tile in the world
#[derive(Component, Clone)]
pub struct Province {
    pub id: u32,
    pub position: Vec2,
    pub nation_id: Option<u32>,  // None for ocean provinces
    pub population: f32,
    pub terrain: TerrainType,
    pub elevation: f32,
}

/// Marker component for the currently selected province
#[derive(Component)]
pub struct SelectedProvince;

/// Marker for ghost provinces (duplicates for world wrapping)
/// These are visual duplicates shown at map edges for seamless scrolling
#[derive(Component)]
pub struct GhostProvince {
    pub original_col: u32,  // Original column this is a ghost of
}

/// Nation represents a political entity that controls provinces
#[derive(Component, Clone)]
pub struct Nation {
    pub id: u32,
    pub name: String,
    pub color: Color,
}

/// Marker component for the tile info UI panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text display
#[derive(Component)]
pub struct TileInfoText;

// ============================================================================
// RESOURCE COMPONENTS - Mineral wealth and infrastructure
// ============================================================================

/// Mineral resources present in a province (0-100 abundance)
#[derive(Component, Default, Clone, Debug)]
pub struct ProvinceResources {
    pub iron: u8,      // Common, used for tools and weapons
    pub copper: u8,    // Common, used for bronze
    pub tin: u8,       // Rare, essential for bronze
    pub gold: u8,      // Rare, used for currency
    pub coal: u8,      // Common in lowlands, fuel source
    pub stone: u8,     // Ubiquitous, building material
    pub gems: u8,      // Very rare, luxury goods
}

/// Infrastructure built in a province
#[derive(Component, Default, Clone, Debug)]
pub struct ProvinceInfrastructure {
    pub mine_level: u8,        // 0-5 (none to advanced)
    pub forge_level: u8,       // 0-3 (processes raw ore)
    pub extraction_rate: f32,  // Units per day
    pub workers: u32,          // Population assigned to mining
}

/// Nation's stockpile of resources
#[derive(Component, Default, Clone, Debug)]
pub struct NationStockpile {
    pub iron: f32,
    pub copper: f32,
    pub tin: f32,
    pub gold: f32,
    pub coal: f32,
    pub stone: f32,
    pub gems: f32,
    pub bronze: f32,   // Processed alloy
    pub steel: f32,    // Processed alloy
}

/// Technology level of a nation
#[derive(Component, Clone, Debug)]
pub struct NationTechnology {
    pub age: TechnologyAge,
    pub mining_efficiency: f32,     // 0.5 to 2.0
    pub forge_efficiency: f32,      // 0.5 to 2.0
    pub discovered_resources: Vec<MineralType>,  // What they know about
}

impl Default for NationTechnology {
    fn default() -> Self {
        Self {
            age: TechnologyAge::StoneAge,
            mining_efficiency: 0.5,
            forge_efficiency: 0.5,
            discovered_resources: vec![MineralType::Stone],
        }
    }
}

/// Trade route between two nations
#[derive(Component, Clone, Debug)]
pub struct TradeRoute {
    pub from_nation: Entity,
    pub to_nation: Entity,
    pub exported: MineralType,
    pub imported: MineralType,
    pub volume: f32,
    pub profit_margin: f32,
}

/// Military equipment tier based on available materials
#[derive(Component, Clone, Debug, PartialEq)]
pub enum EquipmentTier {
    Stone,     // 1.0x strength
    Bronze,    // 2.0x strength  
    Iron,      // 3.0x strength
    Steel,     // 4.0x strength
}

impl Default for EquipmentTier {
    fn default() -> Self {
        EquipmentTier::Stone
    }
}

/// Types of minerals in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MineralType {
    Iron,
    Copper,
    Tin,
    Gold,
    Coal,
    Stone,
    Gems,
    Bronze,  // Alloy
    Steel,   // Alloy
}

/// Technology ages that civilizations progress through
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TechnologyAge {
    StoneAge,
    CopperAge,
    BronzeAge,
    IronAge,
    SteelAge,
    Industrial,
    Modern,
}