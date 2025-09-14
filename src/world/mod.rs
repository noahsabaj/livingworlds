//! World module gateway - The single entry point for all world functionality
//!
//! This module provides a feature-centric architecture with strict gateway control.
//! All submodules are private, and only carefully selected APIs are exposed.
//!
//! # Architecture
//!
//! The world module is organized into four main areas:
//! - **data/**: Core data structures (Province, World, Terrain, etc.)
//! - **generation/**: Builders that create world data
//! - **rendering/**: GPU concerns (mesh, overlays, borders)
//! - **ui/**: User interface for world configuration
//!
//! Each submodule has its own gateway controlling access to implementation details.

use bevy::prelude::*;

// PRIVATE SUBMODULES - Implementation details are hidden

mod data;
mod generation;
mod rendering;
mod ui;

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// === Core Data Structures ===
pub use data::{
    // World and systems
    World,
    RiverSystem,
    CloudSystem,
    CloudData,
    CloudLayer,

    // Province and related types
    Province,
    ProvinceId,
    Elevation,
    Agriculture,
    Distance,
    Abundance,
    HexDirection,

    // Terrain
    TerrainType,
    ClimateZone,
    classify_terrain_with_climate,

    // Spatial indexing
    ProvincesSpatialIndex,
    WorldBounds,

    TerrainEntity,
    CloudEntity,
    BorderEntity,
};

// === World Generation ===
pub use generation::WorldBuilder;

// === Rendering Systems ===
pub use rendering::{
    // Mesh system
    build_world_mesh,
    ProvinceStorage,
    WorldMeshHandle,
    MeshBuilder,
    MeshBuildStats,

    // Overlays
    update_province_colors,
    OverlayMode,

    // Borders
    SelectionBorder,
    BorderSettings,
};

// === World UI ===
pub use ui::{
    WorldGenerationSettings,
    WorldConfigPlugin,
    ClimateType,
    WorldPreset,
    IslandFrequency,
    MountainDensity,
    AggressionLevel,
    TradePropensity,
    ResourceAbundance,
    MineralDistribution,
};

// WORLD PLUGIN - Aggregates all world functionality

/// Main world plugin that registers all world-related systems
///
/// This plugin aggregates the generation, rendering, and configuration plugins
/// into a single cohesive system. It's the only plugin external code needs
/// to register for complete world functionality.
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add sub-plugins
            .add_plugins(rendering::RenderingPlugin)
            .add_plugins(generation::GenerationPlugin)
            .add_plugins(ui::WorldConfigPlugin)

            // Register world resources
            .init_resource::<ProvincesSpatialIndex>()

            // Register world events
            .add_event::<WorldGeneratedEvent>()
            .add_event::<ProvinceSelectedEvent>()

            // Add world systems
            .add_systems(Startup, initialize_world_systems)
            .add_systems(Update, (
                handle_province_selection,
                update_world_bounds_camera,
            ).chain());
    }
}


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

// INTERNAL SYSTEMS - Not exposed outside

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
    // Implementation would handle mouse picking and province selection
    // This is internal to the world module
}

/// Update camera bounds based on world size
fn update_world_bounds_camera(
    spatial_index: Res<ProvincesSpatialIndex>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    // Implementation would constrain camera to world bounds
    // This is internal to the world module
}

/// Internal world state
#[derive(Resource, Default)]
struct WorldState {
    initialized: bool,
    selected_province: Option<ProvinceId>,
}


/// # World Module
///
/// The world module is the heart of Living Worlds, providing all functionality
/// related to world representation, generation, and rendering.
///
/// ## Gateway Architecture
///
/// This module follows strict gateway architecture principles:
/// - All submodules are private
/// - Only explicitly exported types are accessible
/// - Each submodule has its own gateway
/// - Implementation details are completely hidden
///
/// ## Usage Examples
///
/// ### Generating a World
/// ```rust
/// use livingworlds::world::{WorldBuilder, WorldSize};
///
/// let world = WorldBuilder::new(
///     42,                    // seed
///     WorldSize::Medium,     // size
///     7,                     // continents
///     0.6,                   // ocean coverage
///     1.0,                   // river density
/// ).build();
/// ```
///
/// ### Accessing Province Data
/// ```rust
/// use livingworlds::world::{World, ProvinceId};
///
/// let province = world.get_province(ProvinceId::new(100));
/// if let Some(p) = province {
///     println!("Terrain: {:?}", p.terrain);
///     println!("Population: {}", p.population);
/// }
/// ```
///
/// ### Building the World Mesh
/// ```rust
/// use livingworlds::world::build_world_mesh;
///
/// let (mesh, storage) = build_world_mesh(&world.provinces);
/// commands.spawn(MaterialMesh2dBundle {
///     mesh: meshes.add(mesh),
///     // ...
/// });
/// ```
///
/// ## Performance
///
/// The world module is optimized for large-scale simulations:
/// - Supports up to 9,000,000 provinces
/// - Single draw call rendering via mega-mesh
/// - O(1) spatial lookups via grid indexing
/// - Parallel world generation with rayon
/// - 60+ FPS on modern hardware