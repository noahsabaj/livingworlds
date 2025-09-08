//! Map overlay rendering system for Living Worlds
//! 
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! It acts as the central coordinator for how provinces are visually
//! represented based on the current viewing mode.

use bevy::prelude::*;
use crate::components::{Province, ProvinceResources, ProvinceInfrastructure};
use crate::resources::ResourceOverlay;
use crate::terrain::TerrainType;
use crate::components::MineralType;
use crate::minerals::{get_mineral_abundance, calculate_total_richness};
use crate::colors::{
    get_terrain_color_gradient,
    mineral_abundance_color, stone_abundance_color, 
    combined_richness_color, infrastructure_level_color
};

/// System that updates province colors based on active overlay mode
/// This is the central rendering system for all map visualization modes
pub fn update_province_colors(
    overlay: Res<ResourceOverlay>,
    mut provinces: Query<(&Province, &ProvinceResources, &ProvinceInfrastructure, &mut Sprite)>,
) {
    // Only update if overlay changed
    if !overlay.is_changed() {
        return;
    }
    
    for (province, resources, infrastructure, mut sprite) in provinces.iter_mut() {
        sprite.color = match *overlay {
            ResourceOverlay::None => {
                // Political/terrain view - show nations or natural terrain
                if let Some(nation_id) = province.nation_id {
                    // Nation territories - use nation color
                    let hue = nation_id as f32 / 8.0;
                    Color::hsl(hue * 360.0, 0.7, 0.5)
                } else {
                    // Unowned territory - show terrain with proper water depths
                    get_terrain_color_gradient(province.terrain, province.elevation)
                }
            },
            
            ResourceOverlay::Mineral(mineral_type) => {
                // Mineral overlay - show abundance as heatmap
                if province.terrain == TerrainType::Ocean {
                    // Ocean keeps its depth-based colors for context
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else {
                    // Land shows mineral abundance heatmap
                    let abundance = get_mineral_abundance(resources, mineral_type);
                    // Stone uses different scale since it's ubiquitous
                    if mineral_type == MineralType::Stone {
                        stone_abundance_color(abundance)
                    } else {
                        mineral_abundance_color(abundance)
                    }
                }
            },
            
            ResourceOverlay::AllMinerals => {
                // Combined mineral richness overlay
                if province.terrain == TerrainType::Ocean {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else {
                    // Show total mineral wealth
                    let total_richness = calculate_total_richness(resources);
                    combined_richness_color(total_richness)
                }
            },
            
            ResourceOverlay::Infrastructure => {
                // Infrastructure development overlay
                if province.terrain == TerrainType::Ocean {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else {
                    // Show infrastructure level (mine + forge)
                    let level = infrastructure.mine_level + 
                               infrastructure.forge_level;
                    infrastructure_level_color(level)
                }
            },
        };
    }
}

/// Plugin that manages map overlay rendering
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_province_colors);
    }
}