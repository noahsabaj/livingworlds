//! Nation system plugin
//!
//! This plugin manages the nation and house systems, including generation,
//! rendering, and simulation.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use crate::states::GameState;
use super::rendering::{update_nation_colors, render_nation_borders, render_nation_labels};
use super::types::{NationRegistry, ProvinceOwnershipCache};

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
        (
            update_nation_colors,
            render_nation_borders,
            render_nation_labels
        ).run_if(in_state(GameState::InGame))
    ]
});