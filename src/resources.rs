//! Global resources for the Living Worlds game
//! 
//! Resources are singleton data that exists globally in the game world,
//! accessible from any system. Unlike components which are attached to
//! entities, resources represent game-wide state and configuration.

use bevy::prelude::*;
use std::collections::HashMap;

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

/// Whether to show the FPS counter overlay
#[derive(Resource)]
pub struct ShowFps(pub bool);

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
        use crate::HEX_SIZE_PIXELS;
        Self {
            cell_size: HEX_SIZE_PIXELS * 2.0, // 2x hex size for good coverage
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