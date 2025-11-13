//! Browser open/close event handlers
//!
//! This module handles the events for opening and closing the mod browser UI.

use crate::modding::manager::ModManager;
use crate::modding::ui::spawning::spawn_mod_browser;
use crate::modding::ui::state::ModBrowserState;
use crate::modding::ui::types::{
    CloseModBrowserButton, CloseModBrowserEvent, ModBrowserRoot, OpenModBrowserEvent,
};
use bevy::prelude::*;

/// Handles the event to open the mod browser
pub fn handle_open_mod_browser(
    mut commands: Commands,
    mut open_events: MessageReader<OpenModBrowserEvent>,
    existing_browser: Query<Entity, With<ModBrowserRoot>>,
    mod_manager: Res<ModManager>,
    browser_state: Res<ModBrowserState>,
) {
    for _ in open_events.read() {
        // Don't open if already open
        if !existing_browser.is_empty() {
            continue;
        }

        debug!("Opening mod browser UI");
        spawn_mod_browser(&mut commands, &mod_manager, &browser_state);
    }
}

/// Handles the event to close the mod browser
pub fn handle_close_mod_browser(
    mut commands: Commands,
    mut close_events: MessageReader<CloseModBrowserEvent>,
    browser_query: Query<Entity, With<ModBrowserRoot>>,
) {
    for _ in close_events.read() {
        for entity in &browser_query {
            debug!("Closing mod browser UI");
            commands.entity(entity).despawn();
        }
    }
}

/// Handles clicks on the close button
pub fn handle_close_button_clicks(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseModBrowserButton>)>,
    mut close_events: MessageWriter<CloseModBrowserEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            close_events.write(CloseModBrowserEvent);
        }
    }
}