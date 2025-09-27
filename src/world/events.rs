//! World-related events

use bevy::prelude::*;
use super::ProvinceId;

/// Event fired when world generation completes
#[derive(Event)]
pub struct WorldGeneratedEvent {
    pub world: super::World,
    pub generation_time: std::time::Duration,
}

/// Event fired when a province is selected
#[derive(Event)]
pub struct ProvinceSelectedEvent {
    pub province_id: Option<ProvinceId>,
    pub position: Vec2,
}