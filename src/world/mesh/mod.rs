//! Mesh feature module gateway

// PRIVATE MODULES
mod builder;

// PUBLIC EXPORTS
pub use builder::{
    build_world_mesh, MeshBuildStats, MeshBuilder, ProvinceStorage, WorldMeshHandle,
};
