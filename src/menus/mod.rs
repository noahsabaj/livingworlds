//! Menu system gateway module
//!
//! This module provides all menu functionality for Living Worlds through
//! a clean gateway architecture. Each menu type is self-contained in its
//! own submodule with its own plugin, while shared types and events are
//! centralized for consistency.
//!
//! # Architecture
//!
//! The menu system follows the gateway pattern where this mod.rs file acts
//! as the sole entry point, controlling access to menu implementations:
//!
//! - **types**: Shared components and enums used across all menus
//! - **main_menu**: Title screen implementation
//! - **pause_menu**: In-game pause overlay
//! - **plugin**: Bevy integration and system coordination
//!
//! Each menu submodule contains its own Plugin that registers its systems,
//! and the MenusPlugin aggregates them all.

// PRIVATE MODULES - Menu implementation details

mod main_menu;
mod pause_menu;
mod plugin;
mod types;

// SELECTIVE PUBLIC EXPORTS - Controlled menu API

// Export main integration point
pub use plugin::MenusPlugin;

// Export shared types for external use
pub use types::{
    ButtonText, MenuAction, MenuButton, SpawnSaveBrowserEvent, SpawnSettingsMenuEvent,
};

// Export menu root markers (needed for queries in other systems)
pub use main_menu::MainMenuRoot;
pub use pause_menu::PauseMenuRoot;

// PURE GATEWAY - No Implementation Logic

// Note: All actual implementations are in their respective files:
// - Main menu logic is in main_menu.rs
// - Pause menu logic is in pause_menu.rs
// - Plugin integration is in plugin.rs
// - Shared types are in types.rs
//
// This gateway file contains ZERO implementation logic - only controlled exports.