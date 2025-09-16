//! Unified loading screen for all loading operations
//!
//! This module provides a consistent loading experience whether generating
//! a new world or loading a saved game. Now using standardized UI builders
//! for consistency and maintainability.

use crate::states::{GameState, RequestStateTransition};
use crate::ui::ProgressBarBuilder;
use crate::ui::{
    colors,
    dimensions,     // Style constants
    get_random_tip, // Utility function
    ButtonBuilder,
    ButtonStyle,
    LabelBuilder,
    LabelStyle,
    PanelBuilder,
    PanelStyle,
};
use crate::ui::ProgressBar;
use bevy::prelude::*;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingState>()
            .add_event::<CancelWorldGeneration>()
            .add_systems(OnEnter(GameState::LoadingWorld), setup_loading_screen)
            .add_systems(OnExit(GameState::LoadingWorld), cleanup_loading_screen)
            .add_systems(
                Update,
                (
                    update_loading_progress,
                    update_loading_text,
                    handle_cancel_button,
                    handle_cancel_generation,
                ).run_if(in_state(GameState::LoadingWorld)),
            );
    }
}

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

#[derive(Component)]
struct LoadingScreenRoot;

#[derive(Component)]
struct LoadingProgressBar;

#[derive(Component)]
struct LoadingStatusText;

#[derive(Component)]
struct CancelGenerationButton;

/// Event to cancel world generation
#[derive(Event)]
pub struct CancelWorldGeneration;

/// Setup the loading screen UI using builders
fn setup_loading_screen(mut commands: Commands, loading_state: Res<LoadingState>) {
    // Root container with proper UI components to avoid B0004 warnings
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.02, 0.02, 0.03)),
        LoadingScreenRoot,
    )).with_children(|root_panel| {
                // ===== TOP SECTION: Title and Operation =====
                spawn_top_section(root_panel, &loading_state);

                // ===== MIDDLE SECTION: Details Panel with Loading Indicator =====
                spawn_details_panel(root_panel, &loading_state);

                // ===== BOTTOM SECTION: Progress Bar and Tips =====
                spawn_bottom_section(root_panel, &loading_state);
    });
}

/// Spawn the top section with title and operation subtitle
fn spawn_top_section(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|top| {
            // Main title using fixed LabelBuilder
            LabelBuilder::new("LIVING WORLDS")
                .style(LabelStyle::Title)
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(top);

            // Operation subtitle
            let subtitle = match loading_state.operation {
                LoadingOperation::GeneratingWorld => "Generating New World",
                LoadingOperation::LoadingSave => "Loading Saved Game",
                LoadingOperation::ApplyingMods => "Applying Mod Changes",
                LoadingOperation::None => "Loading...",
            };

            // Operation subtitle using fixed LabelBuilder
            LabelBuilder::new(subtitle)
                .style(LabelStyle::Heading)
                .build(top);
        });
}

/// Spawn the details panel with loading indicator
fn spawn_details_panel(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    // Create panel with consistent styling using PanelBuilder
    PanelBuilder::new()
        .style(PanelStyle::Card)
        .width(Val::Px(600.0))
        .padding(UiRect::all(Val::Px(30.0)))
        .border(UiRect::all(Val::Px(2.0)))
        .flex_direction(FlexDirection::Column)
        .background_color(colors::SURFACE)
        .border_color(colors::BORDER)
        .build_with_children(parent, |panel| {
            // Loading indicator using direct Bevy API
            panel.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: dimensions::FONT_SIZE_LARGE,
                    ..default()
                },
                TextColor(colors::PRIMARY),
            ));

            // Add spacing
            panel.spawn(Node {
                height: Val::Px(20.0),
                ..default()
            });

            // Details based on operation
            spawn_operation_details(panel, loading_state);
        });
}

