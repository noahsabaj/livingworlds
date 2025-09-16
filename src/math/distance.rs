//! Distance calculation module for Living Worlds
//!
//! This module provides ALL distance calculations including Euclidean, Manhattan,
//! Chebyshev, hexagonal grid distances, and specialized game-specific distance
//! functions. This is the SINGLE SOURCE OF TRUTH for all distance math.
//!
//! # Examples
//!
//! ```rust
//! use crate::math::distance::*;
//!
//! // Euclidean distance (standard distance)
//! let dist = euclidean_2d(x1, y1, x2, y2);
//! let dist = euclidean_vec2(pos1, pos2);
//!
//! // Squared distance (avoids sqrt, faster for comparisons)
//! let dist_sq = euclidean_squared_2d(x1, y1, x2, y2);
//!
//! // Manhattan distance (grid-based movement)
//! let dist = manhattan_2d(x1, y1, x2, y2);
//!
//! // Hexagon grid distance
//! let hex_dist = hex_distance(col1, row1, col2, row2);
//! ```

use bevy::prelude::*;

use super::hexagon::SQRT_3;

// EUCLIDEAN DISTANCE - Standard geometric distance

/// Euclidean distance between two 2D points
///
/// This is the standard "straight-line" distance using the Pythagorean theorem.
/// Use this when you need actual geometric distance.
///
/// # Performance
/// Contains a sqrt operation. Use `euclidean_squared_2d` if you only need to compare distances.
#[inline]
pub fn euclidean_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Euclidean distance between two Vec2 points
///
/// Convenience wrapper for Vec2 types.
#[inline]
pub fn euclidean_vec2(a: Vec2, b: Vec2) -> f32 {
    a.distance(b) // Use Bevy's optimized implementation
}

/// Squared Euclidean distance between two 2D points
///
/// Avoids the expensive sqrt operation. Perfect for distance comparisons
/// since if a² < b², then a < b.
///
/// # Example
/// ```rust
/// // Instead of:
/// if euclidean_2d(x1, y1, x2, y2) < radius { }
///
/// // Use:
/// if euclidean_squared_2d(x1, y1, x2, y2) < radius * radius { }
/// ```
#[inline]
pub fn euclidean_squared_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    dx * dx + dy * dy
}

/// Squared Euclidean distance between two Vec2 points
#[inline]
pub fn euclidean_squared_vec2(a: Vec2, b: Vec2) -> f32 {
    a.distance_squared(b) // Use Bevy's optimized implementation
}

/// Euclidean distance between two 3D points
#[allow(dead_code)]
#[inline]
pub fn euclidean_3d(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let dz = z2 - z1;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Euclidean distance between two Vec3 points
#[inline]
pub fn euclidean_vec3(a: Vec3, b: Vec3) -> f32 {
    a.distance(b) // Use Bevy's optimized implementation
}

// MANHATTAN DISTANCE - Grid-based "taxicab" distance

/// Manhattan distance between two 2D points
///
/// Also known as "taxicab distance" or "L1 distance". This is the distance
/// when you can only move along grid lines (like city blocks).
///
/// # Example
/// ```rust
/// // Distance between (0,0) and (3,4) is 7 (3 + 4)
/// let dist = manhattan_2d(0.0, 0.0, 3.0, 4.0);
/// assert_eq!(dist, 7.0);
/// ```
#[inline]
pub fn manhattan_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

/// Manhattan distance between two Vec2 points
#[inline]
pub fn manhattan_vec2(a: Vec2, b: Vec2) -> f32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

/// Manhattan distance between two 3D points
#[inline]
pub fn manhattan_3d(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs()
}

// CHEBYSHEV DISTANCE - Maximum coordinate difference

/// Chebyshev distance between two 2D points
///
/// Also known as "chessboard distance" or "L∞ distance". This is the
/// maximum absolute difference between coordinates. A king in chess
/// moves with Chebyshev distance of 1.
///
/// # Example
/// ```rust
/// // Distance between (0,0) and (3,4) is 4 (max of 3 and 4)
/// let dist = chebyshev_2d(0.0, 0.0, 3.0, 4.0);
/// assert_eq!(dist, 4.0);
/// ```
#[inline]
pub fn chebyshev_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    dx.max(dy)
}

/// Chebyshev distance between two Vec2 points
#[inline]
pub fn chebyshev_vec2(a: Vec2, b: Vec2) -> f32 {
    let dx = (b.x - a.x).abs();
    let dy = (b.y - a.y).abs();
    dx.max(dy)
}

// HEXAGON GRID DISTANCE - Distance in hexagonal grids

/// Calculate distance between two hexagons in a flat-top odd-q offset grid
///
/// This calculates the minimum number of hexagon steps between two positions.
/// Uses the odd-q offset coordinate system where odd columns are shifted up.
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
    euclidean_vec2(pos1, pos2) / spacing
}

// DISTANCE FROM EDGE - For shapes and boundaries

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

    if dx > 0.0 && dy > 0.0 {
        // Outside corner
        (dx * dx + dy * dy).sqrt()
    } else {
        // Inside or on edge
        dx.max(dy)
    }
}

/// Calculate normalized distance from map center to edge
///
/// Returns 0.0 at center, 1.0 at edge. Useful for island falloff.
/// Uses Euclidean distance for natural circular falloff patterns.
#[inline]
pub fn normalized_edge_distance(
    position: Vec2,
    map_center: Vec2,
    map_half_width: f32,
    map_half_height: f32,
) -> f32 {
    // Use Euclidean distance for natural circular/elliptical falloff
    let dx = (position.x - map_center.x) / map_half_width;
    let dy = (position.y - map_center.y) / map_half_height;
    (dx * dx + dy * dy).sqrt().clamp(0.0, 1.0)
}

// WRAPPING DISTANCE - For toroidal/cylindrical maps

/// Calculate distance on a map that wraps horizontally
///
/// Useful for cylindrical world maps where the left and right edges connect.
#[inline]
pub fn wrapping_distance_2d(x1: f32, y1: f32, x2: f32, y2: f32, map_width: f32) -> f32 {
    let dx_direct = (x2 - x1).abs();
    let dx_wrapped = map_width - dx_direct;
    let dx = dx_direct.min(dx_wrapped);
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate distance on a toroidal map (wraps both horizontally and vertically)
#[inline]
pub fn toroidal_distance_2d(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    map_width: f32,
    map_height: f32,
) -> f32 {
    let dx_direct = (x2 - x1).abs();
    let dx_wrapped = map_width - dx_direct;
    let dx = dx_direct.min(dx_wrapped);

    let dy_direct = (y2 - y1).abs();
    let dy_wrapped = map_height - dy_direct;
    let dy = dy_direct.min(dy_wrapped);

    (dx * dx + dy * dy).sqrt()
}

// DISTANCE FALLOFF - For influence and field calculations

/// Linear distance falloff
///
/// Returns 1.0 at distance 0, linearly decreasing to 0.0 at max_distance.
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
/// Falls off with the square of distance. Stronger initial falloff than linear.
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

/// Calculate influence based on distance with custom falloff
///
/// Combines distance calculation with falloff for game mechanics.
#[inline]
pub fn calculate_influence(
    source: Vec2,
    target: Vec2,
    influence_radius: f32,
    falloff_type: FalloffType,
) -> f32 {
    let distance = euclidean_vec2(source, target);

    match falloff_type {
        FalloffType::Linear => linear_falloff(distance, influence_radius),
        FalloffType::Quadratic => quadratic_falloff(distance, influence_radius),
        FalloffType::Gaussian(sigma) => gaussian_falloff(distance, sigma),
        FalloffType::InverseSquare(scale) => inverse_square_falloff(distance, scale),
        FalloffType::Smooth(inner) => smooth_falloff(distance, inner, influence_radius),
    }
}

/// Falloff type for influence calculations
#[derive(Debug, Clone, Copy)]
pub enum FalloffType {
    Linear,
    Quadratic,
    Gaussian(f32),      // sigma parameter
    InverseSquare(f32), // scale parameter
    Smooth(f32),        // inner radius
}

/// Find the closest point from a list, returning index and distance
///
/// Returns None if the list is empty.
#[inline]
pub fn find_closest(target: Vec2, points: &[Vec2]) -> Option<(usize, f32)> {
    points
        .iter()
        .enumerate()
        .map(|(idx, &pos)| (idx, euclidean_vec2(target, pos)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}

/// Find all points within a radius, returning indices and distances
#[inline]
pub fn find_within_radius(center: Vec2, points: &[Vec2], radius: f32) -> Vec<(usize, f32)> {
    let radius_sq = radius * radius;
    points
        .iter()
        .enumerate()
        .filter_map(|(idx, &pos)| {
            let dist_sq = euclidean_squared_vec2(center, pos);
            if dist_sq <= radius_sq {
                Some((idx, dist_sq.sqrt()))
            } else {
                None
            }
        })
        .collect()
}

/// Batch calculate distances from one point to many
///
/// More efficient than calling euclidean_vec2 in a loop due to
/// better cache locality and potential SIMD optimization.
#[inline]
pub fn batch_distances(from: Vec2, to_points: &[Vec2]) -> Vec<f32> {
    to_points.iter().map(|&p| euclidean_vec2(from, p)).collect()
}

/// Batch calculate squared distances (avoids sqrt)
#[inline]
pub fn batch_distances_squared(from: Vec2, to_points: &[Vec2]) -> Vec<f32> {
    to_points
        .iter()
        .map(|&p| euclidean_squared_vec2(from, p))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        // 3-4-5 triangle
        assert_eq!(euclidean_2d(0.0, 0.0, 3.0, 4.0), 5.0);
        assert_eq!(euclidean_squared_2d(0.0, 0.0, 3.0, 4.0), 25.0);

        // Vec2 version
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(euclidean_vec2(a, b), 5.0);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_2d(0.0, 0.0, 3.0, 4.0), 7.0);
        assert_eq!(manhattan_2d(-2.0, -3.0, 1.0, 2.0), 8.0);
    }

    #[test]
    fn test_chebyshev_distance() {
        assert_eq!(chebyshev_2d(0.0, 0.0, 3.0, 4.0), 4.0);
        assert_eq!(chebyshev_2d(-2.0, -3.0, 1.0, 2.0), 5.0);
    }

    #[test]
    fn test_hex_distance() {
        // Adjacent hexagons
        assert_eq!(hex_distance(0, 0, 1, 0), 1);
        assert_eq!(hex_distance(0, 0, 0, 1), 1);

        // Diagonal in hex grid
        assert_eq!(hex_distance(0, 0, 1, 1), 1); // Odd columns shift up

        // Further distances
        assert_eq!(hex_distance(0, 0, 2, 0), 2);
        assert_eq!(hex_distance(0, 0, 2, 2), 2);
    }

    #[test]
    fn test_falloff_functions() {
        // Linear falloff
        assert_eq!(linear_falloff(0.0, 10.0), 1.0);
        assert_eq!(linear_falloff(5.0, 10.0), 0.5);
        assert_eq!(linear_falloff(10.0, 10.0), 0.0);
        assert_eq!(linear_falloff(15.0, 10.0), 0.0);

        // Quadratic falloff
        assert_eq!(quadratic_falloff(0.0, 10.0), 1.0);
        assert_eq!(quadratic_falloff(5.0, 10.0), 0.75);
        assert_eq!(quadratic_falloff(10.0, 10.0), 0.0);

        // Gaussian falloff
        let g = gaussian_falloff(0.0, 1.0);
        assert!(g > 0.99 && g <= 1.0);
        let g = gaussian_falloff(1.0, 1.0);
        assert!(g > 0.6 && g < 0.7); // ~0.606

        // Smooth falloff
        assert_eq!(smooth_falloff(0.0, 5.0, 10.0), 1.0);
        assert_eq!(smooth_falloff(5.0, 5.0, 10.0), 1.0);
        assert_eq!(smooth_falloff(10.0, 5.0, 10.0), 0.0);
        assert_eq!(smooth_falloff(7.5, 5.0, 10.0), 0.5);
    }

    #[test]
    fn test_find_closest() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 4.0),
            Vec2::new(-2.0, 1.0),
        ];

        let target = Vec2::new(2.0, 3.0);
        let (idx, dist) = find_closest(target, &points).unwrap();
        assert_eq!(idx, 1); // Closest to (3, 4)
        assert!(dist < 2.0);
    }

    #[test]
    fn test_find_within_radius() {
        let points = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 4.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(10.0, 10.0),
        ];

        let results = find_within_radius(Vec2::ZERO, &points, 5.5);
        assert_eq!(results.len(), 3); // Should find first 3 points
        assert!(!results.iter().any(|&(idx, _)| idx == 3)); // Not the far point
    }

    #[test]
    fn test_wrapping_distance() {
        // On a map with width 10
        let d1 = wrapping_distance_2d(1.0, 0.0, 9.0, 0.0, 10.0);
        assert_eq!(d1, 2.0); // Wraps around (shorter than direct distance of 8)

        let d2 = wrapping_distance_2d(1.0, 0.0, 4.0, 0.0, 10.0);
        assert_eq!(d2, 3.0); // Direct distance is shorter
    }

    #[test]
    fn test_edge_distance() {
        let center = Vec2::new(50.0, 50.0);

        // At center
        assert_eq!(normalized_edge_distance(center, center, 50.0, 50.0), 0.0);

        // At edge
        assert_eq!(
            normalized_edge_distance(Vec2::new(100.0, 50.0), center, 50.0, 50.0),
            1.0
        );

        // Halfway
        assert_eq!(
            normalized_edge_distance(Vec2::new(75.0, 50.0), center, 50.0, 50.0),
            0.5
        );
    }
}
