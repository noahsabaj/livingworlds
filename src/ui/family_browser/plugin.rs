//! Family browser plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use crate::states::GameState;
use super::types::*;
use super::systems::*;
use super::ui::*;
use super::toggle::*;

define_plugin!(FamilyBrowserPlugin {
    messages: [
        OpenFamilyTreeEvent,
        CloseFamilyTreeEvent
    ],

    resources: [
        FamilyBrowserFilters,
        HousePrestigeCache,
        SelectedHouseTree
    ],

    on_enter: {
        GameState::InGame => [spawn_family_browser_panel]
    },

    update: [
        (
            update_prestige_cache,
            update_house_list,
            handle_view_tree_button,
            handle_toggle_button,
            handle_keyboard_toggle,
        ).run_if(in_state(GameState::InGame))
    ]
});
