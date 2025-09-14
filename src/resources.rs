//! Global resources for the Living Worlds game
//!
//! This module now serves as a centralized re-export point for resources that have been
//! moved to their domain-specific modules. Only truly global state (GameTime) remains here.
//!
//! # Architecture Note
//!
//! Resources have been refactored into their logical domains:
//! - World configuration → world::core
//! - Generation errors → world::generation
//! - Weather system → world::clouds::weather
//! - Spatial indexing → world::provinces (already there)
//! - Overlay system → world::overlay
//! - World tension → simulation
//! - UI interaction → ui::interaction

use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Serialize, Deserialize};

// ===== BACKWARD COMPATIBILITY RE-EXPORTS =====
// These maintain the existing API surface while the actual implementations
// live in their domain-specific modules

// World configuration types
pub use crate::world::{WorldSeed, WorldName, WorldSize, MapDimensions, MapBounds};

// Generation error types
pub use crate::world::{WorldGenerationError, WorldGenerationErrorType};

// Weather system
pub use crate::world::{WeatherState, WeatherSystem};

// Province selection
pub use crate::ui::SelectedProvinceInfo;

// Spatial indexing
pub use crate::world::ProvincesSpatialIndex;

// Overlay system
pub use crate::world::{CachedOverlayColors, ResourceOverlay};

// World tension
pub use crate::simulation::WorldTension;

// ===== TRULY GLOBAL STATE =====

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