//! Debug utilities for the relationships system

use super::political::Controls;
use bevy::prelude::*;

/// Resource that enables debug relationship printing
/// Add this resource to enable relationship debugging
#[derive(Resource, Default)]
pub struct DebugRelationships {
    pub enabled: bool,
    pub print_interval_seconds: f32,
}

/// System for debugging relationship states
/// Only runs when DebugRelationships resource exists
pub fn debug_relationships(
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