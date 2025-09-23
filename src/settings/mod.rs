//! Settings Module - Gateway Architecture
//!
//! Pure gateway module for settings functionality. Provides controlled API
//! for settings management, persistence, and UI interactions.
//!
//! Follows the gateway architecture pattern established by the ui/ module.

use bevy::prelude::*;

// PRIVATE MODULES - All implementation hidden behind gateway
mod components;
mod navigation;
mod persistence;
mod resolution;
mod setting_builder; // NEW: Declarative settings automation system
mod settings_ui;
mod types;

// CONTROLLED EXPORTS - Minimal public API

// Essential types for external use
pub use types::{
    AudioSettings, ControlSettings, GameSettings, GraphicsPreset, GraphicsSettings,
    InterfaceSettings, QualityLevel, RequestResolutionConfirm, ResolutionConfirmation,
    ResolutionOption, SettingType, SettingsChanged, SettingsDirtyState, TempGameSettings,
    WindowModeOption,
};

// Essential components for external queries (minimal exposure)
pub use components::{
    ApplyButton, CancelButton, CycleButton, Focusable, FocusedElement, PresetButton,
    SettingsMenuRoot, SettingsSlider, TabButton, ToggleButton,
};

// Core functionality
pub use persistence::{load_settings, save_settings};

// Settings UI builders (eating our own dog food)
pub use settings_ui::{
    spawn_settings_menu, PresetGridBuilder, SettingRowBuilder, SettingSectionBuilder,
};

// NEW: Declarative settings automation system
pub use setting_builder::define_setting_tab;

// Main plugin - Clean architecture with no legacy wrappers
pub use settings_ui::SettingsUIPlugin;

/// Helper function to despawn the settings menu
pub fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
