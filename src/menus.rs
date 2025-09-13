//! Main Menu module for Living Worlds
//! 
//! This module handles all menu UI including the main menu, pause menu,
//! and other menu screens. It uses Bevy's built-in UI system with Node
//! components for layout and Interaction components for button handling.

use bevy::prelude::*;
use bevy::app::AppExit;
use crate::states::{GameState, RequestStateTransition};
use crate::ui::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};
use crate::save_load::{SaveGameEvent, SaveGameList, scan_save_files_internal, SaveCompleteEvent};

/// Event to trigger settings menu spawning
#[derive(Event)]
pub struct SpawnSettingsMenuEvent;

/// Event to trigger save browser spawning
#[derive(Event)]
pub struct SpawnSaveBrowserEvent;

/// Plugin that manages all menu-related UI and interactions
pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register menu events
            .add_event::<SpawnSettingsMenuEvent>()
            .add_event::<SpawnSaveBrowserEvent>()
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
                update_load_button_after_save,  // Update Load Game button after save
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
    Mods,
    Exit,
    Resume,
    BackToMainMenu,
}

// Exit confirmation dialog components are now in crate::ui::dialogs

// ============================================================================
// MAIN MENU
// ============================================================================

/// Spawns the main menu UI
fn spawn_main_menu(
    mut commands: Commands,
    mut save_list: ResMut<SaveGameList>,
) {
    println!("Spawning main menu UI");
    
    // Scan for save files to determine if Load Game should be enabled
    scan_save_files_internal(&mut save_list);
    let has_saves = !save_list.saves.is_empty();
    
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
            create_button("Load Game", MenuAction::LoadGame, has_saves);  // Only enabled if saves exist
            create_button("Settings", MenuAction::Settings, true);  // Now enabled
            create_button("Mods", MenuAction::Mods, true);  // Enabled for mod browser
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
    mut settings_events: EventWriter<SpawnSettingsMenuEvent>,
    mut save_browser_events: EventWriter<SpawnSaveBrowserEvent>,
    mut mod_browser_events: EventWriter<crate::modding::ui::OpenModBrowserEvent>,
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
                    println!("New World button pressed - transitioning to WorldConfiguration");
                    state_events.write(RequestStateTransition {
                        from: **current_state,
                        to: GameState::WorldConfiguration,
                    });
                }
                MenuAction::LoadGame => {
                    println!("Load Game button pressed - opening save browser");
                    // Trigger save browser spawning via event
                    save_browser_events.write(SpawnSaveBrowserEvent);
                }
                MenuAction::Settings => {
                    println!("Settings button pressed - opening settings menu");
                    // Trigger settings menu spawning via event
                    settings_events.send(SpawnSettingsMenuEvent);
                }
                MenuAction::Mods => {
                    println!("Opening Mods Browser");
                    mod_browser_events.send(crate::modding::ui::OpenModBrowserEvent);
                }
                MenuAction::Exit => {
                    println!("Exit button pressed - showing confirmation dialog");
                    // Inline the dialog creation to avoid borrowing issues
                    use crate::ui::dialogs::presets;
                    presets::exit_confirmation_dialog(commands);
                    return; // Exit after spawning dialog
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
    interactions: Query<
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
fn spawn_pause_menu(
    mut commands: Commands,
    mut save_list: ResMut<SaveGameList>,
) {
    println!("Spawning pause menu UI");
    
    // Scan for save files to determine if Load Game should be enabled
    scan_save_files_internal(&mut save_list);
    let has_saves = !save_list.saves.is_empty();
    
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
            create_button("Save Game", MenuAction::SaveGame, true);  // Now enabled!
            create_button("Load Game", MenuAction::LoadGame, has_saves);  // Only enabled if saves exist
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

/// Update Load Game button after a save completes
fn update_load_button_after_save(
    mut save_complete_events: EventReader<SaveCompleteEvent>,
    mut menu_buttons: Query<(&mut MenuButton, &mut BackgroundColor, &mut BorderColor, &Children), With<MenuButton>>,
    mut button_texts: Query<&mut TextColor>,
    mut save_list: ResMut<SaveGameList>,
) {
    // Check if a save just completed
    for event in save_complete_events.read() {
        if event.success {
            println!("Save completed - updating Load Game button state");
            
            // Rescan saves to update the list
            scan_save_files_internal(&mut save_list);
            let has_saves = !save_list.saves.is_empty();
            
            // Find and update the Load Game button
            for (mut button, mut bg_color, mut border_color, children) in &mut menu_buttons {
                if matches!(button.action, MenuAction::LoadGame) && !button.enabled && has_saves {
                    // Enable the button
                    button.enabled = true;
                    
                    // Update button appearance to enabled state
                    *bg_color = BackgroundColor(crate::ui::buttons::ButtonStyle::Secondary.base_color());
                    *border_color = BorderColor(crate::ui::buttons::ButtonStyle::Secondary.border_color());
                    
                    // Update text color for children
                    for child in children.iter() {
                        if let Ok(mut text_color) = button_texts.get_mut(child) {
                            *text_color = TextColor(Color::WHITE);
                        }
                    }
                    
                    println!("Load Game button enabled after save");
                }
            }
        }
    }
}

/// Handles button interactions in the pause menu
fn handle_pause_button_interactions(
    mut interactions: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut state_events: EventWriter<RequestStateTransition>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut settings_events: EventWriter<SpawnSettingsMenuEvent>,
    mut save_browser_events: EventWriter<SpawnSaveBrowserEvent>,
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
                    // Trigger settings menu spawning via event
                    settings_events.write(SpawnSettingsMenuEvent);
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
                    // Inline the dialog creation to avoid borrowing issues
                    use crate::ui::dialogs::presets;
                    presets::exit_confirmation_dialog(commands);
                    return; // Exit after spawning dialog
                }
                MenuAction::SaveGame => {
                    println!("Save Game button pressed - saving game");
                    // Send save event with timestamp-based name
                    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
                    save_events.write(SaveGameEvent {
                        slot_name: format!("save_{}", timestamp.to_string()),
                    });
                }
                MenuAction::LoadGame => {
                    println!("Load Game button pressed from pause menu - opening save browser");
                    // Close pause menu first
                    if let Ok(entity) = pause_menu_query.get_single() {
                        commands.entity(entity).despawn_recursive();
                    }
                    // Trigger save browser spawning via event
                    save_browser_events.write(SpawnSaveBrowserEvent);
                }
                _ => {}
            }
        }
    }
}

// ============================================================================
// EXIT CONFIRMATION DIALOG
// ============================================================================

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