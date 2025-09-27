//! UI Spawning Subsystem - Gateway
//!
//! Gateway for all settings menu spawning functionality. Controls the creation
//! of the settings menu and its various UI components.

// PRIVATE MODULES - Implementation hidden
mod apply_cancel;
mod menu;
mod presets;
mod tabs;

// CONTROLLED EXPORTS - Main spawning function and utilities
pub use menu::spawn_settings_menu;

// Internal spawning utilities (used by menu.rs)
pub(in crate::settings::ui) use apply_cancel::spawn_apply_cancel_buttons;
pub(in crate::settings::ui) use tabs::spawn_tab_buttons;
