//! Camera movement and physics gateway
//!
//! Handles smooth interpolation, boundary constraints, and zoom calculations.

// Private modules
mod bounds;
mod interpolation;

// Public exports for use by camera plugin
pub use bounds::{apply_camera_bounds, calculate_camera_bounds, CameraBounds};
pub use interpolation::apply_smooth_movement;
