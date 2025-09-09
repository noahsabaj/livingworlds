//! Utility functions for world generation

/// Get the 6 neighbors of a hexagon in odd-q offset coordinates
pub fn hex_neighbors(col: i32, row: i32) -> Vec<(i32, i32)> {
    if col % 2 == 0 {
        // Even column neighbors
        vec![
            (col, row - 1),     // North
            (col + 1, row - 1), // Northeast
            (col + 1, row),     // Southeast
            (col, row + 1),     // South
            (col - 1, row),     // Southwest
            (col - 1, row - 1), // Northwest
        ]
    } else {
        // Odd column neighbors
        vec![
            (col, row - 1),     // North
            (col + 1, row),     // Northeast
            (col + 1, row + 1), // Southeast
            (col, row + 1),     // South
            (col - 1, row + 1), // Southwest
            (col - 1, row),     // Northwest
        ]
    }
}