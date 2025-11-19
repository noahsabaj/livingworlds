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
mod cultural; // Geographic-cultural assignment system
mod gpu; // GPU compute acceleration for world generation
mod infrastructure; // Infrastructure and development systems
mod mesh; // World mesh rendering
mod minerals; // Mineral resources
mod overlay;
mod provinces; // Province data, spatial indexing, agriculture
mod rivers; // River systems and flow
mod terrain; // Terrain types, climate, erosion // Overlay rendering modes

// Non-feature modules
mod core; // Core world data structures (World)
mod events; // World events
mod generation; // World generation orchestrator
mod plugin; // Main world plugin (Bevy integration)
mod setup; // World setup and generation systems
mod ui; // World configuration UI

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// === Core Data Structure ===
pub use core::{MapBounds, MapDimensions, World, WorldName, WorldSeed, WorldSize};

// === Clouds Feature ===
pub use clouds::{
    CloudBuilder, CloudData, CloudEntity,
    CloudPlugin, WeatherState, WeatherSystem,
};

// === Terrain Feature ===
pub use terrain::{
    apply_climate_to_provinces, apply_erosion_to_provinces, Biome,
    ClimateZone, TerrainEntity, TerrainPlugin, TerrainType,
};

// === Infrastructure Feature ===
pub use infrastructure::{analyze_infrastructure, InfrastructureStorage};

// === Provinces Feature ===
pub use provinces::{
    calculate_agriculture_values, calculate_ocean_depths, Abundance, Agriculture, Distance,
    Elevation, Province, ProvinceBuilder, ProvinceEntity, ProvinceId,
    ProvincesSpatialIndex, ProvinceEventsPlugin,
    CoastalProvinceCache, initialize_coastal_cache, NavalRangeCalculator, NAVAL_RANGE_HEXES,
};

// === Rivers Feature ===
pub use rivers::RiverBuilder;

// === Minerals Feature ===
pub use minerals::*; // Re-export all mineral types

// === Borders Feature ===
pub use borders::{BorderEntity, BorderPlugin};

// === Mesh Rendering ===
pub use mesh::{build_world_mesh, ProvinceStorage, WorldMeshHandle};

// === Overlay System ===
pub use overlay::{CachedOverlayColors, MapMode, OverlayPlugin};

// === Color System ===
pub use colors::WorldColors;

// === Cultural Assignment ===
pub use cultural::assign_cultures_to_province_storage;

// === World UI ===
pub use ui::{
    ClimateType, WorldConfigPlugin, WorldGenerationSettings,
};

// === World Generation ===
pub use generation::{WorldBuilder, WorldGenerationError, WorldGenerationErrorType};

// === GPU Compute Acceleration ===
pub use gpu::NoiseComputePlugin;

// === Bevy Plugin and Events ===
pub use plugin::WorldPlugin; // Main plugin that registers everything

// === World Generation (from setup.rs) ===
pub use setup::{
    poll_async_world_generation,  // Progress polling system
    start_async_world_generation, // Async generation starter
    handle_world_generation_transition_delay, // Transition delay handler
    AsyncWorldGeneration,
};
