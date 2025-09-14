//! World tension module gateway
//!
//! Manages global conflict and instability metrics.
//! Tension ranges from 0.0 (perfect peace) to 1.0 (world war).

// PRIVATE modules - internal implementation
mod types;
mod calculations;
mod systems;

// Re-export for backward compatibility
pub use types::WorldTension;

// Internal exports for future use
pub(super) use calculations::calculate_from_war_percentage;