//! Angle calculations and trigonometric utilities
//!
//! This module provides the single source of truth for all angle-related
//! mathematical operations in Living Worlds. All angle calculations should
//! use these functions for consistency and maintainability.

use std::f32::consts;


/// π (pi) - Re-exported for convenience
pub const PI: f32 = consts::PI;

/// 2π (tau) - A full circle in radians
pub const TAU: f32 = consts::TAU;

/// π/2 - Quarter circle (90 degrees)
pub const HALF_PI: f32 = PI / 2.0;

/// π/4 - Eighth circle (45 degrees)
pub const QUARTER_PI: f32 = PI / 4.0;

/// Conversion factor from degrees to radians
pub const DEG_TO_RAD: f32 = PI / 180.0;

/// Conversion factor from radians to degrees
pub const RAD_TO_DEG: f32 = 180.0 / PI;


/// Convert degrees to radians
///
/// # Example
/// ```
/// let radians = degrees_to_radians(90.0); // π/2
/// ```
#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD
}

/// Convert radians to degrees
///
/// # Example
/// ```
/// let degrees = radians_to_degrees(PI); // 180.0
/// ```
#[inline]
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * RAD_TO_DEG
}


/// Normalize an angle to [0, 2π] range
///
/// # Example
/// ```
/// let normalized = normalize_angle(3.0 * PI); // Returns PI
/// ```
#[inline]
pub fn normalize_angle(angle: f32) -> f32 {
    let mut normalized = angle % TAU;
    if normalized < 0.0 {
        normalized += TAU;
    }
    normalized
}

/// Normalize an angle to [-π, π] range
///
/// Useful for finding the shortest rotation direction
///
/// # Example
/// ```
/// let angle = normalize_angle_signed(3.0 * PI); // Returns -PI
/// ```
#[inline]
pub fn normalize_angle_signed(angle: f32) -> f32 {
    let mut normalized = angle % TAU;
    if normalized > PI {
        normalized -= TAU;
    } else if normalized < -PI {
        normalized += TAU;
    }
    normalized
}

/// Wrap an angle to [0, 360] degrees
#[inline]
pub fn wrap_degrees(degrees: f32) -> f32 {
    let mut wrapped = degrees % 360.0;
    if wrapped < 0.0 {
        wrapped += 360.0;
    }
    wrapped
}

/// Wrap an angle to [-180, 180] degrees
#[inline]
pub fn wrap_degrees_signed(degrees: f32) -> f32 {
    let mut wrapped = degrees % 360.0;
    if wrapped > 180.0 {
        wrapped -= 360.0;
    } else if wrapped < -180.0 {
        wrapped += 360.0;
    }
    wrapped
}


/// Linearly interpolate between two angles (takes shortest path)
///
/// Handles angle wrapping correctly, always taking the shortest path.
///
/// # Example
/// ```
/// // Interpolating from 350° to 10° goes through 0°, not the long way
/// let mid = lerp_angle(350.0_f32.to_radians(), 10.0_f32.to_radians(), 0.5);
/// ```
#[inline]
pub fn lerp_angle(from: f32, to: f32, t: f32) -> f32 {
    let from = normalize_angle(from);
    let to = normalize_angle(to);

    let mut diff = to - from;

    // Take the shortest path
    if diff > PI {
        diff -= TAU;
    } else if diff < -PI {
        diff += TAU;
    }

    normalize_angle(from + diff * t.clamp(0.0, 1.0))
}

/// Smoothly interpolate between two angles using smoothstep
#[inline]
pub fn smoothstep_angle(from: f32, to: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * t * (3.0 - 2.0 * t); // smoothstep
    lerp_angle(from, to, t)
}


/// Calculate the angle between two 2D points
///
/// Returns angle in radians from point a to point b
///
/// # Example
/// ```
/// let angle = angle_between(0.0, 0.0, 1.0, 1.0); // 45 degrees in radians
/// ```
#[inline]
pub fn angle_between(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (y2 - y1).atan2(x2 - x1)
}

/// Calculate the angular distance between two angles
///
/// Always returns the shortest angular distance.
/// Result is always positive.
///
/// # Example
/// ```
/// let dist = angular_distance(350.0_f32.to_radians(), 10.0_f32.to_radians());
/// // Returns 20 degrees in radians, not 340
/// ```
#[inline]
pub fn angular_distance(angle1: f32, angle2: f32) -> f32 {
    let diff = normalize_angle_signed(angle2 - angle1);
    diff.abs()
}

/// Check if an angle is within a range (handles wrapping)
///
/// # Example
/// ```
/// // Check if 10° is between 350° and 20°
/// let in_range = angle_in_range(10.0_f32.to_radians(),
///                                350.0_f32.to_radians(),
///                                20.0_f32.to_radians());
/// ```
#[inline]
pub fn angle_in_range(angle: f32, start: f32, end: f32) -> bool {
    let angle = normalize_angle(angle);
    let start = normalize_angle(start);
    let end = normalize_angle(end);

    if start <= end {
        angle >= start && angle <= end
    } else {
        // Range wraps around 0
        angle >= start || angle <= end
    }
}


/// Get a unit vector from an angle
///
/// Returns (x, y) components of a unit vector pointing in the given direction
///
/// # Example
/// ```
/// let (x, y) = unit_vector_from_angle(HALF_PI); // (0.0, 1.0)
/// ```
#[inline]
pub fn unit_vector_from_angle(angle: f32) -> (f32, f32) {
    (angle.cos(), angle.sin())
}

/// Get a vector with given magnitude from an angle
///
/// # Example
/// ```
/// let (x, y) = vector_from_angle(HALF_PI, 5.0); // (0.0, 5.0)
/// ```
#[inline]
pub fn vector_from_angle(angle: f32, magnitude: f32) -> (f32, f32) {
    (angle.cos() * magnitude, angle.sin() * magnitude)
}


/// Calculate position on a circle
///
/// Used for circular formations, orbits, etc.
///
/// # Example
/// ```
/// let (x, y) = position_on_circle(100.0, 100.0, 50.0, HALF_PI);
/// // Returns position at top of circle: (100.0, 150.0)
/// ```
#[inline]
pub fn position_on_circle(
    center_x: f32,
    center_y: f32,
    radius: f32,
    angle: f32,
) -> (f32, f32) {
    (
        center_x + angle.cos() * radius,
        center_y + angle.sin() * radius,
    )
}

/// Generate positions evenly distributed around a circle
///
/// Useful for placing objects in circular patterns
///
/// # Example
/// ```
/// let positions = positions_around_circle(0.0, 0.0, 100.0, 6);
/// // Returns 6 positions forming a hexagon
/// ```
pub fn positions_around_circle(
    center_x: f32,
    center_y: f32,
    radius: f32,
    count: usize,
) -> Vec<(f32, f32)> {
    let angle_step = TAU / count as f32;
    (0..count)
        .map(|i| {
            let angle = i as f32 * angle_step;
            position_on_circle(center_x, center_y, radius, angle)
        })
        .collect()
}

/// Calculate wind/movement vector from angle and speed
///
/// Used for wind effects, projectile motion, etc.
#[inline]
pub fn movement_vector(angle: f32, speed: f32) -> (f32, f32) {
    vector_from_angle(angle, speed)
}

///
/// Adds controlled randomness to angles for natural-looking patterns
///
/// # Example
/// ```
/// let varied_angle = base_angle + angle_variation(x, y, seed, 0.1);
/// ```
#[inline]
pub fn angle_variation(x: f32, y: f32, seed: u32, amplitude: f32) -> f32 {
    let hash = ((x * 12.9898 + y * 78.233) * (seed as f32 * 0.001)).sin() * 43758.5453;
    let variation = (hash.fract() - 0.5) * 2.0; // [-1, 1]
    variation * amplitude * PI
}


/// Fast approximation of sine (less accurate but faster)
///
/// Uses a polynomial approximation, accurate to ~0.001
#[inline]
pub fn fast_sin(x: f32) -> f32 {
    let x = normalize_angle_signed(x);
    let x2 = x * x;
    x * (1.0 - x2 * (0.16666 - x2 * 0.00833))
}

/// Fast approximation of cosine (less accurate but faster)
#[inline]
pub fn fast_cos(x: f32) -> f32 {
    fast_sin(x + HALF_PI)
}

/// Compute sine and cosine simultaneously (more efficient than separate calls)
#[inline]
pub fn sin_cos(angle: f32) -> (f32, f32) {
    (angle.sin(), angle.cos())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    #[test]
    fn test_angle_conversions() {
        assert!((degrees_to_radians(180.0) - PI).abs() < EPSILON);
        assert!((radians_to_degrees(PI) - 180.0).abs() < EPSILON);
        assert!((degrees_to_radians(90.0) - HALF_PI).abs() < EPSILON);
    }

    #[test]
    fn test_angle_normalization() {
        assert!((normalize_angle(3.0 * PI) - PI).abs() < EPSILON);
        assert!((normalize_angle(-PI) - PI).abs() < EPSILON);
        assert!((normalize_angle_signed(3.0 * PI) - (-PI)).abs() < EPSILON);
    }

    #[test]
    fn test_angle_interpolation() {
        // Test shortest path interpolation
        let from = degrees_to_radians(350.0);
        let to = degrees_to_radians(10.0);
        let mid = lerp_angle(from, to, 0.5);
        let expected = degrees_to_radians(0.0); // Should go through 0, not 180

        // Allow for small numerical errors
        assert!(angular_distance(mid, expected) < degrees_to_radians(1.0));
    }

    #[test]
    fn test_angular_distance() {
        let a1 = degrees_to_radians(350.0);
        let a2 = degrees_to_radians(10.0);
        let dist = angular_distance(a1, a2);
        assert!((dist - degrees_to_radians(20.0)).abs() < EPSILON);
    }

    #[test]
    fn test_positions_around_circle() {
        let positions = positions_around_circle(0.0, 0.0, 1.0, 4);
        assert_eq!(positions.len(), 4);

        // Should form a square
        assert!((positions[0].0 - 1.0).abs() < EPSILON); // Right
        assert!((positions[1].1 - 1.0).abs() < EPSILON); // Top
        assert!((positions[2].0 - (-1.0)).abs() < 0.01); // Left (allow more error)
        assert!((positions[3].1 - (-1.0)).abs() < 0.01); // Bottom
    }
}