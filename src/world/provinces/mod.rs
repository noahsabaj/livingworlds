//! Provinces feature module gateway
//!
//! Core province system with spatial indexing

// PRIVATE MODULES
mod agriculture;
mod coastal;
mod elevation;
mod events;
mod generation;  // Now points to the new generation/ subfolder
mod naval_range;
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
    ProvincePopulationChanged, ProvinceTerrainChanged, ProvinceMineralsChanged,
    ProvinceAgricultureChanged, ProvinceFreshWaterChanged, ProvinceChanged,
    BatchPopulationUpdate, BatchMineralDiscovery, ProvinceEventsPlugin,
};

// Coastal and naval range systems
pub use coastal::{CoastalProvinceCache, initialize_coastal_cache};
pub use naval_range::{NavalRangeCalculator, NAVAL_RANGE_HEXES};
