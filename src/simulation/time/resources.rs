//! Time-related resources

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

/// Current game time and simulation speed
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct GameTime {
    pub current_date: f32, // Days since start
    pub speed: f32,        // Time multiplier
    pub paused: bool,
    pub speed_before_pause: f32, // Speed to restore when unpausing
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            current_date: 0.0,
            speed: 1.0,
            paused: false,
            speed_before_pause: 1.0,
        }
    }
}
