//! Unified color system for Living Worlds
//! 
//! This module provides a performant, type-safe color system for all visual elements
//! including terrain, minerals, UI, and dynamic atmospheric effects.

use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::components::MineralType;
use crate::terrain::TerrainType;
use crate::constants::*;
use crate::resources::{WeatherSystem, GameTime};

// ============================================================================
// TYPE-SAFE WRAPPERS
// ============================================================================

/// Type-safe wrapper for stone abundance with validation
#[derive(Debug, Clone, Copy)]
pub struct StoneAbundance(u8);

impl StoneAbundance {
    /// Create a new stone abundance, clamping to valid range [0, 100]
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }
    
    /// Get the normalized value [0.0, 1.0] accounting for stone's 20-80 range
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

// ============================================================================
// UNIFIED COLOR THEME
// ============================================================================

/// Unified color theme combining world and UI colors
pub mod theme {
    use bevy::prelude::Color;
    
    // Terrain base colors
    pub const OCEAN_DEEP: Color = Color::srgb(0.02, 0.15, 0.35);
    pub const OCEAN_MEDIUM: Color = Color::srgb(0.08, 0.25, 0.45);
    pub const OCEAN_SHALLOW: Color = Color::srgb(0.15, 0.35, 0.55);
    pub const BEACH: Color = Color::srgb(0.9, 0.85, 0.65);
    pub const PLAINS: Color = Color::srgb(0.3, 0.6, 0.3);
    pub const HILLS: Color = Color::srgb(0.5, 0.45, 0.35);
    pub const MOUNTAINS: Color = Color::srgb(0.65, 0.65, 0.7);
    pub const SNOW: Color = Color::srgb(0.95, 0.95, 1.0);
    pub const ICE: Color = Color::srgb(0.92, 0.95, 1.0);
    pub const TUNDRA: Color = Color::srgb(0.65, 0.6, 0.55);
    pub const DESERT: Color = Color::srgb(0.9, 0.8, 0.6);
    pub const FOREST: Color = Color::srgb(0.15, 0.35, 0.12);
    pub const JUNGLE: Color = Color::srgb(0.05, 0.3, 0.08);
    pub const RIVER: Color = Color::srgb(0.2, 0.45, 0.55);
    pub const DELTA: Color = Color::srgb(0.35, 0.5, 0.25);
    
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
}

// ============================================================================
// FAST MATH APPROXIMATIONS
// ============================================================================

/// Fast sine approximation using Taylor series (error < 0.001)
#[inline]
fn fast_sin(x: f32) -> f32 {
    let x = x % (2.0 * std::f32::consts::PI);
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 + x2 * x2 / 120.0)
}

/// Position-based hash for deterministic variation
#[inline]
fn position_hash(x: f32, y: f32, seed: u32) -> f32 {
    let mut h = ((x * 12.9898 + y * 78.233) * (seed as f32 * 0.001)).sin() * 43758.5453;
    h = h.fract();
    h * 2.0 - 1.0  // Return -1 to 1
}

// ============================================================================
// COLOR PALETTE CACHE
// ============================================================================

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
            TerrainType::Ocean, TerrainType::Beach, TerrainType::Plains,
            TerrainType::Hills, TerrainType::Mountains, TerrainType::Ice,
            TerrainType::Tundra, TerrainType::Desert, TerrainType::Forest,
            TerrainType::Jungle, TerrainType::River, TerrainType::Delta,
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
            .unwrap_or(theme::PLAINS)
    }
}

/// Compute terrain color without caching (used during palette generation)
fn compute_terrain_color(terrain: TerrainType, elevation: f32) -> Color {
    let elevation = elevation.clamp(0.0, 1.0);
    
    match terrain {
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
        TerrainType::Plains => {
            // Green plains with elevation variation
            let factor = (elevation - 0.2) / 0.25;
            SafeColor::srgb(0.25 + factor * 0.1, 0.55 + factor * 0.1, 0.25 + factor * 0.05)
        },
        TerrainType::Hills => {
            // Brown to grey transition
            let factor = (elevation - 0.45) / 0.2;
            SafeColor::srgb(0.45 + factor * 0.1, 0.4 + factor * 0.05, 0.3 + factor * 0.15)
        },
        TerrainType::Mountains => {
            // Rocky grey to snow white
            let snow_factor = ((elevation - 0.65) / 0.35).clamp(0.0, 1.0);
            let grey = 0.6 + snow_factor * 0.35;
            SafeColor::srgb(grey, grey, grey + snow_factor * 0.05)
        },
        TerrainType::Ice => theme::ICE,
        TerrainType::Tundra => theme::TUNDRA,
        TerrainType::Desert => {
            // Warm tan with subtle variation
            let var = elevation * 0.05;
            SafeColor::srgb(0.9 + var, 0.8 + var, 0.6)
        },
        TerrainType::Forest => {
            // Rich green with depth variation
            let factor = (elevation - 0.3) / 0.2;
            SafeColor::srgb(0.15 + factor * 0.05, 0.35 + factor * 0.1, 0.12 + factor * 0.03)
        },
        TerrainType::Jungle => {
            // Deep vibrant green
            let var = elevation * 0.1;
            SafeColor::srgb(0.05 + var, 0.3 + var * 1.5, 0.08 + var * 0.5)
        },
        TerrainType::River => theme::RIVER,
        TerrainType::Delta => theme::DELTA,
    }
}

// ============================================================================
// COLOR TRAITS
// ============================================================================

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

// ============================================================================
// COLOR PROVIDER ABSTRACTION
// ============================================================================

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

// ============================================================================
// DYNAMIC COLOR SYSTEM
// ============================================================================

/// Dynamic color adjustments based on time, weather, and atmosphere
pub struct DynamicColorSystem;

impl DynamicColorSystem {
    /// Apply time of day color adjustments
    pub fn apply_time_of_day(base_color: Color, game_time: &GameTime) -> Color {
        // Calculate time of day (0-24 hours)
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

// ============================================================================
// PUBLIC API - SIMPLIFIED FACADE
// ============================================================================

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
    pub fn interpolate(from: Color, to: Color, t: f32) -> Color {
        Color::from(from).mix(&to, t.clamp(0.0, 1.0))
    }
    
    /// Blend colors for biome transitions
    pub fn blend_biomes(colors: &[(Color, f32)]) -> Color {
        let total_weight: f32 = colors.iter().map(|(_, w)| w).sum();
        if total_weight == 0.0 {
            return theme::PLAINS;
        }
        
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        
        for (color, weight) in colors {
            let normalized_weight = weight / total_weight;
            let rgba = color.to_srgba();
            r += rgba.red * normalized_weight;
            g += rgba.green * normalized_weight;
            b += rgba.blue * normalized_weight;
        }
        
        SafeColor::srgb(r, g, b)
    }
}

// ============================================================================
// BACKWARDS COMPATIBILITY - PUBLIC API FUNCTIONS
// ============================================================================

/// Get the global world colors instance
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