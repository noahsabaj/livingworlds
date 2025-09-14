//! Auto-save functionality
//!
//! This module handles automatic saving at regular intervals.

use bevy::prelude::*;
use super::{SaveGameEvent, AutoSaveTimer};

/// Handle auto-save timer
pub fn handle_auto_save(
    time: Res<Time>,
    mut timer: ResMut<AutoSaveTimer>,
    mut save_events: EventWriter<SaveGameEvent>,
) {
    if !timer.enabled {
        return;
    }

    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        println!("Auto-saving game...");
        save_events.write(SaveGameEvent {
            slot_name: "autosave".to_string(),
        });
    }
}