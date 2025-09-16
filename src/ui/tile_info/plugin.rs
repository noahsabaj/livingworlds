//! Tile info plugin implementation
//!
//! This module contains the TileInfoPlugin that manages the province/tile
//! information panel that shows details about the selected province.

use bevy::prelude::*;
use crate::states::GameState;

use super::{panel, setup};

/// Plugin that manages tile information display
pub struct TileInfoPlugin;

impl Plugin for TileInfoPlugin {
    fn build(&self, app: &mut App) {
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