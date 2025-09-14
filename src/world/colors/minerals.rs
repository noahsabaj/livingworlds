//! Mineral color functions and abundance gradients
//!
//! This module provides color functions for mineral resources, including
//! type-specific colors and abundance heat maps.

use super::theme;
use super::utils::StoneAbundance;
use crate::components::MineralType;
use crate::math::lerp_color;
use bevy::prelude::Color;

/// Get color for a specific mineral type
pub fn get_mineral_color(mineral_type: MineralType) -> Color {
    match mineral_type {
        MineralType::Iron => theme::IRON,
        MineralType::Copper => theme::COPPER,
        MineralType::Tin => theme::TIN,
        MineralType::Gold => theme::GOLD,
        MineralType::Coal => theme::COAL,
        MineralType::Stone => theme::STONE,
        MineralType::Gems => theme::GEMS,
    }
}

/// Calculate color for mineral abundance (0-100)
///
/// Creates a heat map gradient from dark grey (no minerals) to
/// bright white (maximum abundance).
///
/// # Arguments
/// * `abundance` - Mineral abundance from 0 (none) to 100 (maximum)
///
/// # Returns
/// Heat map color representing the abundance level
pub fn mineral_abundance_color(abundance: u8) -> Color {
    if abundance == 0 {
        theme::HEAT_NONE
    } else {
        let normalized = abundance as f32 / 100.0;

        if normalized < 0.33 {
            // Interpolate between none and low
            let t = normalized * 3.0;
            lerp_color(theme::HEAT_NONE, theme::HEAT_LOW, t)
        } else if normalized < 0.67 {
            // Interpolate between low and medium
            let t = (normalized - 0.33) * 3.0;
            lerp_color(theme::HEAT_LOW, theme::HEAT_MEDIUM, t)
        } else {
            // Interpolate between medium and high
            let t = (normalized - 0.67) * 3.0;
            lerp_color(theme::HEAT_MEDIUM, theme::HEAT_HIGH, t)
        }
    }
}

/// Special color scale for stone (which ranges from 20-80, not 0-100)
///
/// Stone has a different abundance range than other minerals, so it
/// needs special handling to produce appropriate earth tone colors.
///
/// # Arguments
/// * `abundance` - Stone abundance wrapper with validation
///
/// # Returns
/// Earth tone colors for stone deposits, or ocean blue for no stone
pub fn stone_abundance_color(abundance: StoneAbundance) -> Color {
    if abundance.value() == 0 {
        Color::srgb(0.0, 0.1, 0.2) // Ocean blue
    } else {
        let normalized = abundance.normalized();
        if normalized < 0.5 {
            let t = normalized * 2.0;
            lerp_color(Color::srgb(0.3, 0.2, 0.1), Color::srgb(0.5, 0.4, 0.2), t)
        } else {
            let t = (normalized - 0.5) * 2.0;
            lerp_color(Color::srgb(0.5, 0.4, 0.2), Color::srgb(0.8, 0.7, 0.5), t)
        }
    }
}

/// Color for combined mineral richness
///
/// Used when showing overall mineral wealth rather than specific types.
///
/// # Arguments
/// * `richness` - Combined richness value, typically 0.0 to 10.0
///
/// # Returns
/// Gradient from brown (poor) to bright gold (extremely rich)
pub fn combined_richness_color(richness: f32) -> Color {
    let clamped = richness.clamp(0.0, 10.0) / 10.0;

    if clamped < 0.5 {
        let t = clamped * 2.0;
        lerp_color(theme::HEAT_LOW, theme::HEAT_MEDIUM, t)
    } else {
        let t = (clamped - 0.5) * 2.0;
        lerp_color(theme::HEAT_MEDIUM, theme::HEAT_HIGH, t)
    }
}

/// Color for mining infrastructure level
///
/// Shows the development level of mining operations.
///
/// # Arguments
/// * `level` - Infrastructure level from 0 (none) to 5 (maximum)
///
/// # Returns
/// Gradient from dark grey to light tan
pub fn infrastructure_level_color(level: u8) -> Color {
    // Validate level and provide gradient
    let level = level.min(5);
    let t = level as f32 / 5.0;
    lerp_color(theme::HEAT_NONE, theme::HEAT_HIGH, t)
}
