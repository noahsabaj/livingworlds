//! Settings UI Handlers Subsystem - Gateway
//!
//! Gateway for all settings interaction handlers. Contains only high-level
//! handlers that are not replaced by the declarative macro system.
//!
//! ## REVOLUTIONARY CLEANUP
//!
//! The controls.rs module (265 lines of repetitive event handling) has been
//! ELIMINATED and replaced by generated handlers from define_setting_tab! macro.
//! This removes all manual cycle, toggle, and slider event handling code.

// PRIVATE MODULES - Implementation hidden (NON-OBSOLETE HANDLERS ONLY)
mod apply_cancel;
mod presets;
mod tabs;
mod validation;

// CONTROLLED EXPORTS - Handler functions for plugin registration
pub use apply_cancel::{
    handle_apply_cancel_buttons, handle_unsaved_changes_dialog, update_apply_button_state,
    update_apply_exit_button_hover,
};

pub use presets::{handle_preset_buttons, handle_reset_button};

pub use tabs::handle_tab_buttons;

pub use validation::apply_settings_changes;

// Components marker struct for external queries (minimal exposure)
pub struct SettingsUIComponents;

impl SettingsUIComponents {
    // Provide controlled access to component queries if needed
}
