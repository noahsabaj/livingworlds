//! Terrain feature module gateway
//!
//! This module contains everything related to terrain in Living Worlds:
//! - Terrain types and classification
//! - Climate zones and biomes
//! - Terrain marker components
//!
//! Following gateway architecture - all submodules are private.

// PRIVATE MODULES
mod climate;
mod erosion;
mod types;

// PUBLIC EXPORTS - The only way to access terrain functionality

// Core types and plugin
pub use types::{
    classify_terrain_with_climate, ClimateZone, TerrainEntity, TerrainPlugin, TerrainType,
};

// Climate types
pub use climate::Biome;

// Generation functions (these modules use direct functions, not builders)
pub use climate::apply_climate_to_provinces;
pub use erosion::apply_erosion_to_provinces;
