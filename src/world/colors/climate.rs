//! Climate visualization color functions
//!
//! This module provides color generation for climate overlays,
//! mapping temperature, rainfall, and climate zones to intuitive colors.

use bevy::prelude::Color;
use crate::world::terrain::StoredClimateZone;
use crate::math::lerp_color;

/// Get color for a specific climate zone
pub fn climate_zone_color(zone: StoredClimateZone) -> Color {
    match zone {
        StoredClimateZone::Arctic => Color::srgb(0.75, 0.85, 1.0),      // Icy blue-white
        StoredClimateZone::Subarctic => Color::srgb(0.65, 0.75, 0.9),   // Cold blue
        StoredClimateZone::Temperate => Color::srgb(0.4, 0.7, 0.4),     // Pleasant green
        StoredClimateZone::Subtropical => Color::srgb(0.8, 0.75, 0.4),  // Warm yellow-green
        StoredClimateZone::Tropical => Color::srgb(0.9, 0.7, 0.3),      // Hot yellow-orange
        StoredClimateZone::Desert => Color::srgb(0.95, 0.6, 0.35),      // Desert orange-red
        StoredClimateZone::Alpine => Color::srgb(0.9, 0.92, 1.0),       // Mountain white-blue
    }
}

/// Get color based on temperature with smooth gradients
pub fn temperature_gradient_color(temperature: f32, normalized: f32) -> Color {
    // Define temperature color stops (from cold to hot)
    let color_stops = [
        (Color::srgb(0.2, 0.3, 0.8), -20.0),   // Deep blue for extreme cold
        (Color::srgb(0.4, 0.6, 0.9), -10.0),   // Light blue for very cold
        (Color::srgb(0.5, 0.75, 0.85), 0.0),   // Cyan for cold
        (Color::srgb(0.4, 0.8, 0.4), 10.0),    // Green for cool
        (Color::srgb(0.6, 0.85, 0.3), 20.0),   // Yellow-green for warm
        (Color::srgb(0.9, 0.85, 0.3), 25.0),   // Yellow for hot
        (Color::srgb(1.0, 0.6, 0.2), 30.0),    // Orange for very hot
        (Color::srgb(1.0, 0.3, 0.1), 35.0),    // Red for extreme heat
    ];

    // Find the appropriate color range
    for i in 1..color_stops.len() {
        if temperature <= color_stops[i].1 {
            let prev = &color_stops[i - 1];
            let next = &color_stops[i];
            let t = (temperature - prev.1) / (next.1 - prev.1);
            return lerp_color(prev.0, next.0, t.clamp(0.0, 1.0));
        }
    }

    // Beyond hottest threshold
    color_stops.last().unwrap().0
}

/// Get color modulated by rainfall (for rainforest/dry variations)
pub fn rainfall_modulated_color(base_color: Color, rainfall: f32) -> Color {
    // High rainfall makes colors more green/lush
    // Low rainfall makes colors more brown/arid

    if rainfall > 2000.0 {
        // Very wet - shift toward deep green
        let green_tint = Color::srgb(0.1, 0.5, 0.1);
        lerp_color(base_color, green_tint, 0.3)
    } else if rainfall < 500.0 {
        // Very dry - shift toward brown
        let brown_shift = Color::srgb(0.65, 0.5, 0.35);
        lerp_color(base_color, brown_shift, 0.3)
    } else {
        // Normal rainfall - minimal adjustment
        base_color
    }
}

/// Get composite climate color combining temperature and rainfall
pub fn composite_climate_color(
    temperature: f32,
    rainfall: f32,
    zone: StoredClimateZone,
    normalized_temp: f32,
) -> Color {
    // Start with temperature gradient
    let temp_color = temperature_gradient_color(temperature, normalized_temp);

    // Blend with zone color for consistency
    let zone_color = climate_zone_color(zone);
    let blended = lerp_color(temp_color, zone_color, 0.3);

    // Apply rainfall modulation for final color
    rainfall_modulated_color(blended, rainfall)
}

/// Get humidity overlay color (for additional visualization mode)
pub fn humidity_color(humidity: f32) -> Color {
    // Blue gradient for humidity levels
    let dry = Color::srgb(0.9, 0.8, 0.6);  // Sandy beige
    let humid = Color::srgb(0.2, 0.4, 0.8); // Deep blue

    lerp_color(dry, humid, humidity.clamp(0.0, 1.0))
}