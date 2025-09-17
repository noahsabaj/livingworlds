//! Noble houses and ruling families - Gateway Module
//!
//! Houses represent the dynasties that rule nations. Each house has its own
//! personality traits, history, and relationships that persist across generations.
//!
//! This is a pure gateway module - all implementation lives in submodules.

// Private submodules - implementation details hidden from external code
mod types;
mod traits;
mod mottos;
mod motto_data;
mod motto_data_extended;
mod influence;

// Public re-exports - carefully controlled API surface

// Core types
pub use types::{House, Ruler, RulerPersonality};

// Trait system
pub use traits::{HouseTraits, HouseArchetype, DominantTrait};

// Motto generation (only the public function, not internals)
pub use mottos::generate_motto;