//! Event handlers subsystem for the mod browser
//!
//! This module contains all event handling logic separated by concern,
//! keeping the UI spawning logic separate from interaction handling.

// Internal modules - all private
mod browser;
mod interactions;
mod search;
mod tabs;

// Re-export public handler functions
pub use browser::{handle_close_mod_browser, handle_close_button_clicks, handle_open_mod_browser};
pub use interactions::{handle_apply_changes, handle_confirm_modset_clicks, handle_mod_toggles};
pub use search::{handle_search_input_changes, handle_search_submit};
pub use tabs::{handle_tab_button_clicks, handle_tab_switching, update_tab_buttons};