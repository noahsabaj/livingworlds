//! Global resources for the Living Worlds game
//! 
//! Resources are singleton data that exists globally in the game world,
//! accessible from any system. Unlike components which are attached to
//! entities, resources represent game-wide state and configuration.

use bevy::prelude::*;
use bevy::math::Vec2;
use std::collections::HashMap;
use crate::components::MineralType;

// ============================================================================
// WORLD CONFIGURATION RESOURCES
// ============================================================================

/// Configuration for world generation - the seed determines the entire world
#[derive(Resource)]
pub struct WorldSeed(pub u32);

/// World size configuration controlling map dimensions
#[derive(Resource, Clone, Copy)]
pub enum WorldSize {
    Small,   // 150x100 provinces
    Medium,  // 300x200 provinces (default)
    Large,   // 450x300 provinces
}

impl WorldSize {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" => WorldSize::Small,
            "large" => WorldSize::Large,
            _ => WorldSize::Medium,
        }
    }
    
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            WorldSize::Small => (150, 100),
            WorldSize::Medium => (300, 200),
            WorldSize::Large => (450, 300),
        }
    }
}

/// Map dimensions calculated from world size
#[derive(Resource, Debug, Clone, Copy)]
pub struct MapDimensions {
    pub provinces_per_row: u32,
    pub provinces_per_col: u32,
    pub width_pixels: f32,
    pub height_pixels: f32,
}

impl MapDimensions {
    pub fn from_world_size(size: &WorldSize) -> Self {
        let (provinces_per_row, provinces_per_col) = size.dimensions();
        let provinces_per_row = provinces_per_row as u32;
        let provinces_per_col = provinces_per_col as u32;
        
        use crate::constants::{HEX_SIZE_PIXELS, SQRT3};
        let width_pixels = provinces_per_row as f32 * HEX_SIZE_PIXELS * 1.5;
        let height_pixels = provinces_per_col as f32 * HEX_SIZE_PIXELS * SQRT3;
        
        Self {
            provinces_per_row,
            provinces_per_col,
            width_pixels,
            height_pixels,
        }
    }
}

// ============================================================================
// GAME STATE RESOURCES
// ============================================================================

/// Current game time and simulation speed
#[derive(Resource)]
pub struct GameTime {
    pub current_date: f32, // Days since start
    pub speed: f32,        // Time multiplier
    pub paused: bool,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            current_date: 0.0,
            speed: 1.0,
            paused: false,
        }
    }
}

/// World Tension - Global metric driving music and atmosphere
/// 
/// Tension ranges from 0.0 (perfect peace) to 1.0 (world war).
/// It rises quickly with conflicts but falls slowly during peace,
/// simulating how real-world tensions have momentum.
#[derive(Resource)]
pub struct WorldTension {
    /// Current tension level (0.0 to 1.0)
    pub current: f32,
    /// Target tension based on world state
    pub target: f32,
    /// Rate of change
    pub velocity: f32,
    
    // Contributing factors (each 0.0 to 1.0)
    /// Percentage of nations at war
    pub war_factor: f32,
    /// Power imbalance (one nation too dominant)
    pub power_imbalance: f32,
    /// Economic disruption (trade routes broken)
    pub economic_stress: f32,
    /// Recent collapses or disasters
    pub instability_factor: f32,
    
    // Physics parameters
    /// How fast tension rises (default: 2.0)
    pub heating_rate: f32,
    /// How slowly tension falls (default: 0.3)
    pub cooling_rate: f32,
    /// Resistance to change (default: 0.8)
    pub inertia: f32,
}

impl Default for WorldTension {
    fn default() -> Self {
        Self {
            current: 0.0,  // Start at perfect peace
            target: 0.0,
            velocity: 0.0,
            
            war_factor: 0.0,
            power_imbalance: 0.0,
            economic_stress: 0.0,
            instability_factor: 0.0,
            
            heating_rate: 2.0,    // Wars escalate quickly
            cooling_rate: 0.3,    // Peace returns slowly
            inertia: 0.8,         // Smooth transitions
        }
    }
}

impl WorldTension {
    /// Calculate tension from war percentage using exponential curve
    /// 
    /// This uses a power function to make tension rise exponentially:
    /// - 10% at war = ~18% tension (local conflicts)
    /// - 25% at war = ~40% tension (regional wars)  
    /// - 50% at war = ~70% tension (world crisis)
    /// - 75% at war = ~90% tension (near apocalypse)
    /// - 100% at war = 100% tension (total war)
    pub fn calculate_from_war_percentage(war_percentage: f32) -> f32 {
        // Use square root for exponential growth
        // This makes small conflicts barely register but large wars escalate rapidly
        war_percentage.sqrt().clamp(0.0, 1.0)
    }
}

// ============================================================================
// WEATHER SYSTEM RESOURCES
// ============================================================================

/// Weather states representing different atmospheric conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherState {
    Clear,      // 0-10% cloud coverage - bright sunny day
    Fair,       // 10-30% cloud coverage - pleasant with some clouds
    Partly,     // 30-60% cloud coverage - mix of sun and clouds
    Cloudy,     // 60-80% cloud coverage - mostly cloudy
    Overcast,   // 80-100% cloud coverage - completely grey sky
    Storm,      // 90-100% coverage + dark clouds and rain
}

impl WeatherState {
    /// Get the cloud coverage range for this weather state
    pub fn coverage_range(&self) -> (f32, f32) {
        match self {
            WeatherState::Clear => (0.0, 0.1),
            WeatherState::Fair => (0.1, 0.3),
            WeatherState::Partly => (0.3, 0.6),
            WeatherState::Cloudy => (0.6, 0.8),
            WeatherState::Overcast => (0.8, 1.0),
            WeatherState::Storm => (0.9, 1.0),
        }
    }
    
    /// Get a descriptive name for the weather
    pub fn description(&self) -> &str {
        match self {
            WeatherState::Clear => "Clear skies",
            WeatherState::Fair => "Fair weather",
            WeatherState::Partly => "Partly cloudy",
            WeatherState::Cloudy => "Cloudy",
            WeatherState::Overcast => "Overcast",
            WeatherState::Storm => "Stormy",
        }
    }
}

/// Dynamic weather system controlling cloud coverage and atmospheric conditions
#[derive(Resource)]
pub struct WeatherSystem {
    /// Current weather state
    pub current_state: WeatherState,
    /// Target weather state we're transitioning to
    pub target_state: WeatherState,
    /// Progress of transition (0.0 = start, 1.0 = complete)
    pub transition_progress: f32,
    /// Current cloud coverage (0.0 = clear, 1.0 = overcast)
    pub cloud_coverage: f32,
    /// Wind speed and direction
    pub wind_speed: Vec2,
    /// Time since last weather change in seconds
    pub time_since_change: f32,
    /// Minimum time before next weather change
    pub min_weather_duration: f32,
    /// Random weather change chance per second
    pub weather_change_chance: f32,
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            current_state: WeatherState::Partly,  // More clouds initially
            target_state: WeatherState::Partly,
            transition_progress: 1.0,
            cloud_coverage: 0.5,  // Start with 50% coverage instead of 20%
            wind_speed: Vec2::new(5.0, 1.0),
            time_since_change: 0.0,
            min_weather_duration: 60.0,  // At least 1 minute per weather
            weather_change_chance: 0.01, // 1% chance per second after min duration
        }
    }
}

// ============================================================================
// GAMEPLAY RESOURCES
// ============================================================================

/// Tracks information about the currently selected province
#[derive(Resource, Default)]
pub struct SelectedProvinceInfo {
    pub entity: Option<Entity>,
    pub province_id: Option<u32>,
}

/// Spatial index for O(1) province lookups instead of O(n) linear search
/// This dramatically improves performance for mouse picking and neighbor queries
#[derive(Resource)]
pub struct ProvincesSpatialIndex {
    /// Grid cell size - should be about 2x hexagon size for optimal performance
    pub cell_size: f32,
    /// HashMap: grid_coord -> list of (entity, position, province_id)
    pub grid: HashMap<(i32, i32), Vec<(Entity, Vec2, u32)>>,
}

impl Default for ProvincesSpatialIndex {
    fn default() -> Self {
        use crate::constants::{HEX_SIZE_PIXELS, SPATIAL_INDEX_CELL_SIZE_MULTIPLIER};
        Self {
            cell_size: HEX_SIZE_PIXELS * SPATIAL_INDEX_CELL_SIZE_MULTIPLIER,
            grid: HashMap::new(),
        }
    }
}

impl ProvincesSpatialIndex {
    /// Insert a province into the spatial index
    pub fn insert(&mut self, entity: Entity, position: Vec2, province_id: u32) {
        let grid_x = (position.x / self.cell_size).floor() as i32;
        let grid_y = (position.y / self.cell_size).floor() as i32;
        
        self.grid
            .entry((grid_x, grid_y))
            .or_insert_with(Vec::new)
            .push((entity, position, province_id));
    }
    
    /// Query provinces near a world position
    /// Returns all provinces within search_radius of the given position
    pub fn query_near(&self, world_pos: Vec2, search_radius: f32) -> Vec<(Entity, Vec2, u32)> {
        let mut results = Vec::new();
        
        // Calculate grid cells to check based on search radius
        let min_x = ((world_pos.x - search_radius) / self.cell_size).floor() as i32;
        let max_x = ((world_pos.x + search_radius) / self.cell_size).floor() as i32;
        let min_y = ((world_pos.y - search_radius) / self.cell_size).floor() as i32;
        let max_y = ((world_pos.y + search_radius) / self.cell_size).floor() as i32;
        
        // Check all relevant grid cells
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(provinces) = self.grid.get(&(x, y)) {
                    for &(entity, pos, id) in provinces {
                        let dist = world_pos.distance(pos);
                        if dist <= search_radius {
                            results.push((entity, pos, id));
                        }
                    }
                }
            }
        }
        
        results
    }
}

// ============================================================================
// VISUALIZATION RESOURCES
// ============================================================================

/// Resource visualization overlay modes for displaying mineral distribution
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceOverlay {
    /// No overlay - show normal political/terrain colors
    None,
    /// Show specific mineral abundance
    Mineral(MineralType),
    /// Show all minerals combined (richness heat map)
    AllMinerals,
    /// Show extraction infrastructure (mine levels)
    Infrastructure,
}

impl Default for ResourceOverlay {
    fn default() -> Self {
        ResourceOverlay::None
    }
}

impl ResourceOverlay {
    /// Cycle to the next overlay mode
    pub fn cycle(&mut self) {
        *self = match self {
            ResourceOverlay::None => ResourceOverlay::Mineral(MineralType::Iron),
            ResourceOverlay::Mineral(MineralType::Iron) => ResourceOverlay::Mineral(MineralType::Copper),
            ResourceOverlay::Mineral(MineralType::Copper) => ResourceOverlay::Mineral(MineralType::Tin),
            ResourceOverlay::Mineral(MineralType::Tin) => ResourceOverlay::Mineral(MineralType::Gold),
            ResourceOverlay::Mineral(MineralType::Gold) => ResourceOverlay::Mineral(MineralType::Coal),
            ResourceOverlay::Mineral(MineralType::Coal) => ResourceOverlay::Mineral(MineralType::Stone),
            ResourceOverlay::Mineral(MineralType::Stone) => ResourceOverlay::Mineral(MineralType::Gems),
            ResourceOverlay::Mineral(MineralType::Gems) => ResourceOverlay::AllMinerals,
            ResourceOverlay::AllMinerals => ResourceOverlay::Infrastructure,
            ResourceOverlay::Infrastructure => ResourceOverlay::None,
            _ => ResourceOverlay::None,
        }
    }
    
    /// Get display name for current overlay
    pub fn display_name(&self) -> &str {
        match self {
            ResourceOverlay::None => "Political Map",
            ResourceOverlay::Mineral(MineralType::Iron) => "Iron Deposits",
            ResourceOverlay::Mineral(MineralType::Copper) => "Copper Deposits",
            ResourceOverlay::Mineral(MineralType::Tin) => "Tin Deposits",
            ResourceOverlay::Mineral(MineralType::Gold) => "Gold Deposits",
            ResourceOverlay::Mineral(MineralType::Coal) => "Coal Deposits",
            ResourceOverlay::Mineral(MineralType::Stone) => "Stone Deposits",
            ResourceOverlay::Mineral(MineralType::Gems) => "Gem Deposits",
            ResourceOverlay::AllMinerals => "All Minerals",
            ResourceOverlay::Infrastructure => "Mining Infrastructure",
            _ => "Unknown",
        }
    }
}