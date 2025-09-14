//! Terrain feature module gateway
//!
//! This module contains everything related to terrain in Living Worlds:
//! - Terrain types and classification
//! - Climate zones and biomes
//! - Terrain marker components
//!
//! Following gateway architecture - all submodules are private.

// PRIVATE MODULES
mod types;
mod climate;
mod erosion;

// PUBLIC EXPORTS - The only way to access terrain functionality

// Core types and plugin
pub use types::{
    TerrainEntity,
    TerrainType,
    ClimateZone,
    classify_terrain_with_climate,
    TerrainPlugin,
};

// Climate types
pub use climate::Biome;

// Generation functions (these modules use direct functions, not builders)
pub use erosion::apply_erosion_to_provinces;
pub use climate::apply_climate_to_provinces;