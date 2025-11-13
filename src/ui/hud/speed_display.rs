//! Speed display component for showing simulation speed

use crate::ui::{ChildBuilder, LabelBuilder, LabelStyle};
use crate::simulation::GameTime;
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
        debug!("🎛️ Speed display updating: paused={}, speed={}",
            game_time.is_paused(), game_time.get_speed().name());

        // Find the speed display entity and get its children
        if let Ok(children) = speed_display_query.single() {
            // Look for the Text component in the children
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    let speed_text = if game_time.is_paused() {
                        "Paused".to_string()
                    } else {
                        game_time.get_speed().name().to_string()
                    };
                    **text = format!("Speed: {}", speed_text);
                    debug!("🎛️ Speed display updated to: {}", text.as_str());
                    break; // Found and updated the text
                }
            }
        } else {
            warn!("🎛️ Speed display component not found!");
        }
    }
}
