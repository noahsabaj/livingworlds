//! Terrain color generation and palette management
//!
//! This module handles terrain-specific color computation, including
//! pre-computed palettes for performance optimization and elevation-based
//! color gradients.

use bevy::prelude::Color;
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::world::TerrainType;
use crate::constants::*;
use crate::math::fast_sin;
use super::theme;
use super::utils::SafeColor;

/// Pre-computed color palettes for all terrain types
pub(super) static COLOR_PALETTES: LazyLock<TerrainColorPalettes> = LazyLock::new(|| {
    TerrainColorPalettes::generate()
});

pub(super) struct TerrainColorPalettes {
    /// 256 pre-computed colors per terrain type for different elevations
    palettes: HashMap<TerrainType, Vec<Color>>,
}

impl TerrainColorPalettes {
    fn generate() -> Self {
        let mut palettes = HashMap::new();

        // Generate 256 color steps for each terrain type
        for terrain_type in [
            // Water terrains
            TerrainType::Ocean, TerrainType::Beach, TerrainType::River, TerrainType::Delta,
            // Polar biomes
            TerrainType::PolarDesert, TerrainType::Tundra,
            // Cold biomes
            TerrainType::Taiga, TerrainType::BorealForest,
            // Temperate biomes
            TerrainType::TemperateRainforest, TerrainType::TemperateDeciduousForest,
            TerrainType::TemperateGrassland, TerrainType::ColdDesert,
            // Subtropical biomes
            TerrainType::MediterraneanForest, TerrainType::Chaparral, TerrainType::SubtropicalDesert,
            // Tropical biomes
            TerrainType::TropicalRainforest, TerrainType::TropicalSeasonalForest,
            TerrainType::Savanna, TerrainType::TropicalDesert,
            // Special biomes
            TerrainType::Alpine, TerrainType::Wetlands, TerrainType::Mangrove,
        ] {
            let mut colors = Vec::with_capacity(256);
            for i in 0..256 {
                let elevation = i as f32 / 255.0;
                colors.push(compute_terrain_color(terrain_type, elevation));
            }
            palettes.insert(terrain_type, colors);
        }

        Self { palettes }
    }

    #[inline]
    pub fn get(&self, terrain: TerrainType, elevation: f32) -> Color {
        let index = (elevation.clamp(0.0, 1.0) * 255.0) as usize;
        self.palettes.get(&terrain)
            .and_then(|palette| palette.get(index))
            .copied()
            .unwrap_or(theme::TEMPERATE_GRASSLAND)
    }
}

/// Compute terrain color without caching (used during palette generation)
pub(super) fn compute_terrain_color(terrain: TerrainType, elevation: f32) -> Color {
    let elevation = elevation.clamp(0.0, 1.0);

    match terrain {
        // Water terrains
        TerrainType::Ocean => {
            // Correctly interpret elevation for ocean depth
            if elevation >= OCEAN_ELEVATION_SHALLOW {
                theme::OCEAN_SHALLOW
            } else if elevation >= OCEAN_ELEVATION_MEDIUM {
                theme::OCEAN_MEDIUM
            } else {
                theme::OCEAN_DEEP
            }
        },
        TerrainType::Beach => {
            // Sandy beach with slight elevation-based variation
            let sand_var = elevation * 0.1;
            SafeColor::srgb(0.9 + sand_var * 0.5, 0.85 + sand_var * 0.5, 0.65 + sand_var)
        },
        TerrainType::River => theme::RIVER,
        TerrainType::Delta => theme::DELTA,

        // Polar biomes
        TerrainType::PolarDesert => {
            // Icy white with blue tint at higher elevations
            let ice_factor = elevation * 0.1;
            SafeColor::srgb(0.88 + ice_factor, 0.88 + ice_factor, 0.92 + ice_factor * 0.5)
        },
        TerrainType::Tundra => {
            // Grey-brown with lighter shades at elevation
            let tundra_var = elevation * 0.15;
            SafeColor::srgb(0.65 + tundra_var, 0.6 + tundra_var, 0.55 + tundra_var)
        },

        // Cold biomes
        TerrainType::Taiga => {
            // Dark evergreen with variation
            let forest_depth = elevation * 0.1;
            SafeColor::srgb(0.1 + forest_depth, 0.25 + forest_depth * 1.5, 0.15 + forest_depth)
        },
        TerrainType::BorealForest => {
            // Slightly lighter evergreen
            let boreal_var = elevation * 0.12;
            SafeColor::srgb(0.12 + boreal_var, 0.3 + boreal_var * 1.2, 0.18 + boreal_var)
        },

        // Temperate biomes
        TerrainType::TemperateRainforest => {
            // Deep lush green with elevation darkening
            let rain_factor = (1.0 - elevation) * 0.15;
            SafeColor::srgb(0.05 + rain_factor, 0.35 + rain_factor * 2.0, 0.15 + rain_factor)
        },
        TerrainType::TemperateDeciduousForest => {
            // Mixed forest green with seasonal variation hint
            let deciduous_var = elevation * 0.2;
            SafeColor::srgb(0.15 + deciduous_var * 0.5, 0.4 + deciduous_var, 0.12 + deciduous_var * 0.3)
        },
        TerrainType::TemperateGrassland => {
            // Prairie green-yellow
            let grass_factor = elevation * 0.15;
            SafeColor::srgb(0.4 + grass_factor, 0.65 + grass_factor * 0.5, 0.3 + grass_factor * 0.3)
        },
        TerrainType::ColdDesert => {
            // Grey-tan cold desert
            let cold_var = elevation * 0.1;
            SafeColor::srgb(0.7 + cold_var, 0.65 + cold_var, 0.55 + cold_var * 0.5)
        },

        // Subtropical biomes
        TerrainType::MediterraneanForest => {
            // Olive green with warm tones
            let med_factor = elevation * 0.15;
            SafeColor::srgb(0.3 + med_factor * 0.5, 0.45 + med_factor * 0.3, 0.25 + med_factor * 0.2)
        },
        TerrainType::Chaparral => {
            // Dry shrubland brown-green
            let chap_var = elevation * 0.12;
            SafeColor::srgb(0.55 + chap_var, 0.5 + chap_var * 0.8, 0.35 + chap_var * 0.5)
        },
        TerrainType::SubtropicalDesert => {
            // Sandy yellow with reddish tint
            let sub_desert = elevation * 0.08;
            SafeColor::srgb(0.92 + sub_desert * 0.5, 0.82 + sub_desert * 0.3, 0.6 - sub_desert)
        },

        // Tropical biomes
        TerrainType::TropicalRainforest => {
            // Deep jungle green, darkest of all forests
            let jungle_depth = (1.0 - elevation) * 0.1;
            SafeColor::srgb(0.02 + jungle_depth, 0.28 + jungle_depth * 2.0, 0.05 + jungle_depth * 0.5)
        },
        TerrainType::TropicalSeasonalForest => {
            // Monsoon forest - between rainforest and savanna
            let seasonal_var = elevation * 0.15;
            SafeColor::srgb(0.15 + seasonal_var, 0.38 + seasonal_var * 0.5, 0.08 + seasonal_var * 0.3)
        },
        TerrainType::Savanna => {
            // Dry grass yellow-brown
            let savanna_factor = elevation * 0.1;
            SafeColor::srgb(0.75 + savanna_factor, 0.7 + savanna_factor * 0.5, 0.4 - savanna_factor)
        },
        TerrainType::TropicalDesert => {
            // Bright sand with intense heat shimmer
            let trop_desert = elevation * 0.05;
            SafeColor::srgb(0.95 + trop_desert * 0.3, 0.85 + trop_desert * 0.2, 0.55 - trop_desert * 0.5)
        },

        // Special biomes
        TerrainType::Alpine => {
            // Mountain meadow transitioning to snow
            let alpine_snow = elevation.powf(2.0);  // More snow at higher elevations
            let base_grey = 0.75 + alpine_snow * 0.2;
            SafeColor::srgb(base_grey, base_grey, base_grey + alpine_snow * 0.05)
        },
        TerrainType::Wetlands => {
            // Swamp dark green-brown
            let wetland_var = elevation * 0.08;
            SafeColor::srgb(0.25 + wetland_var * 0.5, 0.35 + wetland_var, 0.2 + wetland_var * 0.3)
        },
        TerrainType::Mangrove => {
            // Coastal marsh green with tidal influence
            let mangrove_tide = fast_sin(elevation * 2.0) * 0.05 + elevation * 0.1;
            SafeColor::srgb(0.18 + mangrove_tide, 0.32 + mangrove_tide * 1.5, 0.22 + mangrove_tide)
        },
    }
}

/// Get terrain color for a specific terrain type and elevation
#[inline]
pub fn get_terrain_color(terrain: TerrainType, elevation: f32) -> Color {
    COLOR_PALETTES.get(terrain, elevation)
}