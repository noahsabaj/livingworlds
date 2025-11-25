//! Pressure-triggered war declaration system
//!
//! This module connects the pressure system to war declarations,
//! making AI nations declare wars when military pressure is critical.

use bevy::prelude::*;
use crate::simulation::{PressureVector, PressureType};
use crate::nations::{Nation, NationHistory, Governance};
use crate::nations::warfare::{DeclareWarEvent, WarGoal, CasusBelli};
use super::casus_belli::CasusBelliExt;

/// System to check if high military pressure should trigger war
/// System to check if high military pressure should trigger war
pub fn evaluate_war_triggers_from_pressure(
    nations_query: Query<(
        Entity,
        &crate::nations::NationId,
        &Nation,
        &PressureVector,
        &NationHistory,
        &Governance,
        Option<&crate::nations::relationships::LandNeighbors>,
        Option<&crate::nations::relationships::NavalNeighbors>,
    )>,
    mut war_events: MessageWriter<DeclareWarEvent>,
) {
    for (entity, nation_id, nation, pressures, history, _governance, land_neighbors, naval_neighbors) in &nations_query {
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
            if let Some(target) = find_weakest_neighbor(
                land_neighbors,
                naval_neighbors,
                &nations_query
            ) {
                // Determine war goal and CB
                let war_goal = WarGoal::Conquest {
                    target_provinces: vec![], // TODO: Select specific provinces
                };
                
                // Check if it's a land neighbor for Border Dispute CB
                let is_land_neighbor = land_neighbors
                    .map(|n| n.neighbors().contains(&target.0))
                    .unwrap_or(false);
                    
                let casus_belli = if is_land_neighbor {
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
                    nation.name, target.2.name
                );
            }
        }
    }
}

/// Find weakest neighboring nation
fn find_weakest_neighbor(
    land_neighbors: Option<&crate::nations::relationships::LandNeighbors>,
    naval_neighbors: Option<&crate::nations::relationships::NavalNeighbors>,
    nations_query: &Query<(
        Entity,
        &crate::nations::NationId,
        &Nation,
        &PressureVector,
        &NationHistory,
        &Governance,
        Option<&crate::nations::relationships::LandNeighbors>,
        Option<&crate::nations::relationships::NavalNeighbors>,
    )>,
) -> Option<(Entity, crate::nations::NationId, Nation)> {
    let mut best_target: Option<(Entity, crate::nations::NationId, Nation)> = None;
    let mut min_strength = f32::MAX;

    // Helper to process a list of neighbor entities
    let mut process_neighbors = |entities: &[Entity]| {
        for &neighbor_entity in entities {
            if let Ok((_, neighbor_id, neighbor_nation, _, _, _, _, _)) = nations_query.get(neighbor_entity) {
                if neighbor_nation.military_strength < min_strength {
                    min_strength = neighbor_nation.military_strength;
                    best_target = Some((neighbor_entity, *neighbor_id, neighbor_nation.clone()));
                }
            }
        }
    };

    if let Some(land) = land_neighbors {
        process_neighbors(land.neighbors());
    }
    
    if let Some(naval) = naval_neighbors {
        process_neighbors(naval.neighbors());
    }

    best_target
}
