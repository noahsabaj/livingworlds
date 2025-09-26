//! Nation system plugin
//!
//! This plugin manages the nation and house systems, including generation,
//! rendering, and simulation.

use super::territory_analysis::TerritoryMetricsCache;
use super::types::{NationRegistry, ProvinceOwnershipCache};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Nation system plugin using declarative syntax with territory-aware labels
define_plugin!(NationPlugin {
    plugins: [
        super::governance::GovernancePlugin,
        super::laws::LawPlugin
    ],

    resources: [
        NationRegistry,
        ProvinceOwnershipCache,
        TerritoryMetricsCache
    ],

    reflect: [
        super::types::Nation,
        super::house::House,
        super::house::Ruler,
        super::house::RulerPersonality,
        super::house::HouseTraits,
        super::types::NationId,
        super::types::NationPersonality,
        super::governance::GovernmentType,
        super::governance::Governance,
        super::governance::PoliticalPressure,
        super::governance::GovernmentHistory
    ],

    update: [
        super::rendering::update_nation_colors.run_if(in_state(GameState::InGame)),
        super::rendering::render_nation_borders.run_if(in_state(GameState::InGame)),
        // Nation label systems with territory-spanning text
        super::rendering::spawn_nation_labels.run_if(in_state(GameState::InGame)),
        super::rendering::update_nation_label_sizes.run_if(in_state(GameState::InGame)),
        super::rendering::update_label_visibility.run_if(in_state(GameState::InGame))
    ]
});
