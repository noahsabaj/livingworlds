//! Steam Workshop tab UI
//!
//! This module handles the Steam Workshop browser interface,
//! allowing users to browse and subscribe to community mods.

use crate::ui::{colors, LabelBuilder, LabelStyle};
use bevy::prelude::*;

/// Spawns the workshop tab content
///
/// Currently displays a placeholder while Steam Workshop
/// integration is being implemented.
pub fn spawn_workshop_tab(parent: &mut ChildSpawnerCommands) {
    // Workshop content placeholder
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .with_children(|workshop| {
            spawn_workshop_header(workshop);
            spawn_workshop_description(workshop);
            spawn_coming_soon_notice(workshop);
        });
}

/// Spawns the workshop header text
fn spawn_workshop_header(workshop: &mut ChildSpawnerCommands) {
    workshop.spawn((
        Text::new("Steam Workshop"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(colors::TEXT_PRIMARY),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    ));
}

/// Spawns the workshop description text
fn spawn_workshop_description(workshop: &mut ChildSpawnerCommands) {
    workshop.spawn((
        Text::new("Browse and subscribe to community mods"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(colors::TEXT_SECONDARY),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));
}

/// Spawns the coming soon notice
fn spawn_coming_soon_notice(workshop: &mut ChildSpawnerCommands) {
    workshop.spawn((
        Text::new("(Steam Workshop integration coming soon)"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(colors::TEXT_TERTIARY),
    ));
}

// TODO: Future workshop functionality
// - Browse workshop items with filters
// - Show mod ratings and subscriber counts
// - One-click subscribe/unsubscribe
// - Preview mod screenshots
// - View mod comments and discussions
// - Sort by popularity, recent, trending