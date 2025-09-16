//! Color theme constants for Living Worlds
//!
//! This module contains all color constant definitions for terrain, biomes,
//! minerals, UI elements, and other visual components. These are the base
//! colors used throughout the rendering system.

use bevy::prelude::Color;

// WATER COLORS
pub const OCEAN_DEEP: Color = Color::srgb(0.02, 0.15, 0.35);
pub const OCEAN_MEDIUM: Color = Color::srgb(0.08, 0.25, 0.45);
pub const OCEAN_SHALLOW: Color = Color::srgb(0.15, 0.35, 0.55);
pub const BEACH: Color = Color::srgb(0.9, 0.85, 0.65);
pub const RIVER: Color = Color::srgb(0.15, 0.4, 0.6);  // Slightly bluer and more visible
pub const DELTA: Color = Color::srgb(0.35, 0.5, 0.25);

// POLAR BIOME COLORS

pub const POLAR_DESERT: Color = Color::srgb(0.88, 0.88, 0.92); // Icy grey-white
pub const TUNDRA: Color = Color::srgb(0.65, 0.6, 0.55); // Grey-brown

// COLD BIOME COLORS

pub const TAIGA: Color = Color::srgb(0.1, 0.25, 0.15); // Dark evergreen
pub const BOREAL_FOREST: Color = Color::srgb(0.12, 0.3, 0.18); // Slightly lighter evergreen

// TEMPERATE BIOME COLORS

pub const TEMPERATE_RAINFOREST: Color = Color::srgb(0.05, 0.35, 0.15); // Deep lush green
pub const TEMPERATE_DECIDUOUS_FOREST: Color = Color::srgb(0.15, 0.4, 0.12); // Mixed forest green
pub const TEMPERATE_GRASSLAND: Color = Color::srgb(0.4, 0.65, 0.3); // Prairie green
pub const COLD_DESERT: Color = Color::srgb(0.7, 0.65, 0.55); // Grey-tan

// SUBTROPICAL BIOME COLORS

pub const MEDITERRANEAN_FOREST: Color = Color::srgb(0.3, 0.45, 0.25); // Olive green
pub const CHAPARRAL: Color = Color::srgb(0.55, 0.5, 0.35); // Dry shrubland brown
pub const SUBTROPICAL_DESERT: Color = Color::srgb(0.92, 0.82, 0.6); // Sandy yellow

// TROPICAL BIOME COLORS

pub const TROPICAL_RAINFOREST: Color = Color::srgb(0.02, 0.28, 0.05); // Deep jungle green
pub const TROPICAL_SEASONAL_FOREST: Color = Color::srgb(0.15, 0.38, 0.08); // Monsoon forest
pub const SAVANNA: Color = Color::srgb(0.75, 0.7, 0.4); // Dry grass yellow
pub const TROPICAL_DESERT: Color = Color::srgb(0.95, 0.85, 0.55); // Bright sand

// SPECIAL BIOME COLORS

pub const ALPINE: Color = Color::srgb(0.75, 0.75, 0.8); // Mountain meadow grey-green
pub const WETLANDS: Color = Color::srgb(0.25, 0.35, 0.2); // Swamp dark green
pub const MANGROVE: Color = Color::srgb(0.18, 0.32, 0.22); // Coastal marsh green

// MINERAL COLORS

pub const IRON: Color = Color::srgb(0.5, 0.3, 0.2);
pub const COPPER: Color = Color::srgb(0.7, 0.4, 0.2);
pub const TIN: Color = Color::srgb(0.7, 0.7, 0.8);
pub const GOLD: Color = Color::srgb(1.0, 0.84, 0.0);
pub const COAL: Color = Color::srgb(0.2, 0.2, 0.2);
pub const STONE: Color = Color::srgb(0.6, 0.6, 0.6);
pub const GEMS: Color = Color::srgb(0.6, 0.2, 0.9);

// HEAT MAP COLORS

pub const HEAT_NONE: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HEAT_LOW: Color = Color::srgb(0.5, 0.0, 0.0);
pub const HEAT_MEDIUM: Color = Color::srgb(1.0, 0.5, 0.0);
pub const HEAT_HIGH: Color = Color::srgb(1.0, 1.0, 0.0);
pub const HEAT_MAX: Color = Color::srgb(1.0, 1.0, 1.0);

// UI COLORS

pub const DIALOG_BACKGROUND: Color = Color::srgba(0.05, 0.05, 0.05, 0.95);
