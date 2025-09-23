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
    Political,      // Nation territories and borders (now default)
    Terrain,        // Natural terrain types and biomes
    Climate,        // Climate zones and temperature
    Population,     // Population density heat map
    Agriculture,    // Agricultural productivity
    Infrastructure, // Roads, cities, and development
    Minerals,       // Combined mineral richness (compressed from 7 individual modes)
}

impl MapMode {
    /// Cycle to the next map mode
    pub fn cycle(&mut self) {
        *self = match self {
            MapMode::Political => MapMode::Terrain,
            MapMode::Terrain => MapMode::Climate,
            MapMode::Climate => MapMode::Population,
            MapMode::Population => MapMode::Agriculture,
            MapMode::Agriculture => MapMode::Infrastructure,
            MapMode::Infrastructure => MapMode::Minerals,
            MapMode::Minerals => MapMode::Political,
        }
    }

    /// Get display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            MapMode::Political => "Political Map",
            MapMode::Terrain => "Terrain Map",
            MapMode::Climate => "Climate Zones",
            MapMode::Population => "Population Density",
            MapMode::Agriculture => "Agriculture",
            MapMode::Infrastructure => "Infrastructure",
            MapMode::Minerals => "Minerals",
        }
    }

    /// Check if this is a mineral-specific mode
    pub fn is_mineral_mode(&self) -> bool {
        matches!(self, MapMode::Minerals)
    }

    /// Get the mineral type if this is a mineral mode
    /// Returns None for unified Minerals mode as it shows all minerals combined
    pub fn get_mineral_type(&self) -> Option<crate::components::MineralType> {
        // Unified Minerals mode shows all minerals combined, so no specific type
        None
    }
}
