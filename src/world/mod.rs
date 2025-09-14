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

mod borders; // Border rendering
mod clouds; // Cloud system (data, generation, rendering)
mod colors; // Color system (themes, providers, calculations)
mod mesh; // World mesh rendering
mod minerals; // Mineral resources
mod overlay;
mod provinces; // Province data, spatial indexing, agriculture
mod rivers; // River systems and flow
mod terrain; // Terrain types, climate, erosion // Overlay rendering modes

// Non-feature modules
mod core; // Core world data structures (World)
mod generation; // World generation orchestrator
mod setup;
mod simulation; // Simulation events
mod ui; // World configuration UI // Bevy integration layer (Plugin, systems, resources)

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// === Core Data Structure ===
pub use core::{MapBounds, MapDimensions, World, WorldName, WorldSeed, WorldSize};

// === Clouds Feature ===
pub use clouds::{
    animate_clouds, create_cloud_texture, dynamic_cloud_spawn_system, generate_cloud_formation,
    update_weather_system, CloudBuilder, CloudData, CloudEntity, CloudFormationType, CloudLayer,
    CloudPlugin, CloudSprite, CloudSystem, CloudTextureParams, WeatherState, WeatherSystem,
};

// === Terrain Feature ===
pub use terrain::{
    apply_climate_to_provinces, apply_erosion_to_provinces, classify_terrain_with_climate, Biome,
    ClimateZone, TerrainEntity, TerrainPlugin, TerrainType,
};

// === Provinces Feature ===
pub use provinces::{
    calculate_agriculture_values, calculate_ocean_depths, Abundance, Agriculture, Distance,
    Elevation, HexDirection, Province, ProvinceBuilder, ProvinceId, ProvincesSpatialIndex,
    WorldBounds,
};

// === Rivers Feature ===
pub use rivers::{RiverBuilder, RiverSystem};

// === Minerals Feature ===
pub use minerals::*; // Re-export all mineral types

// === Borders Feature ===
pub use borders::{BorderEntity, BorderPlugin, SelectionBorder};

// === Mesh Rendering ===
pub use mesh::{build_world_mesh, MeshBuildStats, MeshBuilder, ProvinceStorage, WorldMeshHandle};

// === Overlay System ===
pub use overlay::{
    update_province_colors, CachedOverlayColors, OverlayMode, OverlayPlugin, ResourceOverlay,
};

// === Color System ===
pub use colors::{theme_colors, ColorProvider, Colorable, SafeColor, StoneAbundance, WorldColors};

// === World UI ===
pub use ui::{
    AggressionLevel, ClimateType, IslandFrequency, MineralDistribution, MountainDensity,
    ResourceAbundance, TradePropensity, WorldConfigPlugin, WorldGenerationSettings, WorldPreset,
};

// === World Generation ===
pub use generation::{WorldBuilder, WorldGenerationError, WorldGenerationErrorType};

// === World Simulation ===
pub use simulation::*; // Events and simulation systems

// === Bevy Integration (from setup.rs) ===
pub use setup::{
    setup_world,
    ProvinceSelectedEvent,
    WorldGeneratedEvent,
    WorldPlugin, // Main plugin that registers everything
    WorldSetupError,
};
