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
mod core;
mod events;
mod handlers;
mod io;
mod plugin;
mod resources;
mod types;
mod ui;

// CONTROLLED PUBLIC EXPORTS

// Plugin - the main integration point
pub use plugin::SaveLoadPlugin;

// Types - data structures (selective exports)
pub use types::{
    SaveGameData,
    SaveGameInfo,
    AUTO_SAVE_INTERVAL, // Needed by resources module
    SAVE_DIRECTORY,
    SAVE_EXTENSION,
    SAVE_VERSION, // Needed by core module
};

// Events - all events are public for external triggering
pub use events::{
    CloseSaveDialogEvent, DeleteSaveEvent, LoadCompleteEvent, LoadGameEvent, OpenSaveDialogEvent,
    SaveCompleteEvent, SaveGameEvent,
};

// Resources - selective exports for external access
pub use resources::{
    AutoSaveTimer,   // Needed for plugin
    PendingLoadData, // Needed for load system
    SaveBrowserState,
    SaveDialogState,
    SaveGameList,
};

// Public utility functions
pub use io::scan_save_files_internal;

// Note: We do NOT export:
// - UI components (internal implementation)
// - Handler systems (internal logic)
// - I/O internals (compression, serialization details)
// - Core save/load internals
// These remain private implementation details!
