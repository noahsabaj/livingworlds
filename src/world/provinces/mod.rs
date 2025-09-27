//! Provinces feature module gateway
//!
//! Core province system with spatial indexing

// PRIVATE MODULES
mod agriculture;
mod elevation;
mod events;
mod generation;  // Now points to the new generation/ subfolder
mod spatial;
mod types;

// PUBLIC EXPORTS

// Province types
pub use types::{
    Abundance, Agriculture, Distance, Elevation, HexDirection, Province, ProvinceEntity, ProvinceId,
};

// Spatial indexing
pub use spatial::{ProvincesSpatialIndex, WorldBounds};

// Generation and processing
pub use agriculture::calculate as calculate_agriculture_values;
pub use generation::{ProvinceBuilder, calculate_ocean_depths, precompute_neighbor_indices};

// Province events
pub use events::{
    ProvincePopulationChanged, ProvinceTerrainChanged, ProvinceOwnershipChanged,
    ProvinceInfrastructureChanged, ProvinceMineralDiscovered, ProvinceAgriculturalEvent,
    ProvinceDevelopmentChanged,
};
