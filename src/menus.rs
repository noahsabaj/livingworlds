//! Main Menu module for Living Worlds
//! 
//! This module handles all menu UI including the main menu, pause menu,
//! and other menu screens. It uses Bevy's built-in UI system with Node
//! components for layout and Interaction components for button handling.

use bevy::prelude::*;
use bevy::app::AppExit;
use crate::states::{GameState, RequestStateTransition};
use crate::settings::{GameSettings, TempGameSettings, SettingsDirtyState, spawn_settings_menu};

/// Plugin that manages all menu-related UI and interactions
pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main menu systems
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(Update, (
                handle_button_interactions,
                update_button_visuals,
            ).chain().run_if(in_state(GameState::MainMenu)))
            
            // Pause menu systems
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
            .add_systems(Update, (
                handle_pause_button_interactions,
                handle_pause_esc_key,
                update_button_visuals,  // Add hover effects for pause menu
            ).run_if(in_state(GameState::Paused)));
    }
}

// ============================================================================
// COMPONENTS
// ============================================================================

/// Marker component for the main menu root entity
#[derive(Component)]
struct MainMenuRoot;

/// Marker component for the pause menu root entity
#[derive(Component)]
struct PauseMenuRoot;

/// Component for menu buttons that defines their action
#[derive(Component)]
pub struct MenuButton {
    pub action: MenuAction,
    pub enabled: bool,
}

/// Marker component for button text entities
#[derive(Component)]
struct ButtonText;

/// Actions that menu buttons can trigger
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuAction {
    NewWorld,
    LoadGame,
    Settings,
    Credits,
    Exit,
    Resume,
    BackToMainMenu,
}

// ============================================================================
// MAIN MENU
// ============================================================================

/// Spawns the main menu UI
fn spawn_main_menu(mut commands: Commands) {
    println!("Spawning main menu UI");
    
    // Root container - full screen with dark semi-transparent overlay
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.98)),
        MainMenuRoot,
    )).with_children(|parent| {
        // Title section
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|title_parent| {
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
        parent.spawn(Node {
            height: Val::Px(40.0),
            ..default()
        });
        
        // Button container
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|button_parent| {
            // Helper closure for creating buttons
            let mut create_button = |text: &str, action: MenuAction, enabled: bool| {
                let base_color = if enabled {
                    Color::srgb(0.15, 0.15, 0.18)
                } else {
                    Color::srgb(0.08, 0.08, 0.08)
                };
                
                let text_color = if enabled {
                    Color::srgb(0.9, 0.9, 0.9)
                } else {
                    Color::srgb(0.4, 0.4, 0.4)
                };
                
                let border_color = if enabled {
                    Color::srgb(0.3, 0.3, 0.35)
                } else {
                    Color::srgb(0.15, 0.15, 0.15)
                };
                
                button_parent.spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(base_color),
                    BorderColor(border_color),
                    MenuButton { action, enabled },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(text_color),
                        ButtonText,
                    ));
                });
            };
            
            // Create menu buttons
            create_button("New World", MenuAction::NewWorld, true);
            create_button("Load Game", MenuAction::LoadGame, false);
            create_button("Settings", MenuAction::Settings, true);  // Now enabled
            create_button("Credits", MenuAction::Credits, false);
            create_button("Exit", MenuAction::Exit, true);
        });
        
        // Version info at bottom
        parent.spawn((
            Text::new("v0.1.0 - Early Development"),
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
fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuRoot>>,
) {
    println!("Despawning main menu UI");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============================================================================
// BUTTON INTERACTIONS
// ============================================================================

/// Handles button click interactions in the main menu
fn handle_button_interactions(
    mut interactions: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut state_events: EventWriter<RequestStateTransition>,
    mut exit_events: EventWriter<AppExit>,
    current_state: Res<State<GameState>>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<crate::states::CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    for (interaction, button) in &mut interactions {
        // Only respond to enabled buttons
        if !button.enabled {
            continue;
        }
        
        if *interaction == Interaction::Pressed {
            match button.action {
                MenuAction::NewWorld => {
                    println!("New World button pressed - transitioning to WorldGeneration");
                    state_events.write(RequestStateTransition {
                        from: **current_state,
                        to: GameState::WorldGeneration,
                    });
                }
                MenuAction::LoadGame => {
                    println!("Load Game - Not yet implemented");
                }
                MenuAction::Settings => {
                    println!("Settings button pressed - opening settings menu");
                    // Spawn settings menu
                    spawn_settings_menu(commands, settings, temp_settings, current_tab, dirty_state);
                    return;  // Exit after spawning settings menu
                }
                MenuAction::Credits => {
                    println!("Credits - Not yet implemented");
                }
                MenuAction::Exit => {
                    println!("Exit button pressed - closing application");
                    exit_events.write(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}

/// Updates button visuals based on interaction state
fn update_button_visuals(
    mut interactions: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>)
    >,
) {
    for (interaction, button, mut bg_color, mut border_color) in &mut interactions {
        // Skip disabled buttons
        if !button.enabled {
            continue;
        }
        
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
                *border_color = BorderColor(Color::srgb(0.7, 0.7, 0.75));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
                *border_color = BorderColor(Color::srgb(0.5, 0.5, 0.55));
            }
            Interaction::None => {
                *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.18));
                *border_color = BorderColor(Color::srgb(0.3, 0.3, 0.35));
            }
        }
    }
}

// ============================================================================
// PAUSE MENU
// ============================================================================

/// Spawns the pause menu UI
fn spawn_pause_menu(mut commands: Commands) {
    println!("Spawning pause menu UI");
    
    // Root container - full screen with dark semi-transparent overlay
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        PauseMenuRoot,
    )).with_children(|parent| {
        // Pause menu panel
        parent.spawn((
            Node {
                width: Val::Px(400.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            BorderColor(Color::srgb(0.4, 0.4, 0.45)),
        )).with_children(|panel| {
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
            
            // Helper closure for creating buttons
            let mut create_button = |text: &str, action: MenuAction, enabled: bool| {
                let base_color = if enabled {
                    Color::srgb(0.15, 0.15, 0.18)
                } else {
                    Color::srgb(0.08, 0.08, 0.08)
                };
                
                let text_color = if enabled {
                    Color::srgb(0.9, 0.9, 0.9)
                } else {
                    Color::srgb(0.4, 0.4, 0.4)
                };
                
                let border_color = if enabled {
                    Color::srgb(0.3, 0.3, 0.35)
                } else {
                    Color::srgb(0.15, 0.15, 0.15)
                };
                
                panel.spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(base_color),
                    BorderColor(border_color),
                    MenuButton { action, enabled },
                )).with_children(|button| {
                    button.spawn((
                        Text::new(text),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(text_color),
                        ButtonText,
                    ));
                });
            };
            
            // Buttons
            create_button("Resume", MenuAction::Resume, true);
            create_button("Settings", MenuAction::Settings, true);  // Now enabled
            create_button("Save Game", MenuAction::LoadGame, false);
            create_button("Main Menu", MenuAction::BackToMainMenu, true);
            create_button("Exit Game", MenuAction::Exit, true);
        });
    });
}

