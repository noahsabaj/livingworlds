//! Advanced settings layout
//!
//! This module creates the UI for advanced world configuration settings.

use super::super::components::*;
use super::super::types::*;
use crate::ui::colors;
use crate::ui::{SliderBuilder, ValueFormat};
use crate::ui::{ButtonBuilder, ButtonSize, PanelBuilder, PanelStyle};
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
                    margin: UiRect::bottom(Val::Px(10.0)),
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
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Two columns layout
            panel
                .spawn((Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    ..default()
                },))
                .with_children(|columns| {
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
    parent
        .spawn((Node {
            width: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|column| {
            // Section header
            column.spawn((
                Text::new("World Geography"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));
            column.spawn((
                Text::new("Shape the physical world: continents, oceans, and terrain."),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Continent count slider
            let slider_entity = SliderBuilder::new(1.0..12.0)
                .label("Continents")
                .value(7.0)
                .step(1.0)
                .format(ValueFormat::Integer)
                .width(Val::Percent(100.0))
                .build(column);
            column.commands().entity(slider_entity).insert(ContinentSlider);

            // Ocean coverage slider
            let slider_entity = SliderBuilder::new(30.0..80.0)
                .label("Ocean Coverage")
                .value(60.0)
                .step(5.0)
                .format(ValueFormat::Custom(|v| format!("{}%", v as i32)))
                .width(Val::Percent(100.0))
                .build(column);
            column.commands().entity(slider_entity).insert(OceanSlider);

            // River density slider
            let slider_entity = SliderBuilder::new(0.5..2.0)
                .label("River Density")
                .value(1.0)
                .step(0.1)
                .format(ValueFormat::Custom(|v| format!("{:.1}x", v)))
                .width(Val::Percent(100.0))
                .build(column);
            column.commands().entity(slider_entity).insert(RiverSlider);

            // Climate Type Selection
            spawn_selection_row(
                column,
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
            column.spawn((
                Text::new("Affects temperature, rainfall, and biome distribution."),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

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
            width: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .with_children(|column| {
            // Section header
            column.spawn((
                Text::new("Civilizations & Resources"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));
            column.spawn((
                Text::new("Configure nations, their behavior, and available resources."),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));

            // Starting nations slider
            let slider_entity = SliderBuilder::new(2.0..20.0)
                .label("Starting Nations")
                .value(8.0)
                .step(1.0)
                .format(ValueFormat::Integer)
                .width(Val::Percent(100.0))
                .build(column);
            column.commands().entity(slider_entity).insert(StartingNationsSlider);

            // Tech progression speed slider
            let slider_entity = SliderBuilder::new(0.5..2.0)
                .label("Tech Speed")
                .value(1.0)
                .step(0.1)
                .format(ValueFormat::Custom(|v| format!("{:.1}x", v)))
                .width(Val::Percent(100.0))
                .build(column);
            column.commands().entity(slider_entity).insert(TechSpeedSlider);

            // Aggression Level Selection
            spawn_selection_row(
                column,
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
            column.spawn((
                Text::new("How likely nations are to declare war and expand."),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

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

// Generic selection row builder - using bevy-ui-builders v0.2.1 selection system
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

            // Create group entity for exclusive radio button selection
            let group = control.commands().spawn(()).id();

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

                        let button = ButtonBuilder::new(option_label)
                            .size(ButtonSize::Small)
                            .selected(is_selected)
                            .in_group(group)
                            .build(row);

                        row.commands()
                            .entity(button)
                            .insert(make_component(value.clone()));
                    }
                });
        });
}
