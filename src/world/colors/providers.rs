//! Color provider traits and implementations
//!
//! This module defines the abstraction for providing colors to different
//! game systems, allowing for different implementations with various
//! optimization strategies.

use super::minerals::{
    combined_richness_color, infrastructure_level_color, mineral_abundance_color,
};
use super::terrain::get_terrain_color;
use super::utils::position_hash;
use crate::world::MineralType;
use crate::world::TerrainType;
use bevy::prelude::{Color, Vec2};

/// Abstraction for providing colors to different game systems
pub trait ColorProvider {
    fn terrain_color(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color;
    fn mineral_abundance_color(&self, abundance: u8) -> Color;
    fn combined_richness_color(&self, richness: f32) -> Color;
    fn infrastructure_color(&self, level: u8) -> Color;
}

/// Standard color provider with all optimizations
pub struct StandardColorProvider {
    seed: u32,
}

impl StandardColorProvider {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

impl ColorProvider for StandardColorProvider {
    fn terrain_color(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color {
        let base = get_terrain_color(terrain, elevation);

        // Skip color variation for water tiles - they should be uniform
        if matches!(terrain, TerrainType::Ocean | TerrainType::River) {
            return base;
        }

        // Use HSL color space for natural variation that avoids RGB ceiling artifacts
        // This prevents blue channel from hitting 1.0 while red/green keep growing
        let base_srgba = base.to_srgba();
        let base_hsl = bevy::color::Hsla::from(base_srgba);

        // Apply very subtle lightness variation only (preserves hue and saturation)
        let lightness_variation = position_hash(world_pos.x, world_pos.y, self.seed) * 0.005; // ±0.5%
        let new_lightness = (base_hsl.lightness + lightness_variation).clamp(0.0, 1.0);

        let varied_hsl = bevy::color::Hsla::new(
            base_hsl.hue,
            base_hsl.saturation,
            new_lightness,
            base_hsl.alpha,
        );

        Color::from(varied_hsl)
    }

    fn mineral_abundance_color(&self, abundance: u8) -> Color {
        mineral_abundance_color(abundance)
    }

    fn combined_richness_color(&self, richness: f32) -> Color {
        combined_richness_color(richness)
    }

    fn infrastructure_color(&self, level: u8) -> Color {
        infrastructure_level_color(level)
    }
}

/// Trait for types that can provide colors
pub trait Colorable {
    fn color(&self) -> Color;
    fn color_with_elevation(&self, elevation: f32) -> Color;
}

impl Colorable for TerrainType {
    fn color(&self) -> Color {
        self.color_with_elevation(0.5)
    }

    fn color_with_elevation(&self, elevation: f32) -> Color {
        get_terrain_color(*self, elevation)
    }
}

impl Colorable for MineralType {
    fn color(&self) -> Color {
        use super::minerals::get_mineral_color;
        get_mineral_color(*self)
    }

    fn color_with_elevation(&self, _elevation: f32) -> Color {
        self.color() // Minerals don't vary with elevation
    }
}
