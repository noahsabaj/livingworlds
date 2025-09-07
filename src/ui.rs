//! User Interface module for Living Worlds
//! 
//! Handles all UI elements including FPS display, simulation controls,
//! and game state information display.

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::constants::*;

/// Marker component for FPS text display
#[derive(Component)]
pub struct FpsText;

/// UI Plugin that handles all user interface elements
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, fps_display_system);
    }
}

/// Setup the UI elements including FPS counter
pub fn setup_ui(mut commands: Commands) {
    // Setup FPS display in bottom-right corner with responsive scaling
    let _fps_container = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(UI_MARGIN_PERCENT),
            right: Val::Percent(UI_MARGIN_PERCENT),
            padding: UiRect::all(Val::Percent(UI_PADDING_PERCENT)),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        BackgroundColor(COLOR_UI_BACKGROUND),
        Visibility::Visible,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("FPS: LOADING"),  // Initial loading text
            TextFont {
                font_size: UI_FPS_TEXT_SIZE,
                ..default()
            },
            TextColor(COLOR_FPS_ACCEPTABLE),
            FpsText,
        ));
    }).id();
    
    // UI initialized with FPS display
}

/// FPS display update system - Updates the FPS counter with color coding
pub fn fps_display_system(
    diagnostics: Res<DiagnosticsStore>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<FpsText>>,
) {
    for (mut text, mut text_color) in &mut text_query {
        // Always update FPS text
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the text content
                *text = Text::new(format!("FPS: {:.1}", value));
                
                // Color code based on performance
                // Color based on FPS thresholds
                let color = if value >= FPS_GOOD_THRESHOLD as f64 {
                    COLOR_FPS_GOOD
                } else if value >= FPS_ACCEPTABLE_THRESHOLD as f64 {
                    COLOR_FPS_ACCEPTABLE
                } else {
                    COLOR_FPS_POOR
                };
                
                // Update the text color
                text_color.0 = color;
            } else {
                // No smoothed data yet, show raw FPS
                if let Some(value) = fps.value() {
                    *text = Text::new(format!("FPS: {:.1}", value));
                    text_color.0 = COLOR_FPS_ACCEPTABLE;
                }
            }
        } else {
            // Diagnostics not available yet
            *text = Text::new("FPS: Initializing...");
            text_color.0 = COLOR_FPS_INITIALIZING;
        }
    }
}