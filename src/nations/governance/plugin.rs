//! Governance plugin for Bevy integration
//!
//! This plugin manages all governance systems including political evolution,
//! government transitions, and legitimacy calculations.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::types::GovernanceSettings;
use super::transitions::{check_for_transitions, process_government_transitions};
use super::legitimacy::update_government_legitimacy;
use super::pressure::update_political_pressure;

define_plugin!(GovernancePlugin {
    resources: [
        GovernanceSettings,
    ],

    update: [
        update_political_pressure.run_if(in_state(crate::states::GameState::InGame)),
        update_government_legitimacy.run_if(in_state(crate::states::GameState::InGame)),
        check_for_transitions.run_if(in_state(crate::states::GameState::InGame)),
        process_government_transitions.run_if(in_state(crate::states::GameState::InGame)),
    ],
});