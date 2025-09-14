//! Interpolation and smoothing functions for Living Worlds
//!
//! This module is the SINGLE SOURCE OF TRUTH for all interpolation, smoothing,
//! and blending operations. No other module should implement their own versions
//! of lerp, smoothstep, or similar functions.
//!
//! # Usage
//!
//! ```rust
//! use crate::math::interpolation::*;
//!
//! // Basic linear interpolation
//! let value = lerp(0.0, 10.0, 0.5); // Returns 5.0
//!
//! // Smooth camera movement
//! let smooth_pos = lerp_exp_vec3(current_pos, target_pos, smoothing, delta_time);
//!
//! // Smooth transitions with smoothstep
//! let smooth_value = smoothstep(0.0, 1.0, t);
//!
//! // Weighted blending of multiple values
//! let result = weighted_blend(&[(value1, 0.3), (value2, 0.7)]);
//! ```

use bevy::prelude::*;
use super::angles::{PI, fast_sin};


/// Linear interpolation between two values
///
/// Returns a value between `a` and `b` based on parameter `t`.
/// When t=0, returns a. When t=1, returns b.
///
/// # Examples
/// ```
/// let mid = lerp(0.0, 10.0, 0.5); // Returns 5.0
/// let quarter = lerp(0.0, 10.0, 0.25); // Returns 2.5
/// ```
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Linear interpolation for Vec2
#[inline]
pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a.lerp(b, t.clamp(0.0, 1.0))
}

/// Linear interpolation for Vec3
#[inline]
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a.lerp(b, t.clamp(0.0, 1.0))
}

/// Linear interpolation without clamping (extrapolation allowed)
#[inline]
pub fn lerp_unclamped(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}


/// Inverse linear interpolation (unlerp)
///
/// Returns where `value` falls between `a` and `b` as a 0-1 parameter.
/// This is the inverse of lerp: if lerp(a, b, t) = value, then inverse_lerp(a, b, value) = t
///
/// # Examples
/// ```
/// let t = inverse_lerp(0.0, 10.0, 5.0); // Returns 0.5
/// let t = inverse_lerp(10.0, 20.0, 15.0); // Returns 0.5
/// ```
#[inline]
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        return 0.0;
    }
    ((value - a) / (b - a)).clamp(0.0, 1.0)
}

/// Remap a value from one range to another
///
/// Maps `value` from range [from_min, from_max] to range [to_min, to_max]
///
/// # Examples
/// ```
/// let remapped = remap(5.0, 0.0, 10.0, 0.0, 100.0); // Returns 50.0
/// let remapped = remap(0.5, 0.0, 1.0, -1.0, 1.0); // Returns 0.0
/// ```
#[inline]
pub fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}


/// Smoothstep interpolation
///
/// Provides smooth interpolation with zero derivatives at edges.
/// Uses the formula: 3t² - 2t³
///
/// Perfect for smooth transitions and fade effects.
///
/// # Examples
/// ```
/// let smooth = smoothstep(0.0, 1.0, 0.5); // Smooth S-curve
/// ```
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = inverse_lerp(edge0, edge1, x);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep interpolation (Ken Perlin's improved version)
///
/// Even smoother than smoothstep with zero first AND second derivatives.
/// Uses the formula: 6t⁵ - 15t⁴ + 10t³
///
/// # Examples
/// ```
/// let smoother = smootherstep(0.0, 1.0, 0.5); // Even smoother S-curve
/// ```
#[inline]
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = inverse_lerp(edge0, edge1, x);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}


/// Exponential interpolation (frame-rate independent smoothing)
///
/// This is what camera.rs uses: `1.0 - (0.01_f32.powf(smoothing * delta_time))`
/// Higher smoothing values = smoother movement
///
/// # Examples
/// ```
/// let smooth_pos = lerp_exp(current, target, 8.0, delta_time);
/// ```
#[inline]
pub fn lerp_exp(current: f32, target: f32, smoothing: f32, delta_time: f32) -> f32 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    lerp(current, target, t)
}

/// Exponential interpolation for Vec2
#[inline]
pub fn lerp_exp_vec2(current: Vec2, target: Vec2, smoothing: f32, delta_time: f32) -> Vec2 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    lerp_vec2(current, target, t)
}

/// Exponential interpolation for Vec3
#[inline]
pub fn lerp_exp_vec3(current: Vec3, target: Vec3, smoothing: f32, delta_time: f32) -> Vec3 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    lerp_vec3(current, target, t)
}

/// Simple exponential smoothing (like climate.rs rainfall blending)
///
/// Blends current value with new value using exponential smoothing.
/// Alpha is the smoothing factor (0-1). Higher = more responsive to changes.
///
/// # Examples
/// ```
/// // Like climate.rs: rainfall * 0.7 + new * 0.3
/// let smoothed = exponential_smooth(current_rainfall, new_rainfall, 0.3);
/// ```
#[inline]
pub fn exponential_smooth(current: f32, new_value: f32, alpha: f32) -> f32 {
    current * (1.0 - alpha) + new_value * alpha
}


/// Asymmetric smoothing with different rates for increase/decrease
///
/// Perfect for systems like WorldTension where values rise quickly but fall slowly.
///
/// # Examples
/// ```
/// // Tension rises quickly (rate=2.0) but falls slowly (rate=0.3)
/// let new_tension = asymmetric_smooth(current, target, 2.0, 0.3, delta_time);
/// ```
#[inline]
pub fn asymmetric_smooth(
    current: f32,
    target: f32,
    increase_rate: f32,
    decrease_rate: f32,
    delta_time: f32
) -> f32 {
    let rate = if target > current { increase_rate } else { decrease_rate };
    let t = 1.0 - (-rate * delta_time).exp();
    lerp(current, target, t)
}


/// Weighted blend of multiple values
///
/// Each tuple contains (value, weight). Weights are normalized automatically.
///
/// # Examples
/// ```
/// let blended = weighted_blend(&[(10.0, 0.3), (20.0, 0.7)]); // Returns 17.0
/// let equal = weighted_blend(&[(5.0, 1.0), (15.0, 1.0)]); // Returns 10.0
/// ```
pub fn weighted_blend(values: &[(f32, f32)]) -> f32 {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return 0.0;
    }

    values.iter()
        .map(|(value, weight)| value * weight / total_weight)
        .sum()
}

/// Weighted blend for Vec2
pub fn weighted_blend_vec2(values: &[(Vec2, f32)]) -> Vec2 {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Vec2::ZERO;
    }

    values.iter()
        .map(|(value, weight)| *value * (weight / total_weight))
        .sum()
}

/// Weighted blend for Vec3
pub fn weighted_blend_vec3(values: &[(Vec3, f32)]) -> Vec3 {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Vec3::ZERO;
    }

    values.iter()
        .map(|(value, weight)| *value * (weight / total_weight))
        .sum()
}


/// Interpolate between two colors
///
/// This is the centralized version of colors.rs interpolate()
#[inline]
pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    from.mix(&to, t.clamp(0.0, 1.0))
}

/// Weighted blend of multiple colors
///
/// This is the centralized version of colors.rs blend_biomes()
pub fn weighted_blend_colors(colors: &[(Color, f32)]) -> Color {
    let total_weight: f32 = colors.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Color::BLACK;
    }

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;

    for (color, weight) in colors {
        let normalized_weight = weight / total_weight;
        let [cr, cg, cb, ca] = color.to_linear().to_f32_array();
        r += cr * normalized_weight;
        g += cg * normalized_weight;
        b += cb * normalized_weight;
        a += ca * normalized_weight;
    }

    Color::LinearRgba(LinearRgba::new(r, g, b, a))
}


/// Quadratic ease-in (accelerating from zero velocity)
#[inline]
pub fn ease_in_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t
}

/// Quadratic ease-out (decelerating to zero velocity)
#[inline]
pub fn ease_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * (2.0 - t)
}

/// Quadratic ease-in-out (accelerate then decelerate)
#[inline]
pub fn ease_in_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

/// Cubic ease-in (stronger acceleration)
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * t
}

/// Cubic ease-out (stronger deceleration)
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t1 = t - 1.0;
    t1 * t1 * t1 + 1.0
}

/// Cubic ease-in-out
#[inline]
pub fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t2 = 2.0 * t - 2.0;
        1.0 + t2 * t2 * t2 / 2.0
    }
}

/// Elastic ease-out (bouncy spring effect)
#[inline]
pub fn ease_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let p = 0.3;
    let a = 1.0;
    let s = p / 4.0;
    a * 2.0_f32.powf(-10.0 * t) * fast_sin((t - s) * (2.0 * PI) / p) + 1.0
}

/// Bounce ease-out (bouncing ball effect)
#[inline]
pub fn ease_out_bounce(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t2 = t - 1.5 / 2.75;
        7.5625 * t2 * t2 + 0.75
    } else if t < 2.5 / 2.75 {
        let t2 = t - 2.25 / 2.75;
        7.5625 * t2 * t2 + 0.9375
    } else {
        let t2 = t - 2.625 / 2.75;
        7.5625 * t2 * t2 + 0.984375
    }
}

// BILINEAR INTERPOLATION (for 2D grids like heightmaps)

/// Bilinear interpolation on a 2D grid
///
/// Used for smooth heightmap sampling in erosion.rs
///
/// # Parameters
/// - `x`, `y`: Position in grid coordinates (can be fractional)
/// - `get_value`: Function that returns value at integer grid coordinates
#[inline]
pub fn bilinear_interpolate<F>(x: f32, y: f32, get_value: F) -> f32
where
    F: Fn(i32, i32) -> f32,
{
    let x0 = x.floor() as i32;
    let x1 = x0 + 1;
    let y0 = y.floor() as i32;
    let y1 = y0 + 1;

    let fx = x - x0 as f32;
    let fy = y - y0 as f32;

    let v00 = get_value(x0, y0);
    let v10 = get_value(x1, y0);
    let v01 = get_value(x0, y1);
    let v11 = get_value(x1, y1);

    let v0 = lerp(v00, v10, fx);
    let v1 = lerp(v01, v11, fx);
    lerp(v0, v1, fy)
}


/// Clamp a value between min and max
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

/// Normalize a value from [min, max] to [0, 1]
#[inline]
pub fn normalize(value: f32, min: f32, max: f32) -> f32 {
    inverse_lerp(min, max, value)
}

/// Denormalize a value from [0, 1] to [min, max]
#[inline]
pub fn denormalize(value: f32, min: f32, max: f32) -> f32 {
    lerp(min, max, value)
}

/// Wrap a value to range [0, max)
#[inline]
pub fn wrap(value: f32, max: f32) -> f32 {
    ((value % max) + max) % max
}

/// Ping-pong a value between 0 and max
#[inline]
pub fn ping_pong(value: f32, max: f32) -> f32 {
    let t = wrap(value, max * 2.0);
    if t < max {
        t
    } else {
        max * 2.0 - t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 2.0), 10.0); // Clamped
    }

    #[test]
    fn test_inverse_lerp() {
        assert_eq!(inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq!(inverse_lerp(10.0, 20.0, 15.0), 0.5);
        assert_eq!(inverse_lerp(0.0, 10.0, -5.0), 0.0); // Clamped
        assert_eq!(inverse_lerp(0.0, 10.0, 15.0), 1.0); // Clamped
    }

    #[test]
    fn test_remap() {
        assert_eq!(remap(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(remap(0.5, 0.0, 1.0, -1.0, 1.0), 0.0);
        assert_eq!(remap(0.0, 0.0, 1.0, 100.0, 200.0), 100.0);
    }

    #[test]
    fn test_smoothstep() {
        assert_eq!(smoothstep(0.0, 1.0, 0.0), 0.0);
        assert_eq!(smoothstep(0.0, 1.0, 1.0), 1.0);
        assert!(smoothstep(0.0, 1.0, 0.5) > 0.4 && smoothstep(0.0, 1.0, 0.5) < 0.6);
    }

    #[test]
    fn test_weighted_blend() {
        assert_eq!(weighted_blend(&[(10.0, 1.0), (20.0, 1.0)]), 15.0);
        assert_eq!(weighted_blend(&[(10.0, 0.3), (20.0, 0.7)]), 17.0);
        assert_eq!(weighted_blend(&[(5.0, 1.0)]), 5.0);
        assert_eq!(weighted_blend(&[]), 0.0);
    }

    #[test]
    fn test_asymmetric_smooth() {
        let current = 0.5;
        let target_up = 1.0;
        let target_down = 0.0;
        let delta = 0.016; // ~60 FPS

        // Should increase faster than decrease
        let up = asymmetric_smooth(current, target_up, 2.0, 0.3, delta);
        let down = asymmetric_smooth(current, target_down, 2.0, 0.3, delta);

        assert!(up - current > current - down);
    }
}