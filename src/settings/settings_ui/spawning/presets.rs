//! Graphics Preset Button Spawner
//!
//! Handles creation of the graphics quality preset buttons.

use crate::settings::{components::*, types::*};
use crate::ui::{styles::colors, ChildBuilder};
use bevy::prelude::*;

/// Spawns the graphics preset buttons
pub fn spawn_graphics_presets(parent: &mut ChildBuilder, settings: &GraphicsSettings) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|presets| {
            // Label
            presets.spawn((
                Text::new("Quality Presets:"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                Node {
                    margin: UiRect::right(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Preset buttons
            for preset in [
                GraphicsPreset::Low,
                GraphicsPreset::Medium,
                GraphicsPreset::High,
                GraphicsPreset::Ultra,
            ] {
                let is_active = settings.current_preset() == Some(preset);
                let preset_text = match preset {
                    GraphicsPreset::Low => "Low",
                    GraphicsPreset::Medium => "Medium",
                    GraphicsPreset::High => "High",
                    GraphicsPreset::Ultra => "Ultra",
                };

                let mut entity_commands = presets.spawn((
                    Button,
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(if is_active {
                        colors::SURFACE_SELECTED
                    } else {
                        colors::SECONDARY
                    }),
                    BorderColor(if is_active {
                        colors::BORDER_SELECTED
                    } else {
                        colors::BORDER_DEFAULT
                    }),
                    PresetButton { preset },
                    Focusable {
                        order: preset as u32,
                    },
                ));

                entity_commands.with_children(|btn| {
                    btn.spawn((
                        Text::new(preset_text),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
            }

            // Show "Custom" indicator if no preset matches
            if settings.current_preset().is_none() {
                presets.spawn((
                    Text::new("(Custom)"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(colors::WARNING),
                    Node {
                        margin: UiRect::left(Val::Px(10.0)),
                        ..default()
                    },
                ));
            }
        });
}
