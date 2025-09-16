//! World tension module gateway
//!
//! Manages global conflict and instability metrics.
//! Tension ranges from 0.0 (perfect peace) to 1.0 (world war).

// PRIVATE modules - internal implementation
mod calculations;
mod systems;
mod types;

// Public exports
pub use types::WorldTension;

// Internal exports for future use
