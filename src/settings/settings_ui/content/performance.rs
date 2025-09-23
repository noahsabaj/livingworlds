//! Performance Tab Content
//!
//! Spawns the content for the Performance settings tab.

use crate::ui::colors;
use bevy::prelude::*;

/// Spawns performance settings content (placeholder for now)
pub fn spawn_performance_content(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new("Performance settings coming soon"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
    ));
}
