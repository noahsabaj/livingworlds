//! Events for Settings UI interactions
//!
//! Events that communicate between settings UI components and the broader system.

use crate::settings::types::{GraphicsPreset, SettingType};
use crate::states::SettingsTab;
use bevy::prelude::*;

/// Generic settings UI event for various interactions
#[derive(Message, Debug, Clone)]
pub enum SettingsUIEvent {
    /// A setting value was modified
    SettingChanged {
        setting_type: SettingType,
        value: SettingValue,
    },
    /// Menu was opened
    MenuOpened,
    /// Menu was closed
    MenuClosed,
    /// Apply button was pressed
    ApplyPressed,
    /// Cancel/Exit button was pressed
    CancelPressed,
}

/// Wrapper for different setting value types
#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    Float(f32),
    String(String),
    WindowMode(crate::settings::types::WindowModeOption),
    Resolution(crate::settings::types::ResolutionOption),
    ShadowQuality(crate::settings::types::QualityLevel),
}

/// Event fired when user switches tabs
#[derive(Message, Debug, Clone)]
pub struct TabSwitchEvent {
    pub from_tab: SettingsTab,
    pub to_tab: SettingsTab,
}

/// Event fired when a graphics preset is applied
#[derive(Message, Debug, Clone)]
pub struct PresetAppliedEvent {
    pub preset: GraphicsPreset,
}
