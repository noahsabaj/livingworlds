//! Event handlers for settings UI

use crate::menus::SpawnSettingsMenuEvent;
use crate::settings::{components::*, types::*};
use crate::settings::settings_ui::spawning::spawn_settings_menu;
use bevy::prelude::*;

/// System to handle the SpawnSettingsMenuEvent by spawning the settings menu
pub fn handle_spawn_settings_menu_event(
    mut events: EventReader<SpawnSettingsMenuEvent>,
    commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<crate::states::CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    for _ in events.read() {
        // Call the spawn function from the spawning module
        spawn_settings_menu(
            commands,
            &*settings,
            &mut *temp_settings,
            &*current_tab,
            &mut *dirty_state,
        );
        return; // Only spawn once per frame
    }
}