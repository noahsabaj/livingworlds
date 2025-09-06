//! Province Logic Systems
//! 
//! All logic for Province components extracted to follow ECS principles.

use bevy::prelude::*;
use lw_core::Fixed32;
use crate::components::*;

/// Setup production capacity for a province based on terrain
pub fn setup_province_production_capacity(
    terrain: TerrainType,
    area: Fixed32,
) -> (Fixed32, Fixed32, Fixed32, Fixed32) {
    let base_hectares = area * Fixed32::from_num(100); // 100 hectares per km²
    
    match terrain {
        TerrainType::Plains => (
            base_hectares * Fixed32::from_float(0.7),  // arable_land
            Fixed32::ZERO,                              // coastal_access
            base_hectares * Fixed32::from_float(0.2),  // pasture_land
            base_hectares * Fixed32::from_float(0.1),  // forest_coverage
        ),
        TerrainType::Coastal => (
            base_hectares * Fixed32::from_float(0.2),  // arable_land
            Fixed32::from_num(50),                      // coastal_access km
            base_hectares * Fixed32::from_float(0.1),  // pasture_land
            Fixed32::ZERO,                              // forest_coverage
        ),
        TerrainType::Mountain => (
            base_hectares * Fixed32::from_float(0.2),  // arable_land
            Fixed32::ZERO,                              // coastal_access
            base_hectares * Fixed32::from_float(0.5),  // pasture_land
            base_hectares * Fixed32::from_float(0.2),  // forest_coverage
        ),
        TerrainType::Forest => (
            base_hectares * Fixed32::from_float(0.1),  // arable_land
            Fixed32::ZERO,                              // coastal_access
            Fixed32::ZERO,                              // pasture_land
            base_hectares * Fixed32::from_float(0.8),  // forest_coverage
        ),
        TerrainType::Desert => (
            Fixed32::ZERO,                              // arable_land
            Fixed32::ZERO,                              // coastal_access
            base_hectares * Fixed32::from_float(0.1),  // pasture_land
            Fixed32::ZERO,                              // forest_coverage
        ),
        TerrainType::Mountain => (
            Fixed32::ZERO,                              // arable_land
            Fixed32::ZERO,                              // coastal_access
            base_hectares * Fixed32::from_float(0.05), // pasture_land
            Fixed32::ZERO,                              // forest_coverage
        ),
        _ => (Fixed32::ZERO, Fixed32::ZERO, Fixed32::ZERO, Fixed32::ZERO),
    }
}

/// System to initialize provinces with production capacity
pub fn initialize_province_production_system(
    mut provinces: Query<(&mut Province, &TerrainType), Added<Province>>,
) {
    // TODO: Province production fields moved to separate component
    // Need to create ProvinceProduction component with:
    // - arable_land, coastal_access, pasture_land, forest_coverage
    /*
    for (mut province, terrain) in &mut provinces {
        let (arable, coastal, pasture, forest) = 
            setup_province_production_capacity(*terrain, province.area);
        
        province.arable_land = arable;
        province.coastal_access = coastal;
        province.pasture_land = pasture;
        province.forest_coverage = forest;
    }
    */
}

/// System to calculate province development
pub fn province_development_system(
    mut provinces: Query<(&mut Province, &Population, &ResourceProduction)>,
) {
    // TODO: Province development field moved to separate component
    // Need to create ProvinceDevelopment component
    /*
    for (mut province, population, production) in &mut provinces {
        // Development based on:
        // - Population density
        // - Infrastructure investment
        // - Resource extraction
        // - Trade connections
        
        let pop_density = population.count / province.area;
        let productivity = production.efficiency;
        
        // Development grows slowly over time
        let development_change = (pop_density * productivity - province.development) * Fixed32::from_float(0.01);
        province.development = (province.development + development_change).clamp(Fixed32::ZERO, Fixed32::ONE);
    }
    */
}

/// System to handle resource depletion
pub fn resource_depletion_system(
    mut provinces: Query<(&mut Province, &ResourceProduction)>,
    time: Res<Time>,
) {
    // TODO: Province resource fields moved to separate component
    // Need to create ProvinceResources component
    /*
    let dt = Fixed32::from_float(time.delta().as_secs_f32());
    
    for (mut province, production) in &mut provinces {
        // Resources deplete based on extraction rate
        if let Some(resource_type) = province.resource {
            let extraction_rate = production.get_production(resource_type);
            let depletion = extraction_rate * dt * Fixed32::from_float(0.0001); // Very slow depletion
            
            province.resource_quantity = (province.resource_quantity - depletion).max(Fixed32::ZERO);
            
            // If depleted, mark as exhausted
            if province.resource_quantity == Fixed32::ZERO {
                province.resource = None;
            }
        }
    }
    */
}

/// System to calculate province carrying capacity
pub fn calculate_carrying_capacity_system(
    mut provinces: Query<(&Province, &mut Territory, &Climate)>,
) {
    // TODO: Province arable_land field moved to separate component
    // Need to create ProvinceProduction component
    /*
    for (province, mut territory, climate) in &mut provinces {
        // Carrying capacity depends on:
        // - Arable land
        // - Water availability
        // - Climate conditions
        // - Technology level
        
        let base_capacity = province.arable_land * Fixed32::from_num(100); // 100 people per hectare
        let climate_modifier = climate.growing_season_length;
        // TODO: Add fortification/infrastructure component to provinces for tech modifier
        let tech_modifier = Fixed32::from_float(1.0);
        
        let carrying_capacity = base_capacity * climate_modifier * tech_modifier;
        
        // Store in territory component (should probably be its own component)
        // This is a temporary solution
    }
    */
}

/// System to handle natural disasters
pub fn natural_disaster_system(
    mut provinces: Query<(Entity, &mut Province, &GeologicalStability, &Climate)>,
    mut disaster_events: EventWriter<NaturalDisaster>,
    time: Res<Time>,
) {
    for (entity, mut province, stability, climate) in &mut provinces {
        // Calculate disaster risk based on:
        // - Geological stability
        // - Climate extremes
        // - Recent disasters (recovery period)
        
        // Calculate average stability from all factors
        let avg_instability = (stability.tectonic_activity + stability.volcanic_activity + 
                               stability.soil_stability + stability.flood_risk + 
                               stability.coastal_erosion) / Fixed32::from_num(5);
        let base_risk = avg_instability;
        let climate_risk = climate.extreme_weather_frequency;
        let total_risk = (base_risk + climate_risk) / Fixed32::from_num(2);
        
        // Random chance based on risk
        // In real implementation, would use deterministic random
        if total_risk > Fixed32::from_float(0.95) {
            // Trigger disaster
            disaster_events.send(NaturalDisaster {
                disaster_type: DisasterType::Earthquake, // Would determine based on conditions
                epicenter: entity,
                affected_provinces: vec![entity], // Just this province for now
                intensity: total_risk,
                duration: 30, // days
                casualties: 0, // Would calculate based on population
                infrastructure_damage: total_risk * Fixed32::from_float(0.5),
            });
            
            // Apply immediate effects
            // TODO: Province development field moved to separate component
            // province.development = (province.development * Fixed32::from_float(0.7)).max(Fixed32::ZERO);
        }
    }
}