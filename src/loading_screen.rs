//! Unified loading screen for all loading operations
//! 
//! This module provides a consistent loading experience whether generating
//! a new world or loading a saved game.

use bevy::prelude::*;
use crate::states::GameState;
use crate::ui::styles::colors;

// ============================================================================
// PLUGIN
// ============================================================================

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingState>()
            .add_systems(OnEnter(GameState::LoadingWorld), setup_loading_screen)
            .add_systems(OnExit(GameState::LoadingWorld), cleanup_loading_screen)
            .add_systems(Update, (
                update_loading_progress,
                animate_progress_bar,
                update_loading_text,
                rotate_loading_spinner,
            ).run_if(in_state(GameState::LoadingWorld)));
    }
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Tracks what's being loaded and the current progress
#[derive(Resource, Default)]
pub struct LoadingState {
    pub operation: LoadingOperation,
    pub progress: f32, // 0.0 to 1.0
    pub current_step: String,
    pub details: LoadingDetails,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LoadingOperation {
    #[default]
    None,
    GeneratingWorld,
    LoadingSave,
    ApplyingMods,
}

#[derive(Debug, Clone, Default)]
pub struct LoadingDetails {
    // For world generation
    pub world_seed: Option<u32>,
    pub world_size: Option<String>,
    
