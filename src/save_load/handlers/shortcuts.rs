//! Keyboard shortcut handling
//!
//! This module handles keyboard shortcuts for save and load operations.

use super::SaveGameList;
use super::{LoadGameEvent, SaveGameEvent};
use bevy::prelude::*;

/// Handle keyboard shortcuts for save/load (F5 = quick save, F9 = quick load)
pub fn handle_save_load_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut save_list: ResMut<SaveGameList>,
) {
    // F5 for quick save
    if keyboard.just_pressed(KeyCode::F5) {
        println!("F5 pressed - Quick saving...");
        save_events.write(SaveGameEvent {
            slot_name: "quicksave".to_string(),
        });
    }

    // F9 for quick load
    if keyboard.just_pressed(KeyCode::F9) {
        println!("F9 pressed - Quick loading...");
        // Scan for saves directly
        super::scan_save_files_internal(&mut save_list);

        if let Some(latest) = save_list.saves.first() {
            load_events.write(LoadGameEvent {
                save_path: latest.path.clone(),
            });
        } else {
            println!("No save files found to load");
        }
    }
}
