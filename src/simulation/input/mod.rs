//! Input handling module gateway
//!
//! Handles user input for simulation controls (speed, pause, etc.)
//! This module is internal to simulation - no public exports needed.

// PRIVATE modules - internal implementation
mod time_controls;
mod speed_mapping;

// Internal exports for use by the simulation plugin
pub(super) use time_controls::handle_time_controls;