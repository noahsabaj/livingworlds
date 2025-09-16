//! Save browser UI implementation
//!
//! This module creates the save browser UI using our standard UI builders.

use super::components::*;
use super::{LoadGameEvent, SaveBrowserState, SaveGameList};
use crate::menus::SpawnSaveBrowserEvent;
use crate::ui::{colors, ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::prelude::*;

/// System to handle the SpawnSaveBrowserEvent
pub fn spawn_save_browser(
    mut events: EventReader<SpawnSaveBrowserEvent>,
    mut commands: Commands,
    mut save_list: ResMut<SaveGameList>,
    mut browser_state: ResMut<SaveBrowserState>,
) {
    for _ in events.read() {
        // Mark browser as open
        browser_state.is_open = true;

        // Scan for saves
        super::scan_save_files_internal(&mut save_list);

        // Create save browser manually (too complex for DialogBuilder)
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors::OVERLAY_DARK),
                SaveBrowserRoot,
            ))
            .with_children(|overlay| {
                // Dialog container
                overlay
                    .spawn((
                        Node {
                            width: Val::Px(800.0),                    // Width constraint for readability
                            max_height: Val::Vh(90.0),               // Safety valve - never bigger than 90% viewport
                            padding: UiRect::all(Val::Px(20.0)),
                            flex_direction: FlexDirection::Column,
                            // NO HEIGHT SPECIFIED! Content determines it naturally
                            ..default()
                        },
                        BackgroundColor(colors::BACKGROUND_MEDIUM),
                        BorderColor(colors::BORDER),
                    ))
                    .with_children(|parent| {
                        // Title
                        parent.spawn((
                            Text::new("Load Game"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_PRIMARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(15.0)),
                                ..default()
                            },
                        ));
                        // Scrollable save list
                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Column,
                                    flex_grow: 1.0,
                                    overflow: Overflow::scroll_y(),
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::bottom(Val::Px(20.0)),
                                    ..default()
                                },
                                BackgroundColor(colors::BACKGROUND_DARK),
                            ))
                            .with_children(|list| {
                                // Add save slots
                                for (index, save_info) in save_list.saves.iter().enumerate() {
                                    spawn_save_slot(list, index, save_info.clone());
                                }
                            });

                        // Bottom buttons
                        PanelBuilder::new()
                            .style(PanelStyle::Transparent)
                            .flex_direction(FlexDirection::Row)
                            .justify_content(JustifyContent::SpaceBetween)
                            .width(Val::Percent(100.0))
                            .build_with_children(parent, |buttons| {
                                ButtonBuilder::new("Load Selected")
                                    .style(ButtonStyle::Primary)
                                    .size(ButtonSize::Large)
                                    .with_marker(LoadSelectedButton)
                                    .build(buttons);

                                ButtonBuilder::new("Cancel")
                                    .style(ButtonStyle::Secondary)
                                    .size(ButtonSize::Large)
                                    .with_marker(CancelBrowserButton)
                                    .build(buttons);
                            });
                    });
            });
    }
}

fn spawn_save_slot(
    parent: &mut ChildSpawnerCommands,
    index: usize,
    save_info: super::super::types::SaveGameInfo,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                ..default()
            },
            BackgroundColor(colors::SECONDARY),
            SaveSlotButton {
                index,
                save_info: save_info.clone(),
            },
        ))
        .with_children(|slot| {
            PanelBuilder::new()
                .style(PanelStyle::Transparent)
                .width(Val::Percent(100.0))
                .flex_direction(FlexDirection::Row)
                .justify_content(JustifyContent::SpaceBetween)
                .align_items(AlignItems::Start)
                .build_with_children(slot, |row| {
                    // Left side: Save info
                    PanelBuilder::new()
                        .style(PanelStyle::Transparent)
                        .flex_direction(FlexDirection::Column)
                        .flex_grow(1.0)
                        .build_with_children(row, |info| {
                            // Save name
                            info.spawn((
                                Text::new(&save_info.name),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_PRIMARY),
                            ));

                            // World info
                            info.spawn((
                                Text::new(format!(
                                    "World: {} | Seed: {} | Size: {}",
                                    save_info.world_name,
                                    save_info.world_seed,
                                    save_info.world_size
                                )),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_SECONDARY),
                                Node {
                                    margin: UiRect::top(Val::Px(3.0)),
                                    ..default()
                                },
                            ));

                            // Date and size info
                            info.spawn((
                                Text::new(format!(
                                    "Date: {} | Size: {} | Game Time: {:.0} days",
                                    save_info.date_created.format("%Y-%m-%d %H:%M"),
                                    super::format_file_size(save_info.compressed_size),
                                    save_info.game_time
                                )),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(colors::TEXT_TERTIARY),
                                Node {
                                    margin: UiRect::top(Val::Px(5.0)),
                                    ..default()
                                },
                            ));
                        });

                    // Right side: Delete button
                    ButtonBuilder::new("Delete")
                        .style(ButtonStyle::Danger)
                        .size(ButtonSize::Small)
                        .with_marker(DeleteSaveButton {
                            save_path: save_info.path.clone(),
                            save_name: save_info.name.clone(),
                        })
                        .build(row);
                });
        });
}

/// Handle save browser button interactions
pub fn handle_save_browser_interactions(
    mut interactions: Query<
        (
            &Interaction,
            Option<&SaveSlotButton>,
            Option<&LoadSelectedButton>,
            Option<&CancelBrowserButton>,
        ),
        Changed<Interaction>,
    >,
    mut browser_state: ResMut<SaveBrowserState>,
    save_list: Res<SaveGameList>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut commands: Commands,
    browser_query: Query<Entity, With<SaveBrowserRoot>>,
) {
    for (interaction, save_slot, load_btn, cancel_btn) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Some(slot) = save_slot {
                // Select this save
                browser_state.selected_save = Some(slot.index);
                debug!("Selected save: {}", slot.save_info.name);
            } else if load_btn.is_some() {
                if let Some(index) = browser_state.selected_save {
                    if let Some(save_info) = save_list.saves.get(index) {
                        load_events.write(LoadGameEvent {
                            save_path: save_info.path.clone(),
                        });

                        // Close browser
                        close_save_browser_internal(
                            &mut commands,
                            &browser_query,
                            &mut browser_state,
                        );
                    }
                }
            } else if cancel_btn.is_some() {
                // Close browser
                close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
            }
        }
    }
}

/// Update save browser visuals
pub fn update_save_browser(
    browser_state: Res<SaveBrowserState>,
    mut save_slots: Query<(&SaveSlotButton, &mut BackgroundColor)>,
) {
    if !browser_state.is_open {
        return;
    }

    for (slot, mut bg_color) in &mut save_slots {
        if Some(slot.index) == browser_state.selected_save {
            *bg_color = BackgroundColor(colors::SURFACE_HOVER);
        } else {
            *bg_color = BackgroundColor(colors::SECONDARY);
        }
    }
}

/// Close the save browser
pub fn close_save_browser(
    mut commands: Commands,
    browser_query: Query<Entity, With<SaveBrowserRoot>>,
    mut browser_state: ResMut<SaveBrowserState>,
) {
    close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
}

fn close_save_browser_internal(
    commands: &mut Commands,
    browser_query: &Query<Entity, With<SaveBrowserRoot>>,
    browser_state: &mut SaveBrowserState,
) {
    for entity in browser_query {
        commands.entity(entity).despawn();
    }
    browser_state.is_open = false;
    browser_state.selected_save = None;
}
