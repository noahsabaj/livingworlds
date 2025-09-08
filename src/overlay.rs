//! Map overlay rendering system for Living Worlds
//! 
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! It acts as the central coordinator for how provinces are visually
//! represented based on the current viewing mode.

use bevy::prelude::*;
use bevy::render::mesh::Mesh;
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
use crate::setup::ProvinceStorage;
use crate::constants::*;

/// System that updates province colors in the mega-mesh based on active overlay mode
/// This rebuilds the entire mesh's vertex colors when the overlay changes
pub fn update_province_colors(
    overlay: Res<ResourceOverlay>,
    province_storage: Res<ProvinceStorage>,
    mesh_handle: Res<crate::setup::WorldMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // System already only runs when overlay changes due to run_if condition
    
    // Get the mesh from the handle
    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        return;
    };
    
    // Calculate world dimensions
    let (provinces_per_row, provinces_per_col) = match province_storage.provinces.len() {
        15000 => (150, 100),  // Small world
        60000 => (300, 200),  // Medium world
        135000 => (450, 300), // Large world
        _ => (150, 100),      // Default to small
    };
    
    // Rebuild color buffer for all provinces
    let mut colors = Vec::with_capacity(province_storage.provinces.len() * 7);
    
    // Process provinces in order by ID to match mesh vertex order
    let mut provinces_vec: Vec<_> = province_storage.provinces.iter().collect();
    provinces_vec.sort_by_key(|(id, _)| **id);
    
    for (&province_id, province) in provinces_vec {
        // Get resources and infrastructure for this province
        let resources = province_storage.resources.get(&province_id);
        let infrastructure = province_storage.infrastructure.get(&province_id);
        
        // Calculate color based on overlay mode
        let province_color = match *overlay {
            ResourceOverlay::None => {
                // Political/terrain view
                if let Some(nation_id) = province.nation_id {
                    let hue = nation_id as f32 / 8.0;
                    let nation_color = Color::hsl(hue * 360.0, 0.7, 0.5);
                    let terrain_color = get_terrain_color_gradient(province.terrain, province.elevation);
                    Color::srgb(
                        nation_color.to_srgba().red * 0.8 + terrain_color.to_srgba().red * 0.2,
                        nation_color.to_srgba().green * 0.8 + terrain_color.to_srgba().green * 0.2,
                        nation_color.to_srgba().blue * 0.8 + terrain_color.to_srgba().blue * 0.2,
                    )
                } else {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                }
            },
            
            ResourceOverlay::Mineral(mineral_type) => {
                if province.terrain == TerrainType::Ocean {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else if let Some(res) = resources {
                    let abundance = get_mineral_abundance(res, mineral_type);
                    if mineral_type == MineralType::Stone {
                        stone_abundance_color(abundance)
                    } else {
                        mineral_abundance_color(abundance)
                    }
                } else {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                }
            },
            
            ResourceOverlay::AllMinerals => {
                if province.terrain == TerrainType::Ocean {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else if let Some(res) = resources {
                    let total_richness = calculate_total_richness(res);
                    combined_richness_color(total_richness)
                } else {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                }
            },
            
            ResourceOverlay::Infrastructure => {
                if province.terrain == TerrainType::Ocean {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                } else if let Some(infra) = infrastructure {
                    let level = infra.mine_level + infra.forge_level;
                    infrastructure_level_color(level)
                } else {
                    get_terrain_color_gradient(province.terrain, province.elevation)
                }
            },
        };
        
        // Convert to vertex color (use linear for proper color mixing)
        let linear = province_color.to_linear();
        let color_array = [
            linear.red,
            linear.green,
            linear.blue,
            linear.alpha,
        ];
        
        // Add color for center vertex and 6 corner vertices (7 total per province)
        for _ in 0..7 {
            colors.push(color_array);
        }
    }
    
    // Update the mesh's color attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    
    println!("Updated mega-mesh colors for overlay: {}", overlay.display_name());
}

/// Plugin that manages map overlay rendering
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        // Only update colors when overlay mode changes, not every frame!
        app.add_systems(Update, 
            update_province_colors.run_if(resource_changed::<ResourceOverlay>)
        );
    }
}