//! Name data collections for the Living Worlds name generator
//!
//! This module contains all the raw name data organized by type and culture.
//! The data is used by the generator to create contextually appropriate names.
//!
//! This module acts as the sole gateway to all name data.
//! All data modules are private and their contents are selectively
//! re-exported through this interface.

// All submodules are PRIVATE - only accessible through this gateway
mod world;
mod geographical;

// Re-export data through controlled interfaces
pub use world::*;
pub use geographical::*;

// Include and re-export cultures module
pub mod cultures;  // This will load the cultures/mod.rs file