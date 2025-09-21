//! State management subsystem for loading operations
//!
//! This module manages all state-related functionality for the loading system:
//! - LoadingState resource
//! - LoadingOperation enum and variants
//! - LoadingDetails structure
//! - State update logic

// Private module declarations
mod operations;
mod resources;

// Controlled exports
pub use operations::{LoadingDetails, LoadingOperation};
pub use resources::LoadingState;
