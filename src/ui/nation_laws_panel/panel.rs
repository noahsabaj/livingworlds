//! Nation laws panel UI spawning
//!
//! Creates the visual interface for displaying active and proposed laws.

use bevy::prelude::*;
use crate::ui::styles::{colors, dimensions};
use super::types::*;

/// Spawn the nation laws panel
pub fn spawn_nation_laws_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(20.0),
                width: Val::Px(450.0),
                max_height: Val::Vh(80.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_LARGE)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_MEDIUM),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(colors::BACKGROUND_DARKER),
            BorderColor::all(colors::BORDER_ACTIVE),
            ZIndex(90), // Below law browser (100) but above other UI
            NationLawsPanel,
        ))
        .with_children(|panel| {
            // Header with title and close button
            spawn_panel_header(panel);

            // Combined effects summary
            spawn_combined_effects_section(panel);

            // Active laws section
            spawn_active_laws_section(panel);

            // Separator
            panel.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(dimensions::BORDER_WIDTH),
                    margin: UiRect::vertical(Val::Px(dimensions::SPACING_MEDIUM)),
                    ..default()
                },
                BackgroundColor(colors::BORDER),
            ));

            // Proposed laws section
            spawn_proposed_laws_section(panel);
        });
}

/// Spawn the panel header
fn spawn_panel_header(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(colors::SURFACE),
        ))
        .with_children(|header| {
            // Title
            header.spawn((
                Text::new("NATION LAWS"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_TITLE,
                    ..default()
                },
                TextColor(colors::TEXT_TITLE),
                PanelTitleText,
            ));

            // Close button
            header
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                        ..default()
                    },
                    BackgroundColor(colors::DANGER),
                    BorderColor::all(colors::BORDER),
                    ClosePanelButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("×"),
                        TextFont {
                            font_size: dimensions::FONT_SIZE_LARGE,
                            ..default()
                        },
                        TextColor(colors::TEXT_PRIMARY),
                    ));
                });
        });
}

/// Spawn combined effects section
fn spawn_combined_effects_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(dimensions::SPACING_MEDIUM)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                border: UiRect::all(Val::Px(dimensions::BORDER_WIDTH_THIN)),
                ..default()
            },
            BackgroundColor(colors::SURFACE_DARK),
            BorderColor::all(colors::BORDER),
        ))
        .with_children(|section| {
            // Header
            section.spawn((
                Text::new("COMBINED LAW EFFECTS"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
            ));

            // Effects text (will be updated dynamically)
            section.spawn((
                Text::new("No laws enacted"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
                CombinedEffectsText,
            ));
        });
}

/// Spawn active laws section
fn spawn_active_laws_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|section| {
            // Header
            section.spawn((
                Text::new("ACTIVE LAWS"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                ActiveLawsHeader,
            ));

            // Container for active laws list (populated dynamically)
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(dimensions::SPACING_TINY),
                    padding: UiRect::left(Val::Px(dimensions::SPACING_MEDIUM)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                ActiveLawsContainer,
            ));
        });
}

/// Spawn proposed laws section
fn spawn_proposed_laws_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(dimensions::SPACING_SMALL),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|section| {
            // Header
            section.spawn((
                Text::new("PROPOSED LAWS"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_NORMAL,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                ProposedLawsHeader,
            ));

            // Container for proposed laws list (populated dynamically)
            section.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(dimensions::SPACING_SMALL),
                    padding: UiRect::left(Val::Px(dimensions::SPACING_MEDIUM)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
                ProposedLawsContainer,
            ));
        });
}