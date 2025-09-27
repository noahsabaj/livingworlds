//! Time display component for showing game date/time

use super::super::{ChildBuilder, LabelBuilder, LabelStyle};
use crate::simulation::GameTime;
use bevy::prelude::*;

/// Marker component for the game time display
#[derive(Component, Reflect)]
pub struct GameTimeDisplay;

/// Spawn the time display UI element
pub fn spawn_time_display(parent: &mut ChildBuilder) {
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
    if let Ok(mut text) = query.single_mut() {
        let year = game_time.current_year();
        let day = game_time.day_of_year();
        **text = format!("Year {} - Day {}", year, day);
    }
}
