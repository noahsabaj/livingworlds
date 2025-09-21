//! Nation system plugin
//!
//! This plugin manages the nation and house systems, including generation,
//! rendering, and simulation.

use super::rendering::{render_nation_borders, render_nation_labels, update_nation_colors};
use super::types::{NationRegistry, ProvinceOwnershipCache};
use crate::states::GameState;
use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

/// Nation system plugin using declarative syntax
define_plugin!(NationPlugin {
    resources: [NationRegistry, ProvinceOwnershipCache],

    reflect: [
        super::types::Nation,
        super::house::House,
        super::house::Ruler,
        super::house::RulerPersonality,
        super::house::HouseTraits,
        super::types::NationId,
        super::types::NationPersonality
    ],

    update: [
        super::rendering::update_nation_colors.run_if(in_state(GameState::InGame)),
        super::rendering::render_nation_borders.run_if(in_state(GameState::InGame)),
        super::rendering::render_nation_labels.run_if(in_state(GameState::InGame))
    ]
});
