//! Collapse system - handles empire decline and fragmentation

use bevy::prelude::*;
use crate::components::{Nation, Province};
use lw_core::{Fixed32, DeterministicRNG};

/// Event for nation collapse
#[derive(Event, Debug)]
pub struct CollapseEvent {
    pub nation_id: u32,
    pub reason: CollapseReason,
    pub successor_nations: Vec<u32>,
}

#[derive(Debug, Clone)]
pub enum CollapseReason {
    Overextension,
    EconomicFailure,
    MilitaryDefeat,
    CivilWar,
    NaturalDisaster,
}

/// Main collapse detection system
pub fn collapse_system(
    mut nations: Query<&mut Nation>,
    provinces: Query<&Province>,
    mut collapse_events: EventWriter<CollapseEvent>,
    mut rng: Local<DeterministicRNG>,
) {
    if !rng.is_initialized() {
        *rng = DeterministicRNG::new(99999);
    }
    
    for mut nation in nations.iter_mut() {
        // Check collapse conditions
        let province_count = provinces.iter()
            .filter(|p| p.owner == Some(nation.id))
            .count();
        
        // Overextension check
        if province_count > 100 && nation.stability < Fixed32::from_float(0.3) {
            if rng.next_bool(0.01) {
                nation.is_collapsing = true;
                collapse_events.send(CollapseEvent {
                    nation_id: nation.id,
                    reason: CollapseReason::Overextension,
                    successor_nations: vec![],
                });
            }
        }
        
        // Economic collapse check
        if nation.treasury < Fixed32::ZERO && rng.next_bool(0.005) {
            nation.is_collapsing = true;
            collapse_events.send(CollapseEvent {
                nation_id: nation.id,
                reason: CollapseReason::EconomicFailure,
                successor_nations: vec![],
            });
        }
        
        // Stability collapse
        if nation.stability <= Fixed32::ZERO {
            nation.is_collapsing = true;
            collapse_events.send(CollapseEvent {
                nation_id: nation.id,
                reason: CollapseReason::CivilWar,
                successor_nations: vec![],
            });
        }
    }
}