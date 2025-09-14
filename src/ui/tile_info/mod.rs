//! Tile Info Module - Pure Gateway
//!
//! Manages the province/tile information panel that shows details about
//! the selected province. This is a pure gateway orchestrating submodules.

use bevy::prelude::*;

// Submodules - all private
mod panel;
mod setup;

/// Plugin that manages tile information display
pub struct TileInfoPlugin;

/// Marker for tile info panel root
#[derive(Component)]
pub struct TileInfoRoot;

// Re-export components for external use
pub use panel::{TileInfoPanel, TileInfoText};

// PLUGIN IMPLEMENTATION - Pure Orchestration

impl Plugin for TileInfoPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;

        app
            // Systems from submodules
            .add_systems(OnEnter(GameState::InGame), setup::setup_tile_info)
            .add_systems(OnExit(GameState::InGame), setup::cleanup_tile_info)
            .add_systems(
                Update,
                panel::update_tile_info_ui
                    .run_if(resource_changed::<crate::resources::SelectedProvinceInfo>)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
