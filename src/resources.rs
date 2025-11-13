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

// This module serves as a centralized gateway for commonly used resources
// Resources are organized in their domain modules but re-exported here for convenience

// World configuration types
pub use crate::world::{MapDimensions, WorldName, WorldSeed, WorldSize};

// Generation error types
pub use crate::world::WorldGenerationError;

// Weather system
pub use crate::world::{WeatherState, WeatherSystem};

// Province selection
pub use crate::ui::SelectedProvinceInfo;

// Spatial indexing
pub use crate::world::ProvincesSpatialIndex;

// Overlay system
pub use crate::world::{CachedOverlayColors, MapMode};

// World tension
pub use crate::simulation::WorldTension;

// Game time (now lives in simulation/time where it belongs)
pub use crate::simulation::GameTime;
