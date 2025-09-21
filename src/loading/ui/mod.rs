//! UI subsystem for loading screen layout and components
//!
//! This module handles all visual aspects of the loading screen:
//! - Screen setup and cleanup
//! - Section layout builders
//! - UI component markers

// Private module declarations
mod components;
mod layout;
mod sections;

// Controlled exports
pub use components::{
    CancelGenerationButton, LoadingProgressBar, LoadingScreenRoot, LoadingStatusText,
};
pub use layout::setup_loading_screen;
