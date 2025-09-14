//! Camera module gateway
//!
//! Controls all camera-related functionality through gateway architecture.
//! All external access to camera systems must go through this gateway module.

// PRIVATE modules - internal implementation details
mod controller;
mod input;
mod movement;
mod plugin;
mod window;

// CONTROLLED PUBLIC EXPORTS - The only way to access camera functionality
pub use controller::CameraController;
pub use plugin::CameraPlugin;

// Note: Input, movement, and window subsystems are intentionally kept private.
// They are only accessible through the CameraPlugin's registered systems.
