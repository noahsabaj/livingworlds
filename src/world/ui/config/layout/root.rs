//! Root layout for world configuration UI
//!
//! This module creates the main configuration panel using UI builders.

use super::super::components::{
    AdvancedToggle, AdvancedToggleText, BackButton, GenerateButton, WorldConfigRoot,
};
use super::super::types::WorldGenerationSettings;
use crate::states::GameState;
use crate::ui::{colors, dimensions};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::prelude::*;

pub fn spawn_world_config_ui(mut commands: Commands, settings: Res<WorldGenerationSettings>) {
    println!(
        "Spawning world configuration UI with seed: {}",
        settings.seed
    );

    // Root container with dark overlay
    commands.spawn((
        Button, // Block clicks behind
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(colors::OVERLAY_DARK),
        WorldConfigRoot,
    )).with_children(|parent| {
        // Use PanelBuilder for the main configuration panel
        PanelBuilder::new()
            .style(PanelStyle::Elevated)
            .width(Val::Px(1000.0))
            .height(Val::Px(700.0))
            .padding(UiRect::all(Val::Px(40.0)))
            .build_with_children(parent, |panel| {
                // Title
                panel.spawn((
                    Text::new("Configure New World"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));

                // World Preview Info Section - using PanelBuilder
                PanelBuilder::new()
                    .style(PanelStyle::Light)
                    .width(Val::Percent(100.0))
                    .padding(UiRect::all(Val::Px(15.0)))
                    .margin(UiRect::bottom(Val::Px(15.0)))
                    .build_with_children(panel, |info| {
                        info.spawn((
                            Text::new("World Preview"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                        info.spawn((
                            Text::new("• Estimated land coverage: ~40%\n• Starting civilizations: 8 nations\n• World complexity: Moderate"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            super::super::components::WorldPreviewText,
                        ));
                    });

                // World Name Section
                super::spawn_world_name_section(panel);

                // World Size Section
                super::spawn_world_size_section(panel);

                // Seed Section
                super::spawn_seed_section(panel, settings.seed);

                // Preset Section
                super::spawn_preset_section(panel);

                // Advanced Settings Toggle - using ButtonBuilder properly
                ButtonBuilder::new("Show Advanced Settings")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Large)
                    .with_marker(AdvancedToggle)
                    .with_marker(AdvancedToggleText)
                    .margin(UiRect::vertical(Val::Px(10.0)))
                    .build(panel);

                // Advanced Settings Panel
                super::spawn_advanced_panel(panel);

                // Generation time estimate - using PanelBuilder
                PanelBuilder::new()
                    .style(PanelStyle::Light)
                    .width(Val::Percent(100.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .margin(UiRect::top(Val::Px(20.0)))
                    .build_with_children(panel, |estimate| {
                        estimate.spawn((
                            Text::new("Estimated generation time: ~3-7 seconds"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_MUTED),
                            super::super::components::GenerationTimeEstimate,
                        ));
                    });

                // Bottom buttons
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        margin: UiRect::top(Val::Px(15.0)),
                        ..default()
                    },
                )).with_children(|buttons| {
                    // Back button
                    ButtonBuilder::new("Back")
                        .style(ButtonStyle::Secondary)
                        .size(ButtonSize::Large)
                        .with_marker(BackButton)
                        .build(buttons);

                    // Generate World button
                    ButtonBuilder::new("Generate World")
                        .style(ButtonStyle::Primary)
                        .size(ButtonSize::Large)
                        .with_marker(GenerateButton)
                        .build(buttons);
                });
            });
    });
}

pub fn despawn_world_config_ui(
    mut commands: Commands,
    query: Query<Entity, With<WorldConfigRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    println!("Despawned world configuration UI");
}
