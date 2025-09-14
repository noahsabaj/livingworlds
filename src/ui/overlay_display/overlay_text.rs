//! Overlay text display for showing current map mode

use super::super::{LabelBuilder, LabelStyle, Orientation, SeparatorBuilder};
use crate::resources::ResourceOverlay;
use bevy::prelude::*;

/// Marker component for the resource overlay display text
#[derive(Component)]
pub struct ResourceOverlayText;

/// Spawn the overlay text display
pub fn spawn_overlay_text(parent: &mut ChildSpawnerCommands) {
    // Current overlay display using fixed LabelBuilder
    let label_entity = LabelBuilder::new("Political Map")
        .font_size(20.0)
        .color(Color::WHITE)
        .margin(UiRect::bottom(Val::Px(4.0)))
        .build(parent);

    // Add our marker to the label's text entity
    parent
        .commands()
        .entity(label_entity)
        .insert(ResourceOverlayText);

    // Control hint using fixed LabelBuilder
    LabelBuilder::new("[M] Cycle Overlay")
        .style(LabelStyle::Caption)
        .color(Color::srgba(0.5, 0.5, 0.5, 1.0))
        .margin(UiRect::top(Val::Px(2.0)))
        .build(parent);

    // Divider using fixed SeparatorBuilder
    SeparatorBuilder::new()
        .orientation(Orientation::Horizontal)
        .color(Color::srgba(1.0, 1.0, 1.0, 0.2))
        .margin(UiRect::vertical(Val::Px(4.0)))
        .build(parent);
}

/// Update the resource overlay display text
pub fn update_overlay_display(
    overlay: Res<ResourceOverlay>,
    mut query: Query<&mut Text, With<ResourceOverlayText>>,
) {
    if !overlay.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        *text = Text::new(overlay.display_name());
    }
}
