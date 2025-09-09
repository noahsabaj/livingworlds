// ============================================================================
// WORLD SETUP USING GENERATION MODULE
// ============================================================================

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::ImageSampler;
use bevy::sprite::{MeshMaterial2d};
use bevy::render::mesh::Mesh2d;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;
use crate::components::{Province, Nation};
use crate::resources::{WorldSeed, WorldSize, ProvincesSpatialIndex, SelectedProvinceInfo};
use crate::terrain::TerrainType;
use crate::colors::get_terrain_color_gradient;
use crate::constants::*;
use crate::generation::{WorldGenerator, MapDimensions};
use crate::clouds::spawn_clouds;

// Province storage for efficient updates
#[derive(Resource)]
pub struct ProvinceStorage {
    pub provinces: Vec<Province>,
    pub mesh_handle: Handle<Mesh>,
}

// Resource to hold the world mesh handle
#[derive(Resource)]
pub struct WorldMeshHandle(pub Handle<Mesh>);

/// Create a hexagon texture (reused from original)
pub fn create_hexagon_texture(size: f32) -> Image {
    let image_size = (size * 2.0) as u32;
    let mut pixels = vec![0u8; (image_size * image_size * 4) as usize];
    
    for y in 0..image_size {
        for x in 0..image_size {
            let idx = ((y * image_size + x) * 4) as usize;
            
            // Calculate distance from center
            let cx = x as f32 - image_size as f32 / 2.0;
            let cy = y as f32 - image_size as f32 / 2.0;
            let abs_x = cx.abs();
            let abs_y = cy.abs();
            
            // Flat-top hexagon distance calculation
            let sqrt3 = 1.732050808;
            let radius = size;
            
            // Check if point is inside hexagon
            let dist_vertical = abs_x - radius * sqrt3 / 2.0;
            let dist_diagonal = (sqrt3 * abs_y + abs_x) / sqrt3 - radius;
            let distance_from_edge = dist_vertical.max(dist_diagonal);
            
            // Smooth antialiasing
            let aa_width = 1.5;
            let alpha = if distance_from_edge < -aa_width {
                255
            } else if distance_from_edge > aa_width {
                0
            } else {
                ((1.0 - (distance_from_edge + aa_width) / (2.0 * aa_width)) * 255.0) as u8
            };
            
            pixels[idx] = 255;     // R
            pixels[idx + 1] = 255; // G
            pixels[idx + 2] = 255; // B
            pixels[idx + 3] = alpha; // A
        }
    }
    
    let mut image = Image::new(
        Extent3d {
            width: image_size,
            height: image_size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    image.sampler = ImageSampler::linear();
    image
}

/// Simplified world setup using the generation module
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<WorldSeed>,
    size: Res<WorldSize>,
) {
    let start_time = std::time::Instant::now();
    
    // =========================================================================
    // GENERATE WORLD DATA
    // =========================================================================
    
    println!("Generating world with seed {} and size {:?}", seed.0, *size);
    let generator = WorldGenerator::new(seed.0, size.clone());
    let generated_world = generator.generate();
    
    // Store map dimensions as resource
    commands.insert_resource(generated_world.map_dimensions);
    
    // =========================================================================
    // BUILD MEGA-MESH FOR RENDERING
    // =========================================================================
    
    println!("Building mega-mesh with {} hexagons...", generated_world.provinces.len());
    let mesh_start = std::time::Instant::now();
    
    let hex_size = HEX_SIZE_PIXELS;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    
    // Generate vertices for each province hexagon
    for province in &generated_world.provinces {
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
        let color = get_terrain_color_gradient(province.terrain, province.elevation);
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
    
    let mesh_handle = meshes.add(mesh);
    
    // Spawn the world as a single entity
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
        Name::new("World Mega-Mesh"),
    ));
    
    println!("Mega-mesh built in {:.2}s - ONE entity instead of 135,000!", 
             mesh_start.elapsed().as_secs_f32());
    
    // =========================================================================
    // ASSIGN NATIONS
    // =========================================================================
    
    let nation_start = std::time::Instant::now();
    let mut provinces = generated_world.provinces.clone();
    let mut rng = StdRng::seed_from_u64(seed.0 as u64);
    
    // Find suitable nation spawn points
    let land_provinces: Vec<_> = provinces.iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .map(|p| p.id)
        .collect();
    
    if !land_provinces.is_empty() {
        let num_nations = NATION_COUNT.min(land_provinces.len());
        let mut nation_spawns = Vec::new();
        
        for i in 0..num_nations {
            let spawn_idx = rng.gen_range(0..land_provinces.len());
            let spawn_id = land_provinces[spawn_idx];
            nation_spawns.push((i as u32, spawn_id));
            
            // Spawn nation entity
            commands.spawn((
                Nation {
                    id: i as u32,
                    name: format!("Nation {}", i + 1),
                    color: Color::hsl(
                        (i as f32 / num_nations as f32) * 360.0,
                        0.7,
                        0.5,
                    ),
                },
                Name::new(format!("Nation {}", i + 1)),
            ));
        }
        
        // Collect spawn positions before the mutable loop
        let spawn_positions: Vec<(u32, Vec2)> = nation_spawns.iter()
            .filter_map(|&(nation_id, spawn_id)| {
                provinces.iter()
                    .find(|p| p.id == spawn_id)
                    .map(|p| (nation_id, p.position))
            })
            .collect();
        
        // Simple distance-based assignment
        for province in &mut provinces {
            if province.terrain != TerrainType::Ocean {
                let mut min_dist = f32::MAX;
                let mut closest_nation = 0;
                
                for &(nation_id, spawn_pos) in &spawn_positions {
                    let dist = province.position.distance(spawn_pos);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_nation = nation_id;
                    }
                }
                
                province.nation_id = Some(closest_nation);
            }
        }
    }
    
    println!("Nation assignment completed in {:.2}s", nation_start.elapsed().as_secs_f32());
    
    // =========================================================================
    // SPAWN CLOUDS
    // =========================================================================
    
    let cloud_start = std::time::Instant::now();
    let map_width = generated_world.map_dimensions.bounds.x_max - generated_world.map_dimensions.bounds.x_min;
    let map_height = generated_world.map_dimensions.bounds.y_max - generated_world.map_dimensions.bounds.y_min;
    spawn_clouds(&mut commands, &mut images, seed.0, map_width, map_height);
    println!("Cloud generation completed in {:.2}s", cloud_start.elapsed().as_secs_f32());
    
    // =========================================================================
    // STORE RESOURCES
    // =========================================================================
    
    // Store provinces for later access
    commands.insert_resource(ProvinceStorage {
        provinces: provinces.clone(),
        mesh_handle: mesh_handle.clone(),
    });
    
    // Store mesh handle for overlay system
    commands.insert_resource(WorldMeshHandle(mesh_handle));
    
    // Initialize spatial index with mega-mesh architecture (no entities per province)
    let mut spatial_index = ProvincesSpatialIndex::default();
    
    // Build HashMap for O(1) province lookups by ID
    let province_by_id: HashMap<u32, &Province> = provinces.iter()
        .map(|p| (p.id, p))
        .collect();
    
    for (grid_pos, province_id) in generated_world.spatial_index {
        // In mega-mesh architecture, we use Entity::PLACEHOLDER since provinces aren't entities
        // The province_id is what matters for lookups
        if let Some(province) = province_by_id.get(&province_id) {
            let entry = spatial_index.grid.entry(grid_pos).or_insert_with(Vec::new);
            entry.push((Entity::PLACEHOLDER, province.position, province_id));
        }
    }
    commands.insert_resource(spatial_index);
    
    // Store minerals in the MineralStorage resource for overlay access
    let mineral_storage = crate::resources::MineralStorage {
        resources: generated_world.minerals,
    };
    commands.insert_resource(mineral_storage);
    
    // Initialize empty infrastructure storage (will be populated during gameplay)
    commands.insert_resource(crate::resources::InfrastructureStorage::default());
    
    let land_count = provinces.iter().filter(|p| p.terrain != TerrainType::Ocean).count();
    println!("Generated world with {} provinces, {} land tiles", 
             provinces.len(), land_count);
    
    let total_time = start_time.elapsed().as_secs_f32();
    println!("Total setup_world completed in {:.2}s", total_time);
}