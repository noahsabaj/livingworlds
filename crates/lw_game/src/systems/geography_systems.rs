//! Geography systems - logic for provinces and terrain
//!
//! All province-related logic extracted from components to here.
//! Pure functions where possible for testability.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::geography::*;
use crate::types::{NaturalDisaster, DisasterType, GameTime};

/// Apply environmental damage to a province
pub fn apply_environmental_consequences_system(
    mut provinces: Query<(
        &mut Province,
        Option<&mut PopulationCapacity>,
        Option<&mut EnvironmentalHealth>,
    )>,
    environmental_events: EventReader<EnvironmentalImpactEvent>,
) {
    for event in environmental_events.iter() {
        if let Ok((mut province, capacity, mut health)) = provinces.get_mut(event.province) {
            if let Some(mut health) = health {
                health.current = (health.current - event.impact)
                    .clamp(Fixed32::ZERO, Fixed32::ONE);
                    
                // Severe damage reduces carrying capacity
                if event.impact > Fixed32::from_float(0.5) {
                    if let Some(mut capacity) = capacity {
                        capacity.current_capacity *= Fixed32::from_float(0.9);
                    }
                }
            }
        }
    }
}

/// Calculate natural disaster risk for provinces
pub fn calculate_disaster_risk(
    geological_stability: Fixed32,
    terrain_type: &TerrainType,
    climate: &Climate,
) -> Fixed32 {
    let base_risk = Fixed32::from_float(0.01);
    
    // Different terrains have different risks
    let terrain_modifier = match terrain_type {
        TerrainType::Mountain => Fixed32::from_float(0.2),  // Earthquakes, avalanches
        TerrainType::Coastal => Fixed32::from_float(0.15),  // Tsunamis, hurricanes
        TerrainType::River => Fixed32::from_float(0.1),     // Floods
        TerrainType::Desert => Fixed32::from_float(0.05),   // Sandstorms
        TerrainType::Plains => Fixed32::from_float(0.03),   // Tornadoes
        TerrainType::Forest => Fixed32::from_float(0.08),   // Fires
        _ => Fixed32::from_float(0.02),
    };
    
    let geological_modifier = Fixed32::ONE - geological_stability;
    let climate_modifier = climate.extreme_weather_frequency;
    
    base_risk + (terrain_modifier * geological_modifier) + climate_modifier
}

/// System to generate natural disasters
pub fn natural_disaster_generation_system(
    provinces: Query<(Entity, &Province, &TerrainType, &Climate, Option<&GeologicalStability>)>,
    mut disaster_events: EventWriter<NaturalDisasterEvent>,
    time: Res<Time>,
) {
    for (entity, province, terrain, climate, stability) in &provinces {
        let geological_stability = stability
            .map(|s| s.stability)
            .unwrap_or(Fixed32::from_float(0.8));
            
        let risk = calculate_disaster_risk(geological_stability, terrain, climate);
        
        // Random chance based on risk (simplified - would use proper RNG)
        if risk > Fixed32::from_float(0.1) {
            let disaster_type = determine_disaster_type(terrain, climate);
            
            disaster_events.send(NaturalDisasterEvent {
                province: entity,
                disaster_type,
                severity: calculate_disaster_severity(risk),
                duration: 1,  // Days
            });
        }
    }
}

/// Determine disaster type based on terrain and climate
pub fn determine_disaster_type(terrain: &TerrainType, climate: &Climate) -> DisasterType {
    match terrain {
        TerrainType::Mountain => DisasterType::Earthquake,
        TerrainType::Coastal => DisasterType::Hurricane,
        TerrainType::River => DisasterType::Flood,
        TerrainType::Desert => DisasterType::Drought,
        TerrainType::Forest => DisasterType::Fire,
        TerrainType::Plains => {
            if climate.extreme_weather_frequency > Fixed32::from_float(0.5) {
                DisasterType::Tornado
            } else {
                DisasterType::Drought
            }
        },
        _ => DisasterType::Earthquake,
    }
}

