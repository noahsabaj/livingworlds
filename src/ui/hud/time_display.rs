//! Time display component for showing game date/time

use bevy::prelude::*;
use crate::resources::GameTime;
use super::super::{LabelBuilder, LabelStyle};

/// Marker component for the game time display
#[derive(Component, Reflect)]
pub struct GameTimeDisplay;

/// Spawn the time display UI element
pub fn spawn_time_display(parent: &mut ChildSpawnerCommands) {
    let entity = LabelBuilder::new("Year 1000")
        .style(LabelStyle::Heading)
        .font_size(24.0)
        .color(Color::WHITE)
        .build(parent);

    // Add our marker component
    parent.commands().entity(entity).insert(GameTimeDisplay);
}

/// Update the game time display
pub fn update_time_display(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<GameTimeDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let year = 1000 + (game_time.current_date / 365.0) as u32;
        let day = (game_time.current_date % 365.0) as u32;
        **text = format!("Year {} - Day {}", year, day);
    }
}