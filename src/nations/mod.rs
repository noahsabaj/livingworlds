//! Nation and dynasty system
//!
//! This module implements the political entities that control provinces,
//! including nations, dynasties, and their interactions.

pub mod types;
pub mod generation;
pub mod rendering;
pub mod plugin;
pub mod house;

pub use types::*;
pub use generation::spawn_nations;
pub use plugin::NationPlugin;
pub use house::{House, Ruler, RulerPersonality, HouseTraits, HouseArchetype, DominantTrait, generate_motto};