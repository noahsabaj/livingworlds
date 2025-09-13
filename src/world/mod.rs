//! World module for Living Worlds
//!
//! This module contains all world representation and rendering systems:
//! - World data structures (the WHAT)
//! - Terrain types and classification
//! - Cloud atmospheric effects
//! - Border selection rendering
//! - Map overlay visualizations
//! - Mesh construction and optimization
//! - World configuration UI

pub mod components;
pub mod data;
pub mod terrain;
pub mod clouds;
pub mod borders;
pub mod overlay;
pub mod mesh;
pub mod config;

// Re-export commonly used world components
pub use components::{TerrainEntity, CloudEntity, BorderEntity};

// Re-export terrain types for convenience
pub use terrain::{TerrainType, TerrainPlugin, ClimateZone};

// Re-export world data structures
pub use data::{World, RiverSystem, CloudSystem, CloudData, CloudLayer};

// Re-export cloud plugin
pub use clouds::CloudPlugin;

// Re-export border system
pub use borders::{BorderPlugin, SelectionBorder};

// Re-export overlay system
pub use overlay::{OverlayPlugin, update_province_colors};

// Re-export mesh building
pub use mesh::{
    build_world_mesh,
    ProvinceStorage,
    WorldMeshHandle,
    MeshBuilder,
    MeshBuildStats,
};

// Re-export world configuration
pub use config::{
    WorldGenerationSettings,
    WorldConfigPlugin,
    ClimateType,
};