/// Spawn operation-specific details
fn spawn_operation_details(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    match &loading_state.operation {
        LoadingOperation::GeneratingWorld => {
            if let Some(seed) = loading_state.details.world_seed {
                parent.spawn((
                    Text::new(format!("World Seed: {}", seed)),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
            }

            if let Some(size) = &loading_state.details.world_size {
                parent.spawn((
                    Text::new(format!("World Size: {}", size)),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            }
        }
        LoadingOperation::ApplyingMods => {
            parent.spawn((
                Text::new("Reloading game systems with new mod configuration"),
                TextFont {
                    font_size: dimensions::FONT_SIZE_SMALL,
                    ..default()
                },
                TextColor(colors::TEXT_MUTED),
            ));
        }
        LoadingOperation::LoadingSave => {
            if let Some(name) = &loading_state.details.save_name {
                parent.spawn((
                    Text::new(format!("Save: {}", name)),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
            }

            if let Some(days) = loading_state.details.game_days {
                parent.spawn((
                    Text::new(format!("World Age: {:.0} days", days)),
                    TextFont {
                        font_size: dimensions::FONT_SIZE_NORMAL,
                        ..default()
                    },
                    TextColor(colors::TEXT_PRIMARY),
                ));
            }
        }
        _ => {}
    }
}

/// Spawn the bottom section with progress bar and tips
fn spawn_bottom_section(parent: &mut ChildSpawnerCommands, loading_state: &LoadingState) {
    parent
        .spawn(Node {
            width: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|bottom| {
            // Status text using LabelBuilder
            bottom.spawn((
                Text::new(&loading_state.current_step),
                TextFont {
                    font_size: dimensions::FONT_SIZE_MEDIUM,
                    ..default()
                },
                TextColor(colors::TEXT_PRIMARY),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                LoadingStatusText,
            ));

            // Progress bar using ProgressBarBuilder with custom label
            let progress_entity = ProgressBarBuilder::new(loading_state.progress)
                .width(Val::Percent(100.0))
                .height(Val::Px(30.0))
                .with_label_text(&loading_state.current_step)
                .animated()
                .margin(UiRect::bottom(Val::Px(20.0)))
                .build(bottom);

            // Mark the progress bar for updates
            bottom
                .commands()
                .entity(progress_entity)
                .insert(LoadingProgressBar);

            // Loading tip using our new tips system
            LabelBuilder::new(get_random_tip())
                .style(LabelStyle::Caption)
                .margin(UiRect::top(Val::Px(20.0)))
                .build(bottom);

            // Cancel button - only show during world generation
            if loading_state.operation == LoadingOperation::GeneratingWorld {
                ButtonBuilder::new("Cancel Generation")
                    .style(ButtonStyle::Danger)
                    .margin(UiRect::top(Val::Px(30.0)))
                    .with_marker(CancelGenerationButton)
                    .build(bottom);
            }
        });
}

/// Cleanup the loading screen
fn cleanup_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingScreenRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// Update the progress bar value
fn update_loading_progress(
    loading_state: Res<LoadingState>,
    mut query: Query<&mut ProgressBar, With<LoadingProgressBar>>,
) {
    if !loading_state.is_changed() {
        return;
    }

    for mut progress_bar in &mut query {
        progress_bar.value = loading_state.progress.clamp(0.0, 1.0);
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
pub fn start_world_generation_loading(loading_state: &mut LoadingState, seed: u32, size: String) {
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
pub fn start_mod_application_loading(loading_state: &mut LoadingState) {
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

/// Handle cancel button interactions
fn handle_cancel_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CancelGenerationButton>)>,
    mut cancel_events: EventWriter<CancelWorldGeneration>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            info!("Cancel Generation button pressed");
            cancel_events.write(CancelWorldGeneration);
        }
    }
}

/// Handle cancel world generation events
fn handle_cancel_generation(
    mut cancel_events: EventReader<CancelWorldGeneration>,
    mut state_events: EventWriter<RequestStateTransition>,
    mut commands: Commands,
) {
    for _event in cancel_events.read() {
        info!("Canceling world generation");

        // Clean up any generation resources
        commands.remove_resource::<crate::world::AsyncWorldGeneration>(); // Dropping this cancels the async task

        // Reset pending world generation flag
        commands.insert_resource(crate::states::PendingWorldGeneration {
            pending: false,
            delay_timer: 0.0,
        });

        // Transition back to world configuration
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::WorldConfiguration,
        });

        info!("Returning to world configuration screen");
    }
}
