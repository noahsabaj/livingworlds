//! Terrain components - separate components for each terrain type
//!
//! Following ECS patterns, each terrain type is its own component.
//! Entities only have the terrain components relevant to them.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Plains terrain component - fertile flatlands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainPlains {
    pub fertility: Fixed32,         // 0.0 (barren) to 1.0 (incredibly fertile)
    pub rainfall: Fixed32,          // Natural irrigation
    pub flood_risk: Fixed32,        // Nile floods vs devastating floods
    pub wind_exposure: Fixed32,     // Affects erosion and weather
}

/// Mountain terrain component - highlands and peaks
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainMountain {
    pub elevation: Fixed32,         // Height above sea level
    pub slope_gradient: Fixed32,    // Steepness affects development
    pub passes: Vec<Entity>,        // References to MountainPass entities
    pub avalanche_risk: Fixed32,
    pub mineral_potential: Fixed32, // Likelihood of ore deposits
}

/// Coastal terrain component - shorelines and beaches
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCoastal {
    pub shore_type: ShoreType,
    pub tidal_range: Fixed32,
    pub storm_exposure: Fixed32,
    pub harbor_potential: Fixed32,
    pub fishing_grounds: Fixed32,
}

/// Desert terrain component - arid lands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainDesert {
    pub aridity: Fixed32,           // Sahara vs semi-arid
    pub temperature_extremes: Fixed32,
    pub oases: Vec<Entity>,         // References to Oasis entities
    pub sandstorm_frequency: Fixed32,
    pub mineral_deposits: Fixed32,  // Often rich in minerals
}

/// Forest terrain component - woodlands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainForest {
    pub tree_density: Fixed32,
    pub wood_quality: WoodQuality,
    pub wildlife_abundance: Fixed32,
    pub fire_risk: Fixed32,
    pub undergrowth_density: Fixed32,
}

/// Tundra terrain component - frozen lands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainTundra {
    pub permafrost_depth: Fixed32,
    pub growing_season_length: Fixed32,
    pub wildlife_migration: bool,
    pub mineral_accessibility: Fixed32, // Frozen ground complicates mining
}

/// River terrain component - flowing water
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainRiver {
    pub flow_rate: Fixed32,
    pub seasonal_variation: Fixed32,
    pub flood_predictability: Fixed32,
    pub navigation_capacity: Fixed32,
    pub fish_abundance: Fixed32,
}

/// Swamp terrain component - wetlands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainSwamp {
    pub water_level_stability: Fixed32,
    pub disease_prevalence: Fixed32,
    pub navigation_difficulty: Fixed32,
    pub unique_resources: Fixed32,  // Rare plants, peat
}

/// Hills terrain component - rolling highlands
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TerrainHills {
    pub average_elevation: Fixed32,
    pub slope_variation: Fixed32,
    pub pasture_quality: Fixed32,   // Good for grazing
    pub defensive_advantage: Fixed32,
}

// Supporting enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShoreType {
    Rocky,
    Sandy,
    Mudflat,
    Cliff,
    Mangrove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WoodQuality {
    Hardwood,
    Softwood,
    Bamboo,
    Tropical,
}

/// Mountain pass entity - strategic chokepoint
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MountainPass {
    pub name: String,
    pub elevation: Fixed32,
    pub width: Fixed32,
    pub seasonal_accessibility: SeasonalAccessibility,
    pub fortification_potential: Fixed32,
    pub trade_importance: Fixed32,
}

/// Oasis entity - water in the desert
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Oasis {
    pub water_capacity: Fixed32,
    pub vegetation_coverage: Fixed32,
    pub seasonal_reliability: Fixed32,
    pub settlement_capacity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalAccessibility {
    pub spring: Fixed32,
    pub summer: Fixed32,
    pub autumn: Fixed32,
    pub winter: Fixed32,
}