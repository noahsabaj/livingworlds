//! Layout module gateway
//!
//! This module handles UI construction for the world configuration screen.
//! It uses the UI builders from src/ui/ to create consistent interfaces.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports. Internal modules handle their own imports.

// PRIVATE MODULES - UI layout implementations
mod root;
mod basic;
mod presets;
mod advanced;

// CONTROLLED PUBLIC EXPORTS - Only what plugin needs
pub use root::{spawn_world_config_ui, despawn_world_config_ui};

// INTERNAL EXPORTS - For use by sibling modules ONLY through this gateway
pub(super) use basic::{spawn_world_name_section, spawn_world_size_section, spawn_seed_section};
pub(super) use presets::spawn_preset_section;
pub(super) use advanced::spawn_advanced_panel;