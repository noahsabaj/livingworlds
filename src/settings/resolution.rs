//! Resolution confirmation dialog system

use bevy::prelude::*;
use super::types::*;
use super::components::*;

/// Handle request to show resolution confirmation dialog  
pub fn handle_resolution_confirm_request(
    mut commands: Commands,
    mut events: EventReader<RequestResolutionConfirm>,
    mut confirmation: ResMut<ResolutionConfirmation>,
    settings: Res<GameSettings>,
) {
    for _ in events.read() {
        if confirmation.active {
            continue; // Already showing dialog
        }
        
        println!("Spawning resolution confirmation dialog");
        confirmation.active = true;
        confirmation.timer.reset();
        confirmation.original_resolution = settings.graphics.resolution.clone();
        confirmation.original_window_mode = settings.graphics.window_mode.clone();
        
        // Spawn dialog UI
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
            ResolutionConfirmDialog,
        )).with_children(|overlay| {
            overlay.spawn((
                Node {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
                BorderColor(Color::srgb(0.4, 0.4, 0.45)),
            )).with_children(|dialog| {
                // Title
                dialog.spawn((
                    Text::new("Keep Display Settings?"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
                
                // Countdown text
                dialog.spawn((
                    Text::new("Reverting in 15 seconds..."),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    Node {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                    CountdownText,
                ));
                
                // Buttons
                dialog.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                )).with_children(|buttons| {
                    // Keep button
                    buttons.spawn((
                        Button,
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.25, 0.15)),
                        BorderColor(Color::srgb(0.3, 0.5, 0.3)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Keep"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });
                    
                    // Revert button
                    buttons.spawn((
                        Button,
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.15, 0.15)),
                        BorderColor(Color::srgb(0.5, 0.3, 0.3)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("Revert"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });
                });
            });
        });
    }
}

/// Update countdown timer for resolution confirmation
pub fn update_resolution_countdown(
    mut confirmation: ResMut<ResolutionConfirmation>,
    time: Res<Time>,
    mut countdown_texts: Query<&mut Text, With<CountdownText>>,
    dialog_query: Query<Entity, With<ResolutionConfirmDialog>>,
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    mut events: EventWriter<SettingsChanged>,
) {
    if !confirmation.active {
        return;
    }
    
    confirmation.timer.tick(time.delta());
    
    // Update countdown text
    let remaining = confirmation.timer.remaining_secs();
    for mut text in &mut countdown_texts {
        **text = format!("Reverting in {} seconds...", remaining.ceil() as u32);
    }
    
    // Auto-revert when timer expires
    if confirmation.timer.finished() {
        println!("Resolution confirmation timed out - reverting");
        
        // Revert settings
        settings.graphics.resolution = confirmation.original_resolution.clone();
        settings.graphics.window_mode = confirmation.original_window_mode.clone();
        events.write(SettingsChanged);
        
        // Clean up dialog
        for entity in &dialog_query {
            commands.entity(entity).despawn();
        }
        
        confirmation.active = false;
    }
}

/// Handle buttons in the resolution confirmation dialog
pub fn handle_resolution_confirm_buttons(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    dialog_query: Query<Entity, With<ResolutionConfirmDialog>>,
    mut confirmation: ResMut<ResolutionConfirmation>,
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    mut events: EventWriter<SettingsChanged>,
) {
    if !confirmation.active {
        return;
    }
    
    for interaction in &mut interactions {
        if *interaction == Interaction::Pressed {
            // For simplicity, first button keeps, second reverts
            // In production, would use proper component markers
            
            // Clean up dialog
            for entity in &dialog_query {
                commands.entity(entity).despawn();
            }
            
            confirmation.active = false;
        }
    }
}

/// Additional fields for ResolutionConfirmation resource
impl ResolutionConfirmation {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(15.0, TimerMode::Once),
            original_resolution: ResolutionOption::default(),
            original_window_mode: WindowModeOption::Windowed,
            active: false,
        }
    }
}

/// Extension trait for ResolutionConfirmation
impl ResolutionConfirmation {
    pub fn active(&self) -> bool {
        self.active
    }
    
    pub fn activate(&mut self) {
        self.active = true;
        self.timer.reset();
    }
    
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}