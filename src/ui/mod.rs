//! User Interface Module - Pure Gateway Architecture
//!
//! This module orchestrates all UI subsystems without containing any
//! implementation logic. Each UI concern is delegated to focused submodules.
//!
//! ## Architecture
//!
//! - **components/**: Reusable UI components (panels, labels, etc.)
//! - **hud/**: Heads-up display (time, speed, controls)
//! - **overlay_display/**: Map overlay and mineral legend
//! - **tile_info/**: Province selection information panel
//! - **Other UI modules**: Buttons, dialogs, forms, etc.

use bevy::prelude::*;

// SUBMODULES - ALL PRIVATE for gateway architecture

// Core UI components (directory with gateway)
mod components;

// UI subsystems (directories with gateways)
mod hud;
mod interaction;
mod overlay_display;
mod tile_info;

// Other UI modules (single files) - ALL PRIVATE
mod builders;
mod buttons;
mod dialogs;
mod form;
mod loading;
mod sliders;
mod styles;
mod text_inputs;
mod tips;
mod toolbar;

// PUBLIC EXPORTS - Gateway for All UI Types

// Re-export builder types for external use
// This is the ONLY way external code should access UI internals

// From buttons module
pub use buttons::{
    presets as button_presets, ButtonBuilder, ButtonSize, ButtonStyle, StyledButton,
};

// From dialogs module
pub use dialogs::{
    presets as dialog_presets,
    CancelButton,
    ConfirmButton,
    CountdownText, // Additional markers
    DialogBody,
    DialogBuilder,
    DialogButton,
    DialogButtonRow,
    DialogContainer,
    // Dialog component types
    DialogOverlay,
    DialogTitle,
    DialogType,
    DiscardButton, // Button markers
    ExitConfirmationDialog,
    KeepButton,
    ResolutionConfirmDialog,
    ResolutionDialog,
    RevertButton,
    SaveButton,
    UnsavedChangesDialog,
    WorldGenerationErrorDialog,
};

// From sliders module
pub use sliders::{slider, Slider, SliderBuilder, ValueFormat};

// From text_inputs module
pub use text_inputs::{text_input, FocusGroupId, InputFilter, InputTransform, TextInputBuilder};

// From components directory (already has its own gateway)
pub use components::{
    LabelBuilder, LabelStyle, Orientation, PanelBuilder, PanelStyle, ProgressBarBuilder,
    ProgressBarFill, ProgressBarLabel, ProgressBarStyle, ProgressBarTrack, SeparatorBuilder,
    SeparatorStyle,
};

// From form module
pub use form::{form, presets as form_presets, FormBuilder};

// From toolbar module
pub use toolbar::{
    presets as toolbar_presets, toolbar, ToolbarBuilder, ToolbarOrientation, ToolbarStyle,
};

// From loading module
pub use loading::{
    loading_dots, loading_pulse, loading_spinner, LabelPosition, LoadingIndicatorBuilder,
    LoadingSize, LoadingStyle,
};

// From HUD subsystem
pub use hud::{HudPlugin, HudRoot};

// From overlay display subsystem
pub use overlay_display::{MapModeText, MineralLegendContainer, OverlayDisplayPlugin};

// From tile info subsystem
pub use tile_info::{TileInfoPanel, TileInfoPlugin, TileInfoText};

// From interaction subsystem
pub use interaction::SelectedProvinceInfo;

// From styles module (commonly needed)
pub use styles::{colors, dimensions, helpers, layers};

// From tips module
pub use tips::{get_random_tip, LoadingTip, TipCategory};

// Builders convenience module - re-export its public interface
pub use builders::{button, danger_button, dialog, primary_button, progress_bar};

/// Main UI Plugin that coordinates all UI subsystems
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Add all UI subsystem plugins
        app
            // Core systems
            .add_plugins(buttons::ButtonPlugin)
            .add_plugins(dialogs::DialogPlugin)
            .add_plugins(text_inputs::TextInputPlugin)
            .add_plugins(loading::LoadingIndicatorPlugin)
            .add_plugins(sliders::SliderPlugin)
            // Game UI systems
            .add_plugins(hud::HudPlugin)
            .add_plugins(overlay_display::OverlayDisplayPlugin)
            .add_plugins(tile_info::TileInfoPlugin);
    }
}
