//! Preset selection layout
//!
//! This module creates the UI for world preset selection.

use super::super::components::{PresetButton, PresetDescription, PresetDescriptionText};
use super::super::types::WorldPreset;
use crate::ui::{colors, dimensions, helpers};
use crate::ui::{ButtonBuilder, ButtonSize, PanelBuilder, PanelStyle};
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

        // Preset buttons (2 rows) - using bevy-ui-builders v0.2.1 selection system
        let preset_group = section.commands().spawn(()).id();

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
                    let is_selected = preset == WorldPreset::Balanced;

                    let button = ButtonBuilder::new(label)
                        .size(ButtonSize::Medium)
                        .width(Val::Percent(33.0))  // Equal width for 3 buttons per row
                        .selected(is_selected)
                        .in_group(preset_group)
                        .build(row);

                    row.commands()
                        .entity(button)
                        .insert((PresetButton(preset), PresetDescription(desc.to_string())));
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
