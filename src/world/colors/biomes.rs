//! Biome-specific color functions
//!
//! This module provides color functions for biomes, moved from the generation
//! layer to maintain proper separation of concerns between data generation
//! and visual rendering.

use bevy::prelude::Color;
use crate::world::terrain::Biome;

/// Get base color for a biome type
///
/// This is the primary color representation for each biome, used
/// when rendering biome overlays or climate views.
pub fn get_biome_color(biome: Biome) -> Color {
    match biome {
        // Polar biomes
        Biome::PolarDesert => Color::srgb(0.95, 0.95, 0.95),
        Biome::Tundra => Color::srgb(0.7, 0.7, 0.6),

        // Cold biomes
        Biome::Taiga => Color::srgb(0.2, 0.4, 0.3),
        Biome::BorealForest => Color::srgb(0.15, 0.35, 0.25),

        // Temperate biomes
        Biome::TemperateRainforest => Color::srgb(0.1, 0.5, 0.2),
        Biome::TemperateDeciduousForest => Color::srgb(0.25, 0.55, 0.3),
        Biome::TemperateGrassland => Color::srgb(0.5, 0.65, 0.35),
        Biome::ColdDesert => Color::srgb(0.7, 0.65, 0.5),

        // Subtropical biomes
        Biome::MediterraneanForest => Color::srgb(0.35, 0.5, 0.35),
        Biome::Chaparral => Color::srgb(0.55, 0.6, 0.4),
        Biome::SubtropicalDesert => Color::srgb(0.85, 0.75, 0.5),

        // Tropical biomes
        Biome::TropicalRainforest => Color::srgb(0.05, 0.4, 0.1),
        Biome::TropicalSeasonalForest => Color::srgb(0.15, 0.45, 0.15),
        Biome::Savanna => Color::srgb(0.65, 0.7, 0.35),
        Biome::TropicalDesert => Color::srgb(0.9, 0.8, 0.5),

        // Special biomes
        Biome::Alpine => Color::srgb(0.8, 0.8, 0.85),
        Biome::Wetlands => Color::srgb(0.3, 0.45, 0.4),
        Biome::Mangrove => Color::srgb(0.2, 0.35, 0.3),
    }
}

/// Get biome color with variation for natural appearance
///
/// Adds random variation to the base biome color to create a more
/// organic, natural look when rendering many provinces of the same biome.
///
/// # Arguments
/// * `biome` - The biome type to get color for
/// * `variation` - Random variation factor (typically 0.0 to 1.0)
///
/// # Returns
/// The biome color with up to 10% variation applied
pub fn get_biome_color_varied(biome: Biome, variation: f32) -> Color {
    let base = get_biome_color(biome);
    let var = variation * 0.1; // Max 10% variation

    Color::srgb(
        (base.to_linear().red + var).clamp(0.0, 1.0),
        (base.to_linear().green + var).clamp(0.0, 1.0),
        (base.to_linear().blue + var).clamp(0.0, 1.0),
    )
}