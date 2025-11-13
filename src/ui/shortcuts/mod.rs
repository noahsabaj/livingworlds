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
//! ```rust,no_run
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
//! ```no_run

// GATEWAY ARCHITECTURE - Pure exports only

mod types;
mod registry;
mod systems;
mod builder;
mod plugin;

// Core types
pub use types::{
    ShortcutId, KeyBinding, ShortcutContext, ShortcutEvent,
    ShortcutConfig,
};

// Registry
pub use registry::{
    ShortcutRegistry, ShortcutDefinition,
};

// Builder API
pub use builder::{
    ShortcutBuilder, shortcuts, ShortcutCommandsExt,
};

// Systems (for advanced users)

// Plugin
pub use plugin::ShortcutPlugin;