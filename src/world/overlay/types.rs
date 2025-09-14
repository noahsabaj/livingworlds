//! Overlay system types and data structures

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Map overlay modes for different visualizations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, Default)]
pub enum OverlayMode {
    #[default]
    Political, // Nation territories and borders
    Terrain,        // Terrain types and biomes
    Climate,        // Climate zones and temperature
    Mineral,        // Mineral and resource distribution
    Infrastructure, // Roads, cities, and development
    Population,     // Population density
    Agriculture,    // Agricultural productivity
    Rivers,         // River systems and watersheds
}

impl OverlayMode {
    /// Get all available overlay modes
    pub fn all() -> &'static [OverlayMode] {
        &[
            OverlayMode::Political,
            OverlayMode::Terrain,
            OverlayMode::Climate,
            OverlayMode::Mineral,
            OverlayMode::Infrastructure,
            OverlayMode::Population,
            OverlayMode::Agriculture,
            OverlayMode::Rivers,
        ]
    }

    /// Get the display name for this overlay mode
    pub fn display_name(&self) -> &'static str {
        match self {
            OverlayMode::Political => "Political Map",
            OverlayMode::Terrain => "Terrain Map",
            OverlayMode::Climate => "Climate Map",
            OverlayMode::Mineral => "Mineral Resources",
            OverlayMode::Infrastructure => "Infrastructure",
            OverlayMode::Population => "Population Density",
            OverlayMode::Agriculture => "Agriculture",
            OverlayMode::Rivers => "River Systems",
        }
    }

    /// Cycle to the next overlay mode
    pub fn next(&self) -> OverlayMode {
        match self {
            OverlayMode::Political => OverlayMode::Terrain,
            OverlayMode::Terrain => OverlayMode::Climate,
            OverlayMode::Climate => OverlayMode::Mineral,
            OverlayMode::Mineral => OverlayMode::Infrastructure,
            OverlayMode::Infrastructure => OverlayMode::Population,
            OverlayMode::Population => OverlayMode::Agriculture,
            OverlayMode::Agriculture => OverlayMode::Rivers,
            OverlayMode::Rivers => OverlayMode::Political,
        }
    }
}

/// Resource visualization overlay modes for displaying mineral distribution
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum ResourceOverlay {
    /// No overlay - show normal political/terrain colors
    None,
    /// Show specific mineral abundance
    Mineral(crate::components::MineralType),
    /// Show all minerals combined (richness heat map)
    AllMinerals,
}

impl Default for ResourceOverlay {
    fn default() -> Self {
        ResourceOverlay::None
    }
}

impl ResourceOverlay {
    /// Cycle to the next overlay mode
    pub fn cycle(&mut self) {
        use crate::components::MineralType;
        *self = match self {
            ResourceOverlay::None => ResourceOverlay::Mineral(MineralType::Iron),
            ResourceOverlay::Mineral(MineralType::Iron) => {
                ResourceOverlay::Mineral(MineralType::Copper)
            }
            ResourceOverlay::Mineral(MineralType::Copper) => {
                ResourceOverlay::Mineral(MineralType::Tin)
            }
            ResourceOverlay::Mineral(MineralType::Tin) => {
                ResourceOverlay::Mineral(MineralType::Gold)
            }
            ResourceOverlay::Mineral(MineralType::Gold) => {
                ResourceOverlay::Mineral(MineralType::Coal)
            }
            ResourceOverlay::Mineral(MineralType::Coal) => {
                ResourceOverlay::Mineral(MineralType::Stone)
            }
            ResourceOverlay::Mineral(MineralType::Stone) => {
                ResourceOverlay::Mineral(MineralType::Gems)
            }
            ResourceOverlay::Mineral(MineralType::Gems) => ResourceOverlay::AllMinerals,
            ResourceOverlay::AllMinerals => ResourceOverlay::None,
        }
    }

    /// Get display name for current overlay
    pub fn display_name(&self) -> &str {
        use crate::components::MineralType;
        match self {
            ResourceOverlay::None => "Natural Terrain",
            ResourceOverlay::Mineral(MineralType::Iron) => "Iron Deposits",
            ResourceOverlay::Mineral(MineralType::Copper) => "Copper Deposits",
            ResourceOverlay::Mineral(MineralType::Tin) => "Tin Deposits",
            ResourceOverlay::Mineral(MineralType::Gold) => "Gold Deposits",
            ResourceOverlay::Mineral(MineralType::Coal) => "Coal Deposits",
            ResourceOverlay::Mineral(MineralType::Stone) => "Stone Deposits",
            ResourceOverlay::Mineral(MineralType::Gems) => "Gem Deposits",
            ResourceOverlay::AllMinerals => "All Minerals",
        }
    }
}
