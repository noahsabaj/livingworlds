//! Mod Browser UI subsystem
//!
//! This module provides a comprehensive mod browser interface with:
//! - Steam Workshop integration for live browsing
//! - Local mod management
//! - Active modset configuration
//! - Soft-reset functionality
//!
//! # Gateway Architecture
//! This module follows the gateway pattern where mod.rs is the sole
//! entry/exit point. All submodules are private with controlled exports.

// Internal modules - all private
mod handlers;
mod plugin;
mod spawning;
mod state;
mod tabs;
mod types;

// Re-export the plugin (primary entry point)
pub use plugin::ModBrowserUIPlugin;

// Re-export events that external code needs
pub use types::{
    ApplyModChangesEvent, CloseModBrowserEvent, OpenModBrowserEvent, SwitchModTabEvent,
};

// Re-export state types if needed by external systems
pub use state::{ModBrowserState, WorkshopCache, WorkshopItem};

// Re-export tab enum for external reference
pub use types::ModBrowserTab;