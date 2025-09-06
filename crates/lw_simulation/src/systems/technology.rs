//! Technology system - handles research and technological progress

use bevy::prelude::*;
use crate::components::{Nation, Province};
use lw_core::{Fixed32, DeterministicRNG};

#[derive(Debug, Clone)]
pub enum TechnologyType {
    Agriculture,
    Military,
    Infrastructure,
    Society,
    Economy,
    Science,
}

#[derive(Event, Debug)]
pub struct TechnologyDiscoveredEvent {
    pub nation_id: u32,
    pub tech_type: TechnologyType,
    pub tech_level: u8,
}

/// Technology advancement system
pub fn technology_system(
    mut nations: Query<&mut Nation>,
    provinces: Query<&Province>,
    mut tech_events: EventWriter<TechnologyDiscoveredEvent>,
    mut rng: Local<DeterministicRNG>,
) {
    if !rng.is_initialized() {
        *rng = DeterministicRNG::new(77777);
    }
    
    for mut nation in nations.iter_mut() {
        // Count developed provinces
        let developed_provinces = provinces.iter()
            .filter(|p| p.owner == Some(nation.id) && p.development > Fixed32::from_float(0.5))
            .count();
        
        // Research rate based on development and stability
        let research_rate = Fixed32::from_float(developed_provinces as f32 * 0.001) 
            * nation.stability;
        
        // Random tech discovery chance
        if rng.next_bool(research_rate.to_f32()) {
            nation.tech_level = nation.tech_level.saturating_add(1);
            
            let tech_type = match rng.range_i32(0, 6) {
                0 => TechnologyType::Agriculture,
                1 => TechnologyType::Military,
                2 => TechnologyType::Infrastructure,
                3 => TechnologyType::Society,
                4 => TechnologyType::Economy,
                _ => TechnologyType::Science,
            };
            
            tech_events.send(TechnologyDiscoveredEvent {
                nation_id: nation.id,
                tech_type,
                tech_level: nation.tech_level,
            });
        }
    }
}