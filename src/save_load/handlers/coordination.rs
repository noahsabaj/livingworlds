//! Event coordination for save/load system
//!
//! This module handles coordination between different save/load events and systems.

use super::{SaveBrowserState, SaveGameList};
use crate::menus::SpawnSaveBrowserEvent;
use bevy::prelude::*;

/// System to handle the SpawnSaveBrowserEvent
pub fn handle_spawn_save_browser_event(
    messages: MessageReader<SpawnSaveBrowserEvent>,
    commands: Commands,
    save_list: ResMut<SaveGameList>,
    browser_state: ResMut<SaveBrowserState>,
) {
    // Simply pass through to the actual spawn function
    // Let spawn_save_browser handle the message reading itself
    super::spawn_save_browser(messages, commands, save_list, browser_state);
}
