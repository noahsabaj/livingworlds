//! Overlay text display for showing current map mode

use bevy::prelude::*;
use crate::resources::ResourceOverlay;
use super::super::{LabelBuilder, LabelStyle, SeparatorBuilder, Orientation};

/// Marker component for the resource overlay display text
#[derive(Component)]
pub struct ResourceOverlayText;

/// Spawn the overlay text display
pub fn spawn_overlay_text(parent: &mut ChildSpawnerCommands) {
    // Current overlay display using LabelBuilder
    // We need to spawn the marker separately since LabelBuilder creates its own structure
    let label_entity = LabelBuilder::new(parent, "Political Map")
        .font_size(20.0)
        .color(Color::WHITE)
        .margin(UiRect::bottom(Val::Px(4.0)))
        .build();

    // Add our marker to the label's text entity
    parent.commands().entity(label_entity).insert(ResourceOverlayText);

    // Control hint using LabelBuilder
    LabelBuilder::new(parent, "[M] Cycle Overlay")
        .style(LabelStyle::Caption)
        .color(Color::srgba(0.5, 0.5, 0.5, 1.0))
        .margin(UiRect::top(Val::Px(2.0)))
        .build();

    // Divider using SeparatorBuilder
    SeparatorBuilder::new(parent)
        .orientation(Orientation::Horizontal)
        .color(Color::srgba(1.0, 1.0, 1.0, 0.2))
        .margin(UiRect::vertical(Val::Px(4.0)))
        .build();
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