//! Overlay system types and data structures

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Unified map mode enum combining all overlay and mineral visualization modes
/// This provides a comprehensive system for all map visualization needs
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, Default, Reflect, Serialize, Deserialize,
)]
pub enum MapMode {
    // Core map visualization modes
    #[default]
    Terrain, // Natural terrain types and biomes
    Political,      // Nation territories and borders
    Climate,        // Climate zones and temperature
    Population,     // Population density heat map
    Agriculture,    // Agricultural productivity
    Rivers,         // River systems and watersheds
    Infrastructure, // Roads, cities, and development

    // Mineral-specific overlays
    MineralIron,   // Iron ore deposits
    MineralCopper, // Copper deposits
    MineralTin,    // Tin deposits
    MineralGold,   // Gold deposits
    MineralCoal,   // Coal deposits
    MineralStone,  // Stone quarries
    MineralGems,   // Gem deposits
    AllMinerals,   // Combined mineral richness
}

impl MapMode {
    /// Cycle to the next map mode
    pub fn cycle(&mut self) {
        *self = match self {
            MapMode::Terrain => MapMode::Political,
            MapMode::Political => MapMode::Climate,
            MapMode::Climate => MapMode::Population,
            MapMode::Population => MapMode::Agriculture,
            MapMode::Agriculture => MapMode::Rivers,
            MapMode::Rivers => MapMode::Infrastructure,
            MapMode::Infrastructure => MapMode::MineralIron,
            MapMode::MineralIron => MapMode::MineralCopper,
            MapMode::MineralCopper => MapMode::MineralTin,
            MapMode::MineralTin => MapMode::MineralGold,
            MapMode::MineralGold => MapMode::MineralCoal,
            MapMode::MineralCoal => MapMode::MineralStone,
            MapMode::MineralStone => MapMode::MineralGems,
            MapMode::MineralGems => MapMode::AllMinerals,
            MapMode::AllMinerals => MapMode::Terrain,
        }
    }

    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            MapMode::Terrain => "Terrain Map",
            MapMode::Political => "Political Map",
            MapMode::Climate => "Climate Zones",
            MapMode::Population => "Population Density",
            MapMode::Agriculture => "Agriculture",
            MapMode::Rivers => "River Systems",
            MapMode::Infrastructure => "Infrastructure",
            MapMode::MineralIron => "Iron Deposits",
            MapMode::MineralCopper => "Copper Deposits",
            MapMode::MineralTin => "Tin Deposits",
            MapMode::MineralGold => "Gold Deposits",
            MapMode::MineralCoal => "Coal Deposits",
            MapMode::MineralStone => "Stone Quarries",
            MapMode::MineralGems => "Gem Deposits",
            MapMode::AllMinerals => "All Minerals",
        }
    }

    /// Check if this is a mineral-specific mode
    pub fn is_mineral_mode(&self) -> bool {
        matches!(
            self,
            MapMode::MineralIron
                | MapMode::MineralCopper
                | MapMode::MineralTin
                | MapMode::MineralGold
                | MapMode::MineralCoal
                | MapMode::MineralStone
                | MapMode::MineralGems
                | MapMode::AllMinerals
        )
    }

    /// Get the mineral type if this is a mineral mode
    pub fn get_mineral_type(&self) -> Option<crate::components::MineralType> {
        use crate::components::MineralType;
        match self {
            MapMode::MineralIron => Some(MineralType::Iron),
            MapMode::MineralCopper => Some(MineralType::Copper),
            MapMode::MineralTin => Some(MineralType::Tin),
            MapMode::MineralGold => Some(MineralType::Gold),
            MapMode::MineralCoal => Some(MineralType::Coal),
            MapMode::MineralStone => Some(MineralType::Stone),
            MapMode::MineralGems => Some(MineralType::Gems),
            _ => None,
        }
    }
}
