//! Map overlay rendering system for Living Worlds
//! 
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! It acts as the central coordinator for how provinces are visually
//! represented based on the current viewing mode.

use bevy::prelude::*;
use rayon::prelude::*;
use bevy::render::mesh::Mesh;
use crate::resources::ResourceOverlay;
use crate::terrain::TerrainType;
use crate::components::MineralType;
use crate::minerals::calculate_total_richness;
use crate::colors::{
    get_terrain_color_gradient,
    mineral_abundance_color, stone_abundance_color, 
    combined_richness_color
};
use crate::mesh::ProvinceStorage;
use crate::resources::MapDimensions;

/// System that updates province colors in the mega-mesh based on active overlay mode
/// Uses pre-calculated cached colors for instant switching
pub fn update_province_colors(
    overlay: Res<ResourceOverlay>,
    cached_colors: Res<crate::resources::CachedOverlayColors>,
    mesh_handle: Res<crate::mesh::WorldMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    let start = std::time::Instant::now();
    println!("[{:.3}s] Starting color update for overlay: {}", 
             time.elapsed_secs(), 
             overlay.display_name());
    
    // System already only runs when overlay changes due to run_if condition
    
    // Get the mesh from the handle
    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        return;
    };
    
    let mesh_lookup_time = start.elapsed();
    
    // Select the appropriate pre-calculated color buffer based on overlay mode
    let colors = match *overlay {
        ResourceOverlay::None => &cached_colors.terrain,
        ResourceOverlay::Mineral(mineral_type) => {
            use crate::components::MineralType;
            match mineral_type {
                MineralType::Iron => &cached_colors.iron,
                MineralType::Copper => &cached_colors.copper,
                MineralType::Tin => &cached_colors.tin,
                MineralType::Gold => &cached_colors.gold,
                MineralType::Coal => &cached_colors.coal,
                MineralType::Stone => &cached_colors.stone,
                MineralType::Gems => &cached_colors.gems,
                _ => &cached_colors.terrain,
            }
        },
        ResourceOverlay::AllMinerals => &cached_colors.all_minerals,
        ResourceOverlay::Infrastructure => &cached_colors.infrastructure,
    };
    
    let selection_time = start.elapsed() - mesh_lookup_time;
    
    // Calculate buffer size for logging
    let buffer_size_mb = (colors.len() * std::mem::size_of::<[f32; 4]>()) as f32 / (1024.0 * 1024.0);
    
    // Fast copy using slice operations (avoids Vec overhead)
    let clone_start = std::time::Instant::now();
    
    // Create a new Vec with exact capacity and copy data directly
    let mut cloned_colors = Vec::with_capacity(colors.len());
    unsafe {
        // Set length before copying to avoid reallocation
        cloned_colors.set_len(colors.len());
        // Fast memcpy of the entire buffer
        std::ptr::copy_nonoverlapping(
            colors.as_ptr(),
            cloned_colors.as_mut_ptr(),
            colors.len()
        );
    }
    let clone_time = clone_start.elapsed();
    
    let insert_start = std::time::Instant::now();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, cloned_colors);
    let insert_time = insert_start.elapsed();
    
    let total_time = start.elapsed();
    
    println!("[{:.3}s] Color update complete: Total={:.1}ms (buffer={:.1}MB, clone={:.1}ms, GPU upload={:.1}ms)", 
             time.elapsed_secs(),
             total_time.as_secs_f32() * 1000.0,
             buffer_size_mb,
             clone_time.as_secs_f32() * 1000.0,
             insert_time.as_secs_f32() * 1000.0);
}

/// Plugin that manages map overlay rendering
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ResourceOverlay>()
            .init_resource::<crate::resources::CachedOverlayColors>()
            .add_systems(PostStartup, precalculate_overlay_colors)
            .add_systems(Update, (
                handle_overlay_input,
                update_province_colors.run_if(resource_changed::<ResourceOverlay>),
            ));
    }
}

