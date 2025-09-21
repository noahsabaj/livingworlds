//! Apply/Cancel Button Spawner
//!
//! Handles creation of the Apply and Exit buttons at the bottom of the settings menu.

use crate::settings::components::{ApplyButton, CancelButton};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, ChildBuilder};
use bevy::prelude::*;

/// Spawns the apply/cancel buttons at the bottom of the settings menu
pub fn spawn_apply_cancel_buttons(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                margin: UiRect::top(Val::Px(10.0)), // Reduced from 20px to ensure buttons fit
                column_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|buttons| {
            // Apply button - using ButtonBuilder (eating our own dog food)
            ButtonBuilder::new("Apply")
                .style(ButtonStyle::Success)
                .size(ButtonSize::Medium)
                .enabled(false) // Initially disabled until settings change
                .with_marker(ApplyButton)
                .build(buttons);

            // Exit button - using ButtonBuilder (eating our own dog food)
            ButtonBuilder::new("Exit")
                .style(ButtonStyle::Danger)
                .size(ButtonSize::Medium)
                .with_marker(CancelButton)
                .build(buttons);
        });
}
