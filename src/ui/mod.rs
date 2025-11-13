//! User Interface Module - Pure Gateway Architecture
//!
//! This is a PURE GATEWAY - no implementation code, only module organization.
//! External modules should handle their own UI creation internally.

use bevy::prelude::ChildSpawnerCommands;

// Back to ChildSpawnerCommands - this was much closer to working
// The type mismatch was small compared to EntityCommands
pub type ChildBuilder<'a> = ChildSpawnerCommands<'a>;

// PRIVATE MODULES - All implementation hidden
mod animation;         // Declarative animation system
mod cleanup;           // Generic cleanup utilities
mod dialogs;           // Game-specific dialogs
mod dropdown;          // Dropdown component system
mod hud;               // Heads-up display
mod interaction;       // UI interaction systems
mod law_browser;       // Law browsing UI
mod loading;           // Loading indicators
mod nation_laws_panel; // Nation laws display
mod nation_info;       // Nation information panel
mod nation_selection;  // Nation selection UI
mod notifications;     // Universal notification system (toasts, banners)
mod overlay_display;   // Map overlay displays
mod performance_dashboard; // Performance monitoring
mod plugin;            // Main UI plugin
mod shortcuts;         // Keyboard shortcuts registry
mod styles;            // Centralized styling
mod tile_info;         // Tile information display
mod tips;              // Game tips system
mod toolbar;           // Main toolbar

// ESSENTIAL EXPORTS - Minimal public API

// Styles module re-exports for controlled access
pub use styles::{colors, dimensions, helpers, layers};

// Convenience aliases from styles module
pub use styles::colors::{
    BACKGROUND_MEDIUM as UI_BACKGROUND_COLOR, BORDER_DEFAULT as UI_BORDER_COLOR,
    TEXT_PRIMARY as TEXT_COLOR_PRIMARY, TEXT_SECONDARY as TEXT_COLOR_SECONDARY,
    TEXT_TITLE as TEXT_COLOR_HEADER,
};
pub use styles::dimensions::{
    FONT_SIZE_LARGE as TEXT_SIZE_LARGE, FONT_SIZE_NORMAL as TEXT_SIZE_NORMAL,
    FONT_SIZE_TITLE as TEXT_SIZE_TITLE,
};

// Animation system exports

// Keyboard shortcuts system exports
pub use shortcuts::{
    // Core types
    ShortcutId, ShortcutContext, ShortcutEvent,
    // Registry
    ShortcutRegistry,
};

// Dropdown system exports

// Marker components for queries
pub use dialogs::{
    DiscardButton, KeepButton, RevertButton, SaveButton,
};

// Nation info markers
pub use nation_info::ViewLawsButton;

// State markers
pub use dialogs::{
    ExitConfirmationDialog, ResolutionConfirmDialog, UnsavedChangesDialog,
    WorldGenerationErrorDialog,
};

// HUD/Display markers
// pub use hud::HudRoot; // TODO: Define HudRoot component if needed
pub use interaction::SelectedProvinceInfo;

// UI interaction automation macros
pub use crate::{define_marker_interactions, define_ui_interactions};
pub use nation_info::SelectedNation;

// Builder components and types - NOW FROM EXTERNAL CRATE!
// Re-export from bevy-ui-builders for compatibility
pub use bevy_ui_builders::{
    // Button system - now with native with_marker support in v0.1.4!
    ButtonBuilder, ButtonSize, ButtonStyle, StyledButton,
    // Dialog system
    DialogBuilder, DialogType,
    // Label system
    LabelBuilder, LabelStyle,
    // Panel system
    PanelBuilder, PanelStyle,
    // Progress bar system
    ProgressBar, ProgressBarBuilder,
    // Separator system
    SeparatorBuilder, Orientation,
    // Slider system
    Slider, SliderBuilder, ValueFormat,
    // Text input system
    TextInputBuilder, FocusGroupId, text_input,
};

// TextBuffer is in a nested module, needs separate import
pub use bevy_ui_builders::text_input::native_input::TextBuffer;

// Keep local-only components for now
pub use dialogs::DialogOverlay;


// CountdownText comes from dialogs module, not components
pub use dialogs::CountdownText;

// Essential preset functions
pub use dialogs::presets as dialog_presets;
pub use tips::get_random_tip;

// Convenience functions now come directly from bevy-ui-builders v0.1.4

// Generic cleanup system

// Wrapper for despawn_ui_entities that matches our system signature
use bevy::prelude::*;
pub fn despawn_ui_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Law browser exports

// Nation laws panel exports

// Notification system exports
pub use notifications::ShowNotification;

// Main plugin (implementation in plugin.rs)
pub use plugin::UIPlugin;

// Text input components - bevy-ui-builders now handles text input natively
use bevy::prelude::Entity;

#[derive(Message, Debug, Clone)]
pub struct TextInputSubmitEvent {
    pub entity: Entity,
    pub value: String,
}