    // For save loading
    pub save_name: Option<String>,
    pub game_days: Option<f32>,
    pub file_size: Option<String>,
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
struct LoadingScreenRoot;

#[derive(Component)]
struct LoadingProgressBar;

#[derive(Component)]
struct LoadingProgressFill;

#[derive(Component)]
struct LoadingStatusText;

#[derive(Component)]
struct LoadingDetailsPanel;

#[derive(Component)]
struct LoadingSpinner;

#[derive(Component)]
struct LoadingTip;

// ============================================================================
// SYSTEMS
// ============================================================================

/// Setup the loading screen UI
fn setup_loading_screen(
    mut commands: Commands,
    loading_state: Res<LoadingState>,
) {
    // Root container - full screen with dark background
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.02, 0.02, 0.03)),
        LoadingScreenRoot,
    )).with_children(|parent| {
        // ===== TOP SECTION: Title and Operation =====
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|top| {
            // Game title
            top.spawn((
                Text::new("LIVING WORLDS"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Operation subtitle
            let subtitle = match loading_state.operation {
                LoadingOperation::GeneratingWorld => "Generating New World",
                LoadingOperation::LoadingSave => "Loading Saved Game",
                LoadingOperation::ApplyingMods => "Applying Mod Changes",
                LoadingOperation::None => "Loading...",
            };
            
            top.spawn((
                Text::new(subtitle),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(colors::TEXT_SECONDARY),
            ));
        });
        
        // ===== MIDDLE SECTION: Info Panel =====
        parent.spawn((
            Node {
                width: Val::Px(600.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.5)),
            BorderColor(colors::BORDER_DEFAULT),
            LoadingDetailsPanel,
        )).with_children(|panel| {
            // Animated spinner
            panel.spawn((
                Text::new("◈"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(colors::PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                LoadingSpinner,
            ));
            
            // Details based on operation
            match &loading_state.operation {
                LoadingOperation::GeneratingWorld => {
                    if let Some(seed) = loading_state.details.world_seed {
                        panel.spawn((
                            Text::new(format!("World Seed: {}", seed)),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    }
                    
                    if let Some(size) = &loading_state.details.world_size {
                        panel.spawn((
                            Text::new(format!("World Size: {}", size)),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    }
                }
                LoadingOperation::ApplyingMods => {
                    panel.spawn((
                        Text::new("Reloading game systems with new mod configuration"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(colors::TEXT_SECONDARY),
                    ));
                }
                LoadingOperation::LoadingSave => {
                    if let Some(name) = &loading_state.details.save_name {
                        panel.spawn((
                            Text::new(format!("Save: {}", name)),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    }
                    
                    if let Some(days) = loading_state.details.game_days {
                        panel.spawn((
                            Text::new(format!("World Age: {:.0} days", days)),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(colors::TEXT_SECONDARY),
                        ));
                    }
                }
                _ => {}
            }
        });
        
        // ===== BOTTOM SECTION: Progress =====
        parent.spawn(Node {
            width: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|bottom| {
            // Status text
            bottom.spawn((
                Text::new(&loading_state.current_step),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                LoadingStatusText,
            ));
            
            // Progress bar container
            bottom.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
                BorderColor(colors::BORDER_DEFAULT),
                LoadingProgressBar,
            )).with_children(|bar| {
                // Progress fill
                bar.spawn((
                    Node {
                        width: Val::Percent(loading_state.progress * 100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(colors::PRIMARY),
                    LoadingProgressFill,
                ));
            });
            
            // Loading tip
            bottom.spawn((
                Text::new(get_random_tip()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(colors::TEXT_TERTIARY),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                LoadingTip,
            ));
        });
    });
}

/// Cleanup the loading screen
fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoadingScreenRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Update the progress bar width
fn update_loading_progress(
    loading_state: Res<LoadingState>,
    mut query: Query<&mut Node, With<LoadingProgressFill>>,
) {
    if loading_state.is_changed() {
        for mut node in &mut query {
            node.width = Val::Percent(loading_state.progress * 100.0);
        }
    }
}

/// Animate the progress bar with a subtle pulse
fn animate_progress_bar(
    time: Res<Time>,
    mut query: Query<&mut BackgroundColor, With<LoadingProgressFill>>,
) {
    let pulse = (time.elapsed_secs() * 2.0).sin() * 0.1 + 0.9;
    for mut bg_color in &mut query {
        // Create a pulsing effect by interpolating between two shades
        if pulse > 0.95 {
            *bg_color = BackgroundColor(colors::PRIMARY_HOVER);
        } else {
            *bg_color = BackgroundColor(colors::PRIMARY);
        }
    }
}

/// Update the status text
fn update_loading_text(
    loading_state: Res<LoadingState>,
    mut query: Query<&mut Text, With<LoadingStatusText>>,
) {
    if loading_state.is_changed() {
        for mut text in &mut query {
            text.0 = loading_state.current_step.clone();
        }
    }
}

/// Rotate the loading spinner
fn rotate_loading_spinner(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<LoadingSpinner>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_secs());
    }
}

/// Get a random loading tip
fn get_random_tip() -> &'static str {
    // In a real implementation, this would randomly select from a list
    "Tip: Press Space to pause the simulation and observe your world"
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Update the loading state from external systems
pub fn set_loading_progress(
    loading_state: &mut LoadingState,
    progress: f32,
    message: impl Into<String>,
) {
    loading_state.progress = progress.clamp(0.0, 1.0);
    loading_state.current_step = message.into();
}

/// Start a world generation loading operation
pub fn start_world_generation_loading(
    loading_state: &mut LoadingState,
    seed: u32,
    size: String,
) {
    loading_state.operation = LoadingOperation::GeneratingWorld;
    loading_state.progress = 0.0;
    loading_state.current_step = "Initializing world generation...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: Some(seed),
        world_size: Some(size),
        save_name: None,
        game_days: None,
        file_size: None,
    };
}

/// Start a save loading operation
pub fn start_save_loading(
    loading_state: &mut LoadingState,
    save_name: String,
    game_days: f32,
    file_size: String,
) {
    loading_state.operation = LoadingOperation::LoadingSave;
    loading_state.progress = 0.0;
    loading_state.current_step = "Reading save file...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: None,
        world_size: None,
        save_name: Some(save_name),
        game_days: Some(game_days),
        file_size: Some(file_size),
    };
}

/// Start a mod application loading operation
pub fn start_mod_application_loading(
    loading_state: &mut LoadingState,
) {
    loading_state.operation = LoadingOperation::ApplyingMods;
    loading_state.progress = 0.0;
    loading_state.current_step = "Applying mod configuration...".to_string();
    loading_state.details = LoadingDetails {
        world_seed: None,
        world_size: None,
        save_name: None,
        game_days: None,
        file_size: None,
    };
}