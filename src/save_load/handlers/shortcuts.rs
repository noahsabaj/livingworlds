//! Keyboard shortcut handling
//!
//! This module handles keyboard shortcuts for save and load operations using the shortcuts registry.

use super::SaveGameList;
use super::{LoadGameEvent, SaveGameEvent};
use crate::ui::shortcuts::{ShortcutEvent, ShortcutId};
use bevy::prelude::*;

/// Handle keyboard shortcuts for save/load using the centralized shortcuts registry
pub fn handle_save_load_shortcuts(
    mut shortcut_events: EventReader<ShortcutEvent>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut save_list: ResMut<SaveGameList>,
) {
    for event in shortcut_events.read() {
        match event.shortcut_id {
            ShortcutId::QuickSave => {
                debug!("Quick save shortcut triggered");
                save_events.send(SaveGameEvent {
                    slot_name: "quicksave".to_string(),
                });
            }
            ShortcutId::QuickLoad => {
                debug!("Quick load shortcut triggered");
                // Scan for saves directly
                super::scan_save_files_internal(&mut save_list);

                if let Some(latest) = save_list.saves.first() {
                    load_events.send(LoadGameEvent {
                        save_path: latest.path.clone(),
                    });
                } else {
                    warn!("No save files found to load");
                }
            }
            _ => {} // Other shortcuts not handled here
        }
    }
}
