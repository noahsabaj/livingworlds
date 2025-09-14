//! Rivers feature module gateway
//!
//! Everything related to river systems, flow, and generation

// PRIVATE MODULES
mod types;
mod generation;

// PUBLIC EXPORTS
pub use types::RiverSystem;
pub use generation::RiverBuilder;