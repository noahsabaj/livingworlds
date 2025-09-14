//! Main color system API
//!
//! This module contains the WorldColors struct which provides
//! the primary interface for obtaining colors throughout the game.

use bevy::prelude::*;
use crate::resources::{GameTime, WeatherSystem};
use crate::components::MineralType;
use crate::world::TerrainType;
use crate::math::{lerp_color, weighted_blend_colors};

use super::utils::StoneAbundance;
use super::providers::{ColorProvider, StandardColorProvider};
use super::{theme, terrain, minerals, dynamic};

/// Main API for getting colors throughout the game
pub struct WorldColors {
    provider: StandardColorProvider,
}

impl WorldColors {
    pub fn new(seed: u32) -> Self {
        Self {
            provider: StandardColorProvider::new(seed),
        }
    }

    /// Get terrain color with all enhancements
    pub fn terrain(&self, terrain: TerrainType, elevation: f32, world_pos: Vec2) -> Color {
        self.provider.terrain_color(terrain, elevation, world_pos)
    }

    /// Get terrain color with dynamic adjustments
    pub fn terrain_dynamic(
        &self,
        terrain: TerrainType,
        elevation: f32,
        world_pos: Vec2,
        game_time: &GameTime,
        weather: &WeatherSystem,
    ) -> Color {
        let base = self.terrain(terrain, elevation, world_pos);
        let time_adjusted = dynamic::apply_time_of_day(base, game_time);
        dynamic::apply_weather(time_adjusted, weather)
    }

    /// Get mineral abundance color
    pub fn mineral_abundance(&self, abundance: u8) -> Color {
        self.provider.mineral_abundance_color(abundance)
    }

    /// Get stone abundance color with special handling
    pub fn stone_abundance(&self, abundance: StoneAbundance) -> Color {
        minerals::stone_abundance_color(abundance)
    }

    /// Get mineral type color
    pub fn mineral_type(&self, mineral: MineralType) -> Color {
        minerals::get_mineral_color(mineral)
    }

    /// Get combined mineral richness color
    pub fn richness(&self, richness: f32) -> Color {
        self.provider.combined_richness_color(richness)
    }

    /// Get infrastructure level color
    pub fn infrastructure(&self, level: u8) -> Color {
        self.provider.infrastructure_color(level)
    }

    /// Interpolate between two colors for smooth transitions
    pub fn interpolate(from: Color, to: Color, t: f32) -> Color {
        lerp_color(from, to, t)
    }

    /// Blend colors for biome transitions
    pub fn blend_biomes(colors: &[(Color, f32)]) -> Color {
        if colors.is_empty() || colors.iter().all(|(_, w)| *w <= 0.0) {
            return theme::TEMPERATE_GRASSLAND;
        }
        weighted_blend_colors(colors)
    }
}