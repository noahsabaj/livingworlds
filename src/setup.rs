// ============================================================================
// WORLD SETUP - Thin orchestrator that delegates to specialized modules
// ============================================================================

use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use bevy::render::mesh::Mesh2d;
use std::collections::HashMap;
use crate::resources::{ProvincesSpatialIndex};
use crate::terrain::TerrainType;
use crate::generation::WorldGenerator;
use crate::mesh::{ProvinceStorage, WorldMeshHandle, build_world_mesh};
use crate::world_config::WorldGenerationSettings;
use crate::states::GameState;

// Mesh-related structs and functions moved to mesh.rs module

/// Main world setup system - orchestrates world generation and initialization
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let start_time = std::time::Instant::now();
    
    // =========================================================================
    // GENERATE WORLD DATA
    // =========================================================================
    
    println!("Generating world '{}' with seed {} and size {:?}", 
        settings.world_name, settings.seed, settings.world_size);
    println!("Advanced settings: {} continents, {:.0}% ocean, {:?} climate",
        settings.continent_count, settings.ocean_coverage * 100.0, settings.climate_type);
    
    let generator = WorldGenerator::new(
        settings.seed, 
        settings.world_size.clone(),
        settings.continent_count,
        settings.ocean_coverage,
        settings.river_density,
    );
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
        crate::components::GameWorld,  // Mark as game world entity
    ));
    
    println!("Mega-mesh built in {:.2}s - ONE entity instead of 900,000!", 
             mesh_start.elapsed().as_secs_f32());
    
    // =========================================================================
    // PREPARE PROVINCES
    // =========================================================================
    
    // NATIONS DISABLED - Not ready for this feature yet
    // All provinces start with no nation - just the natural world
    
    // =========================================================================
    // STORE CLOUD DATA
    // =========================================================================
    
    // Store cloud data for the clouds module to spawn entities from
    commands.insert_resource(generated_world.clouds.clone());
    
    // =========================================================================
    // STORE RESOURCES
    // =========================================================================
    
    // Calculate statistics before moving provinces
    let total_provinces = generated_world.provinces.len();
    let land_count = generated_world.provinces.iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .count();
    
    // Build HashMap for O(1) province position lookups (avoiding O(n²) bug!)
    let province_positions: HashMap<u32, Vec2> = generated_world.provinces.iter()
        .map(|p| (p.id, p.position))
        .collect();
    
    // Build spatial index using the HashMap for fast lookups
    let mut spatial_index = ProvincesSpatialIndex::default();
    for (grid_pos, province_id) in generated_world.spatial_index {
        // In mega-mesh architecture, we use Entity::PLACEHOLDER since provinces aren't entities
        // The province_id is what matters for lookups
        if let Some(&position) = province_positions.get(&province_id) {
            let entry = spatial_index.grid.entry(grid_pos).or_insert_with(Vec::new);
            entry.push((Entity::PLACEHOLDER, position, province_id));
        }
    }
    commands.insert_resource(spatial_index);
    
    // Build HashMap for O(1) province lookups by ID (for UI and selection systems)
    let province_by_id: HashMap<u32, usize> = generated_world.provinces.iter()
        .enumerate()
        .map(|(idx, p)| (p.id, idx))
        .collect();
    
    // Get total provinces count before moving
    let total_provinces = generated_world.provinces.len();
    
    // Store provinces for later access - MOVE ownership instead of cloning!
    commands.insert_resource(ProvinceStorage {
        provinces: generated_world.provinces,  // Move, not clone - saves 36MB!
        province_by_id,  // O(1) lookups for UI/selection
        mesh_handle: mesh_handle.clone(),
    });
    
    // Store mesh handle for overlay system
    commands.insert_resource(WorldMeshHandle(mesh_handle));
    
    // Store minerals in the MineralStorage resource for overlay access
    // Convert HashMap to Vec for O(1) indexed access
    let mut mineral_vec = vec![None; total_provinces];
    for (province_id, resources) in generated_world.minerals {
        if (province_id as usize) < total_provinces {
            mineral_vec[province_id as usize] = Some(resources);
        }
    }
    let mineral_storage = crate::resources::MineralStorage {
        resources: mineral_vec,
    };
    commands.insert_resource(mineral_storage);
    
    println!("Generated world with {} provinces, {} land tiles", 
             total_provinces, land_count);
    
    let total_time = start_time.elapsed().as_secs_f32();
    println!("Total setup_world completed in {:.2}s", total_time);
    
    // Transition to LoadingWorld state
    next_state.set(GameState::LoadingWorld);
}