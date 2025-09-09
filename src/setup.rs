// ============================================================================
// WORLD SETUP - Thin orchestrator that delegates to specialized modules
// ============================================================================

use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use bevy::render::mesh::Mesh2d;
use std::collections::HashMap;
use crate::components::Province;
use crate::resources::{WorldSeed, WorldSize, ProvincesSpatialIndex};
use crate::terrain::TerrainType;
use crate::generation::WorldGenerator;
use crate::clouds::spawn_clouds;
use crate::mesh::{ProvinceStorage, WorldMeshHandle, build_world_mesh};

// Mesh-related structs and functions moved to mesh.rs module

/// Main world setup system - orchestrates world generation and initialization
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
    
    // Delegate mesh building to the mesh module
    let mesh_handle = build_world_mesh(&generated_world.provinces, &mut meshes);
    
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
    // PREPARE PROVINCES
    // =========================================================================
    
    let provinces = generated_world.provinces.clone();
    // NATIONS DISABLED - Not ready for this feature yet
    // All provinces start with no nation - just the natural world
    
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