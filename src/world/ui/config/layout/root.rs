//! Root layout for world configuration UI
//!
//! This module creates the main configuration panel using UI builders.

use super::super::components::{
    AdvancedToggle, BackButton, GenerateButton, WorldConfigRoot,
};
use super::super::types::WorldGenerationSettings;
use crate::ui::colors;
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::prelude::*;

pub fn spawn_world_config_ui(mut commands: Commands, settings: Res<WorldGenerationSettings>) {
    debug!(
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
        // Use PanelBuilder with intrinsic sizing - no fixed height!
        PanelBuilder::new()
            .style(PanelStyle::Elevated)
            .width(Val::Px(900.0))              // Width constraint for readability
            .max_height(Val::Vh(90.0))          // Safety valve - never bigger than 90% viewport
            .padding(UiRect::all(Val::Px(24.0))) // Reduced from 40px for space efficiency
            .flex_direction(FlexDirection::Column)
            // NO HEIGHT SPECIFIED! Content determines it naturally
            .build_with_children(parent, |panel| {
                // Title
                panel.spawn((
                    Text::new("Configure New World"),
                    TextFont {
                        font_size: 36.0,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));

                // Scrollable content container with consistent spacing
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(16.0),          // Consistent spacing between sections
                        overflow: Overflow::scroll_y(),   // Handle overflow gracefully
                        flex_grow: 1.0,                  // Take available space
                        margin: UiRect::vertical(Val::Px(16.0)), // Top/bottom spacing from title/buttons
                        padding: UiRect::bottom(Val::Px(20.0)),   // Extra bottom padding for last item
                        ..default()
                    },
                )).with_children(|content| {
                    // World Preview Info Section - using PanelBuilder
                    PanelBuilder::new()
                        .style(PanelStyle::Light)
                        .width(Val::Percent(100.0))
                        .padding(UiRect::all(Val::Px(12.0))) // Reduced padding
                        .build_with_children(content, |info| {
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
                    super::spawn_world_name_section(content, &settings.world_name);

                    // World Size Section
                    super::spawn_world_size_section(content);

                    // Seed Section
                    super::spawn_seed_section(content, settings.seed);

                    // Preset Section
                    super::spawn_preset_section(content);

                    // Advanced Settings Toggle - using ButtonBuilder properly
                    ButtonBuilder::new("Show Advanced Settings")
                        .style(ButtonStyle::Secondary)
                        .size(ButtonSize::Large)
                        .with_marker(AdvancedToggle)  // Only use the marker needed for interaction
                        .build(content);

                    // Advanced Settings Panel
                    super::spawn_advanced_panel(content);

                    // Generation time estimate - using PanelBuilder
                    PanelBuilder::new()
                        .style(PanelStyle::Light)
                        .width(Val::Percent(100.0))
                        .padding(UiRect::all(Val::Px(8.0))) // Reduced padding
                        .build_with_children(content, |estimate| {
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
                });

                // Fixed bottom buttons (always visible)
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        // No margin - spacing handled by parent gap
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
        commands.entity(entity).despawn();
    }
    debug!("Despawned world configuration UI");
}
