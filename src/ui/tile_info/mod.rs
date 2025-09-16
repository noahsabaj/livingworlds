//! Tile Info Module - Pure Gateway
//!
//! Manages the province/tile information panel that shows details about
//! the selected province. This is a pure gateway orchestrating submodules.

use bevy::prelude::*;

// Submodules - all private
mod panel;
mod plugin;
mod setup;

// CONTROLLED EXPORTS - Gateway Interface

/// Marker for tile info panel root
#[derive(Component)]
pub struct TileInfoRoot;

/// Plugin that manages tile information display (implementation in plugin.rs)
pub use plugin::TileInfoPlugin;

// Re-export components for external use
pub use panel::{TileInfoPanel, TileInfoText};