/// Despawns the pause menu UI
fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuRoot>>,
) {
    println!("Despawning pause menu UI");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle ESC key to resume game from pause menu
fn handle_pause_esc_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("ESC pressed in pause menu - resuming game");
        state_events.write(RequestStateTransition {
            from: GameState::Paused,
            to: GameState::InGame,
        });
    }
}

/// Handles button interactions in the pause menu
fn handle_pause_button_interactions(
    mut interactions: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut state_events: EventWriter<RequestStateTransition>,
    mut exit_events: EventWriter<AppExit>,
    current_state: Res<State<GameState>>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<crate::states::CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
    pause_menu_query: Query<Entity, With<PauseMenuRoot>>,
) {
    for (interaction, button) in &mut interactions {
        if !button.enabled {
            continue;
        }
        
        if *interaction == Interaction::Pressed {
            match button.action {
                MenuAction::Resume => {
                    println!("Resume button pressed");
                    state_events.write(RequestStateTransition {
                        from: GameState::Paused,
                        to: GameState::InGame,
                    });
                }
                MenuAction::Settings => {
                    println!("Settings button pressed from pause menu - opening settings menu");
                    // Despawn pause menu first
                    if let Ok(entity) = pause_menu_query.get_single() {
                        commands.entity(entity).despawn_recursive();
                    }
                    // Spawn settings menu
                    spawn_settings_menu(commands, settings, temp_settings, current_tab, dirty_state);
                    return;  // Exit after spawning settings menu
                }
                MenuAction::BackToMainMenu => {
                    println!("Back to Main Menu pressed");
                    state_events.write(RequestStateTransition {
                        from: GameState::Paused,
                        to: GameState::MainMenu,
                    });
                }
                MenuAction::Exit => {
                    println!("Exit from pause menu");
                    exit_events.write(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}