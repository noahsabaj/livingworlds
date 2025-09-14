//! Single source of truth for hexagon geometry in Living Worlds
//!
//! This module contains ALL hexagon-related calculations, constants, and geometry.
//! We use FLAT-TOP hexagons with odd-q offset coordinate system throughout.

use super::angles::{degrees_to_radians, position_on_circle};
use bevy::prelude::*;

// HEXAGON CONSTANTS - All hexagon-related constants in one place

/// Default hexagon size (radius from center to corner) in pixels
pub const HEX_SIZE: f32 = 50.0;

/// Number of corners in a hexagon
pub const CORNERS: usize = 6;

/// Total vertices per hexagon (center + 6 corners)
pub const VERTICES_PER_HEX: usize = 7;

/// Triangles needed to render a hexagon (6 triangular slices)
pub const TRIANGLES_PER_HEX: usize = 6;

/// Indices needed for triangle list (3 per triangle)
pub const INDICES_PER_HEX: usize = TRIANGLES_PER_HEX * 3;

/// Degrees between each hexagon corner
pub const DEGREES_PER_CORNER: f32 = 60.0;

/// Square root of 3 - fundamental hexagon constant
pub const SQRT_3: f32 = 1.732050808;

/// Half constant for clarity
pub const HALF: f32 = 0.5;

/// Column offset divisor for odd-q system
pub const COLUMN_OFFSET_DIVISOR: u32 = 2;

/// Antialiasing width for smooth hexagon edges
pub const AA_WIDTH: f32 = 1.5;

// HEXAGON GEOMETRY - Core hexagon structure and calculations

/// Single source of truth for hexagon geometry
///
/// This struct encapsulates all hexagon calculations to ensure consistency
/// across the entire codebase. Always use this instead of duplicating logic.
#[derive(Debug, Clone)]
pub struct Hexagon {
    /// Hexagon radius (center to corner)
    pub size: f32,
    /// Center position in world space
    pub center: Vec2,
}

impl Hexagon {
    pub fn new(center: Vec2) -> Self {
        Self {
            size: HEX_SIZE,
            center,
        }
    }

    pub fn with_size(center: Vec2, size: f32) -> Self {
        Self { size, center }
    }

    /// Generate the 7 vertices for this hexagon (center + 6 corners)
    ///
    /// For a FLAT-TOP hexagon, vertices start at 0° (3 o'clock) and go counter-clockwise.
    /// This ensures proper vertex sharing between adjacent hexagons.
    pub fn vertices(&self) -> [Vec3; VERTICES_PER_HEX] {
        let mut vertices = [Vec3::ZERO; VERTICES_PER_HEX];

        // Center vertex
        vertices[0] = Vec3::new(self.center.x, self.center.y, 0.0);

        // Corner vertices (flat-top starts at 0 degrees for vertex sharing)
        for i in 0..CORNERS {
            let angle = degrees_to_radians(i as f32 * DEGREES_PER_CORNER);
            let (x, y) = position_on_circle(self.center.x, self.center.y, self.size, angle);
            vertices[i + 1] = Vec3::new(x, y, 0.0);
        }

        vertices
    }

    /// Generate just the 6 corner vertices (no center)
    ///
    /// Useful for border rendering where we only need the perimeter.
    pub fn corners(&self) -> [Vec2; CORNERS] {
        let mut corners = [Vec2::ZERO; CORNERS];

        for i in 0..CORNERS {
            let angle = degrees_to_radians(i as f32 * DEGREES_PER_CORNER);
            let (x, y) = position_on_circle(self.center.x, self.center.y, self.size, angle);
            corners[i] = Vec2::new(x, y);
        }

        corners
    }

    /// Generate triangle indices for mesh rendering
    ///
    /// Returns indices for 6 triangles that form the hexagon.
    /// Assumes vertices are [center, corner1, corner2, ..., corner6]
    pub fn indices(base_vertex_index: u32) -> Vec<u32> {
        let mut indices = Vec::with_capacity(INDICES_PER_HEX);

        for i in 0..TRIANGLES_PER_HEX {
            let next = (i + 1) % CORNERS;
            indices.push(base_vertex_index); // Center
            indices.push(base_vertex_index + i as u32 + 1); // Current corner
            indices.push(base_vertex_index + next as u32 + 1); // Next corner
        }

        indices
    }

    /// Check if a point is inside this hexagon
    ///
    /// Uses the axis separation method for flat-top hexagons.
    pub fn contains_point(&self, point: Vec2) -> bool {
        let dx = (point.x - self.center.x).abs();
        let dy = (point.y - self.center.y).abs();

        if dx > self.size || dy > self.size * SQRT_3 / 2.0 {
            return false;
        }

        // Exact hexagon test for flat-top orientation
        dy <= self.size * SQRT_3 / 2.0 &&                  // Within horizontal edges
        (dy / SQRT_3 + dx / 2.0) <= self.size / 2.0 // Within diagonal edges
    }

    /// Calculate distance from hexagon edge (negative if inside)
    ///
    /// Useful for antialiasing and smooth transitions.
    pub fn distance_from_edge(&self, point: Vec2) -> f32 {
        let dx = (point.x - self.center.x).abs();
        let dy = (point.y - self.center.y).abs();

        // Distance from vertical edges
        let dist_vertical = dx - self.size * SQRT_3 / 2.0;

        // Distance from diagonal edges
        let dist_diagonal = (SQRT_3 * dy + dx) / SQRT_3 - self.size;

        dist_vertical.max(dist_diagonal)
    }
}

// GRID POSITIONING - Calculate hexagon positions in grid layouts

/// Calculate hexagon position for a given grid coordinate
///
/// Uses flat-top hexagon with odd-q offset coordinate system:
/// - Odd columns (q) shift UP by half the vertical spacing
/// - This creates a honeycomb pattern for perfect tessellation
pub fn calculate_grid_position(
    col: u32,
    row: u32,
    hex_size: f32,
    grid_width: u32,
    grid_height: u32,
) -> Vec2 {
    // Odd columns shift up
    let y_offset = if col % COLUMN_OFFSET_DIVISOR == 1 {
        hex_size * SQRT_3 * HALF
    } else {
        0.0
    };

    // Center the grid at origin
    let x = (col as f32 - grid_width as f32 * HALF) * hex_size * 1.5;
    let y = (row as f32 - grid_height as f32 * HALF) * hex_size * SQRT_3 + y_offset;

    Vec2::new(x, y)
}

/// Find grid coordinates from world position (inverse of calculate_grid_position)
///
/// Returns (col, row) for the hexagon containing the given point.
pub fn world_to_grid(
    position: Vec2,
    hex_size: f32,
    grid_width: u32,
    grid_height: u32,
) -> Option<(u32, u32)> {
    // Approximate column
    let col_f = (position.x / (hex_size * 1.5)) + (grid_width as f32 * HALF);
    let col = col_f.round() as i32;

    if col < 0 || col >= grid_width as i32 {
        return None;
    }

    // Account for odd column offset
    let y_offset = if col as u32 % COLUMN_OFFSET_DIVISOR == 1 {
        hex_size * SQRT_3 * HALF
    } else {
        0.0
    };

    let row_f = ((position.y - y_offset) / (hex_size * SQRT_3)) + (grid_height as f32 * HALF);
    let row = row_f.round() as i32;

    if row < 0 || row >= grid_height as i32 {
        return None;
    }

    Some((col as u32, row as u32))
}

///
/// Returns positions of all 6 neighbors in the odd-q offset system.
pub fn get_neighbor_positions(col: i32, row: i32, hex_size: f32) -> [(i32, i32); 6] {
    // Neighbor offsets depend on whether column is odd or even
    if col % 2 == 0 {
        // Even column
        [
            (col + 1, row - 1), // Top-right
            (col + 1, row),     // Bottom-right
            (col, row + 1),     // Bottom
            (col - 1, row),     // Bottom-left
            (col - 1, row - 1), // Top-left
            (col, row - 1),     // Top
        ]
    } else {
        // Odd column (shifted up)
        [
            (col + 1, row),     // Top-right
            (col + 1, row + 1), // Bottom-right
            (col, row + 1),     // Bottom
            (col - 1, row + 1), // Bottom-left
            (col - 1, row),     // Top-left
            (col, row - 1),     // Top
        ]
    }
}

/// Validate that a position is finite (not NaN or infinite)
pub fn validate_position(pos: Vec2) -> Result<(), String> {
    if !pos.x.is_finite() || !pos.y.is_finite() {
        Err(format!("Invalid position: ({}, {})", pos.x, pos.y))
    } else {
        Ok(())
    }
}

/// Quantize a position to reduce floating point precision issues
pub fn quantize_position(pos: Vec3, precision: f32) -> (i32, i32) {
    (
        (pos.x * precision).round() as i32,
        (pos.y * precision).round() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexagon_vertices() {
        let hex = Hexagon::new(Vec2::ZERO);
        let vertices = hex.vertices();

        // Center should be at origin
        assert_eq!(vertices[0], Vec3::ZERO);

        // First corner should be at 0 degrees (right side)
        assert!((vertices[1].x - HEX_SIZE).abs() < 0.001);
        assert!(vertices[1].y.abs() < 0.001);
    }

    #[test]
    fn test_point_containment() {
        let hex = Hexagon::new(Vec2::ZERO);

        // Center should be inside
        assert!(hex.contains_point(Vec2::ZERO));

        // Points outside should not be inside
        assert!(!hex.contains_point(Vec2::new(HEX_SIZE * 2.0, 0.0)));
    }

    #[test]
    fn test_grid_position_calculation() {
        let pos = calculate_grid_position(0, 0, HEX_SIZE, 10, 10);

        // Should be offset from origin by half the grid size
        assert!(pos.x < 0.0);
        assert!(pos.y < 0.0);
    }
}
