//! Rivers feature module gateway
//!
//! Everything related to river systems, flow, and generation

// PRIVATE MODULES
mod generation;
mod types;

// PUBLIC EXPORTS
pub use generation::RiverBuilder;
pub use types::RiverSystem;
