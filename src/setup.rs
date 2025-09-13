// ============================================================================
// WORLD SETUP - Robust orchestrator with error handling and validation
// ============================================================================

use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;
use bevy::render::mesh::Mesh2d;
use bevy::log::{info, debug, error};
use std::collections::HashMap;
use std::fmt;
use crate::resources::{ProvincesSpatialIndex, WorldSeed, WorldName, WorldGenerationError, WorldGenerationErrorType};
use crate::world::terrain::TerrainType;
use crate::generation::WorldBuilder;
use crate::world::mesh::{ProvinceStorage, WorldMeshHandle, build_world_mesh};
use crate::components::ProvinceId;
use crate::world::config::WorldGenerationSettings;
use crate::states::GameState;
use crate::loading_screen::{LoadingState, set_loading_progress};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Loading progress milestones
const PROGRESS_TERRAIN: f32 = 0.1;
const PROGRESS_MESH: f32 = 0.6;
const PROGRESS_ENTITIES: f32 = 0.8;
const PROGRESS_COMPLETE: f32 = 1.0;

/// Validation bounds
const MAX_CONTINENTS: u32 = 100;
const MIN_CONTINENTS: u32 = 1;
const MAX_OCEAN_COVERAGE: f32 = 0.95;
const MIN_OCEAN_COVERAGE: f32 = 0.05;
const MAX_RIVER_DENSITY: f32 = 1.0;
const MIN_RIVER_DENSITY: f32 = 0.0;

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Custom error type for world setup failures
#[derive(Debug)]
pub enum WorldSetupError {
    /// Invalid settings provided
    InvalidSettings(String),
    /// World generation failed
    GenerationFailed(String),
    /// Mesh building failed
    MeshBuildingFailed(String),
    /// Empty world generated
    EmptyWorld,
    /// Resource insertion failed
    ResourceError(String),
}

impl fmt::Display for WorldSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSettings(msg) => write!(f, "Invalid settings: {}", msg),
            Self::GenerationFailed(msg) => write!(f, "World generation failed: {}", msg),
            Self::MeshBuildingFailed(msg) => write!(f, "Mesh building failed: {}", msg),
            Self::EmptyWorld => write!(f, "Generated world has no provinces"),
            Self::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

impl std::error::Error for WorldSetupError {}

// ============================================================================
// MAIN SETUP FUNCTION
// ============================================================================

/// Main world setup system - orchestrates world generation with full error handling
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut loading_state: ResMut<LoadingState>,
) {
    let start_time = std::time::Instant::now();
    
    // Run setup with error handling
    match setup_world_internal(
        &mut commands,
        &mut meshes,
        &mut materials,
        &settings,
        &mut loading_state,
    ) {
        Ok(()) => {
            let total_time = start_time.elapsed().as_secs_f32();
            info!("World setup completed successfully in {:.2}s", total_time);
            
            // Clear the pending generation flag
            commands.insert_resource(crate::states::PendingWorldGeneration {
                pending: false,
                delay_timer: 0.0,
            });
            
            // Transition to InGame
            next_state.set(GameState::InGame);
        }
        Err(e) => {
            error!("World setup failed: {}", e);
            
            // Store the error information for the error dialog
            let error_type = match &e {
                WorldSetupError::InvalidSettings(_) => WorldGenerationErrorType::InvalidSettings,
                WorldSetupError::GenerationFailed(_) => WorldGenerationErrorType::GenerationFailed,
                WorldSetupError::MeshBuildingFailed(_) => WorldGenerationErrorType::MeshBuildingFailed,
                WorldSetupError::EmptyWorld => WorldGenerationErrorType::EmptyWorld,
                WorldSetupError::ResourceError(_) => WorldGenerationErrorType::ResourceError,
            };
            
            commands.insert_resource(WorldGenerationError {
                error_message: e.to_string(),
                error_type,
            });
            
            // Transition to error state to show dialog
            next_state.set(GameState::WorldGenerationFailed);
        }
    }
}

// ============================================================================
// INTERNAL SETUP IMPLEMENTATION
// ============================================================================

/// Internal setup implementation with Result return type
fn setup_world_internal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &WorldGenerationSettings,
    loading_state: &mut LoadingState,
) -> Result<(), WorldSetupError> {
    // Validate settings first
    validate_settings(settings)?;
    
    // Generate world data
    let world = generate_world_data(settings, loading_state)?;
    
    // Validate generated world
    if world.provinces.is_empty() {
        return Err(WorldSetupError::EmptyWorld);
    }
    
    // Build rendering mesh
    let mesh_handle = build_rendering_mesh(&world, meshes, loading_state)?;
    
    // Setup all resources
    setup_world_resources(commands, world, mesh_handle, materials, settings)?;
    
    // Finalize setup
    finalize_setup(loading_state);
    
    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Validates world generation settings
fn validate_settings(settings: &WorldGenerationSettings) -> Result<(), WorldSetupError> {
    // Validate ocean coverage
    if !(MIN_OCEAN_COVERAGE..=MAX_OCEAN_COVERAGE).contains(&settings.ocean_coverage) {
        return Err(WorldSetupError::InvalidSettings(
            format!("Ocean coverage must be between {} and {}", MIN_OCEAN_COVERAGE, MAX_OCEAN_COVERAGE)
        ));
    }
    
    // Validate continent count
    if !(MIN_CONTINENTS..=MAX_CONTINENTS).contains(&settings.continent_count) {
        return Err(WorldSetupError::InvalidSettings(
            format!("Continent count must be between {} and {}", MIN_CONTINENTS, MAX_CONTINENTS)
        ));
    }
    
    // Validate river density
    if !(MIN_RIVER_DENSITY..=MAX_RIVER_DENSITY).contains(&settings.river_density) {
        return Err(WorldSetupError::InvalidSettings(
            format!("River density must be between {} and {}", MIN_RIVER_DENSITY, MAX_RIVER_DENSITY)
        ));
    }
    
    // Validate world name
    if settings.world_name.is_empty() {
        return Err(WorldSetupError::InvalidSettings(
            "World name cannot be empty".to_string()
        ));
    }
    
    Ok(())
}

/// Generates world data using the WorldBuilder
fn generate_world_data(
    settings: &WorldGenerationSettings,
    loading_state: &mut LoadingState,
) -> Result<crate::world::data::World, WorldSetupError> {
    info!("Generating world '{}' with seed {} and size {:?}", 
        settings.world_name, settings.seed, settings.world_size);
    debug!("Advanced settings: {} continents, {:.0}% ocean, {:?} climate",
        settings.continent_count, settings.ocean_coverage * 100.0, settings.climate_type);
    
    let builder = WorldBuilder::new(
        settings.seed, 
        settings.world_size.clone(),
        settings.continent_count,
        settings.ocean_coverage,
        settings.river_density,
    );
    
    set_loading_progress(loading_state, PROGRESS_TERRAIN, "Generating terrain...");
    
    // Generate world - in the future, builder.build() should return Result
    let world = builder.build();
    
    Ok(world)
}

/// Builds the mega-mesh for rendering
fn build_rendering_mesh(
    world: &crate::world::data::World,
    meshes: &mut ResMut<Assets<Mesh>>,
    loading_state: &mut LoadingState,
) -> Result<Handle<Mesh>, WorldSetupError> {
    info!("Building mega-mesh with {} hexagons...", world.provinces.len());
    let mesh_start = std::time::Instant::now();
    
    set_loading_progress(loading_state, PROGRESS_MESH, "Building world mesh...");
    
    // Delegate mesh building to the mesh module
    let mesh_handle = build_world_mesh(&world.provinces, meshes);
    
    debug!("Mega-mesh built in {:.2}s - ONE entity instead of {}!", 
             mesh_start.elapsed().as_secs_f32(), world.provinces.len());
    
    Ok(mesh_handle)
}

/// Sets up all world resources
fn setup_world_resources(
    commands: &mut Commands,
    world: crate::world::data::World,
    mesh_handle: Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &WorldGenerationSettings,
) -> Result<(), WorldSetupError> {
    set_loading_progress(&mut LoadingState::default(), PROGRESS_ENTITIES, "Setting up world resources...");
    
    // Store world seed and name as resources
    commands.insert_resource(WorldSeed(settings.seed));
    commands.insert_resource(WorldName(settings.world_name.clone()));
    
    // Store map dimensions from the generated world (single source of truth)
    commands.insert_resource(world.map_dimensions.clone());
    
    // Calculate statistics
    let total_provinces = world.provinces.len();
    let land_count = world.provinces.iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .count();
    
    info!("Generated world with {} provinces, {} land tiles", 
             total_provinces, land_count);
    
    // Build spatial index for mega-mesh architecture
    // Now simplified - no Entity needed since provinces are data, not entities
    let mut spatial_index = ProvincesSpatialIndex::default();
    for province in &world.provinces {
        spatial_index.insert(province.position, province.id.value());
    }
    commands.insert_resource(spatial_index);
    
    // Build HashMap for O(1) province lookups by ID
    let province_by_id: HashMap<ProvinceId, usize> = world.provinces.iter()
        .enumerate()
        .map(|(idx, p)| (p.id, idx))
        .collect();
    
    // Store provinces (move ownership, no clone)
    commands.insert_resource(ProvinceStorage {
        provinces: world.provinces,
        province_by_id,
    });
    
    // Store cloud data (move, not clone)
    commands.insert_resource(world.clouds);
    
    // Store mesh handle for overlay system
    commands.insert_resource(WorldMeshHandle(mesh_handle.clone()));
    
    // Spawn the world as a single entity
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
        Name::new("World Mega-Mesh"),
        crate::world::TerrainEntity,
    ));
    
    Ok(())
}

/// Finalizes the setup process
fn finalize_setup(loading_state: &mut LoadingState) {
    set_loading_progress(loading_state, PROGRESS_COMPLETE, "World generation complete!");
}