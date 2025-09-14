//! Event handlers module gateway
//!
//! This module contains event handling and coordination for save/load operations.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports.

// Re-export what our children need from parent gateway (for internal use only)
pub(self) use super::{
    SaveGameEvent, LoadGameEvent,
    SaveGameList, SaveBrowserState,
};

// Re-export I/O and UI functions our children might need
pub(self) use super::io::scan_save_files_internal;
pub(self) use super::ui::spawn_save_browser;

// PRIVATE MODULES - Handler implementation
mod shortcuts;
mod coordination;

// CONTROLLED EXPORTS - Handler functions for plugin

pub(super) use shortcuts::handle_save_load_shortcuts;
pub(super) use coordination::handle_spawn_save_browser_event;