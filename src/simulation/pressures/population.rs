//! Population pressure calculations
//!
//! Population pressure drives expansion, migration, and infrastructure development.

use super::types::{PressureLevel, PressureType};
use crate::nations::Nation;
use crate::world::Province;

/// Population-related pressures affecting a nation
#[derive(Debug, Clone)]
pub struct PopulationPressure {
    /// Overcrowding in provinces (drives expansion)
    pub overcrowding: PressureLevel,
    /// Underpopulation (drives consolidation)
    pub underpopulation: PressureLevel,
    /// Food shortage (drives agricultural development or conquest)
    pub food_shortage: PressureLevel,
    /// Labor shortage (drives population policies)
    pub labor_shortage: PressureLevel,
}

/// Calculate population pressures for a nation
pub fn calculate_population_pressure(
    nation: &Nation,
    controlled_provinces: &[&Province],
) -> PopulationPressure {
    if controlled_provinces.is_empty() {
        return PopulationPressure {
            overcrowding: PressureLevel::NONE,
            underpopulation: PressureLevel::HIGH, // No provinces = crisis
            food_shortage: PressureLevel::CRITICAL,
            labor_shortage: PressureLevel::CRITICAL,
        };
    }

    // Calculate total population and capacity
    let total_population: u32 = controlled_provinces.iter().map(|p| p.population).sum();

    let total_capacity: u32 = controlled_provinces.iter().map(|p| p.max_population).sum();

    let total_food_production: f32 = controlled_provinces
        .iter()
        .map(|p| p.agriculture.value() * 10000.0) // Agriculture value * base food per point
        .sum();

    // Calculate overcrowding (high population relative to capacity)
    let occupancy_rate = total_population as f32 / total_capacity.max(1) as f32;
    let overcrowding = if occupancy_rate > 0.9 {
        PressureLevel::new((occupancy_rate - 0.9) * 10.0) // Scales 0.9-1.0 to 0.0-1.0
    } else {
        PressureLevel::NONE
    };

    // Calculate underpopulation (too few people to maintain territory)
    let provinces_count = controlled_provinces.len() as f32;
    let avg_population_per_province = total_population as f32 / provinces_count;
    let min_viable_population = 5000.0; // Minimum to maintain a province
    let underpopulation = if avg_population_per_province < min_viable_population {
        PressureLevel::new(1.0 - (avg_population_per_province / min_viable_population))
    } else {
        PressureLevel::NONE
    };

    // Calculate food shortage
    let food_needed = total_population as f32 * 1.0; // 1 food unit per person
    let food_ratio = total_food_production / food_needed.max(1.0);
    let food_shortage = if food_ratio < 1.0 {
        PressureLevel::new(1.0 - food_ratio)
    } else {
        PressureLevel::NONE
    };

    // Calculate labor shortage (for infrastructure and military)
    let working_age_pop = (total_population as f32 * 0.6) as u32; // 60% working age
    let labor_needed = provinces_count * 2000.0; // 2000 workers per province for full productivity
    let labor_ratio = working_age_pop as f32 / labor_needed;
    let labor_shortage = if labor_ratio < 1.0 {
        PressureLevel::new(1.0 - labor_ratio)
    } else {
        PressureLevel::NONE
    };

    PopulationPressure {
        overcrowding,
        underpopulation,
        food_shortage,
        labor_shortage,
    }
}

/// Actions a nation might take to address population pressures
#[derive(Debug, Clone)]
pub enum PopulationAction {
    /// Expand into new territory
    Expand { target_richness_threshold: f32 },
    /// Develop agriculture in existing provinces
    DevelopAgriculture { investment: f32 },
    /// Encourage population growth
    EncourageGrowth { incentive_level: f32 },
    /// Build infrastructure to increase capacity
    BuildInfrastructure { focus_provinces: Vec<u32> },
    /// Consolidate population from outlying areas
    Consolidate { abandon_provinces: Vec<u32> },
}

/// Determine what action to take based on population pressures
pub fn resolve_population_pressure(pressure: &PopulationPressure) -> Option<PopulationAction> {
    // Priority order: food shortage > overcrowding > labor shortage > underpopulation

    if pressure.food_shortage.is_critical() {
        // Critical food shortage - must expand to fertile lands or develop agriculture
        return Some(PopulationAction::Expand {
            target_richness_threshold: 2.0, // Seek fertile lands
        });
    }

    if pressure.overcrowding.is_high() {
        // Need more space - expand to any available land
        return Some(PopulationAction::Expand {
            target_richness_threshold: 0.5, // Take any land
        });
    }

    if pressure.food_shortage.is_moderate() {
        // Moderate food issues - develop agriculture
        return Some(PopulationAction::DevelopAgriculture {
            investment: pressure.food_shortage.value(),
        });
    }

    if pressure.labor_shortage.is_high() {
        // Need more people - encourage growth
        return Some(PopulationAction::EncourageGrowth {
            incentive_level: pressure.labor_shortage.value(),
        });
    }

    if pressure.underpopulation.is_critical() {
        // Can't hold territory - consolidate
        return Some(PopulationAction::Consolidate {
            abandon_provinces: Vec::new(), // System will determine which ones
        });
    }

    None
}
