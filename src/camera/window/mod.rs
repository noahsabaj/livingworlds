//! Camera window management gateway
//!
//! Handles cursor confinement, window focus, and alt-tab behavior.

// Private modules
mod cursor;
mod focus;

// Public exports for use by camera plugin
pub use cursor::{release_cursor_confinement, setup_cursor_confinement};
pub use focus::{handle_window_focus, WindowFocusState};
