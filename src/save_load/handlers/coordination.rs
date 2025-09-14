//! Event coordination for save/load system
//!
//! This module handles coordination between different save/load events and systems.

use super::{SaveBrowserState, SaveGameList};
use crate::menus::SpawnSaveBrowserEvent;
use bevy::prelude::*;

/// System to handle the SpawnSaveBrowserEvent
pub fn handle_spawn_save_browser_event(
    mut events: EventReader<SpawnSaveBrowserEvent>,
    commands: Commands,
    save_list: ResMut<SaveGameList>,
    browser_state: ResMut<SaveBrowserState>,
) {
    for _ in events.read() {
        // Delegate to the UI module's spawn function
        super::spawn_save_browser(events, commands, save_list, browser_state);
        return; // Only spawn once per frame
    }
}
