//! User Interface Module - Pure Gateway Architecture
//!
//! This is a PURE GATEWAY - no implementation code, only module organization.
//! External modules should handle their own UI creation internally.

use bevy::prelude::*;

// Back to ChildSpawnerCommands - this was much closer to working
// The type mismatch was small compared to EntityCommands
pub type ChildBuilder<'a> = ChildSpawnerCommands<'a>;

// PRIVATE MODULES - All implementation hidden
mod builders;
mod buttons;
mod cleanup;
mod components;
mod dialogs;
mod form;
mod hud;
mod interaction;
mod loading;
mod nation_info;
mod nation_selection;
mod overlay_display;
mod performance_dashboard;
mod plugin;
mod sliders;
pub mod styles;
mod text_inputs;
mod tile_info;
mod tips;
mod toolbar;

// ESSENTIAL EXPORTS - Minimal public API

// Marker components for queries
pub use dialogs::{
    CancelButton, ConfirmButton, DiscardButton, KeepButton, RevertButton, SaveButton,
};

// State markers
pub use dialogs::{
    ExitConfirmationDialog, ResolutionConfirmDialog, ResolutionDialog, UnsavedChangesDialog,
    WorldGenerationErrorDialog,
};

// HUD/Display markers
pub use hud::HudRoot;
pub use interaction::{
    handle_selection_interaction,
    ButtonValue,
    FieldUpdater,
    SelectedProvinceInfo,
    // UI interaction automation system
    SelectionConfig,
    SelectionStyling,
};

// UI interaction automation macros
pub use crate::{define_marker_interactions, define_ui_interactions};
pub use nation_info::{NationInfoPanel, SelectedNation};
pub use overlay_display::{MapModeText, MineralLegendContainer};
pub use performance_dashboard::{DashboardVisibility, PerformancePanel};
pub use tile_info::{TileInfoPanel, TileInfoText};

// Builder components and types
pub use buttons::{ButtonBuilder, ButtonSize, ButtonStyle, StyledButton};
pub use components::{
    // Label system
    LabelBuilder,
    LabelStyle,
    Orientation,
    // Panel system
    PanelBuilder,
    PanelStyle,
    // Progress bar system
    ProgressBar,
    ProgressBarBuilder,
    ProgressBarPlugin,
    // Separator system
    SeparatorBuilder,
};
pub use dialogs::{DialogBuilder, DialogOverlay, DialogType};
pub use loading::{LoadingIndicatorBuilder, LoadingSize, LoadingStyle};
pub use sliders::{Slider, SliderBuilder, ValueFormat};
pub use text_inputs::{FocusGroupId, TextInputBuilder};

// CountdownText comes from dialogs module, not components
pub use dialogs::CountdownText;

// Essential preset functions
pub use dialogs::presets as dialog_presets;
pub use tips::get_random_tip;

// Style constants and helpers (essential utilities)
pub use styles::{colors, dimensions, helpers, layers};

// Convenience functions from individual modules
pub use sliders::slider;
pub use text_inputs::text_input;

// Generic cleanup system
pub use cleanup::{despawn_entities, despawn_ui_entities};

// Main plugin (implementation in plugin.rs)
pub use plugin::UIPlugin;
