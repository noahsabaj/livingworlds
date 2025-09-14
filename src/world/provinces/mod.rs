//! Provinces feature module gateway
//!
//! Core province system with spatial indexing

// PRIVATE MODULES
mod agriculture;
mod generation;
mod spatial;
mod types;

// PUBLIC EXPORTS

// Province types
pub use types::{Abundance, Agriculture, Distance, Elevation, HexDirection, Province, ProvinceId};

// Spatial indexing
pub use spatial::{ProvincesSpatialIndex, WorldBounds};

// Generation and processing
pub use agriculture::calculate as calculate_agriculture_values;
pub use generation::{calculate_ocean_depths, ProvinceBuilder};
