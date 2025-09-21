//! Loading system - Gateway architecture for unified loading experience
//!
//! This module provides a consistent loading experience for all operations:
//! - World generation
//! - Save file loading
//! - Mod application
//!
//! ## Architecture
//!
//! The loading system follows the gateway pattern with focused subsystems:
//! - `plugin` - LoadingScreenPlugin (Bevy integration)
//! - `state` - Loading state management and operations
//! - `ui` - Loading screen layout and components
//! - `events` - Event handling and cancellation
//! - `progress` - Progress tracking and updates
//! - `api` - Public API functions for external systems
//!
//! ## Usage
//!
//! ```rust
//! use crate::loading::{LoadingScreenPlugin, set_loading_progress, start_world_generation_loading};
//!
//! // Add plugin to app
//! app.add_plugins(LoadingScreenPlugin);
//!
//! // Control loading from external systems
//! start_world_generation_loading(&mut loading_state, seed, size);
//! set_loading_progress(&mut loading_state, 0.5, "Generating terrain...");
//! ```

// Private module declarations - internal implementation hidden
mod api;
mod events;
mod plugin;
mod progress;
mod state;
mod ui;

// Controlled public exports - gateway interface
pub use api::{
    set_loading_progress, start_mod_application_loading, start_save_loading,
    start_world_generation_loading,
};
pub use events::CancelWorldGeneration;
pub use plugin::LoadingScreenPlugin;
pub use state::{LoadingDetails, LoadingOperation, LoadingState};
