//! Hexagonal neighbor calculation and indexing
//!
//! Handles the mathematics of hexagonal grid neighbors and precomputes
//! neighbor indices for efficient O(1) access.

use crate::resources::MapDimensions;
use crate::world::generation::GenerationUtils;
use crate::world::provinces::{Province, ProvinceId};

/// Calculates hexagonal grid neighbors
pub struct NeighborCalculator {
    dimensions: MapDimensions,
    utils: GenerationUtils,
}

impl NeighborCalculator {
    pub fn new(dimensions: MapDimensions) -> Self {
        let utils = GenerationUtils::new(dimensions);
        Self { dimensions, utils }
    }

    /// Calculate hex neighbors for a given grid position
    pub fn calculate_hex_neighbors(&self, col: u32, row: u32) -> [Option<ProvinceId>; 6] {
        // Use shared utilities for neighbor coordinate calculation
        let neighbor_coords = self.utils.get_neighbor_coords(col as i32, row as i32);
        let mut neighbors = [None; 6];

        // Convert coordinates to ProvinceId using shared utilities
        for (i, (neighbor_col, neighbor_row)) in neighbor_coords.iter().enumerate() {
            if let Some(province_id) = self.utils.grid_coords_to_id(*neighbor_col, *neighbor_row) {
                neighbors[i] = Some(province_id);
            }
        }

        neighbors
    }

    /// Get valid neighbor index within bounds
    fn get_neighbor_index(&self, col: i32, row: i32) -> Option<ProvinceId> {
        if col >= 0 && row >= 0
            && col < self.dimensions.provinces_per_row as i32
            && row < self.dimensions.provinces_per_col as i32 {
            Some(ProvinceId::new(row as u32 * self.dimensions.provinces_per_row + col as u32))
        } else {
            None
        }
    }
}

/// Precompute neighbor indices for all provinces for O(1) neighbor access
/// This eliminates HashMap lookups during world generation
pub fn precompute_neighbor_indices(provinces: &mut Vec<Province>) {
    // Since province IDs are sequential (0, 1, 2, ...), we can use direct indexing
    let province_count = provinces.len();

    // Populate neighbor indices for each province
    for province in provinces.iter_mut() {
        // Clear and resize the neighbor_indices array
        province.neighbor_indices = [None; 6];

        for (i, neighbor_id_opt) in province.neighbors.iter().enumerate() {
            if let Some(neighbor_id) = neighbor_id_opt {
                // Province IDs are sequential, so ID == index
                let neighbor_idx = neighbor_id.value() as usize;
                // Bounds check for safety
                if neighbor_idx < province_count {
                    province.neighbor_indices[i] = Some(neighbor_idx);
                }
            }
        }
    }
}