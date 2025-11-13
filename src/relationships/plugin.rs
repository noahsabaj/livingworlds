//! Relationships Plugin

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import all relationship modules
use super::{
    administrative::*, cultural::*, debug::{debug_relationships, DebugRelationships}, diplomatic::*,
    infrastructure::*, legislative::*, military::*, political::*, population::*, religious::*,
};

define_plugin!(RelationshipsPlugin {
    messages: [
        // Cultural events (2)
        CulturalTensionEvent,
        CulturalUnificationEvent,
        // Administrative events (3)
        GovernorAppointedEvent,
        GovernorDismissedEvent,
        CorruptionDetectedEvent,
        // Diplomatic events (4)
        AllianceFormedEvent,
        WarDeclaredEvent,
        PeaceTreatyEvent,
        TradeAgreementEvent,
        // Infrastructure events (3)
        RoadConstructedEvent,
        TradeRouteEstablishedEvent,
        InfrastructureMaintenanceEvent,
        // Military events (3)
        ArmyMovedEvent,
        FortificationBuiltEvent,
        BattleEvent,
        // Religious events (3)
        ReligiousConversionEvent,
        ReligiousConflictEvent,
        ReligionFoundedEvent,
        // Population events (3)
        MigrationEvent,
        PopulationChangeEvent,
        DemographicShiftEvent,
        // Legislative events (3)
        LawEnactedEvent,
        LawProposedEvent,
        LawRepealedEvent
    ],

    update: [
        // Primary relationship systems (chained for order)
        (
            update_cultural_coherence,
            update_administrative_efficiency,
            update_diplomatic_state,
            update_infrastructure_status,
            update_military_status,
            update_religious_status,
            simulate_religious_spread,
            update_provincial_demographics
        )
            .chain()
    ],

    fixed_update: [
        // Validation systems for relationship integrity
        (
            validate_province_ownership,
            validate_capital_assignments,
            validate_cultural_regions,
            validate_exclusive_cultural_membership,
            validate_administrative_assignments,
            validate_diplomatic_consistency,
            validate_infrastructure_connections,
            validate_military_positions,
            validate_army_ownership,
            validate_religious_relationships,
            validate_religious_influence_consistency,
            validate_population_residence,
            validate_demographic_consistency
        )
    ],

    custom_init: |app: &mut App| {
        // Debug systems (conditional compilation)
        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            (debug_relationships.run_if(resource_exists::<DebugRelationships>),),
        );
    }
});