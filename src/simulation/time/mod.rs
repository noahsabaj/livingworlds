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
pub use events::{NewYearEvent, SimulationSpeedChanged};
pub use resources::GameTime; // Now defined locally in this module
pub use systems::{advance_game_time, resume_from_pause_menu, track_year_changes};