/// Handle overlay mode cycling input (M key)
pub fn handle_overlay_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_res: ResMut<ResourceOverlay>,
    time: Res<Time>,
) {
    // M key to cycle resource overlay modes
    if keyboard.just_pressed(KeyCode::KeyM) {
        let start = std::time::Instant::now();
        overlay_res.cycle();
        println!("[{:.3}s] M key pressed, switching to: {} (input lag: {:.1}ms)", 
                 time.elapsed_secs(),
                 overlay_res.display_name(),
                 start.elapsed().as_secs_f32() * 1000.0);
    }
}

/// Pre-calculate all overlay colors at startup for instant switching
pub fn precalculate_overlay_colors(
    province_storage: Res<ProvinceStorage>,
    mineral_storage: Res<crate::resources::MineralStorage>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
) {
    use crate::components::MineralType;
    
    println!("Pre-calculating overlay colors for {} provinces...", province_storage.provinces.len());
    let start = std::time::Instant::now();
    
    // Helper function to calculate colors for a specific overlay
    let calculate_overlay = |overlay_type: ResourceOverlay| -> Vec<[f32; 4]> {
        province_storage.provinces
            .par_iter()
            .flat_map(|province| {
                let resources = mineral_storage.resources
                    .get(province.id as usize)
                    .and_then(|r| r.as_ref());
                
                let province_color = match overlay_type {
                    ResourceOverlay::None => {
                        get_terrain_color_gradient(province.terrain, province.elevation)
                    },
                    ResourceOverlay::Mineral(mineral_type) => {
                        if province.terrain == TerrainType::Ocean {
                            get_terrain_color_gradient(province.terrain, province.elevation)
                        } else {
                            let abundance = resources
                                .map(|res| match mineral_type {
                                    MineralType::Iron => res.iron,
                                    MineralType::Copper => res.copper,
                                    MineralType::Tin => res.tin,
                                    MineralType::Gold => res.gold,
                                    MineralType::Coal => res.coal,
                                    MineralType::Stone => res.stone,
                                    MineralType::Gems => res.gems,
                                    _ => 0,
                                })
                                .unwrap_or(0);
                            if mineral_type == MineralType::Stone {
                                stone_abundance_color(abundance)
                            } else {
                                mineral_abundance_color(abundance)
                            }
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
                        get_terrain_color_gradient(province.terrain, province.elevation)
                    },
                };
                
                let linear = province_color.to_linear();
                let color_array = [linear.red, linear.green, linear.blue, linear.alpha];
                // Return a Vec with 7 copies for parallel collection
                vec![color_array; 7]
            })
            .collect()
    };
    
    // Pre-calculate all overlay colors in parallel
    cached_colors.terrain = calculate_overlay(ResourceOverlay::None);
    cached_colors.iron = calculate_overlay(ResourceOverlay::Mineral(MineralType::Iron));
    cached_colors.copper = calculate_overlay(ResourceOverlay::Mineral(MineralType::Copper));
    cached_colors.tin = calculate_overlay(ResourceOverlay::Mineral(MineralType::Tin));
    cached_colors.gold = calculate_overlay(ResourceOverlay::Mineral(MineralType::Gold));
    cached_colors.coal = calculate_overlay(ResourceOverlay::Mineral(MineralType::Coal));
    cached_colors.stone = calculate_overlay(ResourceOverlay::Mineral(MineralType::Stone));
    cached_colors.gems = calculate_overlay(ResourceOverlay::Mineral(MineralType::Gems));
    cached_colors.all_minerals = calculate_overlay(ResourceOverlay::AllMinerals);
    cached_colors.infrastructure = calculate_overlay(ResourceOverlay::Infrastructure);
    
    let elapsed = start.elapsed();
    println!("Pre-calculated all overlay colors in {:.2}s", elapsed.as_secs_f32());
}