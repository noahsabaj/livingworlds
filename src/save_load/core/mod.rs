//! Core save/load logic module gateway
//!
//! This module contains the core business logic for saving and loading games,
//! separated from UI and I/O concerns.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports.

// Re-export what our children need from parent gateway (for internal use only)
pub(self) use super::{
    AutoSaveTimer, LoadCompleteEvent, LoadGameEvent, PendingLoadData, SaveCompleteEvent,
    SaveGameData, SaveGameEvent, SaveGameInfo, SaveGameList, SAVE_DIRECTORY, SAVE_EXTENSION,
    SAVE_VERSION,
};

// Re-export I/O functions our children need (for internal use only)
pub(self) use super::io::{
    compress_data, decompress_data, deserialize_save_data, extract_save_metadata, format_file_size,
    scan_save_files_internal, serialize_save_data,
};

// PRIVATE MODULES - Core logic implementation
mod auto_save;
mod load;
mod save;

// CONTROLLED EXPORTS - Core functionality

// System functions (used by plugin)
pub(super) use auto_save::handle_auto_save;
pub(super) use load::{check_for_pending_load, handle_load_game};
pub(super) use save::handle_save_game;

// Public utility functions
pub use load::load_latest_save;
pub use save::quick_save;
