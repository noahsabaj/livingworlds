//! Main Menu module for Living Worlds
//! 
//! This module handles all menu UI including the main menu, pause menu,
//! and other menu screens. It uses Bevy's built-in UI system with Node
//! components for layout and Interaction components for button handling.

use bevy::prelude::*;
use bevy::app::AppExit;
use crate::states::{GameState, RequestStateTransition};
use crate::settings::{GameSettings, TempGameSettings, SettingsDirtyState, spawn_settings_menu};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};

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
                handle_exit_confirmation_dialog,
            ).chain().run_if(in_state(GameState::MainMenu)))
            
            // Pause menu systems
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu)
            .add_systems(Update, (
                handle_pause_button_interactions,
                handle_pause_esc_key,
                update_button_visuals,  // Add hover effects for pause menu
                handle_exit_confirmation_dialog,
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
    SaveGame,
    Settings,
    Credits,
    Exit,
    Resume,
    BackToMainMenu,
}

// Exit confirmation dialog components are now in crate::ui::dialogs

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
                    println!("New World button pressed - transitioning to WorldConfiguration");
                    state_events.write(RequestStateTransition {
                        from: **current_state,
                        to: GameState::WorldConfiguration,
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
                    println!("Exit button pressed - showing confirmation dialog");
                    spawn_exit_confirmation_dialog(&mut commands);
                }
                _ => {}
            }
        }
    }
}

/// Updates button visuals based on interaction state
/// Note: ButtonBuilder handles hover effects via styled_button_hover_system,
/// but we keep this for any custom menu-specific visual feedback
fn update_button_visuals(
    mut interactions: Query<
        (&Interaction, &MenuButton, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>)
    >,
) {
    // The styled_button_hover_system in buttons.rs handles the visual updates
    // This function is kept for potential menu-specific visual feedback
    // Currently, all hover effects are handled by the centralized system
}

// ============================================================================
// PAUSE MENU
// ============================================================================

/// Spawns the pause menu UI
fn spawn_pause_menu(mut commands: Commands) {
    println!("Spawning pause menu UI");
    
    // Root container - full screen with dark semi-transparent overlay that blocks clicks
    commands.spawn((
        Button,  // Add Button to block clicks to elements behind pause menu
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
            create_button("Settings", MenuAction::Settings, true);  // Now enabled
            create_button("Save Game", MenuAction::SaveGame, false);
            create_button("Load Game", MenuAction::LoadGame, false);
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
                    println!("Exit from pause menu - showing confirmation dialog");
                    spawn_exit_confirmation_dialog(&mut commands);
                }
                MenuAction::SaveGame => {
                    println!("Save Game - Not yet implemented");
                }
                MenuAction::LoadGame => {
                    println!("Load Game - Not yet implemented");
                }
                _ => {}
            }
        }
    }
}

// ============================================================================
// EXIT CONFIRMATION DIALOG
// ============================================================================

/// Spawns the exit confirmation dialog
fn spawn_exit_confirmation_dialog(commands: &mut Commands) {
    println!("Spawning exit confirmation dialog");
    
    // Use the new dialog builder system
    use crate::ui::dialogs::presets;
    presets::exit_confirmation_dialog(commands.reborrow());
}

/// Handles exit confirmation dialog button interactions
fn handle_exit_confirmation_dialog(
    mut interactions: Query<
        (&Interaction, AnyOf<(&crate::ui::dialogs::ConfirmButton, &crate::ui::dialogs::CancelButton)>), 
        Changed<Interaction>
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::dialogs::ExitConfirmationDialog>>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, (confirm_button, cancel_button)) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Close the dialog first
            if let Ok(dialog_entity) = dialog_query.get_single() {
                commands.entity(dialog_entity).despawn_recursive();
            }
            
            if confirm_button.is_some() {
                println!("Exit confirmed - closing application");
                exit_events.write(AppExit::Success);
            } else if cancel_button.is_some() {
                println!("Exit cancelled - returning to game");
            }
        }
    }
}