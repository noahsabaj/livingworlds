//! Nation system plugin
//!
//! This plugin manages the nation and house systems, including generation,
//! rendering, and simulation.

use super::types::NationRegistry;
// TerritoryMetricsCache and ProvinceOwnershipCache deleted - now using components and ECS queries
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
        NationRegistry
        // NOTE: ProvinceOwnershipCache, TerritoryMetricsCache, and NationNeighborCache
        // have been deleted and replaced with ECS queries and relationship components
    ],

    messages: [
        super::actions::NationActionEvent,
        super::actions::TerritoryOwnershipChanged,
        super::warfare::DeclareWarEvent,
        super::warfare::BattleEvent,
        super::warfare::WarEndEvent
    ],

    reflect: [
        super::types::Nation,
        super::house::House,
        super::house::Ruler,
        super::house::RulerPersonality,
        super::house::HouseTraits,
        super::types::NationPersonality,
        super::governance::GovernmentType,
        super::governance::Governance,
        super::governance::PoliticalPressure,
        super::governance::GovernmentHistory,
        super::warfare::War,
        super::warfare::CasusBelli,
        super::warfare::WarGoal,
        // Relationship components (Bevy 0.17)
        super::relationships::LandNeighborOf,
        super::relationships::LandNeighbors,
        super::relationships::NavalNeighborOf,
        super::relationships::NavalNeighbors,
        super::relationships::ParticipatesInWar,
        super::relationships::WarParticipants,
        super::relationships::Attacking,
        super::relationships::AttackedBy
    ],

    update: [
        // ACTION EXECUTION - This is where nations actually DO things!
        // Uses reactive cache invalidation - no more polling every frame!
        super::actions::execute_expansion_events.run_if(in_state(GameState::InGame)),

        // NEIGHBOR RELATIONSHIPS - Event-driven rebuild when territory ownership changes
        super::neighbors::rebuild_neighbor_relationships_on_ownership_change.run_if(in_state(GameState::InGame)),

        // WAR SYSTEMS - War declaration, battles, and resolution
        super::warfare::process_war_declarations.run_if(in_state(GameState::InGame)),
        super::warfare::process_battle_events.run_if(in_state(GameState::InGame)),
        super::warfare::check_war_resolution.run_if(in_state(GameState::InGame)),

        // DIPLOMACY - Pressure-triggered war declarations
        super::diplomacy::evaluate_war_triggers_from_pressure.run_if(in_state(GameState::InGame)),

        // Rendering systems
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
