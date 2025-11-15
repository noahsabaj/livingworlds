//! Event handlers module gateway
//!
//! This module contains all the event handling systems for the world configuration UI.

mod display;
mod input;
mod interactions;
mod navigation;
mod selection;

// Export all handler functions for the plugin
pub use input::{handle_random_buttons, handle_text_input_changes};

pub use selection::{
    handle_aggression_selection, handle_calendar_selection, handle_climate_selection,
    handle_island_selection, handle_preset_selection, handle_resource_selection,
    handle_size_selection,
};

pub use navigation::{handle_back_button, handle_generate_button, init_default_settings};

pub use display::{update_seed_display, update_slider_displays};

pub use interactions::{handle_advanced_toggle, handle_preset_hover, handle_slider_interactions};
