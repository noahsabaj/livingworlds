//! UI interaction module gateway
//!
//! This module contains types and logic for user interaction with the UI,
//! particularly province selection and other interactive elements.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports.

// PRIVATE MODULES - Implementation details
mod handlers;
mod selection;

// CONTROLLED EXPORTS - Public API for UI interaction

// Selection-related types
pub use selection::SelectedProvinceInfo;

// UI Interaction automation system - eliminates 400+ lines of button boilerplate
pub use handlers::{
    handle_selection_interaction, ButtonValue, FieldUpdater, SelectionConfig, SelectionStyling,
};

// Re-export the UI interaction automation macros
