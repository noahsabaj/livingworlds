//! World Setup - Bevy Integration Layer for World Generation
//!
//! This module provides the Bevy integration layer for world generation.
//! It uses WorldBuilder (pure generation logic) and integrates the result
//! with Bevy's ECS, rendering, and resource systems.
//!
//! # Architecture
//!
//! - **WorldBuilder**: Pure generation logic (world/generation/builder.rs)
//!   - Takes parameters → returns World data structure
//!   - No Bevy dependencies, could work in any context
//!   - Handles: terrain, rivers, climate, erosion, agriculture
//!
//! - **setup_world**: Bevy integration system (this file)
//!   - Uses WorldBuilder internally for generation
//!   - Handles: progress tracking, error handling, state transitions
//!   - Creates: rendering mesh, ECS resources, world entity
//!   - Manages: loading screens, error dialogs
//!
//! This separation allows the core generation to be reused in tests,
//! tools, or other contexts while keeping Bevy-specific concerns isolated.

use super::{build_world_mesh, ProvinceStorage, WorldBuilder, WorldMeshHandle};
use super::{BorderPlugin, CloudPlugin, OverlayPlugin, TerrainPlugin, WorldConfigPlugin};
use super::{ProvinceId, TerrainEntity, TerrainType, World};
use super::{ProvincesSpatialIndex, WorldGenerationSettings};
use crate::loading_screen::{set_loading_progress, LoadingState};
use crate::resources::{WorldGenerationError, WorldGenerationErrorType, WorldName, WorldSeed};
use crate::states::GameState;
use bevy::log::{debug, error, info};
use bevy::prelude::Vec2;
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::MeshMaterial2d;
use std::collections::HashMap;
use std::fmt;

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

// === EVENTS ===

/// Event fired when world generation completes
#[derive(Event)]
pub struct WorldGeneratedEvent {
    pub world: World,
    pub generation_time: std::time::Duration,
}

/// Event fired when a province is selected
#[derive(Event)]
pub struct ProvinceSelectedEvent {
    pub province_id: Option<ProvinceId>,
    pub position: Vec2,
}

// === INTERNAL STATE ===

/// Internal world state resource
#[derive(Resource, Default)]
struct WorldState {
    initialized: bool,
    selected_province: Option<ProvinceId>,
}

/// Bevy system that integrates WorldBuilder with ECS, rendering, and resources
///
/// This function serves as the bridge between pure world generation (WorldBuilder)
/// and Bevy's game engine systems. It:
///
/// 1. **Generation**: Uses WorldBuilder to create World data
/// 2. **Rendering**: Builds mega-mesh from world data
/// 3. **Resources**: Converts World into Bevy resources (ProvinceStorage, spatial index)
/// 4. **Entities**: Spawns the world mesh entity
/// 5. **Integration**: Handles progress tracking, error dialogs, state transitions
///
/// # Role Separation
/// - Pure generation logic → WorldBuilder (world/generation/builder.rs)
/// - Bevy integration → This function (setup_world)
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut loading_state: ResMut<LoadingState>,
) {
    let start_time = std::time::Instant::now();

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
                WorldSetupError::MeshBuildingFailed(_) => {
                    WorldGenerationErrorType::MeshBuildingFailed
                }
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

    let mesh_handle = build_rendering_mesh(&world, meshes, loading_state)?;

    // Setup all resources
    setup_world_resources(commands, world, mesh_handle, materials, settings)?;

    // Finalize setup
    finalize_setup(loading_state);

    Ok(())
}

/// Validates world generation settings
fn validate_settings(settings: &WorldGenerationSettings) -> Result<(), WorldSetupError> {
    // Validate ocean coverage
    if !(MIN_OCEAN_COVERAGE..=MAX_OCEAN_COVERAGE).contains(&settings.ocean_coverage) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Ocean coverage must be between {} and {}",
            MIN_OCEAN_COVERAGE, MAX_OCEAN_COVERAGE
        )));
    }

    // Validate continent count
    if !(MIN_CONTINENTS..=MAX_CONTINENTS).contains(&settings.continent_count) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Continent count must be between {} and {}",
            MIN_CONTINENTS, MAX_CONTINENTS
        )));
    }

    // Validate river density
    if !(MIN_RIVER_DENSITY..=MAX_RIVER_DENSITY).contains(&settings.river_density) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "River density must be between {} and {}",
            MIN_RIVER_DENSITY, MAX_RIVER_DENSITY
        )));
    }

    // Validate world name
    if settings.world_name.is_empty() {
        return Err(WorldSetupError::InvalidSettings(
            "World name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Delegates to WorldBuilder for pure generation logic
///
/// This function wraps WorldBuilder.build() with Bevy-specific concerns
/// like progress tracking. The actual generation is handled by WorldBuilder.
fn generate_world_data(
    settings: &WorldGenerationSettings,
    loading_state: &mut LoadingState,
) -> Result<World, WorldSetupError> {
    info!(
        "Generating world '{}' with seed {} and size {:?}",
        settings.world_name, settings.seed, settings.world_size
    );
    debug!(
        "Advanced settings: {} continents, {:.0}% ocean, {:?} climate",
        settings.continent_count,
        settings.ocean_coverage * 100.0,
        settings.climate_type
    );

    let builder = WorldBuilder::new(
        settings.seed,
        settings.world_size.clone(),
        settings.continent_count,
        settings.ocean_coverage,
        settings.river_density,
    );

    set_loading_progress(
        loading_state,
        PROGRESS_TERRAIN,
        "Generating terrain and climate...",
    );

    // WorldBuilder now handles: provinces, erosion, ocean depths, climate, rivers, agriculture, clouds
    // It's silent - all progress reporting happens here in the Bevy integration layer
    let generation_start = std::time::Instant::now();
    let world = builder.build();
    let generation_time = generation_start.elapsed().as_secs_f32();

    info!(
        "World generation completed in {:.2}s - {} provinces, {} rivers",
        generation_time,
        world.provinces.len(),
        world.rivers.len()
    );

    Ok(world)
}

/// Builds the mega-mesh for rendering
fn build_rendering_mesh(
    world: &World,
    meshes: &mut ResMut<Assets<Mesh>>,
    loading_state: &mut LoadingState,
) -> Result<Handle<Mesh>, WorldSetupError> {
    info!(
        "Building mega-mesh with {} hexagons...",
        world.provinces.len()
    );
    let mesh_start = std::time::Instant::now();

    set_loading_progress(loading_state, PROGRESS_MESH, "Building world mesh...");

    // Delegate mesh building to the mesh module
    let mesh_handle = build_world_mesh(&world.provinces, meshes);

    debug!(
        "Mega-mesh built in {:.2}s - ONE entity instead of {}!",
        mesh_start.elapsed().as_secs_f32(),
        world.provinces.len()
    );

    Ok(mesh_handle)
}

/// Sets up all world resources
fn setup_world_resources(
    commands: &mut Commands,
    world: World,
    mesh_handle: Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    settings: &WorldGenerationSettings,
) -> Result<(), WorldSetupError> {
    set_loading_progress(
        &mut LoadingState::default(),
        PROGRESS_ENTITIES,
        "Setting up world resources...",
    );

    // Store world seed and name as resources
    commands.insert_resource(WorldSeed(settings.seed));
    commands.insert_resource(WorldName(settings.world_name.clone()));

    // Store map dimensions based on world size
    let map_dimensions = crate::resources::MapDimensions::from_world_size(&settings.world_size);
    commands.insert_resource(map_dimensions);

    let total_provinces = world.provinces.len();
    let land_count = world
        .provinces
        .iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .count();

    info!(
        "Generated world with {} provinces, {} land tiles",
        total_provinces, land_count
    );

    // Now simplified - no Entity needed since provinces are data, not entities
    let mut spatial_index = ProvincesSpatialIndex::default();
    for province in &world.provinces {
        spatial_index.insert(province.position, province.id.value());
    }
    commands.insert_resource(spatial_index);

    let province_by_id: HashMap<ProvinceId, usize> = world
        .provinces
        .iter()
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

    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
        ViewVisibility::default(),
        InheritedVisibility::default(),
        Name::new("World Mega-Mesh"),
        TerrainEntity,
    ));

    Ok(())
}

/// Finalizes the setup process
fn finalize_setup(loading_state: &mut LoadingState) {
    set_loading_progress(
        loading_state,
        PROGRESS_COMPLETE,
        "World generation complete!",
    );
}

// === WORLD PLUGIN - Main Bevy integration ===

/// Main world plugin that registers all world-related systems
///
/// This plugin aggregates all world functionality into Bevy.
/// It's the ONLY place where world systems are registered with the app.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add feature plugins
            .add_plugins(CloudPlugin)
            .add_plugins(TerrainPlugin)
            .add_plugins(BorderPlugin)
            .add_plugins(OverlayPlugin)
            .add_plugins(WorldConfigPlugin)
            // Register world resources
            .init_resource::<ProvincesSpatialIndex>()
            .init_resource::<WorldState>()
            // Register world events
            .add_event::<WorldGeneratedEvent>()
            .add_event::<ProvinceSelectedEvent>()
            // Add world systems
            .add_systems(Startup, initialize_world_systems)
            .add_systems(
                Update,
                (handle_province_selection, update_world_bounds_camera).chain(),
            );
    }
}

// === WORLD SYSTEMS - Internal Bevy systems ===

/// Initialize world systems on startup
fn initialize_world_systems(mut commands: Commands) {
    info!("World systems initialized");

    // Initialize any world-specific resources
    commands.insert_resource(WorldState::default());
}

/// Handle province selection from mouse input
fn handle_province_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    spatial_index: Res<ProvincesSpatialIndex>,
    mut selection_events: EventWriter<ProvinceSelectedEvent>,
) {
    // This is where mouse picking and province selection would be implemented
    // Keeping it internal to the setup module as it's Bevy-specific
}

/// Update camera bounds based on world size
fn update_world_bounds_camera(
    spatial_index: Res<ProvincesSpatialIndex>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    // This would constrain camera to world bounds
    // Keeping it internal to the setup module as it's Bevy-specific
}
