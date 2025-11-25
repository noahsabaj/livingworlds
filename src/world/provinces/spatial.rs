//! Spatial indexing and lookups for provinces
//!
//! This module provides O(1) spatial lookups for provinces using grid-based indexing.
//! Essential for mouse picking, neighbor queries, and other spatial operations.

#![allow(dead_code)] // Preserve utility functions for future use

use super::types::ProvinceId;
use crate::math::{HEX_SIZE, SQRT_3};
use bevy::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

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

    /// Actual province positions for accurate distance calculations (indexed by province ID)
    pub province_positions: Vec<Vec2>,

    /// World bounds for clamping
    pub bounds: WorldBounds,
}

impl ProvincesSpatialIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(
        provinces: &[super::types::Province],
        dimensions: &crate::resources::MapDimensions,
    ) -> Self {
        let half_width = (dimensions.provinces_per_row as f32 * dimensions.hex_size * SQRT_3) / 2.0;
        let half_height = (dimensions.provinces_per_col as f32 * dimensions.hex_size * 1.5) / 2.0;

        let bounds = WorldBounds {
            min: Vec2::new(-half_width, -half_height),
            max: Vec2::new(half_width, half_height),
        };

        // Pre-allocate Vec for direct indexing by province ID
        let mut province_positions = vec![Vec2::ZERO; provinces.len()];

        // Parallel build grid and position indices
        let (grid_map, position_map): (Vec<_>, Vec<_>) = provinces
            .par_iter()
            .enumerate()
            .map(|(idx, province)| {
                let col = (idx as u32) % dimensions.provinces_per_row;
                let row = (idx as u32) / dimensions.provinces_per_row;

                let grid_entry = ((col as i32, row as i32), province.id);

                let pos_key = (
                    (province.position.x / dimensions.hex_size) as i32,
                    (province.position.y / dimensions.hex_size) as i32,
                );
                let position_entry = (pos_key, province.id);

                (grid_entry, position_entry)
            })
            .unzip();

        // Parallel collect province positions data
        let position_updates: Vec<(usize, Vec2)> = provinces.par_iter()
            .filter_map(|province| {
                let idx = province.id.value() as usize;
                if idx < province_positions.len() {
                    Some((idx, province.position))
                } else {
                    None
                }
            })
            .collect();

        // Sequential population (safe)
        for (idx, position) in position_updates {
            province_positions[idx] = position;
        }

        // Build HashMaps from parallel-computed entries
        let grid: HashMap<(i32, i32), ProvinceId> = grid_map.into_iter().collect();
        let position_to_province: HashMap<(i32, i32), ProvinceId> =
            position_map.into_iter().collect();

        Self {
            grid,
            position_to_province,
            province_positions,
            bounds,
        }
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

    /// Fast mouse picking - finds the province under the cursor with accurate hex testing
    /// Returns (ProvinceId, actual_position) if a province is found
    pub fn pick_province_at_position(
        &self,
        world_pos: Vec2,
        hex_size: f32,
    ) -> Option<(ProvinceId, Vec2)> {
        // First, get provinces in the immediate vicinity (3x3 grid around the click)
        let center_x = (world_pos.x / hex_size).round() as i32;
        let center_y = (world_pos.y / hex_size).round() as i32;

        let mut closest_province = None;
        let mut closest_distance = f32::MAX;

        // Check a small 3x3 grid around the click position
        for dx in -1..=1 {
            for dy in -1..=1 {
                let grid_x = center_x + dx;
                let grid_y = center_y + dy;

                if let Some(&province_id) = self.position_to_province.get(&(grid_x, grid_y)) {
                    // Get actual position (direct Vec indexing)
                    let id_idx = province_id.value() as usize;
                    if id_idx < self.province_positions.len() {
                        let actual_pos = self.province_positions[id_idx];

                        // Use hexagon geometry for accurate hit testing
                        let hexagon = crate::math::Hexagon::with_size(actual_pos, hex_size);
                        if hexagon.contains_point(world_pos) {
                            let dist = actual_pos.distance(world_pos);
                            if dist < closest_distance {
                                closest_distance = dist;
                                closest_province = Some((province_id, actual_pos));
                            }
                        }
                    }
                }
            }
        }

        closest_province
    }

    /// Check if a position is within world bounds
    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.x >= self.bounds.min.x
            && position.x <= self.bounds.max.x
            && position.y >= self.bounds.min.y
            && position.y <= self.bounds.max.y
    }

    pub fn bounds(&self) -> &WorldBounds {
        &self.bounds
    }

    /// Insert a province at a specific position
    pub fn insert(&mut self, position: Vec2, province_id: u32) {
        let province_id = ProvinceId::new(province_id);

        // Insert into position-based lookup for mouse picking
        let pos_key = (
            (position.x / HEX_SIZE) as i32,
            (position.y / HEX_SIZE) as i32,
        );
        self.position_to_province.insert(pos_key, province_id);

        // Store actual position for accurate distance calculations (extend Vec if needed)
        let id_idx = province_id.value() as usize;
        if id_idx >= self.province_positions.len() {
            self.province_positions.resize(id_idx + 1, Vec2::ZERO);
        }
        self.province_positions[id_idx] = position;
    }

    /// Query provinces near a world position
    /// Returns all provinces within search_radius of the given position
    pub fn query_near(&self, world_pos: Vec2, search_radius: f32) -> Vec<(Vec2, u32)> {
        let mut results = Vec::new();
        let cell_size = HEX_SIZE;

        let min_x = ((world_pos.x - search_radius) / cell_size).floor() as i32;
        let max_x = ((world_pos.x + search_radius) / cell_size).floor() as i32;
        let min_y = ((world_pos.y - search_radius) / cell_size).floor() as i32;
        let max_y = ((world_pos.y + search_radius) / cell_size).floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(province_id) = self.position_to_province.get(&(x, y)) {
                    // Use actual stored position for accurate distance calculations (direct Vec indexing)
                    let id_idx = province_id.value() as usize;
                    if id_idx < self.province_positions.len() {
                        let actual_pos = self.province_positions[id_idx];
                        let dist = world_pos.distance(actual_pos);
                        if dist <= search_radius {
                            results.push((actual_pos, province_id.value()));
                        }
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
        position.x >= self.min.x
            && position.x <= self.max.x
            && position.y >= self.min.y
            && position.y <= self.max.y
    }
}

/// Calculate hexagonal neighbors for a province at the given grid coordinates
///
/// Returns the 6 neighboring province IDs in order: NE, E, SE, SW, W, NW
/// None values indicate off-map or non-existent neighbors.
///
/// Uses crate::math::get_neighbor_positions as the single source of truth
/// for hex neighbor coordinate calculations.
pub fn calculate_hex_neighbors(
    col: u32,
    row: u32,
    provinces_per_row: u32,
    provinces_per_col: u32,
) -> [Option<ProvinceId>; 6] {
    let mut neighbors = [None; 6];

    // Use the canonical neighbor positions from math module
    let neighbor_coords = crate::math::get_neighbor_positions(col as i32, row as i32, 0.0);

    for (i, (new_col, new_row)) in neighbor_coords.iter().enumerate() {
        if *new_col >= 0
            && *new_col < provinces_per_row as i32
            && *new_row >= 0
            && *new_row < provinces_per_col as i32
        {
            let neighbor_id = (*new_row as u32) * provinces_per_row + (*new_col as u32);
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
