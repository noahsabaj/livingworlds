//! Camera input handling gateway
//!
//! Manages keyboard, mouse, and edge panning input for camera control.

// Private modules
mod edge_pan;
mod keyboard;
mod mouse;

// Public exports for use by camera plugin
pub use edge_pan::handle_edge_panning;
pub use keyboard::{handle_camera_reset, handle_keyboard_movement};
pub use mouse::{handle_mouse_drag, handle_mouse_wheel_zoom};
