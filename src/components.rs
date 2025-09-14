//! Core game components with type-safe wrappers
//!
//! This module contains ECS components used throughout the game that are NOT
//! part of the world module. For Province and world-related types, see the
//! world module.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};


/// Types of mineral resources available in the game
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum MineralType {
    /// Iron - Common, used for tools and weapons
    Iron,
    /// Copper - Common, used for bronze when combined with tin
    Copper,
    /// Tin - Rare, essential for bronze production
    Tin,
    /// Gold - Rare, used for currency and luxury goods
    Gold,
    /// Coal - Common in certain regions, fuel source
    Coal,
    /// Stone - Ubiquitous, used for construction
    Stone,
    /// Gems - Very rare, luxury goods and trade
    Gems,
}

impl MineralType {
    /// Get all mineral types as an array
    pub const ALL: [MineralType; 7] = [
        MineralType::Iron,
        MineralType::Copper,
        MineralType::Tin,
        MineralType::Gold,
        MineralType::Coal,
        MineralType::Stone,
        MineralType::Gems,
    ];

    /// Get display name for the mineral
    pub fn name(&self) -> &'static str {
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

    pub fn rarity(&self) -> f32 {
        match self {
            MineralType::Stone => 0.0,     // Everywhere
            MineralType::Iron => 0.2,      // Common
            MineralType::Copper => 0.25,   // Common
            MineralType::Coal => 0.3,      // Somewhat common
            MineralType::Gold => 0.7,      // Rare
            MineralType::Tin => 0.75,      // Rare
            MineralType::Gems => 0.9,      // Very rare
        }
    }

    pub fn value_multiplier(&self) -> f32 {
        match self {
            MineralType::Stone => 0.1,
            MineralType::Coal => 0.5,
            MineralType::Iron => 1.0,
            MineralType::Copper => 1.5,
            MineralType::Tin => 3.0,
            MineralType::Gold => 10.0,
            MineralType::Gems => 20.0,
        }
    }
}


/// Marker component for selected provinces
#[derive(Component)]
pub struct SelectedProvince;

/// Marker component for hoverable provinces
#[derive(Component)]
pub struct HoverableProvince;

/// Component for UI panels showing province info
#[derive(Component)]
pub struct ProvinceInfoPanel;


/// Represents a nation/civilization in the game
#[derive(Component, Debug, Clone)]
pub struct Nation {
    pub id: u32,
    pub name: String,
    pub color: Color,
    pub capital_province_id: Option<u32>,
    pub controlled_provinces: Vec<u32>,
    pub population: u32,
    pub treasury: f32,
    pub stability: f32,
}

impl Nation {
    pub fn new(id: u32, name: String, color: Color) -> Self {
        Self {
            id,
            name,
            color,
            capital_province_id: None,
            controlled_provinces: Vec::new(),
            population: 0,
            treasury: 100.0,
            stability: 1.0,
        }
    }
}


/// Component for UI text that displays information
#[derive(Component)]
pub struct InfoText;

/// Component for the main game speed display
#[derive(Component)]
pub struct GameSpeedText;

/// Component for the pause indicator
#[derive(Component)]
pub struct PauseIndicator;