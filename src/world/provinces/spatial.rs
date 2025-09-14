//! Spatial indexing and lookups for provinces
//!
//! This module provides O(1) spatial lookups for provinces using grid-based indexing.
//! Essential for mouse picking, neighbor queries, and other spatial operations.

use bevy::prelude::*;
use std::collections::HashMap;
use super::types::ProvinceId;


/// Grid-based spatial index for O(1) province lookups
///
/// Maps grid coordinates to province IDs for fast spatial queries.
/// Used extensively for mouse picking and neighbor calculations.
#[derive(Debug, Clone, Default, Resource)]
pub struct ProvincesSpatialIndex {
    /// Grid mapping (col, row) to province ID
    pub grid: HashMap<(i32, i32), ProvinceId>,

    /// Position to province ID mapping for mouse picking
    pub position_to_province: HashMap<(i32, i32), ProvinceId>,

    /// World bounds for clamping
    pub bounds: WorldBounds,
}

impl ProvincesSpatialIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(provinces: &[super::types::Province], dimensions: &crate::resources::MapDimensions) -> Self {
        let mut index = Self::new();

        let half_width = (dimensions.provinces_per_row as f32 * dimensions.hex_size * 1.732) / 2.0;
        let half_height = (dimensions.provinces_per_col as f32 * dimensions.hex_size * 1.5) / 2.0;

        index.bounds = WorldBounds {
            min: Vec2::new(-half_width, -half_height),
            max: Vec2::new(half_width, half_height),
        };

        for (idx, province) in provinces.iter().enumerate() {
            let col = (idx as u32) % dimensions.provinces_per_row;
            let row = (idx as u32) / dimensions.provinces_per_row;

            index.grid.insert((col as i32, row as i32), province.id);

            // Also map quantized positions for mouse picking
            let pos_key = (
                (province.position.x / dimensions.hex_size) as i32,
                (province.position.y / dimensions.hex_size) as i32,
            );
            index.position_to_province.insert(pos_key, province.id);
        }

        index
    }

    /// Get province at grid coordinates
    pub fn get_at_grid(&self, col: i32, row: i32) -> Option<ProvinceId> {
        self.grid.get(&(col, row)).copied()
    }

    /// Get province at world position
    pub fn get_at_position(&self, position: Vec2, hex_size: f32) -> Option<ProvinceId> {
        let pos_key = (
            (position.x / hex_size) as i32,
            (position.y / hex_size) as i32,
        );
        self.position_to_province.get(&pos_key).copied()
    }

    /// Check if a position is within world bounds
    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.x >= self.bounds.min.x &&
        position.x <= self.bounds.max.x &&
        position.y >= self.bounds.min.y &&
        position.y <= self.bounds.max.y
    }

    pub fn bounds(&self) -> &WorldBounds {
        &self.bounds
    }

    /// Insert a province at a specific position
    pub fn insert(&mut self, position: Vec2, province_id: u32) {
        let province_id = ProvinceId::new(province_id);

        // Insert into position-based lookup for mouse picking
        let pos_key = (
            (position.x / 50.0) as i32, // TODO: Pass hex_size parameter instead of hardcoding
            (position.y / 50.0) as i32,
        );
        self.position_to_province.insert(pos_key, province_id);
    }

    /// Query provinces near a world position
    /// Returns all provinces within search_radius of the given position
    pub fn query_near(&self, world_pos: Vec2, search_radius: f32) -> Vec<(Vec2, u32)> {
        let mut results = Vec::new();
        let cell_size = 50.0; // TODO: Make this configurable

        let min_x = ((world_pos.x - search_radius) / cell_size).floor() as i32;
        let max_x = ((world_pos.x + search_radius) / cell_size).floor() as i32;
        let min_y = ((world_pos.y - search_radius) / cell_size).floor() as i32;
        let max_y = ((world_pos.y + search_radius) / cell_size).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(province_id) = self.position_to_province.get(&(x, y)) {
                    // For now, return the grid position as the province position
                    // TODO: Store actual positions in the spatial index
                    let pos = Vec2::new(x as f32 * cell_size, y as f32 * cell_size);
                    let dist = (world_pos - pos).length();
                    if dist <= search_radius {
                        results.push((pos, province_id.value()));
                    }
                }
            }
        }

        results
    }
}

/// World boundary information
#[derive(Debug, Clone, Default)]
pub struct WorldBounds {
    /// Minimum world coordinates (bottom-left)
    pub min: Vec2,

    /// Maximum world coordinates (top-right)
    pub max: Vec2,
}

impl WorldBounds {
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Clamp a position to world bounds
    pub fn clamp(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x.clamp(self.min.x, self.max.x),
            position.y.clamp(self.min.y, self.max.y),
        )
    }

    /// Check if a position is within bounds
    pub fn contains(&self, position: Vec2) -> bool {
        position.x >= self.min.x &&
        position.x <= self.max.x &&
        position.y >= self.min.y &&
        position.y <= self.max.y
    }
}


/// Calculate hexagonal neighbors for a province at the given grid coordinates
///
/// Returns the 6 neighboring province IDs in order: NE, E, SE, SW, W, NW
/// None values indicate off-map or non-existent neighbors.
pub fn calculate_hex_neighbors(
    col: u32,
    row: u32,
    provinces_per_row: u32,
    provinces_per_col: u32,
) -> [Option<ProvinceId>; 6] {
    let mut neighbors = [None; 6];

    // Odd-q offset coordinate system adjustments
    let is_odd_col = col % 2 == 1;

    // Define neighbor offsets based on column parity
    let offsets = if is_odd_col {
        // Odd columns are shifted down
        [
            (1, 0),   // NE
            (1, 1),   // E
            (0, 1),   // SE
            (-1, 1),  // SW
            (-1, 0),  // W
            (0, -1),  // NW
        ]
    } else {
        // Even columns
        [
            (1, -1),  // NE
            (1, 0),   // E
            (0, 1),   // SE
            (-1, 0),  // SW
            (-1, -1), // W
            (0, -1),  // NW
        ]
    };

    for (i, (dc, dr)) in offsets.iter().enumerate() {
        let new_col = col as i32 + dc;
        let new_row = row as i32 + dr;

        if new_col >= 0 &&
           new_col < provinces_per_row as i32 &&
           new_row >= 0 &&
           new_row < provinces_per_col as i32
        {
            let neighbor_id = (new_row as u32) * provinces_per_row + (new_col as u32);
            neighbors[i] = Some(ProvinceId::new(neighbor_id));
        }
    }

    neighbors
}

pub fn opposite_hex_direction(direction: usize) -> usize {
    match direction {
        0 => 3, // NE -> SW
        1 => 4, // E -> W
        2 => 5, // SE -> NW
        3 => 0, // SW -> NE
        4 => 1, // W -> E
        5 => 2, // NW -> SE
        _ => 0, // Invalid direction
    }
}