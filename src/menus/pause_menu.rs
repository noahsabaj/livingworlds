//! Pause Menu implementation for Living Worlds
//!
//! This module handles the in-game pause menu overlay that appears
//! when the player pauses during gameplay. It provides options to
//! resume, save, load, access settings, or return to the main menu.

#![allow(elided_lifetimes_in_paths)]

use super::types::{MenuAction, MenuButton, SpawnSaveBrowserEvent, SpawnSettingsMenuEvent};
use crate::save_load::{scan_save_files_internal, SaveCompleteEvent, SaveGameEvent, SaveGameList};
use crate::states::{GameState, RequestStateTransition};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle};
use crate::ui::{ShortcutEvent, ShortcutId};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// Plugin that manages the pause menu
define_plugin!(PauseMenuPlugin {
    update: [
        (
            handle_pause_button_interactions,
            handle_pause_esc_key,
            handle_exit_confirmation_dialog,
            update_load_button_after_save,
        ).run_if(in_state(GameState::Paused)),
        handle_ingame_esc_key.run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::Paused => [spawn_pause_menu]
    }
});

/// Marker component for the pause menu root
#[derive(Component)]
struct PauseMenuRoot;

/// Spawns the pause menu UI
fn spawn_pause_menu(mut commands: Commands, mut save_list: ResMut<SaveGameList>) {
    debug!("Spawning pause menu UI");

    // Scan for save files to determine if Load Game should be enabled
    scan_save_files_internal(&mut save_list);
    let has_saves = !save_list.saves.is_empty();

    // Root container - full screen with dark semi-transparent overlay that blocks clicks
    // Uses StateScoped for automatic cleanup when exiting Paused state
    commands
        .spawn((
            PauseMenuRoot,
            DespawnOnExit(GameState::Paused),
            Button, // Add Button to block clicks to elements behind pause menu
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            // Pause menu panel
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
                    BorderColor::all(Color::srgb(0.4, 0.4, 0.45)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn((
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    // Helper closure for creating buttons using ButtonBuilder
                    let mut create_button = |text: &str, action: MenuAction, enabled: bool| {
                        ButtonBuilder::new(text)
                            .style(ButtonStyle::Secondary)
                            .size(ButtonSize::XLarge)
                            .enabled(enabled)
                            .margin(UiRect::vertical(Val::Px(8.0)))
                            .with_marker(MenuButton { action, enabled })
                            .build(panel);
                    };

                    // Buttons
                    create_button("Resume", MenuAction::Resume, true);
                    create_button("Settings", MenuAction::Settings, true);
                    create_button("Save Game", MenuAction::SaveGame, true);
                    create_button("Load Game", MenuAction::LoadGame, has_saves);
                    create_button("Main Menu", MenuAction::BackToMainMenu, true);
                    create_button("Exit Game", MenuAction::Exit, true);
                });
        });
}

/// Despawns the pause menu UI
/// Handle ESC shortcut to resume game from pause menu
fn handle_pause_esc_key(
    mut shortcut_events: MessageReader<ShortcutEvent>,
    mut state_events: MessageWriter<RequestStateTransition>,
) {
    for event in shortcut_events.read() {
        if event.shortcut_id == ShortcutId::Escape {
            debug!("ESC shortcut triggered in pause menu - resuming game");
            state_events.write(RequestStateTransition {
                from: GameState::Paused,
                to: GameState::InGame,
            });
        }
    }
}

/// Handle ESC shortcut to open pause menu from in-game
fn handle_ingame_esc_key(
    mut shortcut_events: MessageReader<ShortcutEvent>,
    mut state_events: MessageWriter<RequestStateTransition>,
) {
    for event in shortcut_events.read() {
        // ONLY Escape should open the pause menu, NOT Space (Pause is for time control only)
        if event.shortcut_id == ShortcutId::Escape || event.shortcut_id == ShortcutId::OpenMainMenu {
            debug!("Escape key triggered in game - opening pause menu");
            state_events.write(RequestStateTransition {
                from: GameState::InGame,
                to: GameState::Paused,
            });
        }
    }
}

/// Update Load Game button after a save completes
fn update_load_button_after_save(
    mut save_complete_events: MessageReader<SaveCompleteEvent>,
    mut menu_buttons: Query<
        (
            &mut MenuButton,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<MenuButton>,
    >,
    mut button_texts: Query<&mut TextColor>,
    mut save_list: ResMut<SaveGameList>,
) {
    for event in save_complete_events.read() {
        if event.success {
            info!("Save completed - updating Load Game button state");

            // Rescan saves to update the list
            scan_save_files_internal(&mut save_list);
            let has_saves = !save_list.saves.is_empty();

            for (mut button, mut bg_color, mut border_color, children) in &mut menu_buttons {
                if matches!(button.action, MenuAction::LoadGame) && !button.enabled && has_saves {
                    // Enable the button
                    button.enabled = true;

                    *bg_color = BackgroundColor(crate::ui::ButtonStyle::Secondary.base_color());
                    *border_color = BorderColor::all(crate::ui::ButtonStyle::Secondary.border_color());

                    for child in children.iter() {
                        if let Ok(mut text_color) = button_texts.get_mut(child) {
                            *text_color = TextColor(Color::WHITE);
                        }
                    }

                    info!("Load Game button enabled after save");
                }
            }
        }
    }
}

/// Handles button interactions in the pause menu
fn handle_pause_button_interactions(
    mut interactions: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state_events: MessageWriter<RequestStateTransition>,
    mut save_events: MessageWriter<SaveGameEvent>,
    mut settings_events: MessageWriter<SpawnSettingsMenuEvent>,
    mut save_browser_events: MessageWriter<SpawnSaveBrowserEvent>,
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenuRoot>>,
) {
    for (interaction, button) in &mut interactions {
        if !button.enabled {
            continue;
        }

        if *interaction == Interaction::Pressed {
            match button.action {
                MenuAction::Resume => {
                    info!("Resume button pressed");
                    state_events.write(RequestStateTransition {
                        from: GameState::Paused,
                        to: GameState::InGame,
                    });
                }
                MenuAction::Settings => {
                    info!("Settings button pressed from pause menu - opening settings menu");
                    // Despawn pause menu first
                    if let Ok(entity) = pause_menu_query.single() {
                        commands.entity(entity).despawn();
                    }
                    settings_events.write(SpawnSettingsMenuEvent);
                }
                MenuAction::BackToMainMenu => {
                    info!("Back to Main Menu pressed");
                    state_events.write(RequestStateTransition {
                        from: GameState::Paused,
                        to: GameState::MainMenu,
                    });
                }
                MenuAction::Exit => {
                    info!("Exit from pause menu - showing confirmation dialog");
                    use crate::ui::dialog_presets;
                    dialog_presets::exit_confirmation_dialog(&mut commands);
                    return;
                }
                MenuAction::SaveGame => {
                    info!("Save Game button pressed - saving game");
                    // Send save event with timestamp-based name
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                    save_events.write(SaveGameEvent {
                        slot_name: format!("save_{}", timestamp.to_string()),
                    });
                }
                MenuAction::LoadGame => {
                    info!("Load Game button pressed from pause menu - opening save browser");
                    // Close pause menu first
                    if let Ok(entity) = pause_menu_query.single() {
                        commands.entity(entity).despawn();
                    }
                    save_browser_events.write(SpawnSaveBrowserEvent);
                }
                _ => {}
            }
        }
    }
}

/// Handles exit confirmation dialog button interactions
fn handle_exit_confirmation_dialog(
    mut interactions: Query<
        (
            &Interaction,
            AnyOf<(&bevy_ui_builders::ConfirmButton, &bevy_ui_builders::CancelButton)>,
        ),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::ExitConfirmationDialog>>,
    mut exit_events: MessageWriter<AppExit>,
) {
    for (interaction, (confirm_button, cancel_button)) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Close the dialog first
            if let Ok(dialog_entity) = dialog_query.single() {
                commands.entity(dialog_entity).despawn();
            }

            if confirm_button.is_some() {
                info!("Exit confirmed - closing application");
                exit_events.write(AppExit::Success);
            } else if cancel_button.is_some() {
                info!("Exit cancelled - returning to game");
            }
        }
    }
}
