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
    SaveGameData, SaveGameInfo, SAVE_VERSION, SAVE_DIRECTORY, SAVE_EXTENSION,
    SaveGameEvent, LoadGameEvent, SaveCompleteEvent, LoadCompleteEvent,
    SaveGameList, PendingLoadData, AutoSaveTimer,
};

// Re-export I/O functions our children need (for internal use only)
pub(self) use super::io::{
    compress_data, decompress_data,
    serialize_save_data, deserialize_save_data,
    extract_save_metadata, format_file_size,
    scan_save_files_internal,
};

// PRIVATE MODULES - Core logic implementation
mod save;
mod load;
mod auto_save;

// CONTROLLED EXPORTS - Core functionality

// System functions (used by plugin)
pub(super) use save::handle_save_game;
pub(super) use load::{handle_load_game, check_for_pending_load};
pub(super) use auto_save::handle_auto_save;

// Public utility functions
pub use save::quick_save;
pub use load::load_latest_save;