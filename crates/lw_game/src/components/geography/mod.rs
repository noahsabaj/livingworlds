//! Geography domain module - terrain, climate, resources, and provinces
//!
//! This module contains all geographical components following proper ECS patterns.
//! Components are pure data structures with no logic.

pub mod province;
pub mod terrain;
pub mod climate;
pub mod resources;
pub mod water;
pub mod templates;

// Re-export key types for convenience
pub use province::*;
pub use terrain::*;
pub use climate::*;
pub use resources::*;
pub use water::*;