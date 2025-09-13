//! Geometry module for Living Worlds
//! 
//! This module provides all geometric calculations and constants,
//! serving as the single source of truth for spatial mathematics.

pub mod hexagon;

// Re-export the most commonly used items for convenience
pub use hexagon::{
    Hexagon,
    calculate_grid_position,
    world_to_grid,
    get_neighbor_positions,
    validate_position,
    quantize_position,
    // Constants
    HEX_SIZE,
    CORNERS,
    VERTICES_PER_HEX,
    TRIANGLES_PER_HEX,
    INDICES_PER_HEX,
    SQRT_3,
    HALF,
};

// For backward compatibility during transition
#[deprecated(note = "Use geometry::hexagon::calculate_grid_position instead")]
pub fn calculate_hex_position(
    col: u32,
    row: u32, 
    hex_size: f32,
    provinces_per_row: u32,
    provinces_per_col: u32
) -> (f32, f32) {
    let pos = hexagon::calculate_grid_position(col, row, hex_size, provinces_per_row, provinces_per_col);
    (pos.x, pos.y)
}