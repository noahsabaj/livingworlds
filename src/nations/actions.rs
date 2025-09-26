//! Nation action system for responding to pressures
//!
//! Implements concrete actions that nations take when pressures exceed thresholds,
//! including expansion, taxation, military recruitment, and reforms.

use bevy::prelude::*;
use crate::simulation::{PressureType, PressureLevel};
use crate::world::{ProvinceId, ProvinceStorage};
use super::{Nation, NationId, NationHistory};

/// Actions that nations can take to relieve pressures
#[derive(Debug, Clone, Event)]
pub enum NationActionEvent {
    /// Nation attempts to expand into neighboring territory
    ExpansionAttempt {
        nation_id: NationId,
        nation_name: String,
        target_provinces: Vec<ProvinceId>,
        pressure_level: f32,
    },

    /// Nation increases taxes to address economic pressure
    TaxIncrease {
        nation_id: NationId,
        nation_name: String,
        old_rate: f32,
        new_rate: f32,
        pressure_level: f32,
    },

    /// Nation attempts to raid neighbors for resources
    RaidAttempt {
        nation_id: NationId,
        nation_name: String,
        target_nation: NationId,
        pressure_level: f32,
    },

    /// Nation recruits additional military forces
    MilitaryRecruitment {
        nation_id: NationId,
        nation_name: String,
        units_recruited: u32,
        pressure_level: f32,
    },

    /// Nation seeks alliance for protection
    AllianceSeek {
        nation_id: NationId,
        nation_name: String,
        target_nation: Option<NationId>,
        pressure_level: f32,
    },

    /// Nation implements reforms to improve legitimacy
    ReformImplementation {
        nation_id: NationId,
        nation_name: String,
        reform_type: ReformType,
        pressure_level: f32,
    },

    /// Nation builds public works to improve stability
    PublicWorks {
        nation_id: NationId,
        nation_name: String,
        project_type: PublicWorkType,
        pressure_level: f32,
    },
}

/// Types of reforms nations can implement
#[derive(Debug, Clone, Copy)]
pub enum ReformType {
    TaxReform,
    MilitaryReform,
    AdministrativeReform,
    LandReform,
    ReligiousReform,
}

/// Types of public works projects
#[derive(Debug, Clone, Copy)]
pub enum PublicWorkType {
    Monument,
    Infrastructure,
    Temple,
    Market,
    Fortification,
}

/// Queued action waiting to be executed
#[derive(Component, Debug, Clone)]
pub struct NationAction {
    pub action_type: ActionType,
    pub priority: f32,
    pub time_to_execute: f32,
}

/// Types of actions that can be queued
#[derive(Debug, Clone)]
pub enum ActionType {
    Expand(Vec<ProvinceId>),
    RaiseTaxes(f32),
    Raid(NationId),
    RecruitArmy(u32),
    SeekAlliance,
    ImplementReform(ReformType),
    BuildPublicWork(PublicWorkType),
}

/// Resolve nation actions based on pressures and history
pub fn resolve_nation_actions(
    mut nations_query: Query<(
        &mut Nation,
        &mut crate::simulation::PressureVector,
        &NationHistory,
        Entity
    )>,
    province_storage: Res<ProvinceStorage>,
    mut events: EventWriter<NationActionEvent>,
    time: Res<Time>,
) {
    for (mut nation, mut pressures, history, entity) in &mut nations_query {
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

            // Resolve based on pressure type and nation history
            match pressure_type {
                PressureType::PopulationOvercrowding => {
                    handle_population_pressure(
                        &mut nation,
                        &history,
                        level,
                        &province_storage,
                        &mut events,
                    );
                }
                PressureType::EconomicStrain => {
                    handle_economic_pressure(
                        &mut nation,
                        &history,
                        level,
                        &mut events,
                    );
                }
                PressureType::MilitaryVulnerability => {
                    handle_military_pressure(
                        &mut nation,
                        &history,
                        level,
                        &mut events,
                    );
                }
                PressureType::LegitimacyCrisis => {
                    handle_legitimacy_pressure(
                        &mut nation,
                        &history,
                        level,
                        &mut events,
                    );
                }
                _ => {}
            }

            // Reset pressure resolution timer
            pressures.time_since_resolution = 0.0;
        }
    }
}

/// Handle population pressure with expansion attempts
pub fn handle_population_pressure(
    nation: &mut Nation,
    history: &NationHistory,
    pressure: PressureLevel,
    province_storage: &ProvinceStorage,
    events: &mut EventWriter<NationActionEvent>,
) {
    info!(
        "{}: Population pressure at {:.1} - attempting expansion",
        nation.name, pressure.value()
    );

    // Determine expansion aggressiveness based on history and personality
    let expansion_desire = calculate_expansion_desire(nation, history, pressure);

    if expansion_desire > 0.5 {
        // Find suitable target provinces
        let targets = find_expansion_targets(nation, province_storage);

        if !targets.is_empty() {
            events.send(NationActionEvent::ExpansionAttempt {
                nation_id: nation.id,
                nation_name: nation.name.clone(),
                target_provinces: targets,
                pressure_level: pressure.value(),
            });

            info!(
                "{} initiates expansion due to population pressure",
                nation.name
            );
        }
    }
}

/// Handle economic pressure with tax increases or raids
pub fn handle_economic_pressure(
    nation: &mut Nation,
    history: &NationHistory,
    pressure: PressureLevel,
    events: &mut EventWriter<NationActionEvent>,
) {
    info!(
        "{}: Economic pressure at {:.1}",
        nation.name, pressure.value()
    );

    // Decide between raising taxes or raiding based on personality and history
    let is_aggressive = nation.personality.aggression > 0.3;
    let has_military_strength = nation.military_strength > 0.6;
    let recent_defeats = history.has_recent_defeats();

    if is_aggressive && has_military_strength && !recent_defeats {
        // Attempt to raid neighbors
        events.send(NationActionEvent::RaidAttempt {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            target_nation: NationId::new(0), // TODO: Find actual target
            pressure_level: pressure.value(),
        });

        info!("{} considers raiding neighbors for resources", nation.name);
    } else {
        // Raise taxes
        let old_rate: f32 = 0.2; // TODO: Track actual tax rate
        let new_rate = (old_rate * 1.2).min(0.5); // 20% increase, max 50%

        events.send(NationActionEvent::TaxIncrease {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            old_rate,
            new_rate,
            pressure_level: pressure.value(),
        });

        nation.treasury *= 1.1; // Temporary treasury boost
        info!("{} raises taxes to address economic strain", nation.name);
    }
}

/// Handle military pressure with recruitment or alliances
pub fn handle_military_pressure(
    nation: &mut Nation,
    history: &NationHistory,
    pressure: PressureLevel,
    events: &mut EventWriter<NationActionEvent>,
) {
    info!(
        "{}: Military pressure at {:.1}",
        nation.name, pressure.value()
    );

    // Decide between building army or seeking alliance
    let can_afford_army = nation.treasury > 5000.0;
    let is_diplomatic = nation.personality.diplomacy > 0.3;

    if can_afford_army && !is_diplomatic {
        // Recruit additional forces
        let units_to_recruit = (pressure.value() * 10.0) as u32;

        events.send(NationActionEvent::MilitaryRecruitment {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            units_recruited: units_to_recruit,
            pressure_level: pressure.value(),
        });

        nation.military_strength += 0.1;
        nation.treasury -= units_to_recruit as f32 * 100.0;

        info!("{} recruits {} new military units", nation.name, units_to_recruit);
    } else {
        // Seek alliance for protection
        events.send(NationActionEvent::AllianceSeek {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            target_nation: None, // TODO: Find suitable ally
            pressure_level: pressure.value(),
        });

        info!("{} seeks alliances for military protection", nation.name);
    }
}

/// Handle legitimacy pressure with reforms or public works
pub fn handle_legitimacy_pressure(
    nation: &mut Nation,
    history: &NationHistory,
    pressure: PressureLevel,
    events: &mut EventWriter<NationActionEvent>,
) {
    info!(
        "{}: Legitimacy pressure at {:.1}",
        nation.name, pressure.value()
    );

    // Decide between reforms and public works based on ruler personality
    let is_reformist = history.ruler.personality.administrative > 0.3;
    let can_afford_projects = nation.treasury > 3000.0;

    if is_reformist {
        // Implement reforms
        let reform = choose_reform_type(nation, history);

        events.send(NationActionEvent::ReformImplementation {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            reform_type: reform,
            pressure_level: pressure.value(),
        });

        nation.stability += 0.05;
        info!("{} implements {:?} to improve legitimacy", nation.name, reform);
    } else if can_afford_projects {
        // Build public works
        let project = choose_public_work(nation, history);

        events.send(NationActionEvent::PublicWorks {
            nation_id: nation.id,
            nation_name: nation.name.clone(),
            project_type: project,
            pressure_level: pressure.value(),
        });

        nation.treasury -= 1000.0;
        nation.stability += 0.03;
        info!("{} builds {:?} to boost public support", nation.name, project);
    }
}

/// Calculate how much a nation wants to expand
fn calculate_expansion_desire(
    nation: &Nation,
    history: &NationHistory,
    pressure: PressureLevel,
) -> f32 {
    let mut desire = pressure.value();

    // Personality factors
    desire += nation.personality.expansionism * 0.3;
    desire += nation.personality.aggression * 0.2;

    // Historical factors
    if history.recent_victory_rate() > 0.7 {
        desire += 0.2; // Recent victories embolden expansion
    }
    if history.has_recent_defeats() {
        desire -= 0.3; // Recent defeats discourage expansion
    }
    if history.is_long_peace() {
        desire -= 0.1; // Long peace makes nations less expansionist
    }

    // Ruler factors
    desire += history.ruler.personality.ambitious * 0.2;
    desire += history.ruler.personality.martial * 0.1;

    desire.clamp(0.0, 1.0)
}

/// Find suitable provinces for expansion
fn find_expansion_targets(
    nation: &Nation,
    province_storage: &ProvinceStorage,
) -> Vec<ProvinceId> {
    // TODO: Implement proper neighbor finding and target selection
    // For now, return empty vector
    Vec::new()
}

/// Choose appropriate reform type based on nation state
fn choose_reform_type(nation: &Nation, history: &NationHistory) -> ReformType {
    // Choose reform based on what's most needed
    if nation.military_strength < 0.3 {
        ReformType::MilitaryReform
    } else if nation.treasury < 1000.0 {
        ReformType::TaxReform
    } else if history.ruler.personality.administrative > 0.5 {
        ReformType::AdministrativeReform
    } else {
        ReformType::LandReform
    }
}

/// Choose appropriate public work based on nation needs
fn choose_public_work(nation: &Nation, history: &NationHistory) -> PublicWorkType {
    if nation.military_strength < 0.4 {
        PublicWorkType::Fortification
    } else if nation.treasury < 2000.0 {
        PublicWorkType::Market
    } else if history.ruler.legitimacy < 0.5 {
        PublicWorkType::Monument
    } else {
        PublicWorkType::Infrastructure
    }
}