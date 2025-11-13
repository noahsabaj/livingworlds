//! Auto-save functionality
//!
//! This module handles automatic saving at regular intervals.

use super::{AutoSaveTimer, SaveGameEvent};
use bevy::prelude::*;

/// Handle auto-save timer
pub fn handle_auto_save(
    time: Res<Time>,
    mut timer: ResMut<AutoSaveTimer>,
    mut save_events: MessageWriter<SaveGameEvent>,
) {
    if !timer.enabled {
        return;
    }

    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        info!("Auto-saving game...");
        save_events.write(SaveGameEvent {
            slot_name: "autosave".to_string(),
        });
    }
}
