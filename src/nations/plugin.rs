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
        super::rendering::render_nation_borders.run_if(in_state(GameState::InGame)),
        // Label updates (size/visibility) run every frame in Political mode
        (super::rendering::update_nation_label_sizes,
         super::rendering::update_label_visibility)
            .run_if(in_state(GameState::InGame))
            .run_if(resource_equals(crate::world::MapMode::Political)),
        // Spawn labels when MapMode changes TO Political
        super::rendering::spawn_nation_labels_on_mode_enter
            .run_if(in_state(GameState::InGame))
            .run_if(resource_changed::<crate::world::MapMode>),
        // Cleanup labels when MapMode changes AWAY FROM Political
        super::rendering::cleanup_labels_on_mode_exit
            .run_if(in_state(GameState::InGame))
            .run_if(resource_changed::<crate::world::MapMode>)
    ]
});
