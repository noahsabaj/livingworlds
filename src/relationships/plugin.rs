//! Relationships Plugin - REVOLUTIONARY AUTOMATION ACHIEVEMENT!
//!
//! This plugin demonstrates the ULTIMATE automation victory - 173 lines → ~50 lines!
//! 21 events + complex system scheduling reduced to pure declarative paradise!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import all relationship modules
use super::{
    administrative::*, cultural::*, diplomatic::*, infrastructure::*, military::*, political::*,
    population::*, religious::*,
};

/// Plugin that registers all relationship systems - AUTOMATED WITH DECLARATIVE MAGIC!
///
/// This plugin replaces manual relationship tracking with automatic
/// bidirectional entity relationships throughout Living Worlds.
///
/// **AUTOMATION ACHIEVEMENT**: 173 lines of manual registration → ~50 lines declarative!
define_plugin!(RelationshipsPlugin {
    events: [
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
        DemographicShiftEvent
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

/// Resource that enables debug relationship printing
/// Add this resource to enable relationship debugging
#[derive(Resource, Default)]
pub struct DebugRelationships {
    pub enabled: bool,
    pub print_interval_seconds: f32,
}

/// System for debugging relationship states
/// Only runs when DebugRelationships resource exists
fn debug_relationships(
    time: Res<Time>,
    mut debug_res: ResMut<DebugRelationships>,
    nations_query: Query<Entity, With<crate::nations::Nation>>,
    province_storage: Option<Res<crate::world::ProvinceStorage>>,
    political_query: Query<&Controls>,
) {
    // Simple debug print every N seconds
    debug_res.print_interval_seconds -= time.delta_secs();

    if debug_res.print_interval_seconds <= 0.0 && debug_res.enabled {
        debug_res.print_interval_seconds = 5.0; // Reset to 5 second interval

        let nation_count = nations_query.iter().count();
        let province_count = province_storage
            .as_ref()
            .map_or(0, |storage| storage.provinces.len());
        let controlled_provinces = political_query
            .iter()
            .map(|controls| controls.province_count())
            .sum::<usize>();

        info!(
            "Relationship Debug: {} nations, {} provinces, {} controlled territories",
            nation_count, province_count, controlled_provinces
        );
    }
}
