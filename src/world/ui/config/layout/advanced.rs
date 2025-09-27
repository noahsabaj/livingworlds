//! Advanced settings layout
//!
//! This module creates the UI for advanced world configuration settings.

use super::super::components::*;
use super::super::types::*;
use crate::ui::{colors, dimensions};
use crate::ui::{SliderBuilder, ValueFormat};
use crate::ui::{PanelBuilder, PanelStyle};
use bevy::prelude::*;

pub fn spawn_advanced_panel(parent: &mut ChildSpawnerCommands) {
    // Use PanelBuilder for the advanced panel
    let panel_entity = PanelBuilder::new()
        .style(PanelStyle::Light)
        .width(Val::Percent(100.0))
        .display(Display::None) // Initially hidden
        .padding(UiRect::all(Val::Px(20.0)))
        .build_with_children(parent, |panel| {
            // Title
            panel.spawn((
                Text::new("Advanced Settings"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Help text
            panel.spawn((
                Text::new("Fine-tune world generation parameters for a customized experience."),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Two columns
            PanelBuilder::new()
                .style(PanelStyle::Transparent)
                .width(Val::Percent(100.0))
                .flex_direction(FlexDirection::Row)
                .column_gap(Val::Px(40.0))
                .build_with_children(panel, |columns| {
                    // Left column: World Geography
                    spawn_geography_column(columns);

                    // Right column: Civilizations & Resources
                    spawn_civilizations_column(columns);
                });
        });

    // Add the AdvancedPanel marker to the panel entity
    parent.commands().entity(panel_entity).insert(AdvancedPanel);
}

fn spawn_geography_column(parent: &mut ChildSpawnerCommands) {
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .flex_basis(Val::Percent(50.0))
        .flex_direction(FlexDirection::Column)
        .row_gap(Val::Px(dimensions::MARGIN_MEDIUM))
        .build_with_children(parent, |column| {
            // Section header using PanelBuilder
            PanelBuilder::new()
                .style(PanelStyle::Elevated)
                .width(Val::Percent(100.0))
                .padding(UiRect::all(Val::Px(10.0)))
                .margin(UiRect::bottom(Val::Px(10.0)))
                .build_with_children(column, |header| {
                    header.spawn((
                        Text::new("World Geography"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    header.spawn((
                        Text::new("Shape the physical world: continents, oceans, and terrain."),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                    ));
                });

            // Use our slider builders
            let slider_entity = SliderBuilder::new(1.0..12.0)
                .build(column);
            column.commands().entity(slider_entity).insert(ContinentSlider);

            let slider_entity = SliderBuilder::new(30.0..80.0)
                .build(column);
            column.commands().entity(slider_entity).insert(OceanSlider);

            let slider_entity = SliderBuilder::new(0.5..2.0)
                .build(column);
            column.commands().entity(slider_entity).insert(RiverSlider);

            // Climate Type Selection
            PanelBuilder::new()
                .style(PanelStyle::Transparent)
                .width(Val::Percent(100.0))
                .flex_direction(FlexDirection::Column)
                .row_gap(Val::Px(3.0))
                .build_with_children(column, |climate_section| {
                    spawn_selection_row(
                        climate_section,
                        "Climate Type",
                        vec![
                            ("Arctic", ClimateType::Arctic),
                            ("Temperate", ClimateType::Temperate),
                            ("Tropical", ClimateType::Tropical),
                            ("Desert", ClimateType::Desert),
                            ("Mixed", ClimateType::Mixed),
                        ],
                        ClimateType::Mixed,
                        |climate| ClimateButton(climate),
                    );
                    climate_section.spawn((
                        Text::new("Affects temperature, rainfall, and biome distribution."),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                        Node {
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                });

            // Island Frequency Selection
            spawn_selection_row(
                column,
                "Islands",
                vec![
                    ("None", IslandFrequency::None),
                    ("Sparse", IslandFrequency::Sparse),
                    ("Moderate", IslandFrequency::Moderate),
                    ("Abundant", IslandFrequency::Abundant),
                ],
                IslandFrequency::Moderate,
                |freq| IslandButton(freq),
            );
        });
}

fn spawn_civilizations_column(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((Node {
            flex_basis: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(dimensions::MARGIN_MEDIUM),
            ..default()
        },))
        .with_children(|column| {
            // Section header using PanelBuilder
            PanelBuilder::new()
                .style(PanelStyle::Elevated)
                .width(Val::Percent(100.0))
                .padding(UiRect::all(Val::Px(10.0)))
                .margin(UiRect::bottom(Val::Px(10.0)))
                .build_with_children(column, |header| {
                    header.spawn((
                        Text::new("Civilizations & Resources"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                    header.spawn((
                        Text::new("Configure nations, their behavior, and available resources."),
                        TextFont {
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                    ));
                });

            // Use our slider builders
            let slider_entity = SliderBuilder::new(2.0..20.0)
                .build(column);
            column.commands().entity(slider_entity).insert(StartingNationsSlider);

            let slider_entity = SliderBuilder::new(0.5..2.0)
                .build(column);
            column.commands().entity(slider_entity).insert(TechSpeedSlider);

            // Aggression Level Selection
            column
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.0),
                    ..default()
                },))
                .with_children(|aggression_section| {
                    spawn_selection_row(
                        aggression_section,
                        "Aggression",
                        vec![
                            ("Peaceful", AggressionLevel::Peaceful),
                            ("Balanced", AggressionLevel::Balanced),
                            ("Warlike", AggressionLevel::Warlike),
                            ("Chaotic", AggressionLevel::Chaotic),
                        ],
                        AggressionLevel::Balanced,
                        |aggr| AggressionButton(aggr),
                    );
                    aggression_section.spawn((
                        Text::new("How likely nations are to declare war and expand."),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_MUTED),
                        Node {
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..default()
                        },
                    ));
                });

            // Resource Abundance Selection
            spawn_selection_row(
                column,
                "Resources",
                vec![
                    ("Scarce", ResourceAbundance::Scarce),
                    ("Normal", ResourceAbundance::Normal),
                    ("Rich", ResourceAbundance::Rich),
                    ("Bountiful", ResourceAbundance::Bountiful),
                ],
                ResourceAbundance::Normal,
                |res| ResourceButton(res),
            );
        });
}

// Generic selection row builder
fn spawn_selection_row<T, F, C>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    options: Vec<(&str, T)>,
    default_value: T,
    make_component: F,
) where
    T: Clone + PartialEq + 'static + Send + Sync,
    F: Fn(T) -> C,
    C: Component,
{
    parent
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },))
        .with_children(|control| {
            // Label
            control.spawn((
                Text::new(label),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));

            // Options row
            control
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.0),
                    ..default()
                },))
                .with_children(|row| {
                    for (option_label, value) in options {
                        let is_selected = value == default_value;
                        row.spawn((
                            Button,
                            Node {
                                flex_grow: 1.0,
                                height: Val::Px(35.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(if is_selected {
                                colors::PRIMARY
                            } else {
                                Color::srgb(0.15, 0.15, 0.15)
                            }),
                            BorderRadius::all(Val::Px(dimensions::CORNER_RADIUS)),
                            make_component(value.clone()),
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new(option_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                    }
                });
        });
}
