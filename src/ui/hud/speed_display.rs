//! Speed display component for showing simulation speed

use bevy::prelude::*;
use crate::resources::GameTime;
use super::super::{LabelBuilder, LabelStyle};

/// Marker component for the game speed display
#[derive(Component, Reflect)]
pub struct GameSpeedDisplay;

/// Spawn the speed display UI element
pub fn spawn_speed_display(parent: &mut ChildSpawnerCommands) {
    let entity = LabelBuilder::new(parent, "Speed: 1x")
        .style(LabelStyle::Body)
        .font_size(16.0)
        .color(Color::srgba(0.8, 0.8, 0.8, 1.0))
        .margin(UiRect::top(Val::Px(4.0)))
        .build();

    // Add our marker component
    parent.commands().entity(entity).insert(GameSpeedDisplay);
}

/// Update the speed display when it changes
pub fn update_speed_display(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<GameSpeedDisplay>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if game_time.paused {
            **text = "PAUSED".to_string();
        } else {
            let speed_text = match game_time.speed {
                s if s <= 0.0 => "Paused",
                s if s <= 1.0 => "Normal",
                s if s <= 3.0 => "Fast (3x)",
                s if s <= 6.0 => "Faster (6x)",
                _ => "Fastest (9x)",
            };
            **text = format!("Speed: {}", speed_text);
        }
    }
}