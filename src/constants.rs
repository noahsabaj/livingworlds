//! Global constants for Living Worlds
//! 
//! This module contains all game constants organized by category.
//! Centralizing constants ensures consistency and makes tuning easier.

use bevy::prelude::Color;

// ============================================================================
// WORLD & MAP CONSTANTS
// ============================================================================

/// Size of each hexagon in pixels (radius)
pub const HEX_SIZE_PIXELS: f32 = 50.0;

/// Number of province columns in the world
pub const PROVINCES_PER_ROW: u32 = 300;

/// Number of province rows in the world  
pub const PROVINCES_PER_COL: u32 = 200;

/// Square root of 3, used frequently in hexagon math
pub const SQRT3: f32 = 1.732050808;

/// Distance from map edge where ocean is forced (in pixels)
pub const EDGE_BUFFER: f32 = 200.0;

// Derived constants for convenience
/// Total number of provinces in the world
pub const TOTAL_PROVINCES: u32 = PROVINCES_PER_ROW * PROVINCES_PER_COL;

/// Map width in pixels (for flat-top hexagons)
pub const MAP_WIDTH_PIXELS: f32 = PROVINCES_PER_ROW as f32 * HEX_SIZE_PIXELS * 1.5;

/// Map height in pixels (for flat-top hexagons)
pub const MAP_HEIGHT_PIXELS: f32 = PROVINCES_PER_COL as f32 * HEX_SIZE_PIXELS * SQRT3;

// ============================================================================
// OCEAN DEPTH CONSTANTS
// ============================================================================

/// Ocean depth threshold for shallow water (0-12% of max depth)
pub const OCEAN_DEPTH_SHALLOW: f32 = 0.12;

/// Ocean depth threshold for medium depth (12-7% of max depth) 
pub const OCEAN_DEPTH_MEDIUM: f32 = 0.07;

/// Ocean depth threshold for deep ocean (below 7% of max depth)
pub const OCEAN_DEPTH_DEEP: f32 = 0.02;

// ============================================================================
// CAMERA CONSTANTS
// ============================================================================

/// How fast the camera zooms with mouse wheel
pub const CAMERA_ZOOM_SPEED: f32 = 0.1;

/// Minimum zoom level (zoomed in, 1.0 = normal)
pub const CAMERA_MIN_ZOOM: f32 = 0.3;

/// Maximum zoom level (zoomed out, higher = see more)
pub const CAMERA_MAX_ZOOM: f32 = 6.0;  // Increased for massive 900k hex worlds

/// Base pan speed for keyboard movement (pixels per second)
pub const CAMERA_PAN_SPEED_BASE: f32 = 500.0;

/// Speed multiplier when holding Shift
pub const CAMERA_SPEED_MULTIPLIER: f32 = 3.0;

/// Distance from window edge to trigger edge panning (pixels)
pub const CAMERA_EDGE_PAN_THRESHOLD: f32 = 10.0;

/// Base speed for edge panning (pixels per second)
pub const CAMERA_EDGE_PAN_SPEED_BASE: f32 = 800.0;

/// Padding factor when fitting map to screen (1.25 = 25% padding for better overview)
pub const CAMERA_MAP_PADDING_FACTOR: f32 = 1.25;

// ============================================================================
// UI CONSTANTS
// ============================================================================

/// Font size for tile info panel
pub const UI_TILE_INFO_TEXT_SIZE: f32 = 18.0;

/// UI padding as percentage of container size
pub const UI_PADDING_PERCENT: f32 = 1.0;

/// UI margin from screen edges as percentage
pub const UI_MARGIN_PERCENT: f32 = 2.0;

// ============================================================================
// SIMULATION CONSTANTS
// ============================================================================

/// Starting year for the simulation
pub const SIMULATION_STARTING_YEAR: u64 = 1000;

/// Days per year in simulation
pub const SIMULATION_DAYS_PER_YEAR: f32 = 365.0;

/// Default simulation speed multiplier
pub const SIMULATION_DEFAULT_SPEED: f32 = 1.0;

/// Maximum simulation speed multiplier
pub const SIMULATION_MAX_SPEED: f32 = 10.0;

// ============================================================================
// CLOUD GENERATION CONSTANTS
// ============================================================================

/// Minimum cloud sprite scale
pub const CLOUD_MIN_SCALE: f32 = 3.0;

/// Maximum cloud sprite scale
pub const CLOUD_MAX_SCALE: f32 = 6.0;

/// Number of cloud layers
pub const CLOUD_LAYER_COUNT: usize = 3;

/// Base cloud movement speed
pub const CLOUD_BASE_SPEED: f32 = 10.0;

// ============================================================================
// COLOR CONSTANTS
// ============================================================================

/// Deep ocean background color
pub const COLOR_OCEAN_BACKGROUND: Color = Color::srgb(0.02, 0.08, 0.15);

/// UI panel background color
pub const COLOR_UI_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);  // Dark semi-transparent

/// Tile info panel background color
pub const COLOR_TILE_INFO_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);  // Semi-transparent black

// ============================================================================
// PROVINCE GENERATION CONSTANTS
// ============================================================================

/// Minimum population for land provinces
pub const PROVINCE_MIN_POPULATION: f32 = 1000.0;

/// Maximum additional population for land provinces
pub const PROVINCE_MAX_ADDITIONAL_POPULATION: f32 = 49000.0;

/// Number of nations to spawn
pub const NATION_COUNT: usize = 8;

/// Number of tectonic plates (base value, actual is base + seed variation)
pub const TECTONIC_PLATES_BASE: usize = 4;  // Realistic number like Earth

/// Maximum additional tectonic plates
pub const TECTONIC_PLATES_VARIATION: u32 = 3;  // Results in 4-7 major landmasses

/// Number of island chains to generate
pub const ISLAND_CHAIN_COUNT: usize = 3;  // Much fewer islands

/// Number of archipelagos between continents
pub const ARCHIPELAGO_COUNT: usize = 2;  // Minimal archipelagos

/// Continent size multiplier (1.0 = original, 1.5 = 50% larger for 25% land coverage)
pub const CONTINENT_SIZE_MULTIPLIER: f32 = 1.5;

/// Massive continent base radius (Eurasia-sized) - scaled for 900k hex worlds
pub const CONTINENT_MASSIVE_BASE: f32 = 8000.0;  // MUCH larger for huge worlds
pub const CONTINENT_MASSIVE_VARIATION: f32 = 3000.0;

/// Medium continent base radius (Australia-sized)
pub const CONTINENT_MEDIUM_BASE: f32 = 5000.0;  // Scaled up significantly
pub const CONTINENT_MEDIUM_VARIATION: f32 = 2000.0;

/// Archipelago base radius (Indonesia-sized)
pub const CONTINENT_ARCHIPELAGO_BASE: f32 = 2000.0;  // Still sizable
pub const CONTINENT_ARCHIPELAGO_VARIATION: f32 = 800.0;

/// Tiny island base radius (Hawaii-sized)
pub const CONTINENT_TINY_BASE: f32 = 800.0;  // Even tiny islands are bigger
pub const CONTINENT_TINY_VARIATION: f32 = 400.0;

/// Falloff power for continent edges (higher = sharper edges, less land)
pub const CONTINENT_FALLOFF_BASE: f32 = 0.8;  // Gentler falloff for larger continents
pub const CONTINENT_FALLOFF_VARIATION: f32 = 0.3;

/// Number of rivers to generate
pub const RIVER_COUNT: usize = 50;

/// Minimum mountain elevation to spawn a river
pub const RIVER_MIN_ELEVATION: f32 = 0.7;

// ============================================================================
// SPATIAL INDEX CONSTANTS
// ============================================================================

/// Grid cell size for spatial indexing (as multiple of hex size)
pub const SPATIAL_INDEX_CELL_SIZE_MULTIPLIER: f32 = 2.0;

/// Grid cell size for ocean depth calculation (as multiple of hex size)
pub const OCEAN_DEPTH_GRID_SIZE_MULTIPLIER: f32 = 3.0;

// ============================================================================
// HEXAGON GEOMETRY CONSTANTS
// ============================================================================

/// Antialiasing width for hexagon texture edges (in pixels)
pub const HEXAGON_AA_WIDTH: f32 = 1.5;

/// Full opacity value for texture pixels
pub const TEXTURE_ALPHA_OPAQUE: u8 = 255;

/// Full transparency value for texture pixels
pub const TEXTURE_ALPHA_TRANSPARENT: u8 = 0;

// ============================================================================
// HEXAGON CALCULATIONS
// ============================================================================

/// Calculate hexagon position for a given grid coordinate
/// Uses flat-top hexagon with odd-q offset coordinate system
#[inline]
pub fn calculate_hex_position(col: u32, row: u32, hex_size: f32, provinces_per_row: u32, provinces_per_col: u32) -> (f32, f32) {
    // FLAT-TOP HONEYCOMB: Odd columns shift UP by half the vertical spacing
    let y_offset = if col % 2 == 1 { hex_size * SQRT3 / 2.0 } else { 0.0 };
    let x = (col as f32 - provinces_per_row as f32 / 2.0) * hex_size * 1.5;
    let y = (row as f32 - provinces_per_col as f32 / 2.0) * hex_size * SQRT3 + y_offset;
    (x, y)
}