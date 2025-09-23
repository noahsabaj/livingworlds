//! Tile info plugin implementation - RESOURCE CHANGE AUTOMATION!
//!
//! This module demonstrates PERFECT resource_changed pattern automation!
//! 27 lines of manual registration → 15 lines declarative beauty!

use crate::states::GameState;
use crate::ui::despawn_ui_entities;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::{panel, setup, TileInfoRoot};

// Plugin that manages tile information display using AUTOMATION FRAMEWORK!
///
// **AUTOMATION ACHIEVEMENT**: 27 lines manual → 15 lines declarative!
define_plugin!(TileInfoPlugin {
    update: [
        panel::update_tile_info_ui
            .run_if(resource_changed::<crate::resources::SelectedProvinceInfo>)
            .run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [setup::setup_tile_info]
    },

    on_exit: {
        GameState::InGame => [despawn_ui_entities::<TileInfoRoot>]
    }
});
