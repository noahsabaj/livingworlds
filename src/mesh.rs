//! Mesh building and management for the mega-mesh rendering architecture
//! 
//! This module handles all mesh construction, including the creation of the
//! single world mesh containing all hexagonal provinces and texture generation.

use bevy::prelude::*;
use bevy::reflect::Reflect;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::ImageSampler;
use std::collections::HashMap;
use crate::components::Province;
use crate::colors::get_terrain_color_gradient;
use crate::constants::*;

// ============================================================================
// RESOURCES
// ============================================================================

/// Storage for province data and mesh handle for the mega-mesh architecture
#[derive(Resource, Reflect, Clone)]
pub struct ProvinceStorage {
    pub provinces: Vec<Province>,
    pub province_by_id: HashMap<u32, usize>,  // Maps province ID to index in provinces Vec for O(1) lookups
    pub mesh_handle: Handle<Mesh>,
}

/// Handle to the world mega-mesh for overlay system access
#[derive(Resource)]
pub struct WorldMeshHandle(pub Handle<Mesh>);

// ============================================================================
// MESH BUILDING
// ============================================================================

/// Build the world mega-mesh containing all provinces as a single mesh
/// Returns the mesh handle for storage and future updates
pub fn build_world_mesh(
    provinces: &[Province],
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    let hex_size = HEX_SIZE_PIXELS;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    
    // Generate vertices for each province hexagon
    for province in provinces {
        let base_idx = vertices.len() as u32;
        
        // Center vertex
        vertices.push([province.position.x, province.position.y, 0.0]);
        
        // 6 corner vertices for flat-top hexagon
        for i in 0..6 {
            let angle = i as f32 * 60.0_f32.to_radians();
            let x = province.position.x + hex_size * angle.cos();
            let y = province.position.y + hex_size * angle.sin();
            vertices.push([x, y, 0.0]);
        }
        
        // Create triangles (6 triangles per hexagon)
        for i in 0..6 {
            let next = (i + 1) % 6;
            indices.push(base_idx);           // Center
            indices.push(base_idx + i + 1);   // Current corner
            indices.push(base_idx + next + 1); // Next corner
        }
        
        // Assign colors based on terrain
        let color = get_terrain_color_gradient(province.terrain, province.elevation.value());
        let rgba = color.to_linear().to_f32_array();
        for _ in 0..7 {
            colors.push(rgba);
        }
    }
    
    // Create the mega-mesh with CPU access for overlay updates
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
    
    meshes.add(mesh)
}

// ============================================================================
// TEXTURE GENERATION
// ============================================================================

/// Create an antialiased hexagon texture for individual province rendering (if needed)
pub fn create_hexagon_texture(size: f32) -> Image {
    let texture_size = (size * 2.0) as u32;
    let mut data = vec![0u8; (texture_size * texture_size * 4) as usize];
    let sqrt3 = 3.0_f32.sqrt();
    
    for y in 0..texture_size {
        for x in 0..texture_size {
            let fx = x as f32 - texture_size as f32 / 2.0;
            let fy = y as f32 - texture_size as f32 / 2.0;
            
            // For flat-top hexagon, swap the x and y in calculations
            let abs_x = fx.abs();
            let abs_y = fy.abs();
            
            // FLAT-TOP hexagon bounds checking
            // Maximum extent in Y direction (height/2)
            let max_y = size * sqrt3 / 2.0;
            
            // Distance from hexagon edge (flat-top orientation)
            let dist_vertical = abs_y - max_y;  // Distance from horizontal edges
            let dist_diagonal = (sqrt3 * abs_x + abs_y) / 2.0 - size;  // Distance from diagonal edges
            let distance_from_edge = dist_vertical.max(dist_diagonal);
            
            // Antialiasing
            let aa_width = 1.5;  // Width of the antialiasing border in pixels
            let alpha = if distance_from_edge < -aa_width {
                1.0  // Fully inside
            } else if distance_from_edge > aa_width {
                0.0  // Fully outside
            } else {
                // Smooth transition
                1.0 - (distance_from_edge + aa_width) / (2.0 * aa_width)
            };
            
            let idx = ((y * texture_size + x) * 4) as usize;
            let alpha_byte = (alpha * 255.0) as u8;
            data[idx] = 255;        // R
            data[idx + 1] = 255;    // G
            data[idx + 2] = 255;    // B
            data[idx + 3] = alpha_byte; // A
        }
    }
    
    let mut image = Image::new(
        Extent3d {
            width: texture_size,
            height: texture_size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    
    // Use linear filtering for smoother edges
    image.sampler = ImageSampler::linear();
    
    image
}