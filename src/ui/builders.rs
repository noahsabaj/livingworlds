//! UI Builder Convenience Functions
//!
//! This private module provides convenience functions for creating UI elements.
//! Since this module is private, only the functions that ui/mod.rs explicitly
//! re-exports are available to external code.
//!
//! The actual builder types are exported directly by ui/mod.rs from their
//! source modules, maintaining clean gateway architecture.

// IMPORTS - From parent module's public exports

use super::{ButtonBuilder, ButtonStyle, DialogBuilder, DialogType, ProgressBarBuilder};

// CONVENIENCE FUNCTIONS - Selectively exported by ui/mod.rs

/// Quick button creation
pub fn button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text)
}

/// Quick primary button
pub fn primary_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Primary)
}

/// Quick danger button
pub fn danger_button(text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder::new(text).style(ButtonStyle::Danger)
}

/// Quick dialog creation
pub fn dialog(dialog_type: DialogType) -> DialogBuilder {
    DialogBuilder::new(dialog_type)
}

/// Quick progress bar creation
pub fn progress_bar(value: f32) -> ProgressBarBuilder {
    ProgressBarBuilder::new(value)
}

// Note: This module is PRIVATE. It exists only to provide convenience functions
// that ui/mod.rs selectively exports. The actual builder types are exported
// directly by ui/mod.rs from their source modules.
// This maintains clean gateway architecture where ui/mod.rs is the exclusive
// entry/exit point for the entire UI module.
