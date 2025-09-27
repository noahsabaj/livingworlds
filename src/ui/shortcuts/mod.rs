//! Keyboard Shortcuts Registry System
//!
//! A centralized system for managing all keyboard shortcuts in Living Worlds,
//! eliminating the need for separate input handlers scattered throughout the codebase.
//!
//! # Features
//! - Centralized shortcut registration
//! - Context-aware shortcuts (different in menus vs gameplay)
//! - Conflict detection
//! - Customizable keybindings
//! - Help text generation
//!
//! # Usage
//! ```rust
//! // Register a shortcut
//! shortcuts.register(
//!     ShortcutId::SaveGame,
//!     KeyBinding::single(KeyCode::KeyS).with_ctrl(),
//!     "Save the game"
//! );
//!
//! // Check if triggered
//! if shortcuts.just_pressed(ShortcutId::SaveGame) {
//!     // Handle save
//! }
//! ```

// GATEWAY ARCHITECTURE - Pure exports only

mod types;
mod registry;
mod systems;
mod builder;
mod plugin;

// Core types
pub use types::{
    ShortcutId, KeyBinding, ShortcutContext,
    ShortcutAction, ShortcutEvent, ModifierKeys,
    ShortcutConfig,
};

// Registry
pub use registry::{
    ShortcutRegistry, ShortcutDefinition,
    ShortcutConflict, ShortcutGroup,
};

// Builder API
pub use builder::{
    ShortcutBuilder, shortcuts, ShortcutCommandsExt,
};

// Systems (for advanced users)
pub use systems::{
    process_shortcuts, update_shortcut_hints,
    check_shortcut_conflicts,
};

// Plugin
pub use plugin::ShortcutPlugin;