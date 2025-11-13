//! Search functionality event handlers
//!
//! This module handles search input changes and search submission events
//! for filtering mods in the browser.

use crate::modding::manager::ModManager;
use crate::modding::ui::state::ModBrowserState;
use crate::modding::ui::tabs::spawn_tab_content;
use crate::modding::ui::types::{ContentArea, SearchInputMarker};
use bevy::prelude::*;
use crate::ui::{TextInputSubmitEvent, TextBuffer};

/// Handles search input changes in real-time
pub fn handle_search_input_changes(
    mut commands: Commands,
    mut browser_state: ResMut<ModBrowserState>,
    search_inputs: Query<&TextBuffer, (Changed<TextBuffer>, With<SearchInputMarker>)>,
    content_query: Query<Entity, With<ContentArea>>,
    mod_manager: Res<ModManager>,
) {
    for text_value in &search_inputs {
        let new_query = text_value.content.clone();

        // Only update if search query actually changed
        if browser_state.search_query != new_query {
            debug!("Search query changed: '{}'", new_query);
            browser_state.search_query = new_query;

            // Rebuild content with filtered mods
            for entity in &content_query {
                commands.entity(entity).despawn();
                commands.entity(entity).with_children(|parent| {
                    spawn_tab_content(
                        parent,
                        browser_state.current_tab,
                        &mod_manager,
                        &browser_state.search_query,
                    );
                });
            }
        }
    }
}

/// Handles search submit events (Enter key)
pub fn handle_search_submit(
    mut submit_events: MessageReader<TextInputSubmitEvent>,
    text_inputs: Query<&TextBuffer>,
    _browser_state: Res<ModBrowserState>,
) {
    for event in submit_events.read() {
        if let Ok(text_value) = text_inputs.get(event.entity) {
            // For now, just log the search submission
            debug!("Search submitted: {}", text_value.content);

            // Future: This could trigger more advanced search operations
            // like searching the Steam Workshop API or applying
            // more complex filters
        }
    }
}

// TODO: Future search enhancements
// - Search history
// - Advanced filters (by author, version, tags)
// - Fuzzy search support
// - Search suggestions/autocomplete
// - Workshop API integration for online search