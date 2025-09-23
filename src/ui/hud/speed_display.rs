//! Speed display component for showing simulation speed

use super::super::{ChildBuilder, LabelBuilder, LabelStyle};
use crate::resources::GameTime;
use bevy::prelude::*;

/// Marker component for the game speed display
#[derive(Component, Reflect)]
pub struct GameSpeedDisplay;

/// Spawn the speed display UI element
pub fn spawn_speed_display(parent: &mut ChildBuilder) {
    let entity = LabelBuilder::new("Speed: 1x")
        .style(LabelStyle::Body)
        .font_size(16.0)
        .color(Color::srgba(0.8, 0.8, 0.8, 1.0))
        .margin(UiRect::top(Val::Px(4.0)))
        .build(parent);

    // Add our marker component
    parent.commands().entity(entity).insert(GameSpeedDisplay);
}

/// Update the speed display when it changes
pub fn update_speed_display(
    game_time: Res<GameTime>,
    speed_display_query: Query<&Children, With<GameSpeedDisplay>>,
    mut text_query: Query<&mut Text>,
) {
    // Only update if GameTime has changed
    if game_time.is_changed() {
        debug!("🎛️ Speed display updating: paused={}, speed={}x", game_time.paused, game_time.speed);

        // Find the speed display entity and get its children
        if let Ok(children) = speed_display_query.get_single() {
            // Look for the Text component in the children
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if game_time.paused {
                        **text = "Speed: Paused".to_string();
                    } else {
                        let speed_text = match game_time.speed {
                            s if s <= 0.0 => "Paused".to_string(),
                            s if s == 1.0 => "1x".to_string(),
                            s if s == 3.0 => "3x".to_string(),
                            s if s == 6.0 => "6x".to_string(),
                            s if s == 9.0 => "9x".to_string(),
                            s => format!("{:.0}x", s), // Handle any other speed values
                        };
                        **text = format!("Speed: {}", speed_text);
                    }
                    debug!("🎛️ Speed display updated to: {}", text.as_str());
                    break; // Found and updated the text
                }
            }
        } else {
            warn!("🎛️ Speed display component not found!");
        }
    }
}
