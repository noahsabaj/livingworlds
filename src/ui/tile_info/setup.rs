//! Setup and cleanup systems for tile info panel

use super::{panel, TileInfoRoot};
use bevy::prelude::*;

/// Setup the tile info panel
pub fn setup_tile_info(mut commands: Commands) {
    // Tile info panel - bottom-right
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                min_width: Val::Px(250.0),
                ..default()
            },
            super::TileInfoPanel,
            ZIndex(100),
            TileInfoRoot,
        ))
        .with_children(|parent| {
            panel::spawn_tile_info_panel(parent);
        });
}

/// Cleanup the tile info panel
pub fn cleanup_tile_info(mut commands: Commands, query: Query<Entity, With<TileInfoRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
