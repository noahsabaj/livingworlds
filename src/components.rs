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
// COMPONENTS - Data attached to entities
// ============================================================================

/// Province represents a single hexagonal tile in the world
#[derive(Component, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct Province {
    pub id: u32,
    pub position: Vec2,
    pub population: f32,
    pub terrain: TerrainType,
    pub elevation: f32,
    pub agriculture: f32,         // Food production capacity (0.0 to 3.0)
    pub fresh_water_distance: f32, // Distance to nearest river/delta in hex units
}

/// Marker component for the currently selected province
#[derive(Component)]
pub struct SelectedProvince;

/// Marker component for the tile info UI panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text display
#[derive(Component)]
pub struct TileInfoText;

/// Marker component for all game world entities that should be cleaned up when leaving the game
#[derive(Component)]
pub struct GameWorld;

// ============================================================================
// RESOURCE COMPONENTS - Mineral wealth and infrastructure
// ============================================================================

/// Mineral resources present in a province (0-100 abundance)
#[derive(Component, Default, Clone, Debug, Reflect, Serialize, Deserialize)]
pub struct ProvinceResources {
    pub iron: u8,      // Common, used for tools and weapons
    pub copper: u8,    // Common, used for bronze
    pub tin: u8,       // Rare, essential for bronze
    pub gold: u8,      // Rare, used for currency
    pub coal: u8,      // Common in lowlands, fuel source
    pub stone: u8,     // Ubiquitous, building material
    pub gems: u8,      // Very rare, luxury goods
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
    // Alloys are not naturally occurring, removed Bronze and Steel
}