/// Calculate disaster severity
pub fn calculate_disaster_severity(risk: Fixed32) -> Fixed32 {
    // Higher risk areas have more severe disasters
    (risk * Fixed32::from_num(2)).min(Fixed32::ONE)
}

/// Apply disaster effects to provinces
pub fn apply_disaster_effects_system(
    mut provinces: Query<(&mut Population, &mut DevelopmentLevel)>,
    mut disaster_events: EventReader<NaturalDisasterEvent>,
) {
    for event in disaster_events.iter() {
        if let Ok((mut population, mut development)) = provinces.get_mut(event.province) {
            // Reduce population
            let casualty_rate = event.severity * Fixed32::from_float(0.1);
            population.total = ((population.total as f32) * 
                               (1.0 - casualty_rate.to_f32())) as u32;
            
            // Damage infrastructure
            development.level *= Fixed32::ONE - (event.severity * Fixed32::from_float(0.2));
        }
    }
}

/// Setup production capacity based on terrain
pub fn calculate_production_capacity(
    terrain: &TerrainType,
    area: Fixed32,
) -> ProductionCapacity {
    let base_hectares = area * Fixed32::from_num(100); // 100 hectares per km²
    
    match terrain {
        TerrainType::Plains => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.7),
            pasture_land: base_hectares * Fixed32::from_float(0.2),
            forest_coverage: base_hectares * Fixed32::from_float(0.1),
        },
        TerrainType::Forest => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.1),
            pasture_land: base_hectares * Fixed32::from_float(0.1),
            forest_coverage: base_hectares * Fixed32::from_float(0.8),
        },
        TerrainType::Mountain => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.05),
            pasture_land: base_hectares * Fixed32::from_float(0.15),
            forest_coverage: base_hectares * Fixed32::from_float(0.3),
        },
        TerrainType::Desert => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.01),
            pasture_land: base_hectares * Fixed32::from_float(0.05),
            forest_coverage: Fixed32::ZERO,
        },
        TerrainType::Coastal => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.2),
            pasture_land: base_hectares * Fixed32::from_float(0.1),
            forest_coverage: base_hectares * Fixed32::from_float(0.1),
        },
        _ => ProductionCapacity {
            arable_land: base_hectares * Fixed32::from_float(0.3),
            pasture_land: base_hectares * Fixed32::from_float(0.3),
            forest_coverage: base_hectares * Fixed32::from_float(0.2),
        },
    }
}

// Event types
#[derive(Event)]
pub struct EnvironmentalImpactEvent {
    pub province: Entity,
    pub impact: Fixed32,
}

#[derive(Event)]
pub struct NaturalDisasterEvent {
    pub province: Entity,
    pub disaster_type: DisasterType,
    pub severity: Fixed32,
    pub duration: u64,
}

// Component placeholders for the new architecture
#[derive(Component)]
pub struct PopulationCapacity {
    pub current_capacity: Fixed32,
    pub maximum_capacity: Fixed32,
}

#[derive(Component)]
pub struct EnvironmentalHealth {
    pub current: Fixed32,
    pub regeneration_rate: Fixed32,
}

#[derive(Component)]
pub struct GeologicalStability {
    pub stability: Fixed32,
    pub fault_lines: Vec<Entity>,
}

#[derive(Component)]
pub struct Population {
    pub total: u32,
    pub growth_rate: Fixed32,
}

#[derive(Component)]
pub struct DevelopmentLevel {
    pub level: Fixed32,
    pub infrastructure_quality: Fixed32,
}

#[derive(Component)]
pub struct ProductionCapacity {
    pub arable_land: Fixed32,
    pub pasture_land: Fixed32,
    pub forest_coverage: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    Plains,
    Mountain,
    Forest,
    Desert,
    Coastal,
    River,
    Tundra,
    Swamp,
}