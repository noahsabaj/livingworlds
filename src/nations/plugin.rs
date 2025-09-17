//! Nation system plugin
//!
//! This plugin manages the nation and house systems, including generation,
//! rendering, and simulation.

use bevy::prelude::*;

use crate::states::GameState;
use super::rendering::{update_nation_colors, render_nation_borders, render_nation_labels};
use super::types::{NationRegistry, ProvinceOwnershipCache};

pub struct NationPlugin;

impl Plugin for NationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<NationRegistry>()
            .init_resource::<ProvinceOwnershipCache>()

            // Register types for reflection
            .register_type::<super::types::Nation>()
            .register_type::<super::house::House>()
            .register_type::<super::house::Ruler>()
            .register_type::<super::house::RulerPersonality>()
            .register_type::<super::house::HouseTraits>()
            .register_type::<super::types::NationId>()
            .register_type::<super::types::NationPersonality>()

            // Systems for rendering
            .add_systems(
                Update,
                (
                    update_nation_colors,
                    render_nation_borders,
                    render_nation_labels,
                )
                .run_if(in_state(GameState::InGame)),
            );
    }
}