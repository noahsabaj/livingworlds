//! Settings Menu Spawning Handler
//!
//! Handles the event to spawn the settings menu.

use crate::menus::SpawnSettingsMenuEvent;
use crate::settings::types::*;
use crate::settings::ui::spawning::spawn_settings_menu;
use crate::states::{CurrentSettingsTab};
use bevy::prelude::*;

/// Handle requests to spawn the settings menu
pub fn handle_spawn_settings_menu(
    mut events: MessageReader<SpawnSettingsMenuEvent>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    for _event in events.read() {
        info!("Spawning settings menu");
        spawn_settings_menu(
            commands.reborrow(),
            &settings,
            &mut temp_settings,
            &current_tab,
            &mut dirty_state,
        );
    }
}
