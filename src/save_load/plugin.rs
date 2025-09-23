//! Save/Load plugin implementation - AUTOMATED WITH DECLARATIVE MAGIC!
//!
//! This module demonstrates the POWER of the Plugin Registration Automation Framework!
//! 79 lines of manual boilerplate → 35 lines of declarative paradise!

use super::{
    AutoSaveTimer, CloseSaveDialogEvent, DeleteSaveEvent, LoadCompleteEvent, LoadGameEvent,
    OpenSaveDialogEvent, SaveBrowserState, SaveCompleteEvent, SaveDialogState, SaveGameEvent,
    SaveGameList,
};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Save/Load plugin using REVOLUTIONARY declarative automation
define_plugin!(SaveLoadPlugin {
    resources: [SaveGameList, SaveBrowserState, SaveDialogState, AutoSaveTimer],

    events: [
        SaveGameEvent,
        LoadGameEvent,
        SaveCompleteEvent,
        LoadCompleteEvent,
        DeleteSaveEvent,
        OpenSaveDialogEvent,
        CloseSaveDialogEvent
    ],

    reflect: [
        crate::world::Province,
        crate::components::MineralType,
        crate::world::TerrainType,
        crate::resources::WorldSeed,
        crate::resources::WorldSize,
        crate::resources::MapDimensions,
        crate::resources::GameTime,
        crate::resources::WorldTension,
        crate::resources::MapMode,
        crate::world::ProvinceStorage
    ],

    startup: [super::io::ensure_save_directory],

    update: [
        super::core::handle_save_game,
        super::core::handle_load_game,
        super::core::handle_auto_save.run_if(in_state(GameState::InGame)),
        super::handlers::handle_save_load_shortcuts.run_if(in_state(GameState::InGame)),
        super::handlers::handle_spawn_save_browser_event,
        super::ui::update_save_browser,
        super::ui::handle_save_browser_interactions,
        super::ui::handle_delete_button_click,
        super::ui::handle_delete_confirmation,
        super::ui::handle_open_save_dialog,
        super::ui::handle_close_save_dialog,
        super::ui::handle_save_dialog_interactions
    ],

    on_enter: {
        GameState::LoadingWorld => [super::core::check_for_pending_load]
    },

    on_exit: {
        GameState::MainMenu => [super::ui::close_save_browser],
        GameState::Paused => [super::ui::close_save_browser]
    }
});
