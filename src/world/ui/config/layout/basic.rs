//! Basic settings layout (name, size, seed)
//!
//! This module creates the UI for basic world configuration settings.

use bevy::prelude::*;
use crate::ui::{ButtonBuilder, ButtonStyle, ButtonSize};
use crate::ui::{text_input, FocusGroupId};
use crate::ui::{colors, dimensions};
use crate::resources::WorldSize;
use super::super::components::*;

pub fn spawn_world_name_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Name"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));

        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            // Use our text input builder
            text_input()
                .with_value("Aetheria Prime")
                .with_font_size(18.0)
                .with_width(Val::Px(300.0))
                .with_padding(UiRect::horizontal(Val::Px(15.0)))
                .with_max_length(30)
                .with_focus_group(FocusGroupId::WorldConfig)
                .inactive()
                .with_marker(WorldNameInput)
                .and_marker(WorldNameText)
                .build(row);

            // Random button using ButtonBuilder
            ButtonBuilder::new("Random")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomNameButton)
                .build(row);
        });

        // Help text
        section.spawn((
            Text::new("Give your world a unique identity. The name will appear in game history."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

pub fn spawn_world_size_section(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Size"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));

        // Size buttons row
        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            for (size, label, desc, provinces) in [
                (WorldSize::Small, "Small", "Quick games", "1,000,000 provinces"),
                (WorldSize::Medium, "Medium", "Balanced", "2,000,000 provinces"),
                (WorldSize::Large, "Large", "Epic scale", "3,000,000 provinces"),
            ] {
                // Create a custom button with multiple text lines
                row.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        height: Val::Px(65.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                        ..default()
                    },
                    BackgroundColor(if size == WorldSize::Medium {
                        colors::PRIMARY
                    } else {
                        colors::BACKGROUND_LIGHT
                    }),
                    BorderColor(if size == WorldSize::Medium {
                        colors::PRIMARY
                    } else {
                        colors::BORDER_DEFAULT
                    }),
                    BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
                    SizeButton(size),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::WHITE
                        } else {
                            colors::TEXT_PRIMARY
                        }),
                    ));
                    button.spawn((
                        Text::new(desc),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::srgba(1.0, 1.0, 1.0, 0.8)
                        } else {
                            colors::TEXT_MUTED
                        }),
                    ));
                    button.spawn((
                        Text::new(provinces),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(if size == WorldSize::Medium {
                            Color::srgba(1.0, 1.0, 1.0, 0.9)
                        } else {
                            colors::TEXT_SECONDARY
                        }),
                    ));
                });
            }
        });

        // Help text
        section.spawn((
            Text::new("Larger worlds offer more strategic depth but take longer to generate and simulate."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}

pub fn spawn_seed_section(parent: &mut ChildSpawnerCommands, seed: u32) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
    )).with_children(|section| {
        // Label
        section.spawn((
            Text::new("World Seed"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));

        section.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                column_gap: Val::Px(10.0),
                ..default()
            },
        )).with_children(|row| {
            // Use our text input builder
            text_input()
                .with_value(seed.to_string())
                .with_font_size(18.0)
                .with_width(Val::Px(250.0))
                .with_padding(UiRect::horizontal(Val::Px(15.0)))
                .with_focus_group(FocusGroupId::WorldConfig)
                .numeric_only()
                .with_max_length(20)
                .inactive()
                .with_marker(SeedInput)
                .and_marker(SeedText)
                .build(row);

            // Random button using ButtonBuilder
            ButtonBuilder::new("Random")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .with_marker(RandomSeedButton)
                .build(row);
        });

        // Help text
        section.spawn((
            Text::new("Same seed = same world generation. Share seeds with friends for identical worlds."),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(colors::TEXT_MUTED),
            Node {
                margin: UiRect::left(Val::Px(5.0)),
                ..default()
            },
        ));
    });
}