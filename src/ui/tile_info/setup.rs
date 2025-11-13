//! Setup and cleanup systems for tile info panel

use super::panel;
use crate::states::GameState;
use bevy::prelude::*;

/// Setup the tile info panel
pub fn setup_tile_info(mut commands: Commands) {
    // Tile info panel - bottom-right
    // Uses StateScoped for automatic cleanup when exiting InGame state
    commands
        .spawn((
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                min_width: Val::Px(250.0),
                ..default()
            },
            super::TileInfoPanel,
            ZIndex(100),
        ))
        .with_children(|parent| {
            panel::spawn_tile_info_panel(parent);
        });
}
