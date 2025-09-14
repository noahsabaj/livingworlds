//! Save/Load system gateway module
//!
//! This module provides game saving and loading functionality with compression,
//! versioning, and UI components. It follows strict gateway architecture where
//! all internal modules are private and only specific functionality is exposed.
//!
//! # Architecture
//!
//! - **types**: Core data structures for saves
//! - **events**: Event types for save/load operations
//! - **resources**: Resources for managing save state
//! - **plugin**: Main plugin that registers all systems
//! - **io**: File I/O, compression, and serialization
//! - **core**: Core save/load logic
//! - **ui**: User interface components using our builders
//! - **handlers**: Event handling and coordination
//!
//! # Gateway Pattern
//!
//! This is a PURE gateway - no implementations, only module declarations
//! and controlled exports. All functionality is in private submodules.

// PRIVATE MODULES - Implementation details
mod types;
mod events;
mod resources;
mod plugin;
mod io;
mod core;
mod ui;
mod handlers;

// CONTROLLED PUBLIC EXPORTS

// Plugin - the main integration point
pub use plugin::SaveLoadPlugin;

// Types - data structures (selective exports)
pub use types::{
    SaveGameInfo,
    SaveGameData,
    SAVE_DIRECTORY,
    SAVE_EXTENSION,
    SAVE_VERSION,        // Needed by core module
    AUTO_SAVE_INTERVAL,  // Needed by resources module
};

// Events - all events are public for external triggering
pub use events::{
    SaveGameEvent,
    LoadGameEvent,
    SaveCompleteEvent,
    LoadCompleteEvent,
    DeleteSaveEvent,
    OpenSaveDialogEvent,
    CloseSaveDialogEvent,
};

// Resources - selective exports for external access
pub use resources::{
    SaveGameList,
    SaveBrowserState,
    SaveDialogState,
    PendingLoadData,  // Needed for load system
    AutoSaveTimer,    // Needed for plugin
};

// Public utility functions
pub use core::{quick_save, load_latest_save};
pub use io::{scan_save_files, scan_save_files_internal};

// Note: We do NOT export:
// - UI components (internal implementation)
// - Handler systems (internal logic)
// - I/O internals (compression, serialization details)
// - Core save/load internals
// These remain private implementation details!