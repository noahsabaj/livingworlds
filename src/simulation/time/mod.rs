//! Time management module gateway
//!
//! Controls game time, simulation speed, and year tracking.
//! All time-related functionality must be accessed through this gateway.

// PRIVATE modules - internal implementation
mod constants;
mod events;
mod resources;
mod systems;

// Re-export what parent modules need
pub use constants::{SPEED_PAUSED, SPEED_NORMAL, SPEED_FAST, SPEED_FASTER, SPEED_FASTEST};
pub use events::{SimulationSpeedChanged, NewYearEvent};
pub use resources::GameTime;
pub use systems::{advance_game_time, track_year_changes, resume_from_pause_menu};