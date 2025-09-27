//! Apply/Cancel Button Handlers
//!
//! Focused handlers for Apply/Cancel buttons and unsaved changes dialog.

use crate::settings::{components::*, persistence::save_settings, types::*};
use crate::ui::colors;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

/// Handle Apply and Exit buttons
pub fn handle_apply_cancel_buttons(
    interactions: Query<(&Interaction, AnyOf<(&ApplyButton, &CancelButton)>), Changed<Interaction>>,
    mut commands: Commands,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    mut settings: ResMut<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    mut events: EventWriter<SettingsChanged>,
    mut resolution_events: EventWriter<RequestResolutionConfirm>,
    mut pkv: ResMut<PkvStore>,
    dirty_state: Res<SettingsDirtyState>,
) {
    for (interaction, (apply_button, cancel_button)) in &interactions {
        if *interaction == Interaction::Pressed {
            if apply_button.is_some() {
                // Apply button pressed - but ignore if no changes
                if !dirty_state.is_dirty {
                    // Button is disabled when no changes - ignore the click completely
                    continue;
                }

                info!("Applying settings");

                let resolution_changed = settings.graphics.resolution.width
                    != temp_settings.0.graphics.resolution.width
                    || settings.graphics.resolution.height
                        != temp_settings.0.graphics.resolution.height
                    || settings.graphics.window_mode != temp_settings.0.graphics.window_mode;

                // Copy temp settings to actual settings
                *settings = temp_settings.0.clone();
                save_settings(&*settings, &mut *pkv);
                // Fire event to apply settings
                events.write(SettingsChanged);

                // Trigger resolution confirmation if needed
                if resolution_changed {
                    resolution_events.write(RequestResolutionConfirm);
                }

                // Close settings menu after applying
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if cancel_button.is_some() {
                // Exit button pressed
                if dirty_state.is_dirty {
                    // Show unsaved changes dialog
                    debug!("Unsaved changes detected - spawning confirmation dialog");
                    spawn_unsaved_changes_dialog(commands.reborrow());
                } else {
                    // No changes, just close
                    debug!("Exiting settings (no changes)");
                    // Revert temp settings to match current settings
                    temp_settings.0 = settings.clone();
                    if let Ok(entity) = settings_root.single() {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

/// Handle unsaved changes dialog buttons
pub fn handle_unsaved_changes_dialog(
    interactions: Query<
        (
            &Interaction,
            AnyOf<(
                &crate::ui::SaveButton,
                &crate::ui::DiscardButton,
                &bevy_ui_builders::CancelButton,
            )>,
        ),
        Changed<Interaction>,
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::UnsavedChangesDialog>>,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    mut settings: ResMut<GameSettings>,
    temp_settings: Res<TempGameSettings>,
    mut events: EventWriter<SettingsChanged>,
    mut pkv: ResMut<PkvStore>,
) {
    for (interaction, (save_button, discard_button, cancel_button)) in &interactions {
        if *interaction == Interaction::Pressed {
            // Close the dialog first
            if let Ok(dialog_entity) = dialog_query.single() {
                commands.entity(dialog_entity).despawn();
            }

            if save_button.is_some() {
                info!("Saving changes and exiting");
                *settings = temp_settings.0.clone();
                save_settings(&*settings, &mut *pkv);
                events.write(SettingsChanged);

                // Close settings menu
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if discard_button.is_some() {
                // Discard changes and exit
                info!("Discarding changes and exiting");

                // Close settings menu without saving
                if let Ok(entity) = settings_root.single() {
                    commands.entity(entity).despawn();
                }
            } else if cancel_button.is_some() {
                // Cancel - stay in settings
                debug!("Staying in settings menu");
                // Dialog is already closed, do nothing else
            }
        }
    }
}

/// Update Apply button visual state based on dirty state
pub fn update_apply_button_state(
    dirty_state: Res<SettingsDirtyState>,
    mut apply_buttons: Query<(&mut BackgroundColor, &Children), With<ApplyButton>>,
    mut text_colors: Query<&mut TextColor>,
) {
    if dirty_state.is_changed() {
        for (mut bg_color, children) in &mut apply_buttons {
            if dirty_state.is_dirty {
                // Enable button - green background
                *bg_color = BackgroundColor(colors::SUCCESS);
                // Enable text - bright white
                for child in children.iter() {
                    if let Ok(mut text_color) = text_colors.get_mut(child) {
                        *text_color = TextColor(colors::TEXT_PRIMARY); // Bright white
                    }
                }
            } else {
                // Disable button - grayed out background
                *bg_color = BackgroundColor(colors::DISABLED);
                // Disable text - muted gray
                for child in children.iter() {
                    if let Ok(mut text_color) = text_colors.get_mut(child) {
                        *text_color = TextColor(colors::TEXT_MUTED); // Gray text
                    }
                }
            }
        }
    }
}

/// Update Apply/Exit button hover effects
pub fn update_apply_exit_button_hover(
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            AnyOf<(&ApplyButton, &CancelButton)>,
        ),
        Changed<Interaction>,
    >,
    dirty_state: Res<SettingsDirtyState>,
) {
    for (interaction, mut bg_color, mut border_color, (apply_button, _cancel_button)) in
        &mut interactions
    {
        if apply_button.is_some() {
            // Apply button
            match *interaction {
                Interaction::Hovered => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(colors::SUCCESS_HOVER);
                        *border_color = BorderColor(colors::BORDER_SELECTED_HOVER);
                    } else {
                        *bg_color = BackgroundColor(colors::DISABLED_HOVER);
                        *border_color = BorderColor(colors::BORDER_DISABLED);
                    }
                }
                Interaction::Pressed => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(colors::SUCCESS_HOVER);
                        *border_color = BorderColor(colors::BORDER_SELECTED_HOVER);
                    }
                }
                Interaction::None => {
                    if dirty_state.is_dirty {
                        *bg_color = BackgroundColor(colors::SUCCESS);
                        *border_color = BorderColor(colors::BORDER_SELECTED);
                    } else {
                        *bg_color = BackgroundColor(colors::DISABLED);
                        *border_color = BorderColor(colors::BORDER_DISABLED);
                    }
                }
            }
        } else {
            // Exit button
            match *interaction {
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(colors::DANGER_HOVER);
                    *border_color = BorderColor(colors::BORDER_DANGER_HOVER);
                }
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(colors::DANGER_PRESSED);
                    *border_color = BorderColor(colors::BORDER_DANGER_HOVER);
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(colors::DANGER);
                    *border_color = BorderColor(colors::BORDER_DANGER);
                }
            }
        }
    }
}

/// Spawn unsaved changes confirmation dialog
fn spawn_unsaved_changes_dialog(mut commands: Commands) {
    // Use the new dialog builder system
    use crate::ui::dialog_presets;
    dialog_presets::unsaved_changes_dialog(&mut commands);
}
