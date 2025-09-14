//! World module gateway - The single entry point for all world functionality
//!
//! This is a PURE GATEWAY following strict gateway architecture principles.
//! NO IMPLEMENTATION CODE HERE - only module organization and exports.
//!
//! # Architecture
//!
//! The world module uses feature-based organization:
//! - Each feature (clouds, terrain, rivers, etc.) contains ALL related code
//! - `core.rs` contains shared orchestration types (World struct)
//! - `setup.rs` handles ALL Bevy integration
//! - `generation/` orchestrates world creation using feature builders
//!
//! # Module Responsibilities
//!
//! - **core.rs**: Pure data structures (World) that orchestrate features
//! - **setup.rs**: ALL Bevy integration (Plugin, systems, resources, entities)
//! - **generation/**: Orchestrates builders from feature modules
//! - **Feature modules**: Self-contained functionality (data + logic + rendering)

// PRIVATE FEATURE MODULES - Implementation details are hidden

mod clouds;     // Cloud system (data, generation, rendering)
mod colors;     // Color system (themes, providers, calculations)
mod terrain;    // Terrain types, climate, erosion
mod provinces;  // Province data, spatial indexing, agriculture
mod rivers;     // River systems and flow
mod minerals;   // Mineral resources
mod mesh;       // World mesh rendering
mod borders;    // Border rendering
mod overlay;    // Overlay rendering modes

// Non-feature modules
mod core;        // Core world data structures (World)
mod generation;  // World generation orchestrator
mod simulation;  // Simulation events
mod ui;          // World configuration UI
mod setup;       // Bevy integration layer (Plugin, systems, resources)

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// === Core Data Structure ===
pub use core::{
    World,
    WorldSeed,
    WorldName,
    WorldSize,
    MapDimensions,
    MapBounds,
};

// === Clouds Feature ===
pub use clouds::{
    CloudSystem,
    CloudData,
    CloudLayer,
    CloudEntity,
    CloudBuilder,
    CloudPlugin,
    CloudSprite,
    CloudFormationType,
    generate_cloud_formation,
    create_cloud_texture,
    CloudTextureParams,
    animate_clouds,
    update_weather_system,
    dynamic_cloud_spawn_system,
    WeatherState,
    WeatherSystem,
};

// === Terrain Feature ===
pub use terrain::{
    TerrainEntity,
    TerrainType,
    ClimateZone,
    Biome,
    classify_terrain_with_climate,
    TerrainPlugin,
    apply_erosion_to_provinces,
    apply_climate_to_provinces,
};

// === Provinces Feature ===
pub use provinces::{
    Province,
    ProvinceId,
    Elevation,
    Agriculture,
    Distance,
    Abundance,
    HexDirection,
    ProvincesSpatialIndex,
    WorldBounds,
    ProvinceBuilder,
    calculate_agriculture_values,
    calculate_ocean_depths,
};

// === Rivers Feature ===
pub use rivers::{
    RiverSystem,
    RiverBuilder,
};

// === Minerals Feature ===
pub use minerals::*;  // Re-export all mineral types

// === Borders Feature ===
pub use borders::{
    BorderEntity,
    SelectionBorder,
    BorderPlugin,
};

// === Mesh Rendering ===
pub use mesh::{
    build_world_mesh,
    ProvinceStorage,
    WorldMeshHandle,
    MeshBuilder,
    MeshBuildStats,
};

// === Overlay System ===
pub use overlay::{
    update_province_colors,
    OverlayMode,
    ResourceOverlay,
    CachedOverlayColors,
    OverlayPlugin,
};

// === Color System ===
pub use colors::{
    WorldColors,
    StoneAbundance,
    SafeColor,
    ColorProvider,
    Colorable,
    theme_colors,
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

// === World Generation ===
pub use generation::{
    WorldBuilder,
    WorldGenerationError,
    WorldGenerationErrorType,
};

// === World Simulation ===
pub use simulation::*;  // Events and simulation systems

// === Bevy Integration (from setup.rs) ===
pub use setup::{
    setup_world,
    WorldPlugin,        // Main plugin that registers everything
    WorldGeneratedEvent,
    ProvinceSelectedEvent,
    WorldSetupError,
};