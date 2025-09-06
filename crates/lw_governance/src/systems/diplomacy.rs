//! Diplomacy system - handles relations between nations

use bevy::prelude::*;
use crate::components::{Nation, DiplomaticRelation, Army};
use lw_core::{Fixed32, DeterministicRNG};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RelationType {
    Allied,
    Friendly,
    Neutral,
    Hostile,
    AtWar,
}

#[derive(Event, Debug)]
pub struct DiplomaticEvent {
    pub nation1: u32,
    pub nation2: u32,
    pub event_type: DiplomaticEventType,
}

#[derive(Debug, Clone)]
pub enum DiplomaticEventType {
    AllianceFormed,
    AllianceBroken,
    WarDeclared,
    PeaceTreaty,
    TradeAgreement,
    BorderDispute,
}

/// Diplomatic relations tracking
#[derive(Resource, Default)]
pub struct DiplomacyState {
    pub relations: HashMap<(u32, u32), i32>, // -100 to +100
    pub treaties: HashMap<(u32, u32), Vec<Treaty>>,
}

#[derive(Debug, Clone)]
pub struct Treaty {
    pub treaty_type: TreatyType,
    pub start_date: i32,
    pub duration: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum TreatyType {
    Alliance,
    Trade,
    NonAggression,
    Peace,
}

/// Main diplomacy system
pub fn diplomacy_system(
    nations: Query<&Nation>,
    armies: Query<&Army>,
    mut diplo_state: ResMut<DiplomacyState>,
    mut diplo_events: EventWriter<DiplomaticEvent>,
    mut rng: Local<DeterministicRNG>,
) {
    if !rng.is_initialized() {
        *rng = DeterministicRNG::new(33333);
    }
    
    let nations_list: Vec<_> = nations.iter().collect();
    
    // Update relations based on various factors
    for i in 0..nations_list.len() {
        for j in i+1..nations_list.len() {
            let nation1 = nations_list[i];
            let nation2 = nations_list[j];
            let key = if nation1.id < nation2.id {
                (nation1.id, nation2.id)
            } else {
                (nation2.id, nation1.id)
            };
            
            let current_relation = *diplo_state.relations.get(&key).unwrap_or(&0);
            let mut new_relation = current_relation;
            
            // Government compatibility
            if nation1.government == nation2.government {
                new_relation += 1;
            } else {
                new_relation -= 1;
            }
            
            // Economic system compatibility
            if nation1.economy == nation2.economy {
                new_relation += 1;
            }
            
            // Random events
            if rng.next_bool(0.01) {
                let event_impact = rng.range_i32(-10, 10);
                new_relation += event_impact;
            }
            
            // Clamp relations
            new_relation = new_relation.clamp(-100, 100);
            diplo_state.relations.insert(key, new_relation);
            
            // Check for diplomatic state changes
            if current_relation < -50 && new_relation >= -50 {
                // Relations improved
                diplo_events.send(DiplomaticEvent {
                    nation1: nation1.id,
                    nation2: nation2.id,
                    event_type: DiplomaticEventType::PeaceTreaty,
                });
            } else if current_relation > 50 && new_relation <= 50 {
                // Relations worsened
                diplo_events.send(DiplomaticEvent {
                    nation1: nation1.id,
                    nation2: nation2.id,
                    event_type: DiplomaticEventType::BorderDispute,
                });
            }
            
            // Alliance formation
            if new_relation > 80 && !has_treaty(&diplo_state.treaties, key, TreatyType::Alliance) {
                diplo_state.treaties.entry(key)
                    .or_insert_with(Vec::new)
                    .push(Treaty {
                        treaty_type: TreatyType::Alliance,
                        start_date: 0, // Would use actual game date
                        duration: None,
                    });
                
                diplo_events.send(DiplomaticEvent {
                    nation1: nation1.id,
                    nation2: nation2.id,
                    event_type: DiplomaticEventType::AllianceFormed,
                });
            }
            
            // War declaration
            if new_relation < -80 && !has_treaty(&diplo_state.treaties, key, TreatyType::Peace) {
                diplo_events.send(DiplomaticEvent {
                    nation1: nation1.id,
                    nation2: nation2.id,
                    event_type: DiplomaticEventType::WarDeclared,
                });
            }
        }
    }
}

/// Check if two nations are at war
pub fn nations_at_war(diplo_state: &DiplomacyState, nation1: u32, nation2: u32) -> bool {
    let key = if nation1 < nation2 {
        (nation1, nation2)
    } else {
        (nation2, nation1)
    };
    
    diplo_state.relations.get(&key).map(|&r| r < -80).unwrap_or(false)
}

/// Check if two nations are allied
pub fn nations_allied(diplo_state: &DiplomacyState, nation1: u32, nation2: u32) -> bool {
    let key = if nation1 < nation2 {
        (nation1, nation2)
    } else {
        (nation2, nation1)
    };
    
    has_treaty(&diplo_state.treaties, key, TreatyType::Alliance)
}

/// Check if a specific treaty exists
fn has_treaty(treaties: &HashMap<(u32, u32), Vec<Treaty>>, key: (u32, u32), treaty_type: TreatyType) -> bool {
    treaties.get(&key)
        .map(|t| t.iter().any(|treaty| matches!(treaty.treaty_type, ref tt if std::mem::discriminant(tt) == std::mem::discriminant(&treaty_type))))
        .unwrap_or(false)
}

/// Get diplomatic relation type
pub fn get_relation_type(diplo_state: &DiplomacyState, nation1: u32, nation2: u32) -> RelationType {
    let key = if nation1 < nation2 {
        (nation1, nation2)
    } else {
        (nation2, nation1)
    };
    
    let relation = diplo_state.relations.get(&key).copied().unwrap_or(0);
    
    if relation < -80 {
        RelationType::AtWar
    } else if relation < -40 {
        RelationType::Hostile
    } else if relation < 40 {
        RelationType::Neutral
    } else if relation < 80 {
        RelationType::Friendly
    } else {
        RelationType::Allied
    }
}