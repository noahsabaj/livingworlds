//! Settings UI Module - Gateway Architecture
//!
//! Pure gateway module for settings UI functionality. Provides controlled API
//! surface for settings menu creation, interaction handling, and customization.
//!
//! Follows the gateway architecture pattern established by the ui/ module.

use bevy::prelude::*;

// PRIVATE MODULES - All implementation hidden behind gateway
mod builders;
mod content;
mod handlers;
mod plugin;
mod spawning;

// CONTROLLED EXPORTS - Minimal public API

// Main entry point for spawning settings menu
pub use spawning::spawn_settings_menu;

// Settings-specific builders (eating our own dog food)
pub use builders::{PresetGridBuilder, SettingRowBuilder, SettingSectionBuilder};

// Plugin for settings UI functionality
pub use plugin::SettingsUIPlugin;

// Essential components for external queries (minimal exposure)
pub use handlers::SettingsUIComponents;

// Events for settings UI interactions
mod events;
pub use events::{PresetAppliedEvent, SettingsUIEvent, TabSwitchEvent};
