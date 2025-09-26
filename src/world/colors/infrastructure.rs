//! Infrastructure visualization colors
//!
//! Provides color gradients for visualizing infrastructure development levels,
//! connectivity, and trade networks.

use bevy::prelude::*;
use crate::math::lerp_color;

/// Generate a color representing infrastructure development level
///
/// Returns colors from dark brown (wilderness) to bright red/gold (major hub)
pub fn infrastructure_gradient_color(connectivity: f32) -> Color {
    let normalized = connectivity.clamp(0.0, 1.0);

    if normalized < 0.2 {
        // Wilderness - dark brown
        Color::srgb(0.15, 0.1, 0.05)
    } else if normalized < 0.35 {
        // Rural - light brown to tan
        let t = (normalized - 0.2) / 0.15;
        lerp_color(
            Color::srgb(0.15, 0.1, 0.05),
            Color::srgb(0.4, 0.35, 0.25),
            t,
        )
    } else if normalized < 0.5 {
        // Developing - tan to yellow
        let t = (normalized - 0.35) / 0.15;
        lerp_color(
            Color::srgb(0.4, 0.35, 0.25),
            Color::srgb(0.7, 0.65, 0.3),
            t,
        )
    } else if normalized < 0.65 {
        // Developed - yellow to gold
        let t = (normalized - 0.5) / 0.15;
        lerp_color(
            Color::srgb(0.7, 0.65, 0.3),
            Color::srgb(0.85, 0.7, 0.15),
            t,
        )
    } else if normalized < 0.8 {
        // Urban - gold to orange
        let t = (normalized - 0.65) / 0.15;
        lerp_color(
            Color::srgb(0.85, 0.7, 0.15),
            Color::srgb(0.95, 0.5, 0.1),
            t,
        )
    } else {
        // Major hub - bright orange to red
        let t = (normalized - 0.8) / 0.2;
        lerp_color(
            Color::srgb(0.95, 0.5, 0.1),
            Color::srgb(1.0, 0.3, 0.1),
            t,
        )
    }
}

/// Generate color for road density visualization
pub fn road_density_color(density: f32) -> Color {
    let normalized = density.clamp(0.0, 1.0);

    // Gray scale for roads - darker = more roads
    let gray = 0.8 - (normalized * 0.6);
    Color::srgb(gray, gray, gray * 0.95) // Slight blue tint
}

/// Generate color for trade volume visualization
pub fn trade_volume_color(volume: f32) -> Color {
    let normalized = volume.clamp(0.0, 1.0);

    // Purple to gold gradient for trade
    if normalized < 0.5 {
        let t = normalized * 2.0;
        lerp_color(
            Color::srgb(0.3, 0.1, 0.4), // Dark purple
            Color::srgb(0.6, 0.3, 0.6), // Light purple
            t,
        )
    } else {
        let t = (normalized - 0.5) * 2.0;
        lerp_color(
            Color::srgb(0.6, 0.3, 0.6), // Light purple
            Color::srgb(0.9, 0.7, 0.2), // Gold
            t,
        )
    }
}

/// Composite infrastructure color combining multiple metrics
pub fn composite_infrastructure_color(
    connectivity: f32,
    road_density: f32,
    trade_volume: f32,
    is_hub: bool,
) -> Color {
    // Base color from connectivity
    let mut base_color = infrastructure_gradient_color(connectivity);

    // Modulate with trade volume if significant
    if trade_volume > 0.3 {
        let trade_color = trade_volume_color(trade_volume);
        base_color = lerp_color(base_color, trade_color, 0.3);
    }

    // Add highlights for hubs
    if is_hub {
        // Make hubs brighter and more saturated
        let highlight = Color::srgb(1.0, 0.9, 0.4); // Bright yellow
        base_color = lerp_color(base_color, highlight, 0.4);
    }

    // Darken slightly if low road density despite connectivity
    if road_density < 0.2 && connectivity > 0.4 {
        base_color = lerp_color(base_color, Color::srgb(0.2, 0.15, 0.1), 0.2);
    }

    base_color
}

/// Special color for capital cities
pub fn capital_infrastructure_color() -> Color {
    Color::srgb(1.0, 0.85, 0.0) // Bright gold
}

/// Special color for port cities
pub fn port_infrastructure_color(base_color: Color) -> Color {
    // Add blue tint to indicate port
    lerp_color(base_color, Color::srgb(0.3, 0.6, 0.9), 0.25)
}

/// Development level to descriptive color
pub fn development_level_color(level: u8) -> Color {
    match level {
        0 => Color::srgb(0.15, 0.1, 0.05),   // Wilderness - dark brown
        1 => Color::srgb(0.4, 0.35, 0.25),   // Rural - tan
        2 => Color::srgb(0.7, 0.65, 0.3),    // Developing - yellow
        3 => Color::srgb(0.85, 0.7, 0.15),   // Developed - gold
        4 => Color::srgb(0.95, 0.5, 0.1),    // Urban - orange
        5 => Color::srgb(1.0, 0.3, 0.1),     // Metropolis - red
        _ => Color::srgb(0.5, 0.5, 0.5),     // Unknown - gray
    }
}