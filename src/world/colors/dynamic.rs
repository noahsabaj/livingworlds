//! Dynamic color adjustments based on time and weather
//!
//! This module provides functions to modify colors based on game state,
//! including time of day and weather effects.

use bevy::prelude::Color;
use crate::resources::{GameTime, WeatherSystem};
use super::utils::SafeColor;

/// Apply time of day color adjustments
///
/// Modifies colors based on the current time, adding dawn/dusk tinting
/// and night darkening effects.
///
/// # Arguments
/// * `base_color` - The original color to modify
/// * `game_time` - Current game time for calculating hour of day
///
/// # Returns
/// Color adjusted for time of day effects
pub fn apply_time_of_day(base_color: Color, game_time: &GameTime) -> Color {
    let hours = (game_time.current_date % 1.0) * 24.0;

    // Dawn/dusk tinting
    let dawn_dusk_factor = if (5.0..7.0).contains(&hours) || (17.0..19.0).contains(&hours) {
        0.2
    } else {
        0.0
    };

    // Night darkening
    let night_factor = if (20.0..=24.0).contains(&hours) || (0.0..5.0).contains(&hours) {
        0.3
    } else {
        0.0
    };

    let rgba = base_color.to_srgba();
    let r = rgba.red + dawn_dusk_factor * 0.1 - night_factor * 0.2;
    let g = rgba.green - night_factor * 0.2;
    let b = rgba.blue - dawn_dusk_factor * 0.05 - night_factor * 0.1;

    SafeColor::srgb(r, g, b)
}

/// Apply weather-based color adjustments
///
/// Modifies colors based on weather conditions, primarily cloud coverage
/// which affects saturation and brightness.
///
/// # Arguments
/// * `base_color` - The original color to modify
/// * `weather` - Current weather system state
///
/// # Returns
/// Color adjusted for weather effects
pub fn apply_weather(base_color: Color, weather: &WeatherSystem) -> Color {
    // Desaturate and darken based on cloud coverage
    let coverage = weather.cloud_coverage;
    let rgba = base_color.to_srgba();

    // Reduce saturation
    let luminance = rgba.red * 0.299 + rgba.green * 0.587 + rgba.blue * 0.114;
    let r = rgba.red * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;
    let g = rgba.green * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;
    let b = rgba.blue * (1.0 - coverage * 0.3) + luminance * coverage * 0.3;

    // Darken
    let darkness = coverage * 0.2;
    SafeColor::srgb(r - darkness, g - darkness, b - darkness)
}

/// Apply both time and weather effects to a color
///
/// Convenience function that applies both time of day and weather
/// adjustments in the correct order.
///
/// # Arguments
/// * `base_color` - The original color to modify
/// * `game_time` - Current game time
/// * `weather` - Current weather system
///
/// # Returns
/// Color with all dynamic adjustments applied
pub fn apply_all_effects(
    base_color: Color,
    game_time: &GameTime,
    weather: &WeatherSystem,
) -> Color {
    let time_adjusted = apply_time_of_day(base_color, game_time);
    apply_weather(time_adjusted, weather)
}