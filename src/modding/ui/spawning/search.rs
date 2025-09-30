//! Search bar spawning functionality
//!
//! This module handles the spawning of the search bar UI component
//! used for filtering mods in the browser.

use crate::modding::ui::types::SearchInputMarker;
use crate::ui::text_input;
use bevy::prelude::*;

/// Spawns the search bar in the header
pub fn spawn_search_bar(parent: &mut ChildSpawnerCommands, initial_query: &str) {
    text_input()
        .with_value(initial_query)
        .with_font_size(16.0)
        .with_width(Val::Px(300.0))
        .with_height(Val::Px(40.0))
        .with_padding(UiRect::all(Val::Px(10.0)))
        .independent() // Search is independent, not part of a group
        .inactive()
        .with_marker(SearchInputMarker)
        .build(parent);
}