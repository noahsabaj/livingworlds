//! Overlay Display Module - Pure Gateway
//!
//! Manages the resource overlay display system including the current overlay
//! indicator and mineral legend. This is a pure gateway that orchestrates
//! submodules without implementation.

use bevy::prelude::*;

// Submodules - all private
mod mineral_legend;
mod overlay_text;
mod plugin;
mod setup;

// CONTROLLED EXPORTS - Gateway Interface

/// Marker for the overlay display root
#[derive(Component)]
pub struct OverlayDisplayRoot;

/// Plugin that manages overlay display UI (implementation in plugin.rs)
pub use plugin::OverlayDisplayPlugin;

// Re-export marker components for external use
pub use mineral_legend::MineralLegendContainer;
