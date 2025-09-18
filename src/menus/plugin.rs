//! Menu system integration - PERFECT AGGREGATION AUTOMATION!
//!
//! This module demonstrates CLEAN menu plugin aggregation automation!
//! 34 lines of manual event + plugin registration → 20 lines declarative!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import from sibling modules through super (gateway pattern)
use super::{main_menu, pause_menu, types::*};

/// Plugin that aggregates all menu subsystems using AUTOMATION FRAMEWORK!
///
/// **AUTOMATION ACHIEVEMENT**: 34 lines manual → 20 lines declarative!
define_plugin!(MenusPlugin {
    events: [
        SpawnSettingsMenuEvent,
        SpawnSaveBrowserEvent
    ],

    plugins: [
        main_menu::MainMenuPlugin,      // Title screen menu
        pause_menu::PauseMenuPlugin     // In-game pause overlay
    ]
});