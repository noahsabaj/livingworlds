//! File I/O module gateway
//!
//! This module handles all file operations, compression, and serialization
//! for the save/load system. It provides a clean abstraction over the
//! underlying file system and data formats.
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports. Internal modules handle their own logic.

// Re-export what our children need from parent gateway (for internal use only)
pub(self) use super::{
    SaveGameData, SaveGameInfo, SAVE_DIRECTORY, SAVE_EXTENSION,
    SaveGameList,
};

// PRIVATE MODULES - I/O implementation
mod compression;
mod serialization;
mod metadata;
mod scanner;

// CONTROLLED EXPORTS - Only what's needed externally

// Directory management
pub use scanner::{ensure_save_directory, scan_save_files, scan_save_files_internal};

// File operations (used by core module)
pub(super) use compression::{compress_data, decompress_data};
pub(super) use serialization::{serialize_save_data, deserialize_save_data};
pub(super) use metadata::extract_save_metadata;

// Utility functions
pub use scanner::format_file_size;