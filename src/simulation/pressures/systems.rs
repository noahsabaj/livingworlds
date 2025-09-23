//! Core pressure system that drives nation behavior
//!
//! This system updates all pressure types and triggers actions when thresholds are exceeded.

use super::economic::calculate_economic_pressure;
use super::legitimacy::{
    calculate_legitimacy_pressure, RecentEvents, RulerPersonality,
};
use super::military::calculate_military_pressure;
use super::population::calculate_population_pressure;
use super::types::{PressureType, PressureVector};
use crate::nations::Nation;
use crate::world::{ProvinceId, ProvinceStorage};
use bevy::prelude::*;

/// Update all pressure levels for nations
pub fn update_nation_pressures(
    mut nations_query: Query<(
        &mut Nation,
        &mut PressureVector,
        &crate::nations::OwnsTerritory,
    )>,
    territories_query: Query<&crate::nations::Territory>,
    province_storage: Res<ProvinceStorage>,
    time: Res<Time>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (nation, mut pressures, owns_territory) in &mut nations_query {
            // Get provinces from all territories owned by this nation
            let mut controlled_provinces = Vec::new();

            for &territory_entity in owns_territory.territories() {
                if let Ok(territory) = territories_query.get(territory_entity) {
                    // Collect provinces from this territory
                    for &province_id in &territory.provinces {
                        if let Some(&idx) = province_storage
                            .province_by_id
                            .get(&ProvinceId::new(province_id))
                        {
                            if let Some(province) = province_storage.provinces.get(idx) {
                                controlled_provinces.push(province);
                            }
                        }
                    }
                }
            }

            // Skip if no provinces (nation has no territories)
            if controlled_provinces.is_empty() {
                return;
            }

            // Calculate population pressures
            let pop_pressure = calculate_population_pressure(&nation, &controlled_provinces);
            pressures.set_pressure(
                PressureType::PopulationOvercrowding,
                pop_pressure.overcrowding,
            );
            pressures.set_pressure(
                PressureType::PopulationUnderpopulation,
                pop_pressure.underpopulation,
            );

            // Calculate economic pressures
            let econ_pressure = calculate_economic_pressure(&nation, &controlled_provinces);
            pressures.set_pressure(
                PressureType::EconomicStrain,
                econ_pressure.treasury_shortage,
            );

            // Calculate military pressures (simplified for now)
            let mil_pressure = calculate_military_pressure(
                &nation,
                &[], // Neighbor strengths - TODO: implement neighbor lookup
                controlled_provinces.len(),
                0, // Recent defeats - TODO: track in nation history
            );
            pressures.set_pressure(
                PressureType::MilitaryVulnerability,
                mil_pressure.military_weakness,
            );

            // Calculate legitimacy pressures
            let ruler_personality = determine_ruler_personality(&nation);
            let economic_health = 1.0 - econ_pressure.treasury_shortage.value();
            let recent_events = RecentEvents {
                victories: 0, // TODO: track victories
                defeats: 0,
                ruler_age: 45, // TODO: track ruler age
                has_heir: true,
                years_of_peace: 5, // TODO: track war/peace
                years_of_war: 0,
            };

            let legit_pressure = calculate_legitimacy_pressure(
                &nation,
                ruler_personality,
                economic_health,
                &recent_events,
            );
            pressures.set_pressure(
                PressureType::LegitimacyCrisis,
                legit_pressure.popular_discontent,
            );

            // Record trend and update time
            pressures.record_trend();
            pressures.time_since_resolution += time.delta_secs();
    }
}

/// Resolve pressure actions when thresholds are exceeded
pub fn resolve_pressure_actions(
    mut nations_query: Query<(&mut Nation, &mut PressureVector, Entity)>,
    commands: Commands,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (nation, mut pressures, entity) in &mut nations_query {
            // Check if enough time has passed since last resolution
            if pressures.time_since_resolution < 5.0 {
                return; // Only resolve every 5 seconds minimum
            }

            // Find the highest pressure
            if let Some((pressure_type, level)) = pressures.highest_pressure() {
                // Only act if pressure exceeds threshold
                if !level.is_moderate() {
                    return;
                }

                // Resolve based on pressure type
                match pressure_type {
                    PressureType::PopulationOvercrowding => {
                        info!(
                            "{}: Population pressure at {:.1}",
                            nation.name,
                            level.value()
                        );
                        // TODO: Trigger expansion
                    }
                    PressureType::EconomicStrain => {
                        info!("{}: Economic pressure at {:.1}", nation.name, level.value());
                        // TODO: Raise taxes or raid
                    }
                    PressureType::MilitaryVulnerability => {
                        info!("{}: Military pressure at {:.1}", nation.name, level.value());
                        // TODO: Build army or seek alliance
                    }
                    PressureType::LegitimacyCrisis => {
                        info!(
                            "{}: Legitimacy pressure at {:.1}",
                            nation.name,
                            level.value()
                        );
                        // TODO: Public works or reforms
                    }
                    _ => {}
                }

                // Reset resolution timer
                pressures.time_since_resolution = 0.0;
            }
    }
}

/// Apply gradual effects of pressures (stability, growth, etc.)
pub fn apply_pressure_effects(
    mut nations_query: Query<(&mut Nation, &PressureVector)>,
    time: Res<Time>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (mut nation, pressures) in &mut nations_query {
            let total_pressure = pressures.total_magnitude();

            // High pressure reduces stability
            if total_pressure > 2.0 {
                nation.stability -= 0.01 * time.delta_secs();
                nation.stability = nation.stability.clamp(0.0, 1.0);
            }

            // Low pressure increases stability (good governance)
            if total_pressure < 1.0 {
                nation.stability += 0.005 * time.delta_secs();
                nation.stability = nation.stability.clamp(0.0, 1.0);
            }

            // Worsening pressures reduce ruler support
            if pressures.is_worsening() {
                // Future: affect ruler legitimacy
            }
    }
}

/// Determine ruler personality from nation/house traits
pub fn determine_ruler_personality(nation: &Nation) -> RulerPersonality {
    // Simplified for now - would look at House traits
    if nation.military_strength > nation.treasury {
        RulerPersonality::Warlike
    } else if nation.stability > 0.7 {
        RulerPersonality::Peaceful
    } else {
        RulerPersonality::Ambitious
    }
}
