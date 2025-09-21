//! Nation and dynasty system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

pub mod generation;
pub mod house;
pub mod plugin;
pub mod rendering;
pub mod types;

pub use generation::{build_territories_from_provinces, spawn_nations};
pub use house::{
    generate_motto, DominantTrait, House, HouseArchetype, HouseTraits, Ruler, RulerPersonality,
};
pub use plugin::NationPlugin;
pub use types::*;
