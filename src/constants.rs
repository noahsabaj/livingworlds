//! Global constants for Living Worlds
//!
//! This module contains all game constants organized by category.
//! Centralizing constants ensures consistency and makes tuning easier.

use bevy::prelude::Color;
use crate::math::{HEX_SIZE, SQRT_3};

// Re-export commonly used hexagon size constant
pub const HEX_SIZE_PIXELS: f32 = HEX_SIZE;

// WORLD & MAP CONSTANTS

/// Number of province columns in the world
pub const PROVINCES_PER_ROW: u32 = 300;

/// Number of province rows in the world  
pub const PROVINCES_PER_COL: u32 = 200;

/// Distance from map edge where ocean is forced (in pixels)
pub const EDGE_BUFFER_PIXELS: f32 = 200.0;

// Derived constants for convenience
/// Total number of provinces in the world
pub const TOTAL_PROVINCES: u32 = PROVINCES_PER_ROW * PROVINCES_PER_COL;

/// Map width in pixels (for flat-top hexagons)
/// Formula: columns * hex_radius * 1.5 (horizontal spacing between columns)
pub const MAP_WIDTH_PIXELS: f32 = PROVINCES_PER_ROW as f32 * HEX_SIZE_PIXELS * 1.5;

/// Map height in pixels (for flat-top hexagons)  
/// Formula: rows * hex_radius * sqrt(3) (vertical spacing between rows)
pub const MAP_HEIGHT_PIXELS: f32 = PROVINCES_PER_COL as f32 * HEX_SIZE_PIXELS * SQRT_3;


/// Ocean elevation threshold for shallow water
/// Above 12% elevation = shallow coastal waters (light blue)
pub const OCEAN_ELEVATION_SHALLOW: f32 = 0.12;

/// Ocean elevation threshold for medium depth
/// 7-12% elevation = continental shelf (medium blue)
pub const OCEAN_ELEVATION_MEDIUM: f32 = 0.07;

/// Ocean elevation threshold for deep ocean
/// Below 2% elevation = abyssal depths (dark blue)
pub const OCEAN_ELEVATION_DEEP: f32 = 0.02;


/// How fast the camera zooms with mouse wheel
pub const CAMERA_ZOOM_SPEED: f32 = 0.1;

/// Minimum zoom level (zoomed in, 1.0 = normal)
/// 0.3 allows close inspection of individual provinces
pub const CAMERA_MIN_ZOOM: f32 = 0.3;

/// Maximum zoom level (zoomed out, higher = see more)
/// 6.0 allows viewing the entire 900k hex world
pub const CAMERA_MAX_ZOOM: f32 = 6.0;

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


/// Font size for tile info panel
pub const UI_TILE_INFO_TEXT_SIZE: f32 = 18.0;

/// UI padding as percentage of container size
pub const UI_PADDING_PERCENT: f32 = 1.0;

/// UI margin from screen edges as percentage
pub const UI_MARGIN_PERCENT: f32 = 2.0;


/// Starting year for the simulation
/// Year 1000 represents a medieval-like starting point
pub const SIMULATION_STARTING_YEAR: u32 = 1000;

/// Days per year in simulation (standard Earth year)
pub const SIMULATION_DAYS_PER_YEAR: u16 = 365;

/// Days per year as float for time calculations
pub const SIMULATION_DAYS_PER_YEAR_F32: f32 = 365.0;

/// Default simulation speed multiplier
pub const SIMULATION_DEFAULT_SPEED: f32 = 1.0;

/// Maximum simulation speed multiplier
pub const SIMULATION_MAX_SPEED: f32 = 10.0;


/// Minimum cloud sprite scale
pub const CLOUD_MIN_SCALE: f32 = 3.0;

/// Maximum cloud sprite scale
pub const CLOUD_MAX_SCALE: f32 = 6.0;

/// Number of cloud layers
pub const CLOUD_LAYER_COUNT: usize = 3;

/// Base cloud movement speed
pub const CLOUD_BASE_SPEED: f32 = 10.0;

/// Z-coordinate for off-screen positioning
pub const OFF_SCREEN_Z: f32 = -1000.0;


/// Deep ocean background color
pub const COLOR_OCEAN_BACKGROUND: Color = Color::srgb(0.02, 0.08, 0.15);

/// UI panel background color
pub const COLOR_UI_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);  // Dark semi-transparent

/// Tile info panel background color
pub const COLOR_TILE_INFO_BACKGROUND: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);  // Semi-transparent black


/// Minimum population for land provinces
/// 1000 represents a small rural settlement
pub const PROVINCE_MIN_POPULATION: u32 = 1000;

/// Maximum additional population for land provinces
/// Total max = 1000 + 49000 = 50000 (a modest city)
pub const PROVINCE_MAX_ADDITIONAL_POPULATION: u32 = 49000;

/// Number of nations to spawn
pub const NATION_COUNT: usize = 8;

/// Number of tectonic plates (base value, actual is base + seed variation)
/// 4-7 plates creates Earth-like continental distribution
pub const TECTONIC_PLATES_BASE: usize = 4;

/// Maximum additional tectonic plates
pub const TECTONIC_PLATES_VARIATION: u32 = 3;

/// Number of island chains to generate
/// Set to 0 to reduce ocean clutter, relying on volcanic hotspots instead
pub const ISLAND_CHAIN_COUNT: usize = 0;

/// Number of archipelagos between continents
/// 2 creates strategic island chains like Indonesia/Caribbean
pub const ARCHIPELAGO_COUNT: usize = 2;

/// Continent size multiplier (reduced to prevent Voronoi patterns)
pub const CONTINENT_SIZE_MULTIPLIER: f32 = 1.0;

/// Massive continent base radius (Eurasia-sized)
pub const CONTINENT_MASSIVE_BASE: f32 = 2500.0;
pub const CONTINENT_MASSIVE_VARIATION: f32 = 1000.0;

/// Medium continent base radius (Australia-sized)
pub const CONTINENT_MEDIUM_BASE: f32 = 1600.0;
pub const CONTINENT_MEDIUM_VARIATION: f32 = 600.0;

/// Archipelago base radius (Indonesia-sized)
pub const CONTINENT_ARCHIPELAGO_BASE: f32 = 800.0;
pub const CONTINENT_ARCHIPELAGO_VARIATION: f32 = 300.0;

/// Tiny island base radius (Hawaii-sized)
pub const CONTINENT_TINY_BASE: f32 = 400.0;
pub const CONTINENT_TINY_VARIATION: f32 = 200.0;

/// Falloff power for continent edges
/// 0.8 creates gentler slopes for more realistic coastlines
pub const CONTINENT_FALLOFF_BASE: f32 = 0.8;
pub const CONTINENT_FALLOFF_VARIATION: f32 = 0.3;

/// Number of landmass seeds to generate (independent of tectonic plates)
/// More seeds = more varied and natural continent distribution
pub const LANDMASS_SEED_COUNT_MIN: u32 = 20;
pub const LANDMASS_SEED_COUNT_MAX: u32 = 35;
          
/// Landmass shape irregularity (0.0 = circular, 1.0 = very irregular)
pub const LANDMASS_SHAPE_IRREGULARITY: f32 = 0.6;
          
/// Number of noise octaves for continent shape complexity
pub const LANDMASS_NOISE_OCTAVES: u32 = 4;
          
/// Coastline complexity factor (fractal dimension)
pub const LANDMASS_COASTLINE_COMPLEXITY: f32 = 0.4;
          
/// Tectonic influence weight (reduced to prevent plate-defined continents)
pub const TECTONIC_INFLUENCE_WEIGHT: f32 = 0.15;
          
/// Base terrain noise weight (increased for more variation)
pub const BASE_TERRAIN_WEIGHT: f32 = 0.5;
          
/// Landmass influence weight (for continent vs ocean)
pub const LANDMASS_INFLUENCE_WEIGHT: f32 = 0.35;

/// Number of rivers to generate
/// 200 rivers provides good coverage for large worlds
pub const RIVER_COUNT: usize = 200;

/// Minimum mountain elevation to spawn a river
/// 0.5 (50% elevation) ensures rivers start from highlands
pub const RIVER_MIN_ELEVATION: f32 = 0.5;


/// Grid cell size for spatial indexing (as multiple of hex size)
pub const SPATIAL_INDEX_CELL_SIZE_MULTIPLIER: f32 = 2.0;

/// Grid cell size for ocean depth calculation (as multiple of hex size)
pub const OCEAN_DEPTH_GRID_SIZE_MULTIPLIER: f32 = 3.0;

/// Full opacity value for texture pixels
pub const TEXTURE_ALPHA_OPAQUE: u8 = 255;

/// Full transparency value for texture pixels
pub const TEXTURE_ALPHA_TRANSPARENT: u8 = 0;


/// Milliseconds per second for time conversions
pub const MS_PER_SECOND: f32 = 1000.0;

/// Degrees in a full circle
pub const DEGREES_IN_CIRCLE: f32 = 360.0;

// COMPILE-TIME ASSERTIONS

// Validate camera zoom invariants
const _: () = assert!(CAMERA_MIN_ZOOM > 0.0, "Minimum zoom must be positive");
const _: () = assert!(CAMERA_MIN_ZOOM < CAMERA_MAX_ZOOM, "Min zoom must be less than max zoom");

// Validate ocean elevation thresholds are in correct order
const _: () = assert!(OCEAN_ELEVATION_DEEP < OCEAN_ELEVATION_MEDIUM, "Deep ocean must be lower than medium");
const _: () = assert!(OCEAN_ELEVATION_MEDIUM < OCEAN_ELEVATION_SHALLOW, "Medium ocean must be lower than shallow");

// Validate world dimensions
const _: () = assert!(PROVINCES_PER_ROW > 0, "Must have at least one column");
const _: () = assert!(PROVINCES_PER_COL > 0, "Must have at least one row");

// Validate simulation speeds
const _: () = assert!(SIMULATION_DEFAULT_SPEED > 0.0, "Default speed must be positive");
const _: () = assert!(SIMULATION_MAX_SPEED >= SIMULATION_DEFAULT_SPEED, "Max speed must be >= default");

// Validate population ranges
const _: () = assert!(PROVINCE_MIN_POPULATION > 0, "Minimum population must be positive");