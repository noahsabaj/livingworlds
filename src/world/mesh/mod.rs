//! Mesh feature module gateway

// PRIVATE MODULES
mod builder;

// PUBLIC EXPORTS
pub use builder::{
    build_world_mesh,
    ProvinceStorage,
    WorldMeshHandle,
    MeshBuilder,
    MeshBuildStats,
};