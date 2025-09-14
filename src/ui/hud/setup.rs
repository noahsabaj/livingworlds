//! HUD setup and cleanup systems

use super::super::{PanelBuilder, PanelStyle};
use super::{control_hints, speed_display, time_display, HudRoot};
use bevy::prelude::*;

/// Setup all HUD elements
pub fn setup_hud(mut commands: Commands) {
    // Top-right HUD container using PanelBuilder
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            ZIndex(100),
            HudRoot,
        ))
        .with_children(|parent| {
            // Create panel with consistent styling using direct Bevy API
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        padding: UiRect::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)),
                ))
                .with_children(|panel| {
                    // Add time display
                    time_display::spawn_time_display(panel);

                    // Add speed display
                    speed_display::spawn_speed_display(panel);

                    // Add control hints
                    control_hints::spawn_control_hints(panel);
                });
        });
}

/// Cleanup all HUD elements
pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
