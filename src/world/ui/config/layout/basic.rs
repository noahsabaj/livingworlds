//! Basic settings layout (name, size, seed, calendar)
//!
//! This module creates the UI for basic world configuration settings.

use super::super::components::*;
use crate::resources::WorldSize;
use crate::simulation::CalendarRegistry;
use crate::ui::{colors, dimensions};
use crate::ui::{text_input, FocusGroupId};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::prelude::*;

pub fn spawn_world_name_section(parent: &mut ChildSpawnerCommands, world_name: &str) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .width(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .row_gap(Val::Px(5.0))
        .padding(UiRect::all(Val::Px(0.0))) // Zero padding to align with other sections
        .build_with_children(parent, |section| {
            // Label
            section.spawn((
                Text::new("World Name"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            PanelBuilder::new()
                .style(PanelStyle::Transparent)
                .width(Val::Percent(100.0))
                .flex_direction(FlexDirection::Row)
                .justify_content(JustifyContent::SpaceBetween)
                .column_gap(Val::Px(10.0))
                .padding(UiRect::all(Val::Px(0.0))) // Zero padding to align with other sections
                .build_with_children(section, |row| {
                    // Use our text input builder
                    text_input()
                        .with_value(world_name)
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
                    let button = ButtonBuilder::new("Random")
                        .style(ButtonStyle::Secondary)
                        .size(ButtonSize::Small)
                        .build(row);
                    row.commands().entity(button).insert(RandomNameButton);
                });

            // Help text
            section.spawn((
                Text::new(
                    "Give your world a unique identity. The name will appear in game history.",
                ),
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

        // Size buttons row - using bevy-ui-builders v0.2.1 selection system
        let size_group = section.commands().spawn(()).id();

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
                let is_selected = size == WorldSize::Medium;

                // Create button - ButtonBuilder will create internal structure
                // We'll pass empty string and add our own text children
                let button = ButtonBuilder::new("")
                    .width(Val::Percent(33.0))
                    .height(Val::Px(65.0))
                    .selected(is_selected)
                    .in_group(size_group)
                    .build(row);

                row.commands().entity(button).insert(SizeButton(size));

                // Add custom text children with column layout
                // Note: We need to find the text container child and modify it
                row.commands().entity(button).with_children(|container| {
                    // Spawn a column layout container for our text
                    container.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                    )).with_children(|text_column| {
                        text_column.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                        ));
                        text_column.spawn((
                            Text::new(desc),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_MUTED),
                        ));
                        text_column.spawn((
                            Text::new(provinces),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    });
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
                .with_value(&seed.to_string())
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
            let button = ButtonBuilder::new("Random")
                .style(ButtonStyle::Secondary)
                .size(ButtonSize::Small)
                .build(row);
            row.commands().entity(button).insert(RandomSeedButton);
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

pub fn spawn_calendar_selection_section(
    parent: &mut ChildSpawnerCommands,
    calendar_id: &str,
    calendar_registry: &CalendarRegistry,
) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .width(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .row_gap(Val::Px(5.0))
        .padding(UiRect::all(Val::Px(0.0)))
        .build_with_children(parent, |section| {
            // Label
            section.spawn((
                Text::new("Calendar System"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            // Calendar button grid with wrapping support
            // Using bevy-ui-builders v0.2.1 selection system (radio button group)

            // Create a group entity for exclusive radio button selection
            let calendar_group = section.commands().spawn(()).id();

            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,  // Enable multi-row wrapping
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(8.0),
                    row_gap: Val::Px(8.0),  // Spacing between wrapped rows
                    ..default()
                },
            )).with_children(|grid| {
                // Create selectable buttons using bevy-ui-builders v0.2.1 selection system
                for (cal_id, calendar) in &calendar_registry.calendars {
                    let is_selected = cal_id == calendar_id;

                    let button = ButtonBuilder::new(&calendar.name)
                        .size(ButtonSize::Small)
                        .width(Val::Px(220.0))
                        .height(Val::Px(40.0))
                        .selected(is_selected)  // Set initial selection
                        .in_group(calendar_group)  // Radio button group (exclusive selection)
                        .build(grid);

                    grid.commands()
                        .entity(button)
                        .insert(CalendarButton(cal_id.clone()));
                }
            });

            // Calendar preview panel
            if let Some(calendar) = calendar_registry.get_calendar(calendar_id) {
                PanelBuilder::new()
                    .style(PanelStyle::Dark)
                    .width(Val::Percent(100.0))
                    .flex_direction(FlexDirection::Column)
                    .row_gap(Val::Px(4.0))
                    .padding(UiRect::all(Val::Px(12.0)))
                    .build_with_children(section, |preview| {
                        preview.spawn((
                            Text::new(format!("{} - {} days/year", calendar.name, calendar.days_per_year())),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            CalendarPreviewName,
                        ));

                        // Show period names (limited to first 6 to keep UI compact)
                        let period_names: Vec<String> = calendar
                            .periods
                            .iter()
                            .take(6)
                            .map(|p| p.name.clone())
                            .collect();
                        let period_display = if calendar.periods.len() > 6 {
                            format!("{}, ... ({} total)", period_names.join(", "), calendar.periods.len())
                        } else {
                            period_names.join(", ")
                        };

                        preview.spawn((
                            Text::new(format!("Periods: {}", period_display)),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_MUTED),
                            CalendarPreviewPeriods,
                        ));
                    });
            }

            // Help text
            section.spawn((
                Text::new("Choose how time is displayed in your world. All calendars use the same simulation speed."),
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

pub fn spawn_starting_year_section(parent: &mut ChildSpawnerCommands, starting_year: u32) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .width(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .row_gap(Val::Px(5.0))
        .padding(UiRect::all(Val::Px(0.0)))
        .build_with_children(parent, |section| {
            // Label
            section.spawn((
                Text::new("Starting Year"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            // Year input
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|row| {
                text_input()
                    .with_value(&starting_year.to_string())
                    .with_font_size(18.0)
                    .with_width(Val::Px(150.0))
                    .with_padding(UiRect::horizontal(Val::Px(15.0)))
                    .with_focus_group(FocusGroupId::WorldConfig)
                    .numeric_only()
                    .with_max_length(6)
                    .inactive()
                    .with_marker(StartingYearInput)
                    .and_marker(StartingYearText)
                    .build(row);
            });

            // Help text
            section.spawn((
                Text::new("The year when your world begins. Affects how dates are displayed."),
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
