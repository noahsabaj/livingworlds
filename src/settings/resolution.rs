//! Resolution confirmation dialog system

use bevy::prelude::*;
use super::types::*;

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
        
        // Use the new dialog system
        use crate::ui::dialogs::presets;
        presets::resolution_confirm_dialog(commands.reborrow());
    }
}

/// Update countdown timer for resolution confirmation
pub fn update_resolution_countdown(
    mut confirmation: ResMut<ResolutionConfirmation>,
    time: Res<Time>,
    mut countdown_texts: Query<&mut Text, With<crate::ui::dialogs::CountdownText>>,
    dialog_query: Query<Entity, With<crate::ui::dialogs::ResolutionConfirmDialog>>,
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
    keep_buttons: Query<&Interaction, (Changed<Interaction>, With<crate::ui::dialogs::KeepButton>)>,
    revert_buttons: Query<&Interaction, (Changed<Interaction>, With<crate::ui::dialogs::RevertButton>)>,
    dialog_query: Query<Entity, With<crate::ui::dialogs::ResolutionConfirmDialog>>,
    mut confirmation: ResMut<ResolutionConfirmation>,
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    mut events: EventWriter<SettingsChanged>,
) {
    if !confirmation.active {
        return;
    }
    
    // Check Keep button
    for interaction in &keep_buttons {
        if *interaction == Interaction::Pressed {
            println!("Resolution confirmed - keeping settings");
            
            // Clean up dialog
            for entity in &dialog_query {
                commands.entity(entity).despawn();
            }
            
            confirmation.active = false;
        }
    }
    
    // Check Revert button
    for interaction in &revert_buttons {
        if *interaction == Interaction::Pressed {
            println!("Resolution rejected - reverting");
            
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