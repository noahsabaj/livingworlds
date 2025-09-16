//! Setup and cleanup systems for overlay display

use super::super::{PanelBuilder, PanelStyle};
use super::{mineral_legend, overlay_text, OverlayDisplayRoot};
use bevy::prelude::*;

/// Setup the overlay display UI
pub fn setup_overlay_display(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ZIndex(100),
            OverlayDisplayRoot,
        ))
        .with_children(|parent| {
            PanelBuilder::new()
                .style(PanelStyle::Dark)
                .custom_background(Color::srgba(0.05, 0.05, 0.05, 0.85))
                .flex_direction(FlexDirection::Column)
                .padding(UiRect::all(Val::Px(8.0)))
                .width(Val::Px(180.0))
                .build_with_children(parent, |panel| {
                    // Add overlay text display
                    overlay_text::spawn_overlay_text(panel);

                    // Add mineral legend
                    mineral_legend::spawn_mineral_legend(panel);
                });
        });
}

/// Cleanup the overlay display
pub fn cleanup_overlay_display(
    mut commands: Commands,
    query: Query<Entity, With<OverlayDisplayRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
