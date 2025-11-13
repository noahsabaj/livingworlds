//! Mineral legend display for resource overlays

use crate::ui::{ChildBuilder, LabelBuilder, PanelBuilder, PanelStyle};
use crate::world::MineralType;
use crate::resources::MapMode;
use crate::ui::colors;
use bevy::prelude::*;

/// Marker component for the mineral legend container
#[derive(Component)]
pub struct MineralLegendContainer;

/// Spawn the mineral legend
pub fn spawn_mineral_legend(parent: &mut ChildBuilder) {
    let panel_entity = PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .flex_direction(FlexDirection::Column)
        .display(Display::None) // Start hidden
        .build_with_children(parent, |container| {
            // Title using LabelBuilder
            LabelBuilder::new("Mineral Legend:")
                .font_size(14.0)
                .color(colors::TEXT_PRIMARY)
                .margin(UiRect::bottom(Val::Px(4.0)))
                .build(container);

            // Define minerals with their colors and chemical symbols
            let minerals = [
                (MineralType::Iron, "Fe", Color::srgb(0.7, 0.3, 0.2)), // Rusty brown
                (MineralType::Copper, "Cu", Color::srgb(0.7, 0.4, 0.2)), // Copper orange
                (MineralType::Tin, "Sn", Color::srgb(0.6, 0.6, 0.7)),  // Silver-grey
                (MineralType::Gold, "Au", Color::srgb(1.0, 0.84, 0.0)), // Gold
                (MineralType::Coal, "C", Color::srgb(0.2, 0.2, 0.2)),  // Black
                (MineralType::Stone, "Si", Color::srgb(0.5, 0.5, 0.5)), // Grey
                (MineralType::Gems, "Gm", Color::srgb(0.5, 0.2, 0.9)), // Purple
            ];

            for (_mineral_type, symbol, color) in minerals.iter() {
                spawn_mineral_row(container, symbol, color, get_mineral_name(symbol));
            }
        });

    parent
        .commands()
        .entity(panel_entity)
        .insert(MineralLegendContainer);
}

/// Spawn a single mineral legend row
fn spawn_mineral_row(parent: &mut ChildBuilder, symbol: &str, color: &Color, name: &str) {
    // Row container using PanelBuilder
    PanelBuilder::new()
        .style(PanelStyle::Transparent)
        .flex_direction(FlexDirection::Row)
        .align_items(AlignItems::Center)
        .margin(UiRect::bottom(Val::Px(2.0)))
        .build_with_children(parent, |row| {
            // Colored square with border
            row.spawn((
                Node {
                    width: Val::Px(16.0),
                    height: Val::Px(16.0),
                    margin: UiRect::right(Val::Px(6.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(*color),
                BorderColor::all(colors::BORDER_DEFAULT),
            ))
            .with_children(|square| {
                // Chemical symbol using LabelBuilder
                LabelBuilder::new(symbol)
                    .font_size(10.0)
                    .color(Color::WHITE)
                    .build(square);
            });

            // Mineral name using LabelBuilder
            LabelBuilder::new(name)
                .font_size(12.0)
                .color(colors::TEXT_SECONDARY)
                .build(row);
        });
}

/// Get mineral name from symbol
fn get_mineral_name(symbol: &str) -> &'static str {
    match symbol {
        "Fe" => "Iron",
        "Cu" => "Copper",
        "Sn" => "Tin",
        "Au" => "Gold",
        "C" => "Coal",
        "Si" => "Stone",
        _ => "Gems",
    }
}

/// Update mineral legend visibility based on current overlay
pub fn update_mineral_legend_visibility(
    map_mode: Res<MapMode>,
    mut legend_query: Query<&mut Node, With<MineralLegendContainer>>,
) {
    if let Ok(mut node) = legend_query.single_mut() {
        // Only show legend when viewing mineral overlays
        node.display = if map_mode.is_mineral_mode() {
            Display::Flex
        } else {
            Display::None
        };
    }
}
