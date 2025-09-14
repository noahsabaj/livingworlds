//! Delete confirmation dialog implementation
//!
//! This module creates delete confirmation dialogs using our standard UI builders.

use super::components::*;
use super::SaveGameList;
use crate::ui::{DialogBuilder, DialogType};
use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Resource to store the path to delete upon confirmation
#[derive(Resource)]
pub struct PendingDeletePath(pub PathBuf);

/// Handle delete button clicks - show confirmation dialog
pub fn handle_delete_button_click(
    mut interactions: Query<
        (&Interaction, &DeleteSaveButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    existing_dialog: Query<Entity, With<DeleteConfirmationDialog>>,
) {
    for (interaction, delete_button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Close any existing dialog first
            for entity in &existing_dialog {
                commands.entity(entity).despawn_recursive();
            }

            // Create delete confirmation dialog
            let dialog_entity = DialogBuilder::new(DialogType::Custom)
                .title("Delete Save File?")
                .body(&format!(
                    "Are you sure you want to delete \"{}\"?\n\nThis action cannot be undone.",
                    delete_button.save_name
                ))
                .confirm_button("Delete")
                .cancel_button("Cancel")
                .build(&mut commands);

            // Add our markers to the dialog
            commands
                .entity(dialog_entity)
                .insert(DeleteConfirmationDialog);

            // Store the save path for confirmation
            commands.insert_resource(PendingDeletePath(delete_button.save_path.clone()));
        }
    }
}

/// Handle delete confirmation dialog buttons
pub fn handle_delete_confirmation(
    mut interactions: Query<(&Interaction, &Button), Changed<Interaction>>,
    mut commands: Commands,
    dialog_query: Query<Entity, With<DeleteConfirmationDialog>>,
    mut save_list: ResMut<SaveGameList>,
    browser_root: Query<Entity, With<SaveBrowserRoot>>,
    mut spawn_browser_events: EventWriter<crate::menus::SpawnSaveBrowserEvent>,
    pending_delete: Option<Res<PendingDeletePath>>,
) {
    // Check if a button in the dialog was pressed
    if let Ok(dialog_entity) = dialog_query.get_single() {
        for (interaction, _) in &interactions {
            if *interaction == Interaction::Pressed {
                // Close the dialog
                commands.entity(dialog_entity).despawn_recursive();

                // If we have a pending delete path, perform the deletion
                if let Some(pending) = pending_delete {
                    // Delete the save file
                    if let Err(e) = fs::remove_file(&pending.0) {
                        eprintln!("Failed to delete save file: {}", e);
                    } else {
                        println!("Deleted save file: {:?}", pending.0);

                        // Refresh the save list
                        super::scan_save_files_internal(&mut save_list);

                        // Refresh the save browser UI
                        if let Ok(browser_entity) = browser_root.get_single() {
                            commands.entity(browser_entity).despawn_recursive();

                            // Trigger respawn of the browser
                            spawn_browser_events.send(crate::menus::SpawnSaveBrowserEvent);
                        }
                    }

                    // Remove the pending delete resource
                    commands.remove_resource::<PendingDeletePath>();
                }

                break;
            }
        }
    }
}
