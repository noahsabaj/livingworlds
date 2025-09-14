//! Preset selection layout
//!
//! This module creates the UI for world preset selection.

use super::super::components::{PresetButton, PresetDescription, PresetDescriptionText};
use super::super::types::WorldPreset;
use crate::ui::{colors, dimensions, helpers};
use crate::ui::{PanelBuilder, PanelStyle};
use bevy::prelude::*;

pub fn spawn_preset_section(parent: &mut ChildSpawnerCommands) {
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
            Text::new("Quick Presets"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(colors::TEXT_SECONDARY),
        ));

        // Preset description panel - using PanelBuilder
        PanelBuilder::new()
            .style(PanelStyle::Light)
            .width(Val::Percent(100.0))
            .height(Val::Px(30.0))
            .padding(UiRect::all(Val::Px(10.0)))
            .margin(UiRect::bottom(Val::Px(5.0)))
            .build_with_children(section, |desc_box| {
                desc_box.spawn((
                    Text::new("Hover over a preset to see its description"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_MUTED),
                    PresetDescriptionText,
                ));
            });

        // Preset buttons (2 rows)
        for row_presets in [
            vec![
                (WorldPreset::Balanced, "Balanced", "Default settings for a well-rounded experience"),
                (WorldPreset::Pangaea, "Pangaea", "One massive supercontinent surrounded by ocean"),
                (WorldPreset::Archipelago, "Archipelago", "Scattered islands connected by trade routes"),
            ],
            vec![
                (WorldPreset::IceAge, "Ice Age", "Frozen world with harsh survival conditions"),
                (WorldPreset::DesertWorld, "Desert", "Arid landscape with rare fertile oases"),
                (WorldPreset::Custom, "Custom", "Your personalized world settings"),
            ],
        ] {
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|row| {
                for (preset, label, desc) in row_presets {
                    // Create preset button
                    row.spawn((
                        Button,
                        Node {
                            flex_grow: 1.0,
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: helpers::standard_border(),
                            ..default()
                        },
                        BackgroundColor(if preset == WorldPreset::Balanced {
                            colors::PRIMARY
                        } else {
                            colors::BACKGROUND_LIGHT
                        }),
                        BorderColor(if preset == WorldPreset::Balanced {
                            colors::PRIMARY
                        } else {
                            colors::BORDER_DEFAULT
                        }),
                        BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
                        PresetButton(preset),
                        PresetDescription(desc.to_string()),
                    )).with_children(|button| {
                        button.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(if preset == WorldPreset::Balanced {
                                Color::WHITE
                            } else {
                                colors::TEXT_PRIMARY
                            }),
                        ));
                    });
                }
            });
        }

        // Help text
        section.spawn((
            Text::new("Presets automatically configure all settings for specific gameplay experiences."),
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
