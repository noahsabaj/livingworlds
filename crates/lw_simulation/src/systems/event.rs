//! Event system for game events and notifications

use bevy::prelude::*;
use lw_core::{Fixed32, DeterministicRNG};
use crate::components::{Nation, Province, City, Population};

/// Types of game events
#[derive(Debug, Clone)]
pub enum EventType {
    // Natural disasters
    Plague { mortality_rate: Fixed32 },
    Famine { severity: Fixed32 },
    Earthquake { magnitude: Fixed32 },
    Flood,
    Drought,
    
    // Political events
    Rebellion { province_id: u32 },
    CivilWar { nation_id: u32 },
    Succession { nation_id: u32 },
    Revolution { nation_id: u32 },
    
    // Economic events
    MarketCrash { nation_id: u32 },
    TradeDisruption { route_id: u32 },
    ResourceDiscovery { province_id: u32, resource: u8 },
    
    // Cultural events
    ReligiousReformation { nation_id: u32 },
    CulturalRenaissance { nation_id: u32 },
    TechnologicalBreakthrough { nation_id: u32 },
    
    // Diplomatic events
    AllianceFormed { nation1: u32, nation2: u32 },
    AllianceBroken { nation1: u32, nation2: u32 },
    WarDeclared { aggressor: u32, defender: u32 },
    PeaceTreaty { nation1: u32, nation2: u32 },
}

/// Game event with metadata
#[derive(Debug, Clone, Event)]
pub struct GameEvent {
    pub event_type: EventType,
    pub timestamp: Fixed32,
    pub location: Option<u32>, // Province ID if applicable
    pub affected_nations: Vec<u32>,
    pub severity: Fixed32, // 0-1 scale
    pub description: String,
}

/// Event generator system
pub fn event_generation_system(
    nations: Query<&Nation>,
    provinces: Query<&Province>,
    cities: Query<&City>,
    populations: Query<&Population>,
    mut events: EventWriter<GameEvent>,
    time: Res<Time>,
    mut rng: Local<DeterministicRNG>,
) {
    // Initialize RNG if needed
    if !rng.is_initialized() {
        *rng = DeterministicRNG::new(54321);
    }
    
    let current_time = Fixed32::from_float(time.elapsed_secs());
    
    // Check for random events (simplified)
    let event_chance = 0.001; // 0.1% chance per frame
    
    if rng.next_bool(event_chance) {
        // Generate a random event
        let event_type = generate_random_event(&mut rng, &nations, &provinces);
        
        if let Some(event_type) = event_type {
            let (location, affected) = get_event_targets(&event_type, &provinces, &nations);
            
            events.send(GameEvent {
                event_type: event_type.clone(),
                timestamp: current_time,
                location,
                affected_nations: affected,
                severity: Fixed32::from_float(rng.next_f32()),
                description: describe_event(&event_type),
            });
        }
    }
    
    // Check for triggered events based on conditions
    check_triggered_events(&nations, &provinces, &populations, &mut events, current_time, &mut rng);
}

/// Generate a random event type
fn generate_random_event(
    rng: &mut DeterministicRNG,
    nations: &Query<&Nation>,
    provinces: &Query<&Province>,
) -> Option<EventType> {
    let event_category = rng.range_i32(0, 5);
    
    match event_category {
        0 => {
            // Natural disaster
            let disaster = rng.range_i32(0, 5);
            match disaster {
                0 => Some(EventType::Plague { 
                    mortality_rate: Fixed32::from_float(0.1 + rng.next_f32() * 0.3) 
                }),
                1 => Some(EventType::Famine { 
                    severity: Fixed32::from_float(rng.next_f32()) 
                }),
                2 => Some(EventType::Earthquake { 
                    magnitude: Fixed32::from_float(3.0 + rng.next_f32() * 4.0) 
                }),
                3 => Some(EventType::Flood),
                4 => Some(EventType::Drought),
                _ => None,
            }
        }
        1 => {
            // Political event
            if let Some(nation) = nations.iter().next() {
                let event = rng.range_i32(0, 3);
                match event {
                    0 => Some(EventType::Succession { nation_id: nation.id }),
                    1 => Some(EventType::Revolution { nation_id: nation.id }),
                    2 => Some(EventType::CivilWar { nation_id: nation.id }),
                    _ => None,
                }
            } else {
                None
            }
        }
        2 => {
            // Economic event
            if let Some(province) = provinces.iter().next() {
                Some(EventType::ResourceDiscovery {
                    province_id: province.id,
                    resource: rng.range_i32(1, 8) as u8,
                })
            } else {
                None
            }
        }
        3 => {
            // Cultural event
            if let Some(nation) = nations.iter().next() {
                let event = rng.range_i32(0, 3);
                match event {
                    0 => Some(EventType::ReligiousReformation { nation_id: nation.id }),
                    1 => Some(EventType::CulturalRenaissance { nation_id: nation.id }),
                    2 => Some(EventType::TechnologicalBreakthrough { nation_id: nation.id }),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get affected location and nations for an event
fn get_event_targets(
    event: &EventType,
    provinces: &Query<&Province>,
    nations: &Query<&Nation>,
) -> (Option<u32>, Vec<u32>) {
    match event {
        EventType::Plague { .. } | EventType::Famine { .. } | EventType::Earthquake { .. } 
        | EventType::Flood | EventType::Drought => {
            // Affects a random province
            if let Some(province) = provinces.iter().next() {
                let affected_nations = province.owner.map_or(vec![], |owner| vec![owner]);
                (Some(province.id), affected_nations)
            } else {
                (None, vec![])
            }
        }
        EventType::Rebellion { province_id } | EventType::ResourceDiscovery { province_id, .. } => {
            // Specific province
            let nation = provinces.iter()
                .find(|p| p.id == *province_id)
                .and_then(|p| p.owner)
                .unwrap_or(0);
            (Some(*province_id), vec![nation])
        }
        EventType::CivilWar { nation_id } | EventType::Succession { nation_id } 
        | EventType::Revolution { nation_id } | EventType::MarketCrash { nation_id }
        | EventType::ReligiousReformation { nation_id } | EventType::CulturalRenaissance { nation_id }
        | EventType::TechnologicalBreakthrough { nation_id } => {
            // Affects entire nation
            (None, vec![*nation_id])
        }
        EventType::AllianceFormed { nation1, nation2 } | EventType::AllianceBroken { nation1, nation2 }
        | EventType::WarDeclared { aggressor: nation1, defender: nation2 } 
        | EventType::PeaceTreaty { nation1, nation2 } => {
            // Affects two nations
            (None, vec![*nation1, *nation2])
        }
        EventType::TradeDisruption { .. } => {
            // Trade route specific
            (None, vec![])
        }
    }
}

/// Generate description for event
fn describe_event(event: &EventType) -> String {
    match event {
        EventType::Plague { mortality_rate } => {
            format!("A plague sweeps through the land, mortality rate: {:.1}%", 
                    mortality_rate.to_f32() * 100.0)
        }
        EventType::Famine { severity } => {
            let level = if severity.to_f32() > 0.7 { 
                "severe" 
            } else if severity.to_f32() > 0.4 { 
                "moderate" 
            } else { 
                "mild" 
            };
            format!("A {} famine affects the region", level)
        }
        EventType::Earthquake { magnitude } => {
            format!("An earthquake of magnitude {:.1} strikes", magnitude.to_f32())
        }
        EventType::Flood => "Flooding devastates the lowlands".to_string(),
        EventType::Drought => "Drought withers the crops".to_string(),
        EventType::Rebellion { .. } => "Rebels rise against the government".to_string(),
        EventType::CivilWar { .. } => "Civil war tears the nation apart".to_string(),
        EventType::Succession { .. } => "A succession crisis grips the throne".to_string(),
        EventType::Revolution { .. } => "Revolution sweeps through the nation".to_string(),
        EventType::MarketCrash { .. } => "Markets crash, fortunes are lost".to_string(),
        EventType::TradeDisruption { .. } => "Trade routes are disrupted".to_string(),
        EventType::ResourceDiscovery { resource, .. } => {
            format!("New resources discovered: type {}", resource)
        }
        EventType::ReligiousReformation { .. } => "Religious reformation begins".to_string(),
        EventType::CulturalRenaissance { .. } => "A cultural renaissance flourishes".to_string(),
        EventType::TechnologicalBreakthrough { .. } => "Technological breakthrough achieved".to_string(),
        EventType::AllianceFormed { .. } => "Nations form an alliance".to_string(),
        EventType::AllianceBroken { .. } => "Alliance dissolved".to_string(),
        EventType::WarDeclared { .. } => "War is declared".to_string(),
        EventType::PeaceTreaty { .. } => "Peace treaty signed".to_string(),
    }
}

/// Check for events triggered by game conditions
fn check_triggered_events(
    nations: &Query<&Nation>,
    provinces: &Query<&Province>,
    populations: &Query<&Population>,
    events: &mut EventWriter<GameEvent>,
    current_time: Fixed32,
    rng: &mut DeterministicRNG,
) {
    // Check for civil wars (low stability)
    for nation in nations.iter() {
        if nation.stability < Fixed32::from_float(0.2) && rng.next_bool(0.01) {
            events.send(GameEvent {
                event_type: EventType::CivilWar { nation_id: nation.id },
                timestamp: current_time,
                location: None,
                affected_nations: vec![nation.id],
                severity: Fixed32::ONE - nation.stability,
                description: format!("{} descends into civil war", nation.name),
            });
        }
    }
    
    // Check for famines (low food)
    for population in populations.iter() {
        if population.food_deficit > Fixed32::from_num(100) && rng.next_bool(0.005) {
            events.send(GameEvent {
                event_type: EventType::Famine { 
                    severity: (population.food_deficit / Fixed32::from_num(1000)).min(Fixed32::ONE) 
                },
                timestamp: current_time,
                location: None,
                affected_nations: vec![],
                severity: Fixed32::from_float(0.7),
                description: "Food shortages lead to famine".to_string(),
            });
        }
    }
    
    // Check for rebellions (low development)
    for province in provinces.iter() {
        if province.development < Fixed32::from_float(0.1) && rng.next_bool(0.001) {
            events.send(GameEvent {
                event_type: EventType::Rebellion { province_id: province.id },
                timestamp: current_time,
                location: Some(province.id),
                affected_nations: province.owner.map_or(vec![], |owner| vec![owner]),
                severity: Fixed32::from_float(0.5),
                description: "Peasants rebel against poor conditions".to_string(),
            });
        }
    }
}