//! Province - the fundamental unit of geography
//!
//! A province is just a location with an owner. All other properties
//! (terrain, climate, resources) are separate components following ECS patterns.

use bevy::prelude::*;
use lw_core::Vec2fx;
use serde::{Deserialize, Serialize};
use crate::types::{ProvinceId, NationId};

/// Core province component - minimal data only
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub id: ProvinceId,
    pub position: Vec2fx,
    pub coordinates: ProvinceCoordinates,
    pub owner: Option<NationId>,
}

// All Province logic moved to systems/province_logic.rs
// Components should be pure data - no methods!

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceCoordinates {
    pub x: i32,
    pub y: i32,
    pub region: GeographicRegion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeographicRegion {
    Continental,
    Coastal,
    Island,
    Archipelago,
    Peninsula,
}