//! User Interface module for Living Worlds
//! 
//! Handles all UI elements including FPS display, simulation controls,
//! and game state information display.

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

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
    let fps_container = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(2.0),  // 2% from bottom
            right: Val::Percent(2.0),   // 2% from right
            padding: UiRect::all(Val::Percent(1.0)),  // 1% padding
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)), // Dark background
        Visibility::Visible,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("FPS: LOADING"),  // Initial loading text
            TextFont {
                font_size: 48.0,  // Large but reasonable size
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)), // Yellow for initial visibility
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
                // Green > 30 FPS, Yellow 15-30 FPS, Red < 15 FPS
                let color = if value >= 30.0 {
                    Color::srgb(0.0, 1.0, 0.0) // Green - good performance
                } else if value >= 15.0 {
                    Color::srgb(1.0, 1.0, 0.0) // Yellow - acceptable
                } else {
                    Color::srgb(1.0, 0.0, 0.0) // Red - poor performance
                };
                
                // Update the text color
                text_color.0 = color;
            } else {
                // No smoothed data yet, show raw FPS
                if let Some(value) = fps.value() {
                    *text = Text::new(format!("FPS: {:.1}", value));
                    text_color.0 = Color::srgb(1.0, 1.0, 0.0); // Yellow while warming up
                }
            }
        } else {
            // Diagnostics not available yet
            *text = Text::new("FPS: Initializing...");
            text_color.0 = Color::srgb(0.5, 0.5, 0.5); // Gray for initializing
        }
    }
}