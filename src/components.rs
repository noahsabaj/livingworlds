//! Core game components and resources
//! 
//! This module contains all the ECS components and resources used throughout
//! the game. Components are data attached to entities, while resources are
//! global singletons accessible from any system.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::terrain::TerrainType;

// ============================================================================
// COMPONENTS - Data attached to entities
// ============================================================================

/// Province represents a single hexagonal tile in the world
#[derive(Component, Clone)]
pub struct Province {
    pub id: u32,
    pub position: Vec2,
    pub nation_id: Option<u32>,  // None for ocean provinces
    pub population: f32,
    pub terrain: TerrainType,
    pub elevation: f32,
}

/// Marker component for the currently selected province
#[derive(Component)]
pub struct SelectedProvince;

/// Marker for ghost provinces (duplicates for world wrapping)
/// These are visual duplicates shown at map edges for seamless scrolling
#[derive(Component)]
pub struct GhostProvince {
    pub original_col: u32,  // Original column this is a ghost of
}

/// Nation represents a political entity that controls provinces
#[derive(Component, Clone)]
pub struct Nation {
    pub id: u32,
    pub name: String,
    pub color: Color,
}

/// Marker component for the tile info UI panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text display
#[derive(Component)]
pub struct TileInfoText;

// ============================================================================
// RESOURCES - Global singletons
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