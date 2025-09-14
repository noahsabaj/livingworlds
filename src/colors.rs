//! Unified color system for Living Worlds
//! 
//! This module provides a performant, type-safe color system for all visual elements
//! including terrain, minerals, UI, and dynamic atmospheric effects.

use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::components::MineralType;
use crate::world::terrain::TerrainType;
use crate::constants::*;
use crate::resources::{WeatherSystem, GameTime};
use crate::math::{lerp_color, weighted_blend_colors, hash_random, fast_sin};

// TYPE-SAFE WRAPPERS

/// Type-safe wrapper for stone abundance with validation
#[derive(Debug, Clone, Copy)]
pub struct StoneAbundance(u8);

impl StoneAbundance {
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }
    
    pub fn normalized(&self) -> f32 {
        if self.0 == 0 {
            0.0  // Ocean/no stone
        } else {
            ((self.0 as f32 - 20.0) / 60.0).clamp(0.0, 1.0)
        }
    }
    
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// Safe color construction with automatic clamping
pub struct SafeColor;

impl SafeColor {
    #[inline]
    pub fn srgb(r: f32, g: f32, b: f32) -> Color {
        Color::srgb(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
    }
    
    #[inline]
    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::srgba(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), a.clamp(0.0, 1.0))
    }
}


/// Unified color theme combining world and UI colors
pub mod theme {
    use bevy::prelude::Color;
    
    // Water colors
    pub const OCEAN_DEEP: Color = Color::srgb(0.02, 0.15, 0.35);
    pub const OCEAN_MEDIUM: Color = Color::srgb(0.08, 0.25, 0.45);
    pub const OCEAN_SHALLOW: Color = Color::srgb(0.15, 0.35, 0.55);
    pub const BEACH: Color = Color::srgb(0.9, 0.85, 0.65);
    pub const RIVER: Color = Color::srgb(0.2, 0.45, 0.55);
    pub const DELTA: Color = Color::srgb(0.35, 0.5, 0.25);

    // Polar biome colors
    pub const POLAR_DESERT: Color = Color::srgb(0.88, 0.88, 0.92);  // Icy grey-white
    pub const TUNDRA: Color = Color::srgb(0.65, 0.6, 0.55);         // Grey-brown

    // Cold biome colors
    pub const TAIGA: Color = Color::srgb(0.1, 0.25, 0.15);          // Dark evergreen
    pub const BOREAL_FOREST: Color = Color::srgb(0.12, 0.3, 0.18);  // Slightly lighter evergreen

    // Temperate biome colors
    pub const TEMPERATE_RAINFOREST: Color = Color::srgb(0.05, 0.35, 0.15);      // Deep lush green
    pub const TEMPERATE_DECIDUOUS_FOREST: Color = Color::srgb(0.15, 0.4, 0.12); // Mixed forest green
    pub const TEMPERATE_GRASSLAND: Color = Color::srgb(0.4, 0.65, 0.3);         // Prairie green
    pub const COLD_DESERT: Color = Color::srgb(0.7, 0.65, 0.55);                // Grey-tan

    // Subtropical biome colors
    pub const MEDITERRANEAN_FOREST: Color = Color::srgb(0.3, 0.45, 0.25);  // Olive green
    pub const CHAPARRAL: Color = Color::srgb(0.55, 0.5, 0.35);            // Dry shrubland brown
    pub const SUBTROPICAL_DESERT: Color = Color::srgb(0.92, 0.82, 0.6);    // Sandy yellow

    // Tropical biome colors
    pub const TROPICAL_RAINFOREST: Color = Color::srgb(0.02, 0.28, 0.05);       // Deep jungle green
    pub const TROPICAL_SEASONAL_FOREST: Color = Color::srgb(0.15, 0.38, 0.08);  // Monsoon forest
    pub const SAVANNA: Color = Color::srgb(0.75, 0.7, 0.4);                     // Dry grass yellow
    pub const TROPICAL_DESERT: Color = Color::srgb(0.95, 0.85, 0.55);           // Bright sand

    // Special biome colors
    pub const ALPINE: Color = Color::srgb(0.75, 0.75, 0.8);        // Mountain meadow grey-green
    pub const WETLANDS: Color = Color::srgb(0.25, 0.35, 0.2);      // Swamp dark green
    pub const MANGROVE: Color = Color::srgb(0.18, 0.32, 0.22);     // Coastal marsh green
    
