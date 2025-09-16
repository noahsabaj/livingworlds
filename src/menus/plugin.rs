//! Menu system integration with Bevy
//!
//! This module contains the MenusPlugin that integrates all menu subsystems
//! with the Bevy engine. It handles event registration, plugin coordination,
//! and system scheduling for the entire menu system.

use bevy::prelude::*;

// Import from sibling modules through super (gateway pattern)
use super::{main_menu, pause_menu, types::*};

/// Plugin that aggregates all menu subsystems
///
/// This plugin doesn't implement any systems directly - it delegates to
/// specialized plugins in each menu submodule following the gateway pattern.
/// It also registers shared events that multiple menu systems use.
pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register shared events used across menu systems
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