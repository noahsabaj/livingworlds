//! UI spawning subsystem for the mod browser
//!
//! This module contains all the UI construction logic,
//! separated from event handling and state management.

// Internal modules - all private
mod browser;
mod search;

// Re-export public spawning functions
pub use browser::spawn_mod_browser;
pub use search::spawn_search_bar;