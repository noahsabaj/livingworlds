//! Control hints display for showing keyboard shortcuts

use super::super::{LabelBuilder, LabelStyle};
use crate::resources::GameTime;
use bevy::prelude::*;

/// Marker component for the control hints text
#[derive(Component, Reflect)]
pub struct ControlHintsText;

/// Spawn the control hints UI element
pub fn spawn_control_hints(parent: &mut ChildSpawnerCommands) {
    let entity = LabelBuilder::new("[1-5] Speed | [Space] Pause")
        .style(LabelStyle::Caption)
        .color(Color::srgba(0.5, 0.5, 0.5, 1.0))
        .margin(UiRect::top(Val::Px(8.0)))
        .build(parent);

    // Add our marker component
    parent.commands().entity(entity).insert(ControlHintsText);
}

/// Update the control hints based on pause state
pub fn update_control_hints(
    game_time: Res<GameTime>,
    mut query: Query<&mut Text, With<ControlHintsText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let pause_text = if game_time.paused { "Unpause" } else { "Pause" };
        **text = format!("[1-5] Speed | [Space] {}", pause_text);
    }
}