    // Mineral colors
    pub const IRON: Color = Color::srgb(0.5, 0.3, 0.2);
    pub const COPPER: Color = Color::srgb(0.7, 0.4, 0.2);
    pub const TIN: Color = Color::srgb(0.7, 0.7, 0.8);
    pub const GOLD: Color = Color::srgb(1.0, 0.84, 0.0);
    pub const COAL: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const STONE: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const GEMS: Color = Color::srgb(0.6, 0.2, 0.9);
    
    // Heat map colors for abundance
    pub const HEAT_NONE: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const HEAT_LOW: Color = Color::srgb(0.5, 0.0, 0.0);
    pub const HEAT_MEDIUM: Color = Color::srgb(1.0, 0.5, 0.0);
    pub const HEAT_HIGH: Color = Color::srgb(1.0, 1.0, 0.0);
    pub const HEAT_MAX: Color = Color::srgb(1.0, 1.0, 1.0);
    
    // UI colors
    pub const DIALOG_BACKGROUND: Color = Color::srgba(0.05, 0.05, 0.05, 0.95);
}



/// Position-based hash for deterministic variation
#[inline]
fn position_hash(x: f32, y: f32, seed: u32) -> f32 {
    // Use centralized hash_random and convert from [0,1] to [-1,1]
    hash_random(x, y, seed) * 2.0 - 1.0
}


/// Pre-computed color palettes for all terrain types
static COLOR_PALETTES: LazyLock<TerrainColorPalettes> = LazyLock::new(|| {
    TerrainColorPalettes::generate()
});

struct TerrainColorPalettes {
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
    fn get(&self, terrain: TerrainType, elevation: f32) -> Color {
        let index = (elevation.clamp(0.0, 1.0) * 255.0) as usize;
        self.palettes.get(&terrain)
            .and_then(|palette| palette.get(index))
            .copied()
            .unwrap_or(theme::TEMPERATE_GRASSLAND)
    }
}

/// Compute terrain color without caching (used during palette generation)
fn compute_terrain_color(terrain: TerrainType, elevation: f32) -> Color {
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


/// Trait for types that can provide colors
pub trait Colorable {
    fn color(&self) -> Color;
    fn color_with_elevation(&self, elevation: f32) -> Color;
}

impl Colorable for TerrainType {
    fn color(&self) -> Color {
        self.color_with_elevation(0.5)
    }
    
    fn color_with_elevation(&self, elevation: f32) -> Color {
        COLOR_PALETTES.get(*self, elevation)
    }
}

impl Colorable for MineralType {
    fn color(&self) -> Color {
        match self {
            MineralType::Iron => theme::IRON,
            MineralType::Copper => theme::COPPER,
            MineralType::Tin => theme::TIN,
            MineralType::Gold => theme::GOLD,
            MineralType::Coal => theme::COAL,
            MineralType::Stone => theme::STONE,
            MineralType::Gems => theme::GEMS,
        }
    }
    
    fn color_with_elevation(&self, _elevation: f32) -> Color {
        self.color()  // Minerals don't vary with elevation
    }
}


/// Abstraction for providing colors to different game systems
pub trait ColorProvider {
    fn terrain_color(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color;
    fn mineral_abundance_color(&self, abundance: u8) -> Color;
    fn combined_richness_color(&self, richness: f32) -> Color;
    fn infrastructure_color(&self, level: u8) -> Color;
}

/// Standard color provider with all optimizations
pub struct StandardColorProvider {
    seed: u32,
}

impl StandardColorProvider {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

impl ColorProvider for StandardColorProvider {
    fn terrain_color(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color {
        let base = terrain.color_with_elevation(elevation);
        
        // Add position-based variation for organic look
        let variation = position_hash(world_pos.x, world_pos.y, self.seed) * 0.02;
        let r = (base.to_srgba().red + variation).clamp(0.0, 1.0);
        let g = (base.to_srgba().green + variation).clamp(0.0, 1.0);
        let b = (base.to_srgba().blue + variation).clamp(0.0, 1.0);
        
        SafeColor::srgb(r, g, b)
    }
    
    fn mineral_abundance_color(&self, abundance: u8) -> Color {
        if abundance == 0 {
            theme::HEAT_NONE
        } else {
            let normalized = abundance as f32 / 100.0;
            
            if normalized < 0.33 {
                // Interpolate between none and low
                let t = normalized * 3.0;
                Color::from(theme::HEAT_NONE).mix(&theme::HEAT_LOW, t)
            } else if normalized < 0.67 {
                // Interpolate between low and medium
                let t = (normalized - 0.33) * 3.0;
                Color::from(theme::HEAT_LOW).mix(&theme::HEAT_MEDIUM, t)
            } else {
                // Interpolate between medium and high
                let t = (normalized - 0.67) * 3.0;
                Color::from(theme::HEAT_MEDIUM).mix(&theme::HEAT_HIGH, t)
            }
        }
    }
    
    fn combined_richness_color(&self, richness: f32) -> Color {
        let clamped = richness.clamp(0.0, 10.0) / 10.0;
        
        if clamped < 0.5 {
            let t = clamped * 2.0;
            Color::from(theme::HEAT_LOW).mix(&theme::HEAT_MEDIUM, t)
        } else {
            let t = (clamped - 0.5) * 2.0;
            Color::from(theme::HEAT_MEDIUM).mix(&theme::HEAT_HIGH, t)
        }
    }
    
    fn infrastructure_color(&self, level: u8) -> Color {
        // Validate level and provide gradient
        let level = level.min(5);
        let t = level as f32 / 5.0;
        Color::from(theme::HEAT_NONE).mix(&theme::HEAT_HIGH, t)
    }
}


/// Dynamic color adjustments based on time, weather, and atmosphere
pub struct DynamicColorSystem;

impl DynamicColorSystem {
    /// Apply time of day color adjustments
    pub fn apply_time_of_day(base_color: Color, game_time: &GameTime) -> Color {
        let hours = (game_time.current_date % 1.0) * 24.0;
        
        // Dawn/dusk tinting
        let dawn_dusk_factor = if (5.0..7.0).contains(&hours) || (17.0..19.0).contains(&hours) {
            0.2
        } else {
            0.0
        };
        
        // Night darkening
        let night_factor = if (20.0..=24.0).contains(&hours) || (0.0..5.0).contains(&hours) {
            0.3
        } else {
            0.0
        };
        
        let rgba = base_color.to_srgba();
        let r = rgba.red + dawn_dusk_factor * 0.1 - night_factor * 0.2;
        let g = rgba.green - night_factor * 0.2;
        let b = rgba.blue - dawn_dusk_factor * 0.05 - night_factor * 0.1;
        
        SafeColor::srgb(r, g, b)
    }
    
    /// Apply weather-based color adjustments
    pub fn apply_weather(base_color: Color, weather: &WeatherSystem) -> Color {
        // Desaturate and darken based on cloud coverage
        let coverage = weather.cloud_coverage;
        let rgba = base_color.to_srgba();
        
        // Reduce saturation
        let luminance = rgba.red * 0.299 + rgba.green * 0.587 + rgba.blue * 0.114;
        let r = rgba.red * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;
        let g = rgba.green * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;
        let b = rgba.blue * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;
        
        // Darken
        let darkness = coverage * 0.2;
        SafeColor::srgb(r - darkness, g - darkness, b - darkness)
    }
}

// PUBLIC API - SIMPLIFIED FACADE

/// Simplified API for getting colors throughout the game
pub struct WorldColors {
    provider: StandardColorProvider,
}

impl WorldColors {
    pub fn new(seed: u32) -> Self {
        Self {
            provider: StandardColorProvider::new(seed),
        }
    }
    
    /// Get terrain color with all enhancements
    pub fn terrain(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color {
        self.provider.terrain_color(terrain, elevation, world_pos)
    }
    
    /// Get terrain color with dynamic adjustments
    pub fn terrain_dynamic(
        &self,
        terrain: TerrainType,
        elevation: f32,
        world_pos: Vec2,
        game_time: &GameTime,
        weather: &WeatherSystem,
    ) -> Color {
        let base = self.terrain(terrain, elevation, world_pos);
        let time_adjusted = DynamicColorSystem::apply_time_of_day(base, game_time);
        DynamicColorSystem::apply_weather(time_adjusted, weather)
    }
    
    /// Get mineral abundance color
    pub fn mineral_abundance(&self, abundance: u8) -> Color {
        self.provider.mineral_abundance_color(abundance)
    }
    
    /// Get stone abundance color with special handling
    pub fn stone_abundance(&self, abundance: StoneAbundance) -> Color {
        if abundance.value() == 0 {
            Color::srgb(0.0, 0.1, 0.2)  // Ocean blue
        } else {
            let normalized = abundance.normalized();
            if normalized < 0.5 {
                let t = normalized * 2.0;
                Color::from(Color::srgb(0.3, 0.2, 0.1)).mix(&Color::srgb(0.5, 0.4, 0.2), t)
            } else {
                let t = (normalized - 0.5) * 2.0;
                Color::from(Color::srgb(0.5, 0.4, 0.2)).mix(&Color::srgb(0.8, 0.7, 0.5), t)
            }
        }
    }
    
    /// Get mineral type color
    pub fn mineral_type(&self, mineral: MineralType) -> Color {
        mineral.color()
    }
    
    /// Get combined mineral richness color
    pub fn richness(&self, richness: f32) -> Color {
        self.provider.combined_richness_color(richness)
    }
    
    /// Get infrastructure level color
    pub fn infrastructure(&self, level: u8) -> Color {
        self.provider.infrastructure_color(level)
    }
    
    /// Interpolate between two colors for smooth transitions
    /// Now uses centralized interpolation from math module
    pub fn interpolate(from: Color, to: Color, t: f32) -> Color {
        lerp_color(from, to, t)
    }

    /// Blend colors for biome transitions
    /// Now uses centralized weighted blend from math module
    pub fn blend_biomes(colors: &[(Color, f32)]) -> Color {
        if colors.is_empty() || colors.iter().all(|(_, w)| *w <= 0.0) {
            return theme::TEMPERATE_GRASSLAND;
        }
        weighted_blend_colors(colors)
    }
}

// BACKWARDS COMPATIBILITY - PUBLIC API FUNCTIONS

#[inline]
fn get_world_colors() -> WorldColors {
    WorldColors::new(0)
}

/// Get color for terrain based on type and elevation
/// 
/// # Arguments
/// * `terrain` - The terrain type to color
/// * `elevation` - Elevation value from 0.0 (lowest) to 1.0 (highest)
/// 
/// # Returns
/// Color appropriate for the terrain at the given elevation
pub fn get_terrain_color_gradient(terrain: TerrainType, elevation: f32) -> Color {
    terrain.color_with_elevation(elevation)
}

/// Calculate color for mineral abundance (0-100)
/// 
/// # Arguments
/// * `abundance` - Mineral abundance from 0 (none) to 100 (maximum)
/// 
/// # Returns
/// Heat map color from dark grey (0) to white (100)
pub fn mineral_abundance_color(abundance: u8) -> Color {
    get_world_colors().mineral_abundance(abundance)
}

/// Special color scale for stone (which ranges from 20-80, not 0-100)
/// 
/// # Arguments
/// * `abundance` - Stone abundance, typically 20-80 for land, 0 for ocean
/// 
/// # Returns
/// Earth tone colors for stone deposits
pub fn stone_abundance_color(abundance: u8) -> Color {
    get_world_colors().stone_abundance(StoneAbundance::new(abundance))
}

/// Get mineral-specific color based on type
pub fn get_mineral_color(mineral_type: MineralType) -> Color {
    mineral_type.color()
}

/// Color for combined mineral richness
/// 
/// # Arguments
/// * `richness` - Combined richness value, typically 0.0 to 10.0
/// 
/// # Returns
/// Gradient from brown (poor) to bright gold (extremely rich)
pub fn combined_richness_color(richness: f32) -> Color {
    get_world_colors().richness(richness)
}

/// Color for mining infrastructure level
/// 
/// # Arguments
/// * `level` - Infrastructure level from 0 (none) to 5 (maximum)
/// 
/// # Returns
/// Gradient from dark grey to light tan
pub fn infrastructure_level_color(level: u8) -> Color {
    get_world_colors().infrastructure(level)
}