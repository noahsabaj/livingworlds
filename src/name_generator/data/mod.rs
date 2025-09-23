//! Name data collections for the Living Worlds name generator
//!
//! This module contains all the raw name data organized by type and culture.
//! The data is used by the generator to create contextually appropriate names.
//!
//! This module acts as the sole gateway to all name data.
//! All data modules are private and their contents are selectively
//! re-exported through this interface.

// All submodules are PRIVATE - only accessible through this gateway
mod geographical;
mod world;

// Re-export data through controlled interfaces
pub use geographical::*;
pub use world::*;

// Include cultures module as private with controlled re-exports
mod cultures;  // PRIVATE MODULE - Gateway architecture compliance

// Re-export cultures data through controlled interface
pub use cultures::{
    ancient_data, desert_data, eastern_data, island_data,
    mystical_data, northern_data, southern_data, western_data,
};
