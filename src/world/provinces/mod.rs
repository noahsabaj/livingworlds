//! Provinces feature module gateway
//!
//! Core province system with spatial indexing

// PRIVATE MODULES
mod types;
mod spatial;
mod generation;
mod agriculture;

// PUBLIC EXPORTS

// Province types
pub use types::{
    Province,
    ProvinceId,
    Elevation,
    Agriculture,
    Distance,
    Abundance,
    HexDirection,
};

// Spatial indexing
pub use spatial::{
    ProvincesSpatialIndex,
    WorldBounds,
};

// Generation and processing
pub use generation::{ProvinceBuilder, calculate_ocean_depths};
pub use agriculture::calculate as calculate_agriculture_values;