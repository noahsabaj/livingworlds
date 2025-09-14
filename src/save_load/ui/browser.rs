//! Save browser UI implementation
//!
//! This module creates the save browser UI using our standard UI builders.

use bevy::prelude::*;
use crate::ui::{DialogBuilder, DialogType, ButtonBuilder, ButtonStyle, ButtonSize, colors};
use crate::menus::SpawnSaveBrowserEvent;
use super::{LoadGameEvent, SaveGameList, SaveBrowserState};
use super::components::*;

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
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            SaveBrowserRoot,
        )).with_children(|overlay| {
            // Dialog container
            overlay.spawn((
                Node {
                    width: Val::Px(800.0),
                    height: Val::Px(600.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(colors::BACKGROUND_MEDIUM),
                BorderColor(colors::BORDER),
            )).with_children(|parent| {
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
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        overflow: Overflow::scroll_y(),
                        padding: UiRect::all(Val::Px(10.0)),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
                )).with_children(|list| {
                    // Add save slots
                    for (index, save_info) in save_list.saves.iter().enumerate() {
                        spawn_save_slot(list, index, save_info.clone());
                    }
                });

                // Bottom buttons
                parent.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    ..default()
                }).with_children(|buttons| {
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

fn spawn_save_slot(parent: &mut ChildSpawnerCommands, index: usize, save_info: super::super::types::SaveGameInfo) {
    parent.spawn((
        Button,
        Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(15.0)),
            margin: UiRect::bottom(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
        SaveSlotButton {
            index,
            save_info: save_info.clone(),
        },
    )).with_children(|slot| {
        slot.spawn(Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            ..default()
        }).with_children(|row| {
            // Left side: Save info
            row.spawn(Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..default()
            }).with_children(|info| {
                // Save name
                info.spawn((
                    Text::new(&save_info.name),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));

                // World info
                info.spawn((
                    Text::new(format!("World: {} | Seed: {} | Size: {}",
                        save_info.world_name,
                        save_info.world_seed,
                        save_info.world_size
                    )),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.8, 0.9)),
                    Node {
                        margin: UiRect::top(Val::Px(3.0)),
                        ..default()
                    },
                ));

                // Date and size info
                info.spawn((
                    Text::new(format!("Date: {} | Size: {} | Game Time: {:.0} days",
                        save_info.date_created.format("%Y-%m-%d %H:%M"),
                        super::format_file_size(save_info.compressed_size),
                        save_info.game_time
                    )),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
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
        (&Interaction, Option<&SaveSlotButton>, Option<&LoadSelectedButton>, Option<&CancelBrowserButton>),
        Changed<Interaction>
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
                println!("Selected save: {}", slot.save_info.name);
            } else if load_btn.is_some() {
                if let Some(index) = browser_state.selected_save {
                    if let Some(save_info) = save_list.saves.get(index) {
                        load_events.write(LoadGameEvent {
                            save_path: save_info.path.clone(),
                        });

                        // Close browser
                        close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
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
            *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.17));
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
        commands.entity(entity).despawn_recursive();
    }
    browser_state.is_open = false;
    browser_state.selected_save = None;
}