//! Game-specific distance calculations for Living Worlds
//!
//! This module provides specialized distance functions for game mechanics.
//! For basic distance calculations, use Bevy's Vec2/Vec3 methods directly:
//! - `pos1.distance(pos2)` for Euclidean distance
//! - `pos1.distance_squared(pos2)` for faster comparisons
//! - `(pos1 - pos2).abs().x + (pos1 - pos2).abs().y` for Manhattan distance
//!
//! # Examples
//!
//! ```rust
//! use crate::math::{hex_distance, gaussian_falloff};
//!
//! // Hexagon grid distance (game-specific)
//! let hex_dist = hex_distance(col1, row1, col2, row2);
//!
//! // Influence falloff for minerals
//! let influence = gaussian_falloff(distance, sigma);
//!
//! // Basic distance - use Bevy directly
//! let dist = pos1.distance(pos2);
//! ```

use super::hexagon::SQRT_3;
use bevy::prelude::*;

// HEXAGON DISTANCE - Game-specific grid calculations

/// Calculate distance between two hexagon grid positions
///
/// Uses the odd-q offset coordinate system where odd columns are shifted up.
/// This is the complex, game-specific distance calculation that justifies
/// being in this module.
///
/// # Algorithm
/// Converts offset coordinates to cube coordinates, calculates distance,
/// then converts back. This ensures accurate hexagonal grid distance.
#[inline]
pub fn hex_distance(col1: u32, row1: u32, col2: u32, row2: u32) -> u32 {
    // Convert odd-q offset to cube coordinates
    let (q1, r1) = (col1 as i32, row1 as i32);
    let (q2, r2) = (col2 as i32, row2 as i32);

    // Convert to cube coordinates
    let x1 = q1;
    let z1 = r1 - (q1 - (q1 & 1)) / 2;
    let y1 = -x1 - z1;

    let x2 = q2;
    let z2 = r2 - (q2 - (q2 & 1)) / 2;
    let y2 = -x2 - z2;

    // Hexagon distance in cube coordinates
    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as u32 / 2
}

/// Calculate distance between two hexagon positions in world space
///
/// This uses the actual world positions and hex size to calculate
/// an approximate hexagonal grid distance.
#[inline]
pub fn hex_distance_world(pos1: Vec2, pos2: Vec2, hex_size: f32) -> f32 {
    // Approximate hex distance using Euclidean distance
    // divided by the average hex-to-hex spacing
    let spacing = hex_size * SQRT_3; // hex_size * sqrt(3)
    pos1.distance(pos2) / spacing
}

// FALLOFF FUNCTIONS - Game-specific influence calculations

/// Linear distance falloff
///
/// Linearly decreases from 1.0 to 0.0 over the given distance.
/// Used for simple influence areas.
#[inline]
pub fn linear_falloff(distance: f32, max_distance: f32) -> f32 {
    if distance >= max_distance {
        0.0
    } else {
        1.0 - (distance / max_distance)
    }
}

/// Quadratic distance falloff
///
/// Smoother than linear falloff. Falls off as (1 - (d/max)²).
/// Good for natural-looking influence areas.
#[inline]
pub fn quadratic_falloff(distance: f32, max_distance: f32) -> f32 {
    if distance >= max_distance {
        0.0
    } else {
        let t = distance / max_distance;
        1.0 - t * t
    }
}

/// Inverse square distance falloff
///
/// Physically accurate for light, gravity, etc. Never reaches zero.
/// The scale parameter controls the falloff rate.
#[inline]
pub fn inverse_square_falloff(distance: f32, scale: f32) -> f32 {
    let d = distance.max(0.1); // Avoid division by zero
    scale / (d * d)
}

/// Gaussian (bell curve) distance falloff
///
/// Smooth falloff with configurable standard deviation.
/// Most influence is within 3*sigma of the center.
/// Used extensively for mineral vein influence.
#[inline]
pub fn gaussian_falloff(distance: f32, sigma: f32) -> f32 {
    let d_normalized = distance / sigma;
    (-0.5 * d_normalized * d_normalized).exp()
}

/// Smooth step distance falloff
///
/// Smooth transition from 1.0 to 0.0 between inner and outer radius.
/// Flat at 1.0 within inner radius, flat at 0.0 beyond outer radius.
#[inline]
pub fn smooth_falloff(distance: f32, inner_radius: f32, outer_radius: f32) -> f32 {
    if distance <= inner_radius {
        1.0
    } else if distance >= outer_radius {
        0.0
    } else {
        let t = (distance - inner_radius) / (outer_radius - inner_radius);
        let smooth_t = t * t * (3.0 - 2.0 * t); // smoothstep
        1.0 - smooth_t
    }
}

// BOUNDARY CALCULATIONS - Game-specific edge detection

/// Calculate distance from a point to the edge of a rectangle
///
/// Returns negative values if the point is inside the rectangle.
/// Useful for falloff calculations and boundary detection.
#[inline]
pub fn distance_from_rect_edge(
    point: Vec2,
    rect_center: Vec2,
    rect_half_width: f32,
    rect_half_height: f32,
) -> f32 {
    let dx = (point.x - rect_center.x).abs() - rect_half_width;
    let dy = (point.y - rect_center.y).abs() - rect_half_height;

    if dx <= 0.0 && dy <= 0.0 {
        // Inside rectangle - return negative distance to edge
        dx.max(dy)
    } else {
        // Outside rectangle - return positive distance to edge
        Vec2::new(dx.max(0.0), dy.max(0.0)).length()
    }
}

/// Calculate normalized distance from map edge
///
/// Returns 0.0 at edges, 1.0 at center. Used for island generation
/// and continent placement with smooth falloff from map boundaries.
#[inline]
pub fn normalized_edge_distance(position: Vec2, map_width: f32, map_height: f32) -> f32 {
    let center = Vec2::new(map_width / 2.0, map_height / 2.0);
    let edge_dist = distance_from_rect_edge(position, center, map_width / 2.0, map_height / 2.0);

    // Convert to normalized 0-1 range
    let max_distance = (map_width.min(map_height)) / 2.0;
    1.0 - ((-edge_dist / max_distance).clamp(0.0, 1.0))
}

// GAME UTILITIES - Combined distance + influence calculations

/// Falloff type for influence calculations
#[derive(Debug, Clone, Copy)]
pub enum FalloffType {
    Linear(f32),
    Quadratic(f32),
    Gaussian(f32),
    InverseSquare(f32),
    Smooth { inner: f32, outer: f32 },
}

/// Calculate influence based on distance with custom falloff
///
/// Combines distance calculation with falloff for game mechanics.
/// Used for mineral influence, territorial control, etc.
#[inline]
pub fn calculate_influence(
    source: Vec2,
    target: Vec2,
    influence_radius: f32,
    falloff_type: FalloffType,
) -> f32 {
    let distance = source.distance(target);

    match falloff_type {
        FalloffType::Linear(_) => linear_falloff(distance, influence_radius),
        FalloffType::Quadratic(_) => quadratic_falloff(distance, influence_radius),
        FalloffType::Gaussian(sigma) => gaussian_falloff(distance, sigma),
        FalloffType::InverseSquare(scale) => inverse_square_falloff(distance, scale),
        FalloffType::Smooth { inner, outer } => smooth_falloff(distance, inner, outer),
    }
}

/// Find the closest point from a list
///
/// Returns the index and distance of the closest point.
/// Useful for nearest neighbor searches in small collections.
pub fn find_closest(target: Vec2, points: &[Vec2]) -> Option<(usize, f32)> {
    points
        .iter()
        .enumerate()
        .map(|(idx, &pos)| (idx, target.distance(pos)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}

/// Find all points within a radius
///
/// Returns indices of all points within the specified radius.
/// More efficient than filtering distances due to early termination.
pub fn find_within_radius(center: Vec2, points: &[Vec2], radius: f32) -> Vec<usize> {
    let radius_sq = radius * radius;
    points
        .iter()
        .enumerate()
        .filter_map(|(idx, &pos)| {
            if center.distance_squared(pos) <= radius_sq {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}
