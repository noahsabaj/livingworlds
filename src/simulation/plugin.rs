//! Main plugin for the simulation module - AUTOMATION POWERED!

use bevy_plugin_builder::define_plugin;
use crate::resources::GameTime;
use crate::states::GameState;
use crate::world::Province;
use bevy::prelude::*;
use rayon::prelude::*;

// Import from sibling modules through super (gateway pattern)
use super::input::handle_time_controls;
use super::time::{
    advance_game_time, resume_from_pause_menu, track_year_changes, NewYearEvent,
    SimulationSpeedChanged,
};
use super::pressures::{
    update_nation_pressures, resolve_pressure_actions, apply_pressure_effects,
};

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

/// Plugin that manages the simulation time system using AUTOMATION FRAMEWORK
define_plugin!(SimulationPlugin {
    resources: [GameTime, PressureSystemTimer],

    events: [SimulationSpeedChanged, NewYearEvent],

    update: [
        // Time management systems - chained for precise ordering
        (handle_time_controls, advance_game_time, track_year_changes)
            .chain()
            .run_if(in_state(GameState::InGame)),
        // PERFORMANCE: Pressure systems run periodically, not every frame!
        run_pressure_systems_on_timer.run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [resume_from_pause_menu]
    }
});

/// Structure to hold nation pressure calculation data
#[derive(Clone)]
struct NationPressureData {
    entity: Entity,
    nation: crate::nations::Nation,
    pressures: crate::simulation::PressureVector,
    territory_entities: Vec<Entity>,
}

/// Run pressure systems on a timer with parallel processing
fn run_pressure_systems_on_timer(
    time: Res<Time>,
    mut timer: ResMut<PressureSystemTimer>,
    mut nations_query: Query<(Entity, &mut crate::nations::Nation, &mut crate::simulation::PressureVector, &crate::nations::OwnsTerritory), Without<crate::nations::Territory>>,
    territories_query: Query<&crate::nations::Territory>,
    province_storage: Res<crate::world::ProvinceStorage>,
    mut _commands: Commands,
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        // Collect nation data for parallel processing
        // Pre-allocate with known capacity to avoid reallocations
        let nation_count = nations_query.iter().len();
        let mut nation_data: Vec<NationPressureData> = Vec::with_capacity(nation_count);

        for (entity, nation, pressures, owns_territory) in nations_query.iter() {
            nation_data.push(NationPressureData {
                entity,
                nation: nation.clone(),
                pressures: pressures.clone(),
                territory_entities: owns_territory.territories().to_vec(),
            });
        }

        // Parallel calculate all pressures for all nations
        let calculated_pressures: Vec<(Entity, crate::simulation::PressureVector)> = nation_data
            .par_iter()
            .map(|data| {
                // Gather controlled provinces for this nation
                let mut controlled_provinces = Vec::new();

                for &territory_entity in &data.territory_entities {
                    if let Ok(territory) = territories_query.get(territory_entity) {
                        for &province_id in &territory.provinces {
                            if let Some(&idx) = province_storage.province_by_id.get(&crate::world::ProvinceId::new(province_id)) {
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
                    use crate::simulation::pressures::{
                        PressureType,
                        calculate_population_pressure,
                        calculate_economic_pressure,
                        calculate_military_pressure,
                        calculate_legitimacy_pressure,
                        determine_ruler_personality,
                        RecentEvents,
                    };

                    // Population pressures
                    let controlled_provinces_refs: Vec<&Province> = controlled_provinces.iter().collect();
                    let pop_pressure = calculate_population_pressure(&data.nation, &controlled_provinces_refs);
                    new_pressures.set_pressure(PressureType::PopulationOvercrowding, pop_pressure.overcrowding);
                    new_pressures.set_pressure(PressureType::PopulationUnderpopulation, pop_pressure.underpopulation);

                    // Economic pressures
                    let econ_pressure = calculate_economic_pressure(&data.nation, &controlled_provinces_refs);
                    new_pressures.set_pressure(PressureType::EconomicStrain, econ_pressure.treasury_shortage);

                    // Military pressures
                    let mil_pressure = calculate_military_pressure(
                        &data.nation,
                        &[], // TODO: Neighbor strengths
                        controlled_provinces.len(),
                        0, // TODO: Recent defeats
                    );
                    new_pressures.set_pressure(PressureType::MilitaryVulnerability, mil_pressure.military_weakness);

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
                    new_pressures.set_pressure(PressureType::LegitimacyCrisis, legit_pressure.popular_discontent);

                    // Record trend
                    new_pressures.record_trend();
                }

                (data.entity, new_pressures)
            })
            .collect();

        // Apply calculated pressures back to entities
        for (entity, new_pressures) in calculated_pressures {
            if let Ok((_, _, mut pressures, _)) = nations_query.get_mut(entity) {
                *pressures = new_pressures;
            }
        }

        debug!(
            "Parallel pressure calculation complete for {} nations",
            nation_data.len()
        );
    }
}

/// Determine ruler personality from nation/house traits
fn determine_ruler_personality(nation: &crate::nations::Nation) -> crate::simulation::pressures::RulerPersonality {
    use crate::simulation::pressures::RulerPersonality;

    if nation.military_strength > nation.treasury {
        RulerPersonality::Warlike
    } else if nation.stability > 0.7 {
        RulerPersonality::Peaceful
    } else {
        RulerPersonality::Ambitious
    }
}
