//! Nation and dynasty system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

// PRIVATE MODULES - Gateway architecture compliance
mod generation;
mod house;
mod plugin;
mod rendering;
mod types;

pub use generation::{build_territories_from_provinces, spawn_nations};
pub use house::{
    generate_motto, DominantTrait, House, HouseArchetype, HouseTraits, Ruler, RulerPersonality,
};
pub use plugin::NationPlugin;
pub use types::*;
