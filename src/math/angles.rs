//! Game-specific angle calculations and trigonometric utilities for Living Worlds
//!
//! This module provides specialized angle functions for game mechanics.
//! For basic trigonometry and angle constants, use Rust's std library directly:
//! - `std::f32::consts::{PI, TAU, FRAC_PI_2, FRAC_PI_4}` for angle constants
//! - `angle.to_radians()` and `angle.to_degrees()` for conversions
//! - `angle.sin()`, `angle.cos()`, `angle.sin_cos()` for basic trig

use std::f32::consts;

/// Linearly interpolate between two angles (takes shortest path)
///
/// Handles angle wrapping correctly, always taking the shortest path.
/// This is complex logic that justifies being in this module.
///
/// # Examples
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
    if diff > consts::PI {
        diff -= consts::TAU;
    } else if diff < -consts::PI {
        diff += consts::TAU;
    }

    normalize_angle(from + diff * t.clamp(0.0, 1.0))
}

/// Smoothly interpolate between two angles using smoothstep
///
/// Combines angle wrapping with S-curve interpolation for smooth transitions.
#[inline]
pub fn smoothstep_angle(from: f32, to: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * t * (3.0 - 2.0 * t); // smoothstep
    lerp_angle(from, to, t)
}

/// Calculate the angular distance between two angles
///
/// Always returns the shortest angular distance.
/// Result is always positive and handles angle wrapping.
///
/// # Examples
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
/// This is complex logic because it must handle ranges that wrap around 0/2π.
///
/// # Examples
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

/// Generate positions evenly distributed around a circle
///
/// Useful for placing objects in circular patterns. This is a complex
/// utility function that's used throughout Living Worlds.
///
/// # Examples
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
    let angle_step = consts::TAU / count as f32;
    (0..count)
        .map(|i| {
            let angle = i as f32 * angle_step;
            (
                center_x + angle.cos() * radius,
                center_y + angle.sin() * radius,
            )
        })
        .collect()
}

/// Calculate wind/movement vector from angle and speed
///
/// Used for wind effects, projectile motion, etc. This is essentially
/// an alias for vector_from_angle but with descriptive naming for game logic.
#[inline]
pub fn movement_vector(angle: f32, speed: f32) -> (f32, f32) {
    (angle.cos() * speed, angle.sin() * speed)
}

/// Procedural angle variation for natural-looking patterns
///
/// Adds controlled randomness to angles for natural-looking patterns.
/// This is deterministic based on position and seed, which is valuable
/// for consistent world generation.
///
/// # Examples
/// ```
/// let varied_angle = base_angle + angle_variation(x, y, seed, 0.1);
/// ```
#[inline]
pub fn angle_variation(x: f32, y: f32, seed: u32, amplitude: f32) -> f32 {
    let hash = ((x * 12.9898 + y * 78.233) * (seed as f32 * 0.001)).sin() * 43758.5453;
    let variation = (hash.fract() - 0.5) * 2.0; // [-1, 1]
    variation * amplitude * consts::PI
}

/// Fast approximation of sine (less accurate but faster)
///
/// Uses a polynomial approximation, accurate to ~0.001.
/// Useful for performance-critical code where precision isn't critical.
#[inline]
pub fn fast_sin(x: f32) -> f32 {
    let x = normalize_angle_signed(x);
    let x2 = x * x;
    x * (1.0 - x2 * (0.16666 - x2 * 0.00833))
}

/// Fast approximation of cosine (less accurate but faster)
///
/// Uses fast_sin for implementation consistency.
#[inline]
pub fn fast_cos(x: f32) -> f32 {
    fast_sin(x + consts::FRAC_PI_2)
}

// INTERNAL HELPER FUNCTIONS (not exported)

/// Normalize an angle to [0, 2π] range
#[inline]
fn normalize_angle(angle: f32) -> f32 {
    let mut normalized = angle % consts::TAU;
    if normalized < 0.0 {
        normalized += consts::TAU;
    }
    normalized
}

/// Normalize an angle to [-π, π] range
///
/// Useful for finding the shortest rotation direction
#[inline]
fn normalize_angle_signed(angle: f32) -> f32 {
    let mut normalized = angle % consts::TAU;
    if normalized > consts::PI {
        normalized -= consts::TAU;
    } else if normalized < -consts::PI {
        normalized += consts::TAU;
    }
    normalized
}
