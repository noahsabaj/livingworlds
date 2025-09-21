//! Events subsystem for loading screen interactions
//!
//! This module handles all event-driven functionality:
//! - Cancel generation events
//! - Button interaction handling
//! - State transition logic

// Private module declarations
mod handlers;
mod types;

// Controlled exports
pub use handlers::{handle_cancel_button, handle_cancel_generation};
pub use types::CancelWorldGeneration;
