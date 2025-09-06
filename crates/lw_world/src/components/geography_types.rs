//! Geographic type definitions
//!
//! Common types used across geographic components and systems.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};

/// Terrain type component for categorizing land
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Plains,
    Mountain,
    Forest,
    Desert,
    Coastal,
    River,
    Tundra,
    Swamp,
}

/// Population component for provinces
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    pub total: u32,
    pub growth_rate: Fixed32,
}

/// Territory component marking ownership
#[derive(Component, Debug, Clone)]
pub struct Territory {
    pub owner: Option<Entity>,
    pub disputed: bool,
}

/// Resource production capabilities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProduction {
    pub arable_land: Fixed32,
    pub pasture_land: Fixed32,
    pub forest_coverage: Fixed32,
    pub mineral_deposits: Fixed32,
}