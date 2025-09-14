//! Terrain types, climate zones, and classification
//!
//! This module defines terrain types and classification logic.
//! Elevation generation has been moved to generation/elevation.rs

use bevy::prelude::*;
use crate::constants::*;


#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect, serde::Serialize, serde::Deserialize)]
pub enum TerrainType {
    #[default]
    Ocean,              // Deep water
    Beach,              // Coastal areas
    River,              // River tiles
    Delta,              // River deltas (very fertile)

    // Polar biomes
    PolarDesert,        // Extremely cold and dry
    Tundra,             // Arctic tundra

    // Cold biomes
    Taiga,              // Coniferous forest
    BorealForest,       // Northern forest

    // Temperate biomes
    TemperateRainforest,    // Lush temperate rainforest
    TemperateDeciduousForest, // Seasonal forest
    TemperateGrassland,     // Prairie/steppe
    ColdDesert,             // High altitude or cold desert

    // Subtropical biomes
    MediterraneanForest,    // Mediterranean woodland
    Chaparral,              // Dry shrubland
    SubtropicalDesert,      // Hot dry desert

    // Tropical biomes
    TropicalRainforest,     // Dense jungle
    TropicalSeasonalForest, // Monsoon forest
    Savanna,                // Grassland with scattered trees
    TropicalDesert,         // Hot barren desert

    // Special biomes
    Alpine,                 // High mountain meadows
    Wetlands,              // Marshes and swamps
    Mangrove,              // Coastal mangrove swamps
}


pub enum ClimateZone {
    Arctic,
    Temperate,
    Subtropical,
    Tropical,
    Desert,
}

fn get_climate_zone(y: f32, map_height: f32) -> ClimateZone {
    let latitude = (y / map_height + 0.5).clamp(0.0, 1.0);

    // Use smoother transitions without hard boundaries at the equator
    if latitude < 0.15 || latitude > 0.85 {
        ClimateZone::Arctic
    } else if latitude < 0.25 || latitude > 0.75 {
        ClimateZone::Temperate
    } else if latitude < 0.35 || latitude > 0.65 {
        ClimateZone::Subtropical
    } else if latitude < 0.4 || latitude > 0.6 {
        // Tropical zones near but not at equator
        ClimateZone::Tropical
    } else {
        // Desert in the middle tropics (but not a hard band)
        ClimateZone::Desert
    }
}


pub fn classify_terrain_with_climate(elevation: f32, x: f32, y: f32, map_height: f32) -> TerrainType {
    const DEFAULT_SEA_LEVEL: f32 = 0.2;
    classify_terrain_with_sea_level(elevation, x, y, map_height, DEFAULT_SEA_LEVEL)
}

pub fn classify_terrain_with_sea_level(elevation: f32, x: f32, y: f32, map_height: f32, sea_level: f32) -> TerrainType {
    // Ocean classification based on depth
    if elevation < sea_level {
        // Use the sea level as the threshold for ocean
        return TerrainType::Ocean;
    }

    let climate = get_climate_zone(y, map_height);

    // Beaches are just above sea level
    if elevation < sea_level + 0.02 {
        return TerrainType::Beach;
    }

    // Classify land based on elevation and climate
    match climate {
        ClimateZone::Arctic => {
            if elevation < sea_level + 0.4 {
                TerrainType::Tundra
            } else if elevation < sea_level + 0.6 {
                TerrainType::Alpine
            } else {
                TerrainType::PolarDesert
            }
        }
        ClimateZone::Desert => {
            if elevation < sea_level + 0.4 {
                TerrainType::SubtropicalDesert
            } else if elevation < sea_level + 0.6 {
                TerrainType::ColdDesert
            } else {
                TerrainType::Alpine
            }
        }
        ClimateZone::Tropical => {
            if elevation < sea_level + 0.25 {
                TerrainType::Savanna
            } else if elevation < sea_level + 0.45 {
                TerrainType::TropicalRainforest
            } else if elevation < sea_level + 0.65 {
                TerrainType::TropicalSeasonalForest
            } else {
                TerrainType::Alpine
            }
        }
        ClimateZone::Subtropical => {
            if elevation < sea_level + 0.25 {
                TerrainType::Chaparral
            } else if elevation < sea_level + 0.45 {
                TerrainType::MediterraneanForest
            } else if elevation < sea_level + 0.65 {
                TerrainType::Chaparral
            } else {
                TerrainType::Alpine
            }
        }
        ClimateZone::Temperate => {
            if elevation < sea_level + 0.25 {
                TerrainType::TemperateGrassland
            } else if elevation < sea_level + 0.5 {
                TerrainType::TemperateDeciduousForest
            } else if elevation < sea_level + 0.65 {
                TerrainType::TemperateRainforest
            } else {
                TerrainType::Alpine
            }
        }
    }
}

/// Simple terrain classification without climate considerations
fn classify_terrain_with_sea_level_simple(elevation: f32, sea_level: f32) -> TerrainType {
    // NOTE: This is deprecated - use biome_to_terrain instead
    if elevation < sea_level {
        TerrainType::Ocean
    } else if elevation < sea_level + 0.02 {
        TerrainType::Beach
    } else if elevation < sea_level + 0.25 {
        TerrainType::TemperateGrassland
    } else if elevation < sea_level + 0.5 {
        TerrainType::TemperateDeciduousForest
    } else if elevation < sea_level + 0.65 {
        TerrainType::TemperateRainforest
    } else {
        TerrainType::Alpine
    }
}


/// Get terrain population multiplier - uses centralized properties
pub fn get_terrain_population_multiplier(terrain: TerrainType) -> f32 {
    terrain.properties().population_multiplier
}


/// Convert a climate biome to a terrain type - 1:1 mapping for maximum variety!
pub fn biome_to_terrain(biome: crate::generation::climate::Biome, elevation: f32) -> TerrainType {
    use crate::generation::climate::Biome;

    const SEA_LEVEL: f32 = 0.2;
    if elevation < SEA_LEVEL {
        return TerrainType::Ocean;
    } else if elevation < SEA_LEVEL + 0.02 {
        return TerrainType::Beach;
    }

    // Direct 1:1 mapping from biome to terrain type
    match biome {
        Biome::PolarDesert => TerrainType::PolarDesert,
        Biome::Tundra => TerrainType::Tundra,
        Biome::Taiga => TerrainType::Taiga,
        Biome::BorealForest => TerrainType::BorealForest,
        Biome::TemperateRainforest => TerrainType::TemperateRainforest,
        Biome::TemperateDeciduousForest => TerrainType::TemperateDeciduousForest,
        Biome::TemperateGrassland => TerrainType::TemperateGrassland,
        Biome::ColdDesert => TerrainType::ColdDesert,
        Biome::MediterraneanForest => TerrainType::MediterraneanForest,
        Biome::Chaparral => TerrainType::Chaparral,
        Biome::SubtropicalDesert => TerrainType::SubtropicalDesert,
        Biome::TropicalRainforest => TerrainType::TropicalRainforest,
        Biome::TropicalSeasonalForest => TerrainType::TropicalSeasonalForest,
        Biome::Savanna => TerrainType::Savanna,
        Biome::TropicalDesert => TerrainType::TropicalDesert,
        Biome::Alpine => TerrainType::Alpine,
        Biome::Wetlands => TerrainType::Wetlands,
        Biome::Mangrove => TerrainType::Mangrove,
    }
}

// TERRAIN PROPERTIES - SINGLE SOURCE OF TRUTH

/// All properties for a terrain type in one place
#[derive(Debug, Clone, Copy)]
pub struct TerrainProperties {
    /// Population growth multiplier (0.0 = uninhabitable, 3.0 = extremely fertile)
    pub population_multiplier: f32,
    /// Maximum population this terrain can support
    pub max_population_capacity: u32,
    /// Stone abundance (0-100, typically 20-80 for land)
    pub stone_abundance: u8,
    /// Resource extraction difficulty (0.3 = very hard, 1.2 = easy)
    pub extraction_difficulty: f32,
    /// Base agriculture value (0.0-3.0)
    pub agriculture_base: f32,
    /// Is this a water terrain?
    pub is_water: bool,
    /// Is this a desert terrain?
    pub is_desert: bool,
    /// Is this a forest terrain?
    pub is_forest: bool,
    /// Can rivers spawn from this terrain?
    pub allows_rivers: bool,
}

impl TerrainType {
    /// Get ALL properties for this terrain type in one place!
    /// This is the SINGLE SOURCE OF TRUTH for terrain properties.
    pub fn properties(&self) -> TerrainProperties {
        use crate::constants::*;  // For agriculture constants

        match self {
            // Water features
            TerrainType::Ocean => TerrainProperties {
                population_multiplier: 0.0,
                max_population_capacity: 0,
                stone_abundance: 0,
                extraction_difficulty: 0.0,  // Can't extract from ocean
                agriculture_base: 0.0,
                is_water: true,
                is_desert: false,
                is_forest: false,
                allows_rivers: false,
            },
            TerrainType::Beach => TerrainProperties {
                population_multiplier: 1.5,
                max_population_capacity: 10_000,
                stone_abundance: 20,  // STONE_ABUNDANCE_BEACH
                extraction_difficulty: 1.1,
                agriculture_base: 0.2,
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: false,
            },
            TerrainType::River => TerrainProperties {
                population_multiplier: 2.5,
                max_population_capacity: 50_000,
                stone_abundance: 30,
                extraction_difficulty: 1.0,
                agriculture_base: 2.0,  // RIVER_BASE_AGRICULTURE
                is_water: true,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::Delta => TerrainProperties {
                population_multiplier: 3.0,
                max_population_capacity: 50_000,
                stone_abundance: 25,
                extraction_difficulty: 1.0,
                agriculture_base: 3.0,  // DELTA_BASE_AGRICULTURE
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },

            // Polar biomes
            TerrainType::PolarDesert => TerrainProperties {
                population_multiplier: 0.02,
                max_population_capacity: 500,
                stone_abundance: 40,
                extraction_difficulty: 0.3,
                agriculture_base: 0.0,
                is_water: false,
                is_desert: true,  // It's a type of desert
                is_forest: false,
                allows_rivers: false,
            },
            TerrainType::Tundra => TerrainProperties {
                population_multiplier: 0.1,
                max_population_capacity: 2_000,
                stone_abundance: 50,  // STONE_ABUNDANCE_TUNDRA
                extraction_difficulty: 0.6,
                agriculture_base: 0.1,  // TUNDRA_BASE_AGRICULTURE
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },

            // Cold biomes
            TerrainType::Taiga => TerrainProperties {
                population_multiplier: 0.3,
                max_population_capacity: 8_000,
                stone_abundance: 35,
                extraction_difficulty: 0.85,
                agriculture_base: 0.3,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::BorealForest => TerrainProperties {
                population_multiplier: 0.4,
                max_population_capacity: 10_000,
                stone_abundance: 35,
                extraction_difficulty: 0.9,
                agriculture_base: 0.4,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },

            // Temperate biomes
            TerrainType::TemperateRainforest => TerrainProperties {
                population_multiplier: 0.8,
                max_population_capacity: 15_000,
                stone_abundance: 30,
                extraction_difficulty: 0.95,
                agriculture_base: 0.7,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::TemperateDeciduousForest => TerrainProperties {
                population_multiplier: 1.2,
                max_population_capacity: 20_000,
                stone_abundance: 30,
                extraction_difficulty: 0.95,
                agriculture_base: 0.8,  // FOREST_BASE_AGRICULTURE
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::TemperateGrassland => TerrainProperties {
                population_multiplier: 1.8,
                max_population_capacity: 30_000,
                stone_abundance: 40,
                extraction_difficulty: 1.2,
                agriculture_base: 1.2,  // PLAINS_BASE_AGRICULTURE
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::ColdDesert => TerrainProperties {
                population_multiplier: 0.2,
                max_population_capacity: 2_500,
                stone_abundance: 60,  // STONE_ABUNDANCE_DESERT
                extraction_difficulty: 0.9,
                agriculture_base: 0.2,
                is_water: false,
                is_desert: true,
                is_forest: false,
                allows_rivers: false,
            },

            // Subtropical biomes
            TerrainType::MediterraneanForest => TerrainProperties {
                population_multiplier: 1.5,
                max_population_capacity: 18_000,
                stone_abundance: 45,  // STONE_ABUNDANCE_HILLS equivalent
                extraction_difficulty: 1.0,
                agriculture_base: 1.0,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::Chaparral => TerrainProperties {
                population_multiplier: 0.7,
                max_population_capacity: 12_000,
                stone_abundance: 45,  // STONE_ABUNDANCE_HILLS equivalent
                extraction_difficulty: 1.0,
                agriculture_base: 0.6,
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::SubtropicalDesert => TerrainProperties {
                population_multiplier: 0.15,
                max_population_capacity: 3_000,
                stone_abundance: 60,  // STONE_ABUNDANCE_DESERT
                extraction_difficulty: 0.9,
                agriculture_base: 0.1,  // DESERT_BASE_AGRICULTURE
                is_water: false,
                is_desert: true,
                is_forest: false,
                allows_rivers: false,
            },

            // Tropical biomes
            TerrainType::TropicalRainforest => TerrainProperties {
                population_multiplier: 0.6,
                max_population_capacity: 25_000,
                stone_abundance: 25,
                extraction_difficulty: 0.7,
                agriculture_base: 0.5,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::TropicalSeasonalForest => TerrainProperties {
                population_multiplier: 1.0,
                max_population_capacity: 22_000,
                stone_abundance: 30,
                extraction_difficulty: 0.75,
                agriculture_base: 0.9,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
            TerrainType::Savanna => TerrainProperties {
                population_multiplier: 1.3,
                max_population_capacity: 20_000,
                stone_abundance: 40,
                extraction_difficulty: 1.1,
                agriculture_base: 0.7,
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::TropicalDesert => TerrainProperties {
                population_multiplier: 0.1,
                max_population_capacity: 2_000,
                stone_abundance: 60,  // STONE_ABUNDANCE_DESERT
                extraction_difficulty: 0.9,
                agriculture_base: 0.05,
                is_water: false,
                is_desert: true,
                is_forest: false,
                allows_rivers: false,
            },

            // Special biomes
            TerrainType::Alpine => TerrainProperties {
                population_multiplier: 0.25,
                max_population_capacity: 5_000,
                stone_abundance: 80,  // STONE_ABUNDANCE_MOUNTAINS
                extraction_difficulty: 0.8,
                agriculture_base: 0.3,  // MOUNTAINS_BASE_AGRICULTURE
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::Wetlands => TerrainProperties {
                population_multiplier: 0.9,
                max_population_capacity: 15_000,
                stone_abundance: 20,
                extraction_difficulty: 0.85,
                agriculture_base: 1.5,
                is_water: false,
                is_desert: false,
                is_forest: false,
                allows_rivers: true,
            },
            TerrainType::Mangrove => TerrainProperties {
                population_multiplier: 0.4,
                max_population_capacity: 10_000,
                stone_abundance: 15,
                extraction_difficulty: 0.8,
                agriculture_base: 0.8,
                is_water: false,
                is_desert: false,
                is_forest: true,
                allows_rivers: true,
            },
        }
    }
}


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, _app: &mut App) {
        // Currently no systems, just types and functions
    }
}