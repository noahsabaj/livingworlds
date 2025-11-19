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
use crate::world::{Province, ProvinceId, ProvinceStorage};
use bevy::prelude::*;
use rayon::prelude::*;

/// Update all pressure levels for nations
pub fn update_nation_pressures(
    mut nations_query: Query<(
        &mut Nation,
        &mut PressureVector,
        &crate::nations::OwnsTerritory,
        Option<&crate::nations::NationHistory>,
    )>,
    all_nations_query: Query<&Nation>,  // For neighbor strength lookup
    territories_query: Query<&crate::nations::Territory>,
    province_storage: Res<ProvinceStorage>,
    neighbor_cache: Res<crate::nations::NationNeighborCache>,  // For neighbor detection
    time: Res<Time>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (nation, mut pressures, owns_territory, history_opt) in &mut nations_query {
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

            // Calculate military pressures
            let neighbor_strengths = crate::nations::get_neighbor_strengths(
                &nation,
                &neighbor_cache,
                &all_nations_query,
            );
            let recent_defeats = history_opt
                .map(|h| h.calculate_weighted_recent_defeats())
                .unwrap_or(0.0);

            let mil_pressure = calculate_military_pressure(
                &nation,
                &neighbor_strengths,
                controlled_provinces.len(),
                recent_defeats,
            );
            pressures.set_pressure(
                PressureType::MilitaryVulnerability,
                mil_pressure.military_weakness,
            );

            // Calculate legitimacy pressures
            let ruler_personality = determine_ruler_personality(&nation);
            let economic_health = 1.0 - econ_pressure.treasury_shortage.value();
            let recent_events = if let Some(history) = history_opt {
                RecentEvents {
                    victories: history.total_victories,
                    defeats: history.total_defeats,
                    ruler_age: history.ruler.age,
                    has_heir: history.ruler.has_heir,
                    years_of_peace: history.years_at_peace,
                    years_of_war: history.years_at_war,
                }
            } else {
                // Fallback for nations without history (shouldn't happen in normal gameplay)
                RecentEvents {
                    victories: 0,
                    defeats: 0,
                    ruler_age: 45,
                    has_heir: true,
                    years_of_peace: 5,
                    years_of_war: 0,
                }
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
    mut nations_query: Query<(
        &mut Nation,
        &mut PressureVector,
        Option<&crate::nations::NationHistory>,
        Entity
    )>,
    province_storage: Res<ProvinceStorage>,
    mut messages: MessageWriter<crate::nations::NationActionEvent>,
    time: Res<Time>,
) {
    // NOTE: Bevy queries should not be manually parallelized with Rayon
    // Bevy has its own parallel scheduling system
    for (mut nation, mut pressures, history_opt, entity) in &mut nations_query {
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
                // Use the action system if history is available
                if let Some(history) = history_opt {
                    // Handle actions based on pressure type
                    match pressure_type {
                        PressureType::PopulationOvercrowding => {
                            crate::nations::handle_population_pressure(
                                &mut nation,
                                history,
                                level,
                                &province_storage,
                                &mut messages,
                            );
                        }
                        PressureType::EconomicStrain => {
                            crate::nations::handle_economic_pressure(
                                &mut nation,
                                history,
                                level,
                                &mut messages,
                            );
                        }
                        PressureType::MilitaryVulnerability => {
                            crate::nations::handle_military_pressure(
                                &mut nation,
                                history,
                                level,
                                &mut messages,
                            );
                        }
                        PressureType::LegitimacyCrisis => {
                            crate::nations::handle_legitimacy_pressure(
                                &mut nation,
                                history,
                                level,
                                &mut messages,
                            );
                        }
                        _ => {}
                    }
                } else {
                    // Fallback to simple logging if no history
                    match pressure_type {
                        PressureType::PopulationOvercrowding => {
                            info!(
                                "{}: Population pressure at {:.1} (no history)",
                                nation.name,
                                level.value()
                            );
                        }
                        PressureType::EconomicStrain => {
                            info!("{}: Economic pressure at {:.1} (no history)", nation.name, level.value());
                        }
                        PressureType::MilitaryVulnerability => {
                            info!("{}: Military pressure at {:.1} (no history)", nation.name, level.value());
                        }
                        PressureType::LegitimacyCrisis => {
                            info!(
                                "{}: Legitimacy pressure at {:.1} (no history)",
                                nation.name,
                                level.value()
                            );
                        }
                        _ => {}
                    }
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

/// Timer for pressure system updates
#[derive(Resource)]
pub struct PressureSystemTimer {
    pub timer: Timer,
}

impl Default for PressureSystemTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

/// Structure to hold nation pressure calculation data
#[derive(Clone)]
struct NationPressureData {
    entity: Entity,
    nation: Nation,
    pressures: PressureVector,
    territory_entities: Vec<Entity>,
    neighbor_strengths: Vec<f32>,  // Pre-computed neighbor strengths
    recent_defeats: f32,            // Pre-computed weighted defeats
}

/// Run pressure systems on a timer with parallel processing
pub fn run_pressure_systems_on_timer(
    time: Res<Time>,
    mut timer: ResMut<PressureSystemTimer>,
    mut nations_query: Query<
        (
            Entity,
            &mut Nation,
            &mut PressureVector,
            &crate::nations::OwnsTerritory,
            Option<&crate::nations::NationHistory>,  // For defeat tracking
        ),
        Without<crate::nations::Territory>,
    >,
    all_nations_query: Query<&Nation>,  // For neighbor strength lookup
    territories_query: Query<&crate::nations::Territory>,
    province_storage: Res<ProvinceStorage>,
    neighbor_cache: Res<crate::nations::NationNeighborCache>,  // For neighbor detection
    mut _commands: Commands,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        // Collect nation data for parallel processing
        // Pre-allocate with known capacity to avoid reallocations
        let nation_count = nations_query.iter().len();
        let mut nation_data: Vec<NationPressureData> = Vec::with_capacity(nation_count);

        for (entity, nation, pressures, owns_territory, history_opt) in nations_query.iter() {
            // Pre-compute neighbor strengths (can't do in parallel - needs query access)
            let neighbor_strengths = crate::nations::get_neighbor_strengths(
                &nation,
                &neighbor_cache,
                &all_nations_query,
            );

            // Pre-compute weighted recent defeats
            let recent_defeats = history_opt
                .map(|h| h.calculate_weighted_recent_defeats())
                .unwrap_or(0.0);

            nation_data.push(NationPressureData {
                entity,
                nation: nation.clone(),
                pressures: pressures.clone(),
                territory_entities: owns_territory.territories().to_vec(),
                neighbor_strengths,  // Pre-computed
                recent_defeats,       // Pre-computed
            });
        }

        // Parallel calculate all pressures for all nations
        let calculated_pressures: Vec<(Entity, PressureVector)> = nation_data
            .par_iter()
            .map(|data| {
                // Gather controlled provinces for this nation
                let mut controlled_provinces = Vec::new();

                for &territory_entity in &data.territory_entities {
                    if let Ok(territory) = territories_query.get(territory_entity) {
                        for &province_id in &territory.provinces {
                            if let Some(&idx) = province_storage
                                .province_by_id
                                .get(&ProvinceId::new(province_id))
                            {
                                if let Some(province) = province_storage.provinces.get(idx) {
                                    controlled_provinces.push(province.clone());
                                }
                            }
                        }
                    }
                }

                // Calculate all pressure types in parallel
                let mut new_pressures = data.pressures.clone();

                if !controlled_provinces.is_empty() {
                    // Population pressures
                    let controlled_provinces_refs: Vec<&Province> =
                        controlled_provinces.iter().collect();
                    let pop_pressure =
                        calculate_population_pressure(&data.nation, &controlled_provinces_refs);
                    new_pressures.set_pressure(
                        PressureType::PopulationOvercrowding,
                        pop_pressure.overcrowding,
                    );
                    new_pressures.set_pressure(
                        PressureType::PopulationUnderpopulation,
                        pop_pressure.underpopulation,
                    );

                    // Economic pressures
                    let econ_pressure =
                        calculate_economic_pressure(&data.nation, &controlled_provinces_refs);
                    new_pressures.set_pressure(
                        PressureType::EconomicStrain,
                        econ_pressure.treasury_shortage,
                    );

                    // Military pressures
                    let mil_pressure = calculate_military_pressure(
                        &data.nation,
                        &data.neighbor_strengths,  // Pre-computed from neighbor cache
                        controlled_provinces.len(),
                        data.recent_defeats,        // Pre-computed weighted defeats
                    );
                    new_pressures.set_pressure(
                        PressureType::MilitaryVulnerability,
                        mil_pressure.military_weakness,
                    );

                    // Legitimacy pressures
                    let ruler_personality = determine_ruler_personality(&data.nation);
                    let economic_health = 1.0 - econ_pressure.treasury_shortage.value();
                    let recent_events = RecentEvents {
                        victories: 0,
                        defeats: 0,
                        ruler_age: 45,
                        has_heir: true,
                        years_of_peace: 5,
                        years_of_war: 0,
                    };

                    let legit_pressure = calculate_legitimacy_pressure(
                        &data.nation,
                        ruler_personality,
                        economic_health,
                        &recent_events,
                    );
                    new_pressures.set_pressure(
                        PressureType::LegitimacyCrisis,
                        legit_pressure.popular_discontent,
                    );

                    // Record trend
                    new_pressures.record_trend();
                }

                (data.entity, new_pressures)
            })
            .collect();

        // Apply calculated pressures back to entities
        for (entity, new_pressures) in calculated_pressures {
            if let Ok((_, _, mut pressures, _, _)) = nations_query.get_mut(entity) {
                *pressures = new_pressures;
            }
        }

        debug!(
            "Parallel pressure calculation complete for {} nations",
            nation_data.len()
        );
    }
}
