//! Main Menu implementation for Living Worlds
//!
//! This module handles the title screen menu that players see when
//! launching the game. It provides options to start a new world,

use super::types::{MenuAction, MenuButton, SpawnSaveBrowserEvent, SpawnSettingsMenuEvent};
use crate::save_load::{scan_save_files_internal, SaveGameList};
use crate::states::{GameState, RequestStateTransition};
use crate::ui::{ButtonBuilder, ButtonSize, ButtonStyle, PanelBuilder, PanelStyle};
use bevy::app::AppExit;
use bevy::prelude::*;

/// Plugin that manages the main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (handle_button_interactions, handle_exit_confirmation_dialog)
                    .chain()
                    .run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Marker component for the main menu root entity
#[derive(Component)]
pub struct MainMenuRoot;

/// Spawns the main menu UI
fn spawn_main_menu(mut commands: Commands, mut save_list: ResMut<SaveGameList>) {
    debug!("Spawning main menu UI");

    // Scan for save files to determine if Load Game should be enabled
    scan_save_files_internal(&mut save_list);
    let has_saves = !save_list.saves.is_empty();

    // Root container - full screen with dark semi-transparent overlay using proper UI components
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.98)),
            MainMenuRoot,
        ))
        .with_children(|root_panel| {
            // Title section using PanelBuilder
            PanelBuilder::new()
                .style(PanelStyle::Transparent)
                .flex_direction(FlexDirection::Column)
                .align_items(AlignItems::Center)
                .build_with_children(root_panel, |title_parent| {
                    // Main title
                    title_parent.spawn((
                        Text::new("LIVING WORLDS"),
                        TextFont {
                            font_size: 72.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.85, 0.7)),
                        Node {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // Subtitle
                    title_parent.spawn((
                        Text::new("A Civilization Observer"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.65)),
                    ));
                });

            // Spacer
            root_panel.spawn(Node {
                height: Val::Px(40.0),
                ..default()
            });

            root_panel
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|button_parent| {
                    // Helper closure for creating buttons using ButtonBuilder
                    let mut create_button = |text: &str, action: MenuAction, enabled: bool| {
                        ButtonBuilder::new(text)
                            .style(ButtonStyle::Secondary)
                            .size(ButtonSize::XLarge)
                            .enabled(enabled)
                            .margin(UiRect::vertical(Val::Px(8.0)))
                            .with_marker(MenuButton { action, enabled })
                            .build(button_parent);
                    };

                    // Create menu buttons
                    create_button("New World", MenuAction::NewWorld, true);
                    create_button("Load Game", MenuAction::LoadGame, has_saves);
                    create_button("Settings", MenuAction::Settings, true);
                    create_button("Mods", MenuAction::Mods, true);
                    create_button("Exit", MenuAction::Exit, true);
                });

            // Version info at bottom
            root_panel.spawn((
                Text::new(crate::version::version_string()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.4, 0.4, 0.4)),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
            ));
        });
}

/// Despawns the main menu UI
fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    debug!("Despawning main menu UI");
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Handles button click interactions in the main menu
fn handle_button_interactions(
    mut interactions: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state_events: EventWriter<RequestStateTransition>,
    mut settings_events: EventWriter<SpawnSettingsMenuEvent>,
    mut save_browser_events: EventWriter<SpawnSaveBrowserEvent>,
    mut mod_browser_events: EventWriter<crate::modding::OpenModBrowserEvent>,
    current_state: Res<State<GameState>>,
    mut commands: Commands,
) {
    for (interaction, button) in &mut interactions {
        // Only respond to enabled buttons
        if !button.enabled {
            continue;
        }

        if *interaction == Interaction::Pressed {
            match button.action {
                MenuAction::NewWorld => {
                    debug!("New World button pressed - transitioning to WorldConfiguration");
                    state_events.write(RequestStateTransition {
                        from: **current_state,
                        to: GameState::WorldConfiguration,
                    });
                }
                MenuAction::LoadGame => {
                    debug!("Load Game button pressed - opening save browser");
                    save_browser_events.write(SpawnSaveBrowserEvent);
                }
                MenuAction::Settings => {
                    debug!("Settings button pressed - opening settings menu");
                    settings_events.write(SpawnSettingsMenuEvent);
                }
                MenuAction::Mods => {
                    debug!("Opening Mods Browser");
                    mod_browser_events.write(crate::modding::OpenModBrowserEvent);
                }
                MenuAction::Exit => {
                    debug!("Exit button pressed - showing confirmation dialog");
                    use crate::ui::dialog_presets;
                    dialog_presets::exit_confirmation_dialog(&mut commands);
                    return;
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
            AnyOf<(&crate::ui::ConfirmButton, &crate::ui::CancelButton)>,
        ),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::ExitConfirmationDialog>>,
    mut exit_events: EventWriter<AppExit>,
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
                debug!("Exit cancelled - returning to menu");
            }
        }
    }
}
