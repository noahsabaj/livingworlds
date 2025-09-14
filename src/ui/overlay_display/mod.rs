//! Overlay Display Module - Pure Gateway
//!
//! Manages the resource overlay display system including the current overlay
//! indicator and mineral legend. This is a pure gateway that orchestrates
//! submodules without implementation.

use bevy::prelude::*;

// Submodules - all private
mod mineral_legend;
mod overlay_text;
mod setup;

/// Plugin that manages overlay display UI
pub struct OverlayDisplayPlugin;

/// Marker for the overlay display root
#[derive(Component)]
pub struct OverlayDisplayRoot;

// Re-export marker components for external use
pub use mineral_legend::MineralLegendContainer;
pub use overlay_text::MapModeText;

// PLUGIN IMPLEMENTATION - Pure Orchestration

impl Plugin for OverlayDisplayPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;

        app
            // Systems from submodules
            .add_systems(OnEnter(GameState::InGame), setup::setup_overlay_display)
            .add_systems(OnExit(GameState::InGame), setup::cleanup_overlay_display)
            .add_systems(
                Update,
                (
                    overlay_text::update_overlay_display,
                    mineral_legend::update_mineral_legend_visibility
                        .run_if(resource_changed::<crate::resources::MapMode>),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
