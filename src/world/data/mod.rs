//! World data structures gateway module
//!
//! This module contains all core data structures for the world.
//! Access is controlled through this gateway - internal modules are private.

// PRIVATE MODULES - Not directly accessible from outside

mod world;
mod province;
mod terrain;
mod spatial;
mod markers;

// SELECTIVE PUBLIC EXPORTS - The controlled API surface

// Core world data structure
pub use world::{
    World,
    RiverSystem,
    CloudSystem,
    CloudData,
    CloudLayer,
};

// Province and related types
pub use province::{
    Province,
    ProvinceId,
    Elevation,
    Agriculture,
    Distance,
    Abundance,
    HexDirection,
    ProvinceBuilder,
};

// Terrain classification
pub use terrain::{
    TerrainType,
    ClimateZone,
    classify_terrain_with_climate,
    classify_terrain_with_sea_level,
};

// Spatial indexing
pub use spatial::{
    ProvincesSpatialIndex,
    WorldBounds,
    calculate_hex_neighbors,
    opposite_hex_direction,
};

pub use markers::{
    TerrainEntity,
    CloudEntity,
    BorderEntity,
};


/// The data module provides all core data structures for Living Worlds.
///
/// # Architecture
///
/// This module follows the gateway pattern - all submodules are private,
/// and only carefully selected types are exposed through this mod.rs file.
///
/// # Contents
///
/// - **World**: The complete world representation with provinces and systems
/// - **Province**: Individual hexagonal tiles with terrain, population, and resources
/// - **Terrain**: Terrain types and climate classification logic
/// - **Spatial**: Spatial indexing for O(1) lookups and neighbor calculations
/// - **Markers**: Entity marker components for Bevy ECS
///
/// # Usage
///
/// ```rust
/// use crate::world::data::{World, Province, TerrainType};
///
/// // All data types are accessed through this gateway
/// let mut world = World::default();
/// let province = Province::builder(ProvinceId::new(0))
///     .terrain(TerrainType::Forest)
///     .build();
/// ```