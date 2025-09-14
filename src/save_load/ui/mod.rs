//! UI module gateway for save/load system
//!
//! This module contains all UI components for save/load functionality,
//! using our standard UI builders for consistency.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports.

// Re-export what our children need from parent gateway (for internal use only)
pub(self) use super::{
    SaveGameInfo, SaveGameData,
    SaveGameEvent, LoadGameEvent, OpenSaveDialogEvent, CloseSaveDialogEvent,
    SaveGameList, SaveBrowserState, SaveDialogState,
};

// Re-export I/O functions our children might need
pub(self) use super::io::{scan_save_files_internal, format_file_size};

// PRIVATE MODULES - UI implementation
mod components;
mod browser;
mod save_dialog;
mod delete_dialog;

// CONTROLLED EXPORTS - UI system functions for plugin

// Browser systems
pub(super) use browser::{
    spawn_save_browser,
    handle_save_browser_interactions,
    update_save_browser,
    close_save_browser,
};

// Save dialog systems
pub(super) use save_dialog::{
    handle_open_save_dialog,
    handle_close_save_dialog,
    handle_save_dialog_interactions,
};

// Delete dialog systems
pub(super) use delete_dialog::{
    handle_delete_button_click,
    handle_delete_confirmation,
};

// Note: Component markers remain private - they're implementation details