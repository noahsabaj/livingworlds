//! Core pressure system that drives nation behavior
//!
//! This system updates all pressure types and triggers actions when thresholds are exceeded.

use super::economic::calculate_economic_pressure;
use super::legitimacy::{
    calculate_legitimacy_pressure, RecentEvents, RulerPersonality,
};
use super::military::calculate_military_pressure;
use super::population::calculate_population_pressure;
use super::types::{PressureLevel, PressureType, PressureVector};
use crate::nations::Nation;
use crate::world::{Province, ProvinceId, ProvinceStorage};
use bevy::prelude::*;

// ================================================================================================
// HELPER FUNCTIONS - Extracted for readability and reuse
// ================================================================================================

/// Select the best raid target from neighboring nations
/// Higher score = better target (vulnerable + wealthy)
fn select_raid_target(
    neighbor_entities: &[Entity],
    neighbors_query: &Query<(Entity, &Nation)>,
) -> Option<Entity> {
    neighbor_entities
        .iter()
        .filter_map(|&neighbor_entity| {
            neighbors_query.get(neighbor_entity).ok().map(|(e, nation)| {
                let vulnerability = 30.0 * (1.0 - nation.military_strength.min(1.0));
                let wealth = 25.0 * (nation.treasury / 1000.0).min(1.0);
                let score = vulnerability + wealth;
                (e, score)
            })
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(e, _)| e)
}

/// Select the best ally target from neighboring nations
/// Higher score = better ally (strong + stable + diplomatic)
fn select_ally_target(
    neighbor_entities: &[Entity],
    neighbors_query: &Query<(Entity, &Nation)>,
) -> Option<Entity> {
    neighbor_entities
        .iter()
        .filter_map(|&neighbor_entity| {
            neighbors_query.get(neighbor_entity).ok().map(|(e, nation)| {
                let strength = 40.0 * nation.military_strength.min(1.0);
                let stability = 30.0 * nation.stability;
                let diplomatic = 30.0 * nation.personality.diplomacy;
                let score = strength + stability + diplomatic;
                (e, score)
            })
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(e, _)| e)
}

/// Collect all provinces controlled by a nation through its territories
fn collect_controlled_provinces(
    owns_territory: &crate::nations::OwnsTerritory,
    territories_query: &Query<&crate::nations::Territory>,
    province_storage: &ProvinceStorage,
) -> Vec<Province> {
    let mut controlled_provinces = Vec::new();

    for &territory_entity in owns_territory.territories() {
        if let Ok(territory) = territories_query.get(territory_entity) {
            for &province_id in &territory.provinces {
                if let Some(&idx) = province_storage.province_by_id.get(&ProvinceId::new(province_id)) {
                    if let Some(province) = province_storage.provinces.get(idx) {
                        controlled_provinces.push(province.clone());
                    }
                }
            }
        }
    }

    controlled_provinces
}

/// Build RecentEvents struct from nation history (or defaults if no history)
fn build_recent_events(history_opt: Option<&crate::nations::NationHistory>) -> RecentEvents {
    if let Some(history) = history_opt {
        RecentEvents {
            victories: history.total_victories,
            defeats: history.total_defeats,
            ruler_age: history.ruler.age,
            has_heir: history.ruler.has_heir,
            years_of_peace: history.years_at_peace,
            years_of_war: history.years_at_war,
        }
    } else {
        RecentEvents {
            victories: 0,
            defeats: 0,
            ruler_age: 45,
            has_heir: true,
            years_of_peace: 5,
            years_of_war: 0,
        }
    }
}

/// Pressure calculation results for a single nation
struct PressureResults {
    population_overcrowding: f32,
    population_underpopulation: f32,
    economic_strain: f32,
    military_vulnerability: f32,
    legitimacy_crisis: f32,
}

/// Calculate all pressure types for a nation
fn calculate_all_pressures(
    nation: &Nation,
    controlled_provinces: &[Province],
    neighbor_strengths: &[f32],
    recent_defeats: f32,
    history_opt: Option<&crate::nations::NationHistory>,
) -> PressureResults {
    let controlled_provinces_refs: Vec<&Province> = controlled_provinces.iter().collect();

    // Population pressure
    let pop_pressure = calculate_population_pressure(nation, &controlled_provinces_refs);

    // Economic pressure
    let econ_pressure = calculate_economic_pressure(nation, &controlled_provinces_refs);

    // Military pressure
    let mil_pressure = calculate_military_pressure(
        nation,
        neighbor_strengths,
        controlled_provinces.len(),
        recent_defeats,
    );

    // Legitimacy pressure
    let ruler_personality = determine_ruler_personality(nation);
    let economic_health = 1.0 - econ_pressure.treasury_shortage.value();
    let recent_events = build_recent_events(history_opt);
    let legit_pressure = calculate_legitimacy_pressure(
        nation,
        ruler_personality,
        economic_health,
        &recent_events,
    );

    PressureResults {
        population_overcrowding: pop_pressure.overcrowding.value(),
        population_underpopulation: pop_pressure.underpopulation.value(),
        economic_strain: econ_pressure.treasury_shortage.value(),
        military_vulnerability: mil_pressure.military_weakness.value(),
        legitimacy_crisis: legit_pressure.popular_discontent.value(),
    }
}

// ================================================================================================
// SYSTEMS
// ================================================================================================

/// Resolve pressure actions when thresholds are exceeded
pub fn resolve_pressure_actions(
    mut param_set: ParamSet<(
        // P0: Mutable query for nations being processed
        Query<(
            &mut Nation,
            &mut PressureVector,
            Option<&crate::nations::NationHistory>,
            Option<&crate::nations::LandNeighbors>,
            Entity
        )>,
        // P1: Immutable query for neighbor lookups (raid target selection)
        Query<(Entity, &Nation)>,
    )>,
    province_storage: Res<ProvinceStorage>,
    mut messages: MessageWriter<crate::nations::NationActionEvent>,
    time: Res<Time>,
) {
    // First pass: collect nations that need action and their data
    let mut actions_to_take: Vec<(Entity, PressureType, PressureLevel, String, Option<Vec<Entity>>)> = Vec::new();

    {
        let nations_query = param_set.p0();
        for (nation, pressures, history_opt, neighbors_opt, entity) in nations_query.iter() {
            // Check if enough time has passed since last resolution
            if pressures.time_since_resolution < 5.0 {
                continue;
            }

            // Find the highest pressure
            if let Some((pressure_type, level)) = pressures.highest_pressure() {
                // Only act if pressure exceeds threshold
                if !level.is_moderate() {
                    continue;
                }

                // Only process if history is available
                if history_opt.is_some() {
                    let neighbor_entities = neighbors_opt.map(|n| n.neighbors().to_vec());
                    actions_to_take.push((entity, pressure_type, level, nation.name.clone(), neighbor_entities));
                }
            }
        }
    }

    // Second pass: process actions with access to neighbor data for target selection
    for (entity, pressure_type, level, _nation_name, neighbor_entities) in actions_to_take {
        // Get the immutable query for neighbor lookups
        let neighbors_query = param_set.p1();

        // Find targets using extracted helpers
        let raid_target = neighbor_entities
            .as_ref()
            .and_then(|entities| select_raid_target(entities, &neighbors_query));

        let ally_target = neighbor_entities
            .as_ref()
            .and_then(|entities| select_ally_target(entities, &neighbors_query));

        // Now get mutable access for actual updates
        let _ = neighbors_query;
        let mut nations_query = param_set.p0();

        if let Ok((mut nation, mut pressures, history_opt, _neighbors, _)) = nations_query.get_mut(entity) {
            if let Some(history) = history_opt {
                match pressure_type {
                    PressureType::PopulationOvercrowding => {
                        crate::nations::handle_population_pressure(
                            entity,
                            &mut nation,
                            history,
                            level,
                            &province_storage,
                            &mut messages,
                        );
                    }
                    PressureType::EconomicStrain => {
                        // Handle economic pressure with raid target
                        crate::nations::handle_economic_pressure(
                            entity,
                            &mut nation,
                            history,
                            level,
                            raid_target,
                            &mut messages,
                        );
                    }
                    PressureType::MilitaryVulnerability => {
                        // Handle military pressure with ally target
                        crate::nations::handle_military_pressure(
                            entity,
                            &mut nation,
                            history,
                            level,
                            ally_target,
                            &mut messages,
                        );
                    }
                    PressureType::LegitimacyCrisis => {
                        crate::nations::handle_legitimacy_pressure(
                            entity,
                            &mut nation,
                            history,
                            level,
                            &mut messages,
                        );
                    }
                    _ => {}
                }

                pressures.time_since_resolution = 0.0;
            }
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

/// Run pressure systems on a timer (every 2 seconds)
/// Updates all pressure levels for nations based on population, economy, military, and legitimacy
pub fn run_pressure_systems_on_timer(
    time: Res<Time>,
    mut timer: ResMut<PressureSystemTimer>,
    mut param_set: ParamSet<(
        // P0: Mutable query for nations
        Query<(
            Entity,
            &mut Nation,
            &mut PressureVector,
            &crate::nations::OwnsTerritory,
            Option<&crate::nations::NationHistory>,
        ), Without<crate::nations::Territory>>,
        // P1: Immutable query for neighbor lookups
        Query<(&Nation, Option<&crate::nations::relationships::LandNeighbors>, Option<&crate::nations::relationships::NavalNeighbors>)>,
    )>,
    territories_query: Query<&crate::nations::Territory>,
    province_storage: Res<ProvinceStorage>,
    mut _commands: Commands,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.finished() {
        return;
    }

    // Phase 1a: Collect entity IDs for neighbor lookup
    let mut nation_data_for_neighbors: Vec<Entity> = Vec::new();
    for (entity, _, _, _, _) in param_set.p0().iter() {
        nation_data_for_neighbors.push(entity);
    }
    
    // Phase 1b: Get neighbor strengths (requires p1)
    let neighbor_strengths_map: std::collections::HashMap<Entity, Vec<f32>> = nation_data_for_neighbors
        .iter()
        .map(|&entity| {
            let strengths = crate::nations::get_neighbor_strengths(entity, &param_set.p1());
            (entity, strengths)
        })
        .collect();
    
    // Phase 1c: Calculate all pressure updates using extracted helpers
    let mut pressure_updates = Vec::new();

    for (entity, nation, _, owns_territory, history_opt) in param_set.p0().iter() {
        // Get neighbor strengths from pre-calculated map
        let neighbor_strengths = neighbor_strengths_map.get(&entity).cloned().unwrap_or_default();

        let recent_defeats = history_opt
            .map(|h| h.calculate_weighted_recent_defeats())
            .unwrap_or(0.0);

        // Gather controlled provinces using helper
        let controlled_provinces = collect_controlled_provinces(
            owns_territory,
            &territories_query,
            &province_storage,
        );

        if controlled_provinces.is_empty() {
            continue;
        }

        // Calculate all pressures using helper
        let results = calculate_all_pressures(
            &nation,
            &controlled_provinces,
            &neighbor_strengths,
            recent_defeats,
            history_opt,
        );

        pressure_updates.push((
            entity,
            results.population_overcrowding,
            results.population_underpopulation,
            results.economic_strain,
            results.military_vulnerability,
            results.legitimacy_crisis,
        ));
    }
    
    // Phase 2: Apply all updates
    for (entity, pop_over, pop_under, econ, mil, legit) in pressure_updates {
        if let Ok((_, _, mut pressures, _, _)) = param_set.p0().get_mut(entity) {
            pressures.set_pressure(PressureType::PopulationOvercrowding, PressureLevel(pop_over));
            pressures.set_pressure(PressureType::PopulationUnderpopulation, PressureLevel(pop_under));
            pressures.set_pressure(PressureType::EconomicStrain, PressureLevel(econ));
            pressures.set_pressure(PressureType::MilitaryVulnerability, PressureLevel(mil));
            pressures.set_pressure(PressureType::LegitimacyCrisis, PressureLevel(legit));
            pressures.record_trend();
        }
    }
}
