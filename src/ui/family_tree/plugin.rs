//! Family tree viewer plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use crate::states::GameState;
use super::types::*;
use super::systems::*;
use super::ui::*;

define_plugin!(FamilyTreePlugin {
    resources: [
        FamilyTreeLayout,
        BloodlineHighlight
    ],

    on_enter: {
        GameState::InGame => [
            spawn_family_tree_panel,
            insert_tree_filters
        ]
    },

    update: [
        (
            handle_open_tree,
            handle_close_tree,
            handle_close_button,
            handle_node_click,
            apply_relationship_filters,
            update_tree_visualization,
        ).run_if(in_state(GameState::InGame))
    ]
});
