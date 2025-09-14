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
mod overlay_display;
mod tile_info;
mod interaction;

// Other UI modules (single files) - ALL PRIVATE
mod styles;
mod buttons;
mod dialogs;
mod text_inputs;
mod sliders;
mod form;
mod toolbar;
mod loading;
mod tips;
mod builders;

// PUBLIC EXPORTS - Gateway for All UI Types

// Re-export builder types for external use
// This is the ONLY way external code should access UI internals

// From buttons module
pub use buttons::{ButtonBuilder, ButtonStyle, ButtonSize, StyledButton, presets as button_presets};

// From dialogs module
pub use dialogs::{
    DialogBuilder, DialogType, DialogButton, presets as dialog_presets,
    ConfirmButton, CancelButton, SaveButton, DiscardButton,  // Button markers
    KeepButton, RevertButton, CountdownText,  // Additional markers
    // Dialog component types
    DialogOverlay, DialogContainer, DialogTitle, DialogBody, DialogButtonRow,
    ExitConfirmationDialog, UnsavedChangesDialog, ResolutionDialog,
    ResolutionConfirmDialog, WorldGenerationErrorDialog,
};

// From sliders module
pub use sliders::{SliderBuilder, ValueFormat, slider, Slider};

// From text_inputs module
pub use text_inputs::{TextInputBuilder, InputFilter, InputTransform, FocusGroupId, text_input};

// From components directory (already has its own gateway)
pub use components::{
    PanelBuilder, PanelStyle,
    LabelBuilder, LabelStyle,
    SeparatorBuilder, SeparatorStyle,
    Orientation,
    ProgressBarBuilder, ProgressBarStyle,
    ProgressBarFill, ProgressBarTrack, ProgressBarLabel,
};

// From form module
pub use form::{FormBuilder, form, presets as form_presets};

// From toolbar module
pub use toolbar::{ToolbarBuilder, ToolbarOrientation, ToolbarStyle, toolbar, presets as toolbar_presets};

// From loading module
pub use loading::{
    LoadingIndicatorBuilder, LoadingStyle, LoadingSize, LabelPosition,
    loading_spinner, loading_dots, loading_pulse,
};

// From HUD subsystem
pub use hud::{HudPlugin, HudRoot};

// From overlay display subsystem
pub use overlay_display::{OverlayDisplayPlugin, ResourceOverlayText, MineralLegendContainer};

// From tile info subsystem
pub use tile_info::{TileInfoPlugin, TileInfoPanel, TileInfoText};

// From interaction subsystem
pub use interaction::SelectedProvinceInfo;

// From styles module (commonly needed)
pub use styles::{colors, dimensions, layers, helpers};

// From tips module
pub use tips::{get_random_tip, TipCategory, LoadingTip};

// Builders convenience module - re-export its public interface
pub use builders::{
    button, primary_button, danger_button, dialog, progress_bar,
};


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