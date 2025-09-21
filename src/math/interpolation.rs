//! Game-specific interpolation and smoothing functions for Living Worlds
//!
//! This module provides specialized interpolation functions for game mechanics.
//! For basic linear interpolation, use Bevy's built-in methods directly:
//! - `a.lerp(b, t)` for most types (f32, Vec2, Vec3, Color, etc.)
//! - `pos1.lerp(pos2, t)` for position interpolation
//! - `color1.lerp(color2, t)` for color transitions
//!
//! # Usage
//!
//! ```rust
//! use crate::math::interpolation::*;
//!
//! // Basic interpolation - use Bevy directly
//! let value = 0.0_f32.lerp(10.0, 0.5); // Returns 5.0
//! let pos = start_pos.lerp(end_pos, t);
//!
//! // Game-specific functions from this module
//! let smooth_pos = lerp_exp_vec3(current_pos, target_pos, smoothing, delta_time);
//! let tension = asymmetric_smooth(current, target, rise_rate, fall_rate, delta_time);
//! ```

use bevy::prelude::*;

/// Linear interpolation between two values
///
/// Returns a value linearly interpolated from `a` to `b` by parameter `t`.
/// When t=0 returns a, when t=1 returns b, and values in between are interpolated.
///
/// # Examples
/// ```
/// let mid = lerp(0.0, 10.0, 0.5); // Returns 5.0
/// let quarter = lerp(100.0, 200.0, 0.25); // Returns 125.0
/// ```
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

/// Inverse linear interpolation (unlerp)
///
/// Returns where `value` falls between `a` and `b` as a 0-1 parameter.
/// This is the inverse of lerp: if a.lerp(b, t) = value, then inverse_lerp(a, b, value) = t
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
    to_min.lerp(to_max, t)
}

/// Smoothstep interpolation
///
/// Provides smooth interpolation with zero derivatives at edges.
/// Uses the formula: 3t² - 2t³
///
/// # Examples
/// ```
/// let smooth = smoothstep(0.0, 1.0, 0.5); // Smoother than linear
/// ```
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smootherstep interpolation (Ken Perlin's improvement)
///
/// Even smoother than smoothstep with zero 1st and 2nd derivatives at edges.
/// Uses the formula: 6t⁵ - 15t⁴ + 10t³
///
/// # Examples
/// ```
/// let smoother = smootherstep(0.0, 1.0, 0.5); // Even smoother than smoothstep
/// ```
#[inline]
pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Frame-rate independent exponential smoothing for Vec2
///
/// Provides smooth, responsive interpolation that works consistently across different framerates.
/// Higher smoothing values = snappier response, lower values = smoother/slower.
///
/// # Examples
/// ```
/// // Camera following with medium smoothness
/// let new_pos = lerp_exp_vec2(current_pos, target_pos, 8.0, delta_time);
/// ```
#[inline]
pub fn lerp_exp_vec2(current: Vec2, target: Vec2, smoothing: f32, delta_time: f32) -> Vec2 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    current.lerp(target, t)
}

/// Frame-rate independent exponential smoothing for Vec3
///
/// Provides smooth, responsive interpolation that works consistently across different framerates.
/// Higher smoothing values = snappier response, lower values = smoother/slower.
///
/// # Examples
/// ```
/// // Camera following with medium smoothness
/// let new_pos = lerp_exp_vec3(current_pos, target_pos, 8.0, delta_time);
/// ```
#[inline]
pub fn lerp_exp_vec3(current: Vec3, target: Vec3, smoothing: f32, delta_time: f32) -> Vec3 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    current.lerp(target, t)
}

/// Frame-rate independent exponential smoothing for f32
///
/// Provides smooth, responsive interpolation that works consistently across different framerates.
/// Higher smoothing values = snappier response, lower values = smoother/slower.
#[inline]
pub fn lerp_exp(current: f32, target: f32, smoothing: f32, delta_time: f32) -> f32 {
    let t = 1.0 - (0.01_f32.powf(smoothing * delta_time));
    current.lerp(target, t)
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
/// Used extensively in Living Worlds for physics-like behavior.
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
    delta_time: f32,
) -> f32 {
    let rate = if target > current {
        increase_rate
    } else {
        decrease_rate
    };
    let t = 1.0 - (-rate * delta_time).exp();
    current.lerp(target, t)
}

/// Weighted blend of multiple values
///
/// Each tuple contains (value, weight). Weights are normalized automatically.
/// Useful for combining multiple influences with different strengths.
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

    values
        .iter()
        .map(|(value, weight)| value * weight / total_weight)
        .sum()
}

/// Weighted blend for Vec2
///
/// Useful for blending multiple positions with different influences.
pub fn weighted_blend_vec2(values: &[(Vec2, f32)]) -> Vec2 {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Vec2::ZERO;
    }

    values
        .iter()
        .map(|(value, weight)| *value * (weight / total_weight))
        .sum()
}

/// Weighted blend for Vec3
///
/// Useful for blending multiple 3D positions with different influences.
pub fn weighted_blend_vec3(values: &[(Vec3, f32)]) -> Vec3 {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Vec3::ZERO;
    }

    values
        .iter()
        .map(|(value, weight)| *value * (weight / total_weight))
        .sum()
}

/// Weighted blend for colors
///
/// Blends multiple colors with different weights. Useful for terrain color mixing
/// based on multiple biome influences.
///
/// # Examples
/// ```
/// let terrain_color = weighted_blend_colors(&[
///     (grassland_color, 0.6),
///     (forest_color, 0.3),
///     (water_color, 0.1),
/// ]);
/// ```
pub fn weighted_blend_colors(values: &[(Color, f32)]) -> Color {
    let total_weight: f32 = values.iter().map(|(_, w)| w).sum();
    if total_weight <= 0.0 {
        return Color::BLACK;
    }

    // Convert to linear RGB for proper blending
    let blended_linear: Vec4 = values
        .iter()
        .map(|(color, weight)| {
            let linear = color.to_linear();
            Vec4::new(linear.red, linear.green, linear.blue, linear.alpha) * (weight / total_weight)
        })
        .sum();

    // Convert back to sRGB
    Color::linear_rgba(
        blended_linear.x,
        blended_linear.y,
        blended_linear.z,
        blended_linear.w,
    )
}

/// Color interpolation in linear color space
///
/// Proper color blending that converts to linear space, interpolates, then converts back.
/// Use this instead of basic lerp for accurate color transitions.
///
/// # Examples
/// ```
/// let sunset_color = lerp_color(day_color, night_color, time_factor);
/// ```
#[inline]
pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    // Convert to linear space for proper interpolation
    let from_linear = from.to_linear();
    let to_linear = to.to_linear();

    // Interpolate in linear space
    let result_linear = LinearRgba::new(
        lerp(from_linear.red, to_linear.red, t),
        lerp(from_linear.green, to_linear.green, t),
        lerp(from_linear.blue, to_linear.blue, t),
        lerp(from_linear.alpha, to_linear.alpha, t),
    );

    // Convert back to sRGB Color
    Color::from(result_linear)
}
