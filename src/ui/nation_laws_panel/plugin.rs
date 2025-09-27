//! Nation laws panel plugin
//!
//! Manages the display of nation-specific laws and proposals.

use super::handlers::*;
use super::types::NationLawsPanelState;
use super::updates::{update_active_laws_list, update_proposed_laws_list};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

define_plugin!(NationLawsPanelPlugin {
    resources: [
        NationLawsPanelState
    ],

    update: [
        handle_view_laws_button.run_if(in_state(GameState::InGame)),
        handle_close_button.run_if(in_state(GameState::InGame)),
        update_active_laws_list.run_if(in_state(GameState::InGame)),
        update_proposed_laws_list.run_if(in_state(GameState::InGame)),
        handle_repeal_buttons.run_if(in_state(GameState::InGame)),
        handle_support_oppose_buttons.run_if(in_state(GameState::InGame))
    ]
});