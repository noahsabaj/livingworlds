//! Overlay text display for showing current map mode

use super::super::{ChildBuilder, LabelBuilder, LabelStyle, Orientation, SeparatorBuilder};
use crate::resources::MapMode;
use bevy::prelude::*;

/// Marker component for the map mode display text
#[derive(Component)]
pub struct MapModeText;

/// Spawn the overlay text display
pub fn spawn_overlay_text(parent: &mut ChildBuilder) {
    // Current overlay display using fixed LabelBuilder
    let label_entity = LabelBuilder::new("Political Map")
        .font_size(20.0)
        .color(Color::WHITE)
        .margin(UiRect::bottom(Val::Px(4.0)))
        .build(parent);

    // Add our marker to the label's text entity
    parent.commands().entity(label_entity).insert(MapModeText);

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
    overlay: Res<MapMode>,
    mut query: Query<&mut Text, With<MapModeText>>,
) {
    if !overlay.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        *text = Text::new(overlay.display_name());
    }
}
