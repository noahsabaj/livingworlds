//! Save/Load plugin implementation
//!
//! This module contains the Bevy plugin that registers all save/load systems.

use bevy::prelude::*;
use crate::states::GameState;
use super::{
    // Resources (imported through parent gateway)
    SaveGameList, SaveBrowserState, SaveDialogState, AutoSaveTimer,
    // Events (imported through parent gateway)
    SaveGameEvent, LoadGameEvent, SaveCompleteEvent, LoadCompleteEvent,
    DeleteSaveEvent, OpenSaveDialogEvent, CloseSaveDialogEvent,
};

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<SaveGameList>()
            .init_resource::<SaveBrowserState>()
            .init_resource::<SaveDialogState>()
            .init_resource::<AutoSaveTimer>()

            // Events
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<SaveCompleteEvent>()
            .add_event::<LoadCompleteEvent>()
            .add_event::<DeleteSaveEvent>()
            .add_event::<OpenSaveDialogEvent>()
            .add_event::<CloseSaveDialogEvent>()

            // Systems
            .add_systems(Update, (
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
                super::ui::handle_save_dialog_interactions,
            ))
            .add_systems(OnEnter(GameState::LoadingWorld), super::core::check_for_pending_load)
            .add_systems(OnExit(GameState::MainMenu), super::ui::close_save_browser)
            .add_systems(OnExit(GameState::Paused), super::ui::close_save_browser)
            .add_systems(Startup, super::io::ensure_save_directory);

        // Register types for reflection
        app.register_type::<crate::world::Province>()
            .register_type::<crate::components::MineralType>()
            .register_type::<crate::world::TerrainType>()
            .register_type::<crate::resources::WorldSeed>()
            .register_type::<crate::resources::WorldSize>()
            .register_type::<crate::resources::MapDimensions>()
            .register_type::<crate::resources::GameTime>()
            .register_type::<crate::resources::WorldTension>()
            .register_type::<crate::resources::ResourceOverlay>()
            .register_type::<crate::world::ProvinceStorage>();
    }
}