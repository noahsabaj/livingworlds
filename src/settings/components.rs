//! UI Component markers for the settings menu
//!
//! This module contains all the component markers used to identify
//! and interact with various UI elements in the settings menu.

use bevy::prelude::*;
use crate::states::SettingsTab;
use super::types::SettingType;

// ============================================================================
// ROOT MARKERS
// ============================================================================

/// Marker for the settings menu root entity
#[derive(Component)]
pub struct SettingsMenuRoot;

/// Marker for the resolution confirmation dialog
#[derive(Component)]
pub struct ResolutionConfirmDialog;

// ============================================================================
// NAVIGATION COMPONENTS
// ============================================================================

/// Marker for tab buttons
#[derive(Component)]
pub struct TabButton {
    pub tab: SettingsTab,
}

/// Component for keyboard-focusable elements
#[derive(Component)]
pub struct Focusable {
    pub order: u32,
}

/// Resource tracking the currently focused element
#[derive(Resource, Default)]
pub struct FocusedElement {
    pub entity: Option<Entity>,
    pub index: usize,
    pub max_index: usize,
}

// ============================================================================
// INTERACTIVE ELEMENTS
// ============================================================================

/// Component for cycle buttons that cycle through options
#[derive(Component)]
pub struct CycleButton {
    pub setting_type: SettingType,
}

/// Component for toggle checkboxes
#[derive(Component)]
pub struct ToggleButton {
    pub setting_type: SettingType,
    pub enabled: bool,
}

/// Component for sliders
#[derive(Component)]
pub struct Slider {
    pub setting_type: SettingType,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

/// Component for slider handles (draggable part)
#[derive(Component)]
pub struct SliderHandle;

/// Component for slider value text displays
#[derive(Component)]
pub struct SliderValueText {
    pub setting_type: SettingType,
}

// ============================================================================
// SLIDER MARKERS FOR NEW SLIDER SYSTEM
// ============================================================================

/// Marker for settings sliders using the new SliderBuilder
#[derive(Component)]
pub struct SettingsSlider {
    pub setting_type: SettingType,
}

/// Marker for master volume slider
#[derive(Component)]
pub struct MasterVolumeSlider;

/// Marker for music volume slider
#[derive(Component)]
pub struct MusicVolumeSlider;

/// Marker for SFX volume slider
#[derive(Component)]
pub struct SFXVolumeSlider;

/// Marker for render scale slider
#[derive(Component)]
pub struct RenderScaleSlider;

// ============================================================================
// BUTTON MARKERS
// ============================================================================

/// Apply button marker
#[derive(Component)]
pub struct ApplyButton;

/// Cancel button marker
#[derive(Component)]
pub struct CancelButton;

/// Graphics preset button
#[derive(Component)]
pub struct PresetButton {
    pub preset: super::types::GraphicsPreset,
}

/// Reset to defaults button
#[derive(Component)]
pub struct ResetButton;

// ============================================================================
// DIALOG MARKERS
// ============================================================================

// Dialog components are now provided by crate::ui_toolbox::dialogs

// ============================================================================
// TEXT MARKERS
// ============================================================================

/// Countdown text for resolution confirmation
#[derive(Component)]
pub struct CountdownText;