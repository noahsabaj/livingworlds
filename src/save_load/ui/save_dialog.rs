//! Save dialog UI implementation
//!
//! This module creates the save dialog UI using our standard UI builders.

use super::components::*;
use super::{CloseSaveDialogEvent, OpenSaveDialogEvent, SaveGameEvent};
use super::{SaveDialogState, SaveGameList};
use crate::resources::{WorldName, WorldSeed};
use crate::ui::{colors, helpers, TextInputBuilder};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::prelude::*;
use bevy_simple_text_input::TextInputValue;
use chrono::Local;

/// Handle opening the save dialog
pub fn handle_open_save_dialog(
    mut events: EventReader<OpenSaveDialogEvent>,
    mut commands: Commands,
    mut dialog_state: ResMut<SaveDialogState>,
    mut save_list: ResMut<SaveGameList>,
    world_seed: Option<Res<WorldSeed>>,
    world_name: Option<Res<WorldName>>,
) {
    for _ in events.read() {
        if !dialog_state.is_open {
            dialog_state.is_open = true;
            dialog_state.selected_save = None;
            dialog_state.search_filter.clear();

            // Scan for existing saves
            super::scan_save_files_internal(&mut save_list);

            // Generate default save name
            let timestamp = Local::now().format("%Y%m%d_%H%M%S");
            let default_name = format!("save_{}", timestamp);

            let world_name_str = world_name
                .as_ref()
                .map(|n| n.0.clone())
                .unwrap_or_else(|| "Unnamed World".to_string());
            let world_seed_val = world_seed.as_ref().map(|s| s.0).unwrap_or(0);

            // Create save dialog with modal overlay that blocks clicks
            let overlay_entity = helpers::spawn_modal_overlay(
                &mut commands,
                Color::srgba(0.0, 0.0, 0.0, 0.7),
                ZIndex(200),
            );

            // Add our root marker
            commands.entity(overlay_entity).insert(SaveDialogRoot);

            // Add dialog content
            commands.entity(overlay_entity).with_children(|overlay| {
                    // Dialog container
                    overlay
                        .spawn((
                            Node {
                                width: Val::Px(900.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(15.0),
                                ..default()
                            },
                            BackgroundColor(colors::BACKGROUND_MEDIUM),
                            BorderColor(colors::BORDER),
                        ))
                        .with_children(|parent| {
                            // Title
                            parent.spawn((
                                Text::new("Save Game"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_PRIMARY),
                                Node {
                                    margin: UiRect::bottom(Val::Px(10.0)),
                                    ..default()
                                },
                            ));
                            // World info row
                            PanelBuilder::new()
                                .style(PanelStyle::Transparent)
                                .flex_direction(FlexDirection::Row)
                                .margin(UiRect::bottom(Val::Px(15.0)))
                                .column_gap(Val::Px(20.0))
                                .build_with_children(parent, |info_parent| {
                                    // World name
                                    info_parent.spawn((
                                        Text::new(format!("World: {}", world_name_str)),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_SECONDARY),
                                    ));

                                    // Seed
                                    info_parent.spawn((
                                        Text::new(format!("Seed: {}", world_seed_val)),
                                        TextFont {
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_SECONDARY),
                                    ));
                                });

                            // Save name input section
                            PanelBuilder::new()
                                .style(PanelStyle::Transparent)
                                .flex_direction(FlexDirection::Column)
                                .margin(UiRect::bottom(Val::Px(20.0)))
                                .build_with_children(parent, |section| {
                                    // Label
                                    section.spawn((
                                        Text::new("Save Name:"),
                                        TextFont {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(colors::TEXT_PRIMARY),
                                        Node {
                                            margin: UiRect::bottom(Val::Px(8.0)),
                                            ..default()
                                        },
                                    ));

                                    // Use our TextInputBuilder
                                    TextInputBuilder::new()
                                        .with_value(default_name)
                                        .with_placeholder("Enter save name...")
                                        .with_width(Val::Px(850.0))
                                        .with_font_size(18.0)
                                        .retain_on_submit(true)
                                        .with_marker(SaveNameInput)
                                        .build(section);
                                });

                            // Bottom buttons
                            PanelBuilder::new()
                                .style(PanelStyle::Transparent)
                                .width(Val::Percent(100.0))
                                .justify_content(JustifyContent::Center)
                                .column_gap(Val::Px(20.0))
                                .build_with_children(parent, |buttons| {
                                    ButtonBuilder::new("Save Game")
                                        .style(ButtonStyle::Primary)
                                        .size(ButtonSize::Large)
                                        .with_marker(SaveDialogConfirmButton)
                                        .build(buttons);

                                    ButtonBuilder::new("Cancel")
                                        .style(ButtonStyle::Secondary)
                                        .size(ButtonSize::Large)
                                        .with_marker(SaveDialogCancelButton)
                                        .build(buttons);
                                });
                        });
                });
        }
    }
}

/// Handle closing the save dialog
pub fn handle_close_save_dialog(
    mut events: EventReader<CloseSaveDialogEvent>,
    mut commands: Commands,
    mut dialog_state: ResMut<SaveDialogState>,
    dialog_query: Query<Entity, With<SaveDialogRoot>>,
) {
    for _ in events.read() {
        if dialog_state.is_open {
            dialog_state.is_open = false;

            // Despawn the dialog
            if let Ok(dialog_entity) = dialog_query.single() {
                commands.entity(dialog_entity).despawn();
            }
        }
    }
}

/// Handle save dialog interactions
pub fn handle_save_dialog_interactions(
    mut interactions: Query<
        (
            &Interaction,
            AnyOf<(&SaveDialogConfirmButton, &SaveDialogCancelButton)>,
        ),
        Changed<Interaction>,
    >,
    mut save_events: EventWriter<SaveGameEvent>,
    mut close_events: EventWriter<CloseSaveDialogEvent>,
    save_name_query: Query<&TextInputValue, With<SaveNameInput>>,
) {
    for (interaction, (confirm, cancel)) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if confirm.is_some() {
                if let Ok(save_name_value) = save_name_query.single() {
                    let save_name = save_name_value.0.trim();
                    if !save_name.is_empty() {
                        // Trigger save
                        save_events.write(SaveGameEvent {
                            slot_name: save_name.to_string(),
                        });

                        // Close dialog
                        close_events.write(CloseSaveDialogEvent);
                    }
                }
            } else if cancel.is_some() {
                // Just close the dialog
                close_events.write(CloseSaveDialogEvent);
            }
        }
    }
}
