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
//!
//! Each menu submodule contains its own Plugin that registers its systems,
//! and the MenusPlugin aggregates them all.

use bevy::prelude::*;

// PRIVATE MODULES - Menu implementation details

mod main_menu;
mod pause_menu;
mod types;

// SELECTIVE PUBLIC EXPORTS - Controlled menu API

// Export shared types for external use
pub use types::{
    ButtonText, MenuAction, MenuButton, SpawnSaveBrowserEvent, SpawnSettingsMenuEvent,
};

// Export menu root markers (needed for queries in other systems)
pub use main_menu::MainMenuRoot;
pub use pause_menu::PauseMenuRoot;

// MENU PLUGIN - Aggregates all menu subsystems

/// Plugin that aggregates all menu subsystems
///
/// This plugin doesn't implement any systems directly - it delegates to
/// specialized plugins in each menu submodule following the gateway pattern.
pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register shared events
            .add_event::<SpawnSettingsMenuEvent>()
            .add_event::<SpawnSaveBrowserEvent>()
            // Add specialized menu plugins
            // Each plugin manages its own systems and resources
            .add_plugins(main_menu::MainMenuPlugin) // Title screen menu
            .add_plugins(pause_menu::PauseMenuPlugin); // In-game pause overlay

        // Note: Each submodule plugin registers its own systems:
        // - MainMenuPlugin handles title screen and its interactions
        // - PauseMenuPlugin handles pause overlay and save/load from pause
    }
}

// INTERNAL COORDINATION - Plugin wiring only

// Note: All actual menu implementations are in their respective files:
// - Main menu logic is in main_menu.rs
// - Pause menu logic is in pause_menu.rs
// - Shared types are in types.rs
// This gateway file should NEVER contain implementation logic.
