//! Settings UI Plugin
//!
//! Plugin for managing settings user interface and event handling.

use crate::menus::SpawnSettingsMenuEvent;
use crate::settings::{components::*, types::*};
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Plugin for settings UI functionality.
define_plugin!(SettingsUIPlugin {
    resources: [
        TempGameSettings,
        SettingsDirtyState,
        ResolutionConfirmation,
        FocusedElement
    ],

    events: [
        super::events::SettingsUIEvent,
        super::events::TabSwitchEvent,
        super::events::PresetAppliedEvent,
        SettingsChanged,
        RequestResolutionConfirm
    ],

    startup: [crate::settings::persistence::load_settings],

    update: [
        // Main spawning system
        handle_spawn_settings_menu_event,
        // High-level handlers
        (
            super::handlers::handle_tab_buttons,
            super::handlers::handle_apply_cancel_buttons,
            super::handlers::handle_preset_buttons,
            super::handlers::handle_reset_button,
            super::handlers::handle_unsaved_changes_dialog,
            super::handlers::update_apply_button_state,
            super::handlers::update_apply_exit_button_hover,
            // Resolution confirmation systems
            crate::settings::resolution::handle_resolution_confirm_request,
            crate::settings::resolution::update_resolution_countdown,
            crate::settings::resolution::handle_resolution_confirm_buttons,
            // Navigation system
            crate::settings::navigation::handle_keyboard_navigation
        ),
        // Generated declarative handlers
        (
            super::content::handle_graphicstabdeclarative_interactions,
            super::content::handle_audiotabdeclarative_interactions,
            super::content::handle_interfacetabdeclarative_interactions,
            super::content::handle_controlstabdeclarative_interactions
        ),
        // Settings application system
        super::handlers::apply_settings_changes
    ]
});

// System to handle the SpawnSettingsMenuEvent by spawning the settings menu
fn handle_spawn_settings_menu_event(
    mut events: EventReader<SpawnSettingsMenuEvent>,
    commands: Commands,
    settings: Res<GameSettings>,
    mut temp_settings: ResMut<TempGameSettings>,
    current_tab: Res<crate::states::CurrentSettingsTab>,
    mut dirty_state: ResMut<SettingsDirtyState>,
) {
    for _ in events.read() {
        // Call the spawn function from the spawning module
        super::spawning::spawn_settings_menu(
            commands,
            &*settings,
            &mut *temp_settings,
            &*current_tab,
            &mut *dirty_state,
        );
        return; // Only spawn once per frame
    }
}
