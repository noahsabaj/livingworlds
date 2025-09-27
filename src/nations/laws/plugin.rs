//! Law system Bevy plugin
//!
//! Integrates the law system with the Bevy ECS by orchestrating
//! initialization and update systems.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::initialization::initialize_law_registry;
use super::registry::{LawRegistry, NationLaws, ActiveLaws, LawHistory};
use super::systems::{
    propose_laws_system, update_law_debates_system, process_law_votes_system,
    apply_law_effects_system, apply_law_effects_to_nations,
    handle_government_transitions_system, update_law_cooldowns_system,
};

#[cfg(debug_assertions)]
use super::systems::{validate_law_data_system, periodic_recalculation_system};
use super::types::{LawEnactmentEvent, LawRepealEvent};
use crate::states::GameState;

define_plugin!(LawPlugin {
    plugins: [
        // Include the data-driven law loader
        super::loader::LawLoaderPlugin
    ],

    resources: [
        LawRegistry,
        ActiveLaws,
        LawHistory
    ],

    events: [
        LawEnactmentEvent,
        LawRepealEvent
    ],

    startup: [
        initialize_law_registry
    ],

    update: [
        propose_laws_system.run_if(in_state(GameState::InGame)),
        update_law_debates_system.run_if(in_state(GameState::InGame)),
        process_law_votes_system.run_if(in_state(GameState::InGame)),
        apply_law_effects_system.run_if(in_state(GameState::InGame)),
        apply_law_effects_to_nations.run_if(in_state(GameState::InGame)),
        handle_government_transitions_system.run_if(in_state(GameState::InGame)),
        update_law_cooldowns_system.run_if(in_state(GameState::InGame))
    ],

    custom_init: |app: &mut bevy::app::App| {
        // Register law components for reflection
        // NOTE: NationLaws needs Reflect derive to be registered
        // app.register_type::<NationLaws>();

        // Add debug-only validation systems
        #[cfg(debug_assertions)]
        {
            app.add_systems(
                Update,
                (
                    validate_law_data_system,
                    periodic_recalculation_system,
                )
                .run_if(in_state(GameState::InGame))
            );
        }
    }
});