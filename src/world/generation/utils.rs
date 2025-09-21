//! Shared utilities for world generation
//!
//! This module provides a single source of truth for common generation operations
//! across provinces, rivers, clouds, and other world systems. It eliminates
//! duplicate code and provides consistent, optimized implementations.

use bevy::prelude::Vec2;
use rand::rngs::StdRng;
use rand::Rng;

use crate::resources::MapDimensions;
use crate::world::provinces::{Province, ProvinceId};

/// Comprehensive utilities for world generation
///
/// This struct encapsulates all common generation operations and provides
/// a consistent API for grid operations, array indexing, and spatial queries.
pub struct GenerationUtils {
    dimensions: MapDimensions,
}

impl GenerationUtils {
    /// Create new generation utilities for the given map dimensions
    pub fn new(dimensions: MapDimensions) -> Self {
        Self { dimensions }
    }

    /// Get map dimensions
    pub fn dimensions(&self) -> &MapDimensions {
        &self.dimensions
    }

    // ============================================================================
    // GRID COORDINATE OPERATIONS (Single source of truth)
    // ============================================================================

    /// Convert province ID to grid coordinates
    ///
    /// This is the canonical implementation used across all generation systems.
    pub fn id_to_grid_coords(&self, id: ProvinceId) -> (i32, i32) {
        let col = (id.value() % self.dimensions.provinces_per_row) as i32;
        let row = (id.value() / self.dimensions.provinces_per_row) as i32;
        (col, row)
    }

    /// Convert grid coordinates to province ID
    pub fn grid_coords_to_id(&self, col: i32, row: i32) -> Option<ProvinceId> {
        if col < 0
            || row < 0
            || col >= self.dimensions.provinces_per_row as i32
            || row >= self.dimensions.provinces_per_col as i32
        {
            return None;
        }

        let id = (row as u32) * self.dimensions.provinces_per_row + (col as u32);
        Some(ProvinceId::new(id))
    }

    /// Convert grid coordinates to array index
    pub fn grid_coords_to_index(&self, col: i32, row: i32) -> Option<usize> {
        self.grid_coords_to_id(col, row)
            .map(|id| id.value() as usize)
    }

    /// Get total number of provinces
    pub fn total_provinces(&self) -> u32 {
        self.dimensions.provinces_per_row * self.dimensions.provinces_per_col
    }

    // ============================================================================
    // ARRAY ACCESS PATTERNS (Safe, optimized access)
    // ============================================================================

    /// Get province at grid coordinates using direct array indexing
    ///
    /// This is the canonical implementation that eliminates HashMap lookups
    /// while providing safe bounds checking.
    pub fn get_province_at_grid<'a>(
        &self,
        provinces: &'a [Province],
        col: i32,
        row: i32,
    ) -> Option<&'a Province> {
        if let Some(index) = self.grid_coords_to_index(col, row) {
            provinces.get(index)
        } else {
            None
        }
    }

    /// Get mutable province at grid coordinates
    pub fn get_province_at_grid_mut<'a>(
        &self,
        provinces: &'a mut [Province],
        col: i32,
        row: i32,
    ) -> Option<&'a mut Province> {
        if let Some(index) = self.grid_coords_to_index(col, row) {
            provinces.get_mut(index)
        } else {
            None
        }
    }

    /// Safe province access by index with bounds checking
    pub fn safe_province_access<'a>(
        &self,
        provinces: &'a [Province],
        index: usize,
    ) -> Option<&'a Province> {
        provinces.get(index)
    }

    // ============================================================================
    // NEIGHBOR OPERATIONS (Hexagon-aware navigation)
    // ============================================================================

    /// Get all neighbor grid coordinates for a given position
    ///
    /// Uses the canonical hexagon neighbor calculation from math module
    /// but provides a convenient interface for generation systems.
    pub fn get_neighbor_coords(&self, col: i32, row: i32) -> [(i32, i32); 6] {
        // Use the single source of truth from math module
        let neighbor_positions =
            crate::math::get_neighbor_positions(col, row, self.dimensions.hex_size);

        let mut coords = [(0, 0); 6];
        for (i, (neighbor_col, neighbor_row)) in neighbor_positions.iter().enumerate() {
            coords[i] = (*neighbor_col, *neighbor_row);
        }
        coords
    }

    /// Get neighbor province indices for efficient array access
    ///
    /// Returns array indices that can be used for direct Vec access,
    /// eliminating HashMap lookups entirely.
    pub fn get_neighbor_indices(&self, col: i32, row: i32) -> [Option<usize>; 6] {
        let neighbor_coords = self.get_neighbor_coords(col, row);
        let mut indices = [None; 6];

        for (i, (neighbor_col, neighbor_row)) in neighbor_coords.iter().enumerate() {
            indices[i] = self.grid_coords_to_index(*neighbor_col, *neighbor_row);
        }

        indices
    }

    /// Get neighbor provinces directly for convenient iteration
    pub fn get_neighbor_provinces<'a>(
        &self,
        provinces: &'a [Province],
        col: i32,
        row: i32,
    ) -> [Option<&'a Province>; 6] {
        let neighbor_coords = self.get_neighbor_coords(col, row);
        let mut neighbors = [None; 6];

        for (i, (neighbor_col, neighbor_row)) in neighbor_coords.iter().enumerate() {
            neighbors[i] = self.get_province_at_grid(provinces, *neighbor_col, *neighbor_row);
        }

        neighbors
    }

    // ============================================================================
    // RANDOM SAMPLING UTILITIES (Consistent sampling patterns)
    // ============================================================================

    /// Generate random position within map bounds
    ///
    /// This is the canonical implementation for random position sampling
    /// used across all generation systems.
    pub fn random_position(&self, rng: &mut StdRng) -> Vec2 {
        let x = rng.gen_range(self.dimensions.bounds.x_min..self.dimensions.bounds.x_max);
        let y = rng.gen_range(self.dimensions.bounds.y_min..self.dimensions.bounds.y_max);
        Vec2::new(x, y)
    }

    /// Generate random position within custom bounds
    pub fn random_position_in_bounds(&self, rng: &mut StdRng, min: Vec2, max: Vec2) -> Vec2 {
        let x = rng.gen_range(min.x..max.x);
        let y = rng.gen_range(min.y..max.y);
        Vec2::new(x, y)
    }

    /// Generate random grid coordinates
    pub fn random_grid_coords(&self, rng: &mut StdRng) -> (i32, i32) {
        let col = rng.gen_range(0..self.dimensions.provinces_per_row as i32);
        let row = rng.gen_range(0..self.dimensions.provinces_per_col as i32);
        (col, row)
    }

    /// Generate random province index
    pub fn random_province_index(&self, rng: &mut StdRng) -> usize {
        rng.gen_range(0..self.total_provinces() as usize)
    }

    // ============================================================================
    // VALIDATION UTILITIES (Consistency checks)
    // ============================================================================

    /// Validate that province array matches expected dimensions
    pub fn validate_province_array(&self, provinces: &[Province]) -> Result<(), String> {
        let expected_count = self.total_provinces() as usize;
        if provinces.len() != expected_count {
            return Err(format!(
                "Province array size mismatch: expected {}, got {}",
                expected_count,
                provinces.len()
            ));
        }

        // Validate that province IDs are sequential
        for (index, province) in provinces.iter().enumerate() {
            if province.id.value() != index as u32 {
                return Err(format!(
                    "Province ID mismatch at index {}: expected {}, got {}",
                    index,
                    index,
                    province.id.value()
                ));
            }
        }

        Ok(())
    }

    /// Check if coordinates are within map bounds
    pub fn is_valid_coords(&self, col: i32, row: i32) -> bool {
        col >= 0
            && row >= 0
            && col < self.dimensions.provinces_per_row as i32
            && row < self.dimensions.provinces_per_col as i32
    }
}
