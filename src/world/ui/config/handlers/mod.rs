//! Event handlers module gateway
//!
//! This module contains all the event handling systems for the world configuration UI.

mod input;
mod selection;
mod navigation;
mod display;
mod interactions;

// Export all handler functions for the plugin
pub use input::{
    handle_text_input_changes,
    handle_random_buttons,
};

pub use selection::{
    handle_preset_selection,
    handle_size_selection,
    handle_climate_selection,
    handle_island_selection,
    handle_aggression_selection,
    handle_resource_selection,
};

pub use navigation::{
    init_default_settings,
    handle_generate_button,
    handle_back_button,
};

pub use display::{
    update_seed_display,
    update_slider_displays,
};

pub use interactions::{
    handle_preset_hover,
    handle_advanced_toggle,
    handle_slider_interactions,
};