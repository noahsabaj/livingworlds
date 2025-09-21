//! Tab Switching Handler - Revolutionary UI Automation
//!
//! This handler has been transformed using the `define_ui_interactions!` macro,
//! eliminating 20+ lines of repetitive button interaction boilerplate.

use crate::settings::{components::*, settings_ui::spawning::spawn_settings_menu, types::*};
use crate::states::CurrentSettingsTab;
use crate::ui::define_ui_interactions;
use bevy::prelude::*;

define_ui_interactions!(
    handle_tab_buttons(
        TabButton,
        tab,
        mut current_tab: ResMut<CurrentSettingsTab>,
        mut commands: Commands,
        settings_root: Query<Entity, With<SettingsMenuRoot>>,
        settings: Res<GameSettings>,
        mut temp_settings: ResMut<TempGameSettings>,
        mut dirty_state: ResMut<SettingsDirtyState>
    ) => {
        tab => {
            debug!("Switching to tab: {:?}", tab);
            current_tab.0 = tab;

            // Respawn settings menu with new tab
            if let Ok(entity) = settings_root.single() {
                commands.entity(entity).despawn();
            }
            // Respawn the settings menu with the new tab selected
            spawn_settings_menu(
                commands.reborrow(),
                &*settings,
                &mut *temp_settings,
                &*current_tab,
                &mut *dirty_state,
            );
        }
    }
);
