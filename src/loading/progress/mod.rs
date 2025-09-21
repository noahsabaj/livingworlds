//! Progress subsystem for loading tracking and updates
//!
//! This module handles all progress-related functionality:
//! - Progress bar value updates
//! - Status text updates
//! - Loading state synchronization

// Private module declarations
mod text;
mod tracking;

// Controlled exports
pub use text::update_loading_text;
pub use tracking::update_loading_progress;
