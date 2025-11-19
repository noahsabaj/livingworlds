//! Pressure-triggered war declaration system
//!
//! This module connects the pressure system to war declarations,
//! making AI nations declare wars when military pressure is critical.

use bevy::prelude::*;
use crate::simulation::{PressureVector, PressureType};
use crate::nations::{Nation, NationHistory, Governance};
// NationNeighborCache deleted - now using LandNeighbors/NavalNeighbors relationship components
use crate::nations::warfare::{DeclareWarEvent, WarGoal, CasusBelli};
use super::casus_belli::CasusBelliExt;

/// System to check if high military pressure should trigger war
pub fn evaluate_war_triggers_from_pressure(
    nations_query: Query<(Entity, &Nation, &PressureVector, &NationHistory, &Governance)>,
    neighbor_cache: Res<NationNeighborCache>,
    mut war_events: MessageWriter<DeclareWarEvent>,
) {
    for (entity, nation, pressures, history, _governance) in &nations_query {
        // Check if military pressure is critical
        let Some(&mil_pressure) = pressures.pressures.get(&PressureType::MilitaryVulnerability) else {
            continue;
        };

        if !mil_pressure.is_critical() {
            continue;
        }

        // Determine if nation should declare war or seek alliance
        // Aggressive nations declare war, diplomatic nations seek allies
        let is_aggressive = nation.personality.aggression > 0.6;
        let can_afford = nation.treasury > 10000.0;
        let has_recent_defeats = history.calculate_weighted_recent_defeats() > 1.0;

        if is_aggressive && can_afford && !has_recent_defeats {
            // Look for weak neighbor to attack
            if let Some(target) = find_weakest_neighbor(nation, &neighbor_cache, &nations_query) {
                // Determine war goal and CB
                let war_goal = WarGoal::Conquest {
                    target_provinces: vec![], // TODO: Select specific provinces
                };
                let casus_belli = if neighbor_cache
                    .get_land_neighbors(nation.id)
                    .map(|n| n.contains(&target.1.id))
                    .unwrap_or(false)
                {
                    CasusBelli::BorderDispute
                } else {
                    CasusBelli::FabricatedClaim
                };

                war_events.write(DeclareWarEvent {
                    attacker: entity,
                    defender: target.0,
                    war_goal,
                    casus_belli,
                });

                info!(
                    "{} declares war on {} due to critical military pressure",
                    nation.name, target.1.name
                );
            }
        }
    }
}

/// Find weakest neighboring nation
fn find_weakest_neighbor(
    nation: &Nation,
    neighbor_cache: &NationNeighborCache,
    nations_query: &Query<(Entity, &Nation, &PressureVector, &NationHistory, &Governance)>,
) -> Option<(Entity, Nation)> {
    let Some(neighbor_ids) = neighbor_cache.get_neighbors(nation.id) else {
        return None;
    };

    nations_query
        .iter()
        .filter(|(_, n, _, _, _)| neighbor_ids.contains(&n.id))
        .min_by(|(_, a, _, _, _), (_, b, _, _, _)| {
            a.military_strength
                .partial_cmp(&b.military_strength)
                .unwrap()
        })
        .map(|(e, n, _, _, _)| (e, n.clone()))
}
