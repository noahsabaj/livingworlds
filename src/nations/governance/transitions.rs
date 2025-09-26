//! Government transition and evolution systems
//!
//! This module handles how governments change over time through revolution,
//! reform, coups, and other political transitions.

use bevy::prelude::*;
use rand::Rng;

use super::types::{Governance, GovernmentType, GovernmentCategory, PoliticalPressure, GovernanceSettings};

/// Event for government transitions
#[derive(Event, Debug, Clone)]
pub struct GovernmentTransition {
    pub nation_entity: Entity,
    pub from_government: GovernmentType,
    pub to_government: GovernmentType,
    pub transition_type: TransitionType,
    pub peaceful: bool,
}

/// How the government transition occurs
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Reflect)]
pub enum TransitionType {
    Revolution,        // Violent overthrow
    Reform,           // Peaceful reform
    Coup,             // Military takeover
    Collapse,         // Government falls apart
    Election,         // Democratic transition
    Succession,       // Death of ruler
    ForeignImposed,   // Imposed by conqueror
    PopularUprising,  // Mass movement
    EliteConspiracy,  // Palace coup
}

/// Check if nations should transition governments
pub fn check_for_transitions(
    settings: Res<GovernanceSettings>,
    mut nations: Query<(
        Entity,
        &crate::nations::Nation,
        &mut Governance,
        &PoliticalPressure,
    )>,
    mut events: EventWriter<GovernmentTransition>,
) {
    if !settings.allow_revolutions {
        return;
    }

    for (entity, nation, mut governance, pressure) in &mut nations {
        // Calculate total pressure
        let total_pressure = calculate_total_pressure(pressure, &governance);

        // Update reform pressure
        governance.reform_pressure = total_pressure;

        // Check if pressure exceeds threshold
        if total_pressure > settings.revolution_threshold {
            // Determine transition type
            let transition_type = determine_transition_type(&governance, pressure);

            // Determine new government
            let new_government = determine_new_government(
                governance.government_type,
                transition_type,
                pressure,
            );

            // Check if transition is peaceful
            let peaceful = rand::thread_rng().gen::<f32>() < settings.peaceful_transition_chance
                || matches!(transition_type, TransitionType::Reform | TransitionType::Election);

            // Send transition event
            events.send(GovernmentTransition {
                nation_entity: entity,
                from_government: governance.government_type,
                to_government: new_government,
                transition_type,
                peaceful,
            });
        }
    }
}

/// Calculate total political pressure
fn calculate_total_pressure(pressure: &PoliticalPressure, governance: &Governance) -> f32 {
    let mechanics = governance.government_type.mechanics();

    // Base pressures
    let economic = pressure.economic_crisis * 2.0;
    let military = pressure.military_defeat * 1.5;
    let cultural = pressure.cultural_shift * 1.0;
    let external = pressure.external_influence * 1.2;
    let tech = pressure.technological_change * 0.8;
    let religious = pressure.religious_fervor * 1.0;
    let revolutionary = pressure.revolutionary_ideas * 1.5;

    // Sum and apply resistance
    let raw_pressure = economic + military + cultural + external + tech + religious + revolutionary;
    raw_pressure * (1.0 - governance.tradition_strength * mechanics.reform_resistance)
}

/// Determine what type of transition will occur
fn determine_transition_type(
    governance: &Governance,
    pressure: &PoliticalPressure,
) -> TransitionType {
    let mut rng = rand::thread_rng();

    // Check dominant pressure source
    if pressure.military_defeat > 0.6 {
        TransitionType::Coup
    } else if pressure.revolutionary_ideas > 0.7 {
        TransitionType::Revolution
    } else if pressure.economic_crisis > 0.8 {
        TransitionType::Collapse
    } else if pressure.external_influence > 0.7 {
        TransitionType::ForeignImposed
    } else if governance.government_type.category() == GovernmentCategory::Democratic {
        TransitionType::Election
    } else if rng.gen::<f32>() < 0.3 {
        TransitionType::Reform
    } else {
        TransitionType::PopularUprising
    }
}

/// Determine what government type to transition to
fn determine_new_government(
    current: GovernmentType,
    transition_type: TransitionType,
    pressure: &PoliticalPressure,
) -> GovernmentType {
    use GovernmentType::*;
    let mut rng = rand::thread_rng();

    match transition_type {
        TransitionType::Revolution => {
            // Revolutionary governments based on ideas
            if pressure.revolutionary_ideas > 0.8 {
                match rng.gen_range(0..5) {
                    0 => AnarchoSyndicalism,
                    1 => VanguardCommunism,
                    2 => CouncilCommunism,
                    3 => DemocraticSocialism,
                    _ => Syndicalism,
                }
            } else {
                // General revolution
                match current.category() {
                    GovernmentCategory::Traditional => PresidentialRepublic,
                    GovernmentCategory::Democratic => match rng.gen_range(0..3) {
                        0 => VanguardCommunism,
                        1 => FascistState,
                        _ => MilitaryJunta,
                    },
                    _ => ParliamentaryDemocracy,
                }
            }
        },

        TransitionType::Coup => {
            // Military takeover
            match rng.gen_range(0..4) {
                0 => MilitaryJunta,
                1 => Stratocracy,
                2 => FascistState,
                _ => PoliceState,
            }
        },

        TransitionType::Collapse => {
            // Government falls apart
            match current.category() {
                GovernmentCategory::Traditional => TribalFederation,
                GovernmentCategory::Economic => Kleptocracy,
                _ => match rng.gen_range(0..3) {
                    0 => Warlordism,
                    1 => ProvisionalGovernment,
                    _ => TribalFederation,
                },
            }
        },

        TransitionType::Reform => {
            // Gradual reform to similar government
            match current {
                AbsoluteMonarchy => ConstitutionalMonarchy,
                ConstitutionalMonarchy => ParliamentaryDemocracy,
                Feudalism => AbsoluteMonarchy,
                MilitaryJunta => PresidentialRepublic,
                VanguardCommunism => StateSocialism,
                StateSocialism => MarketSocialism,
                MarketSocialism => DemocraticSocialism,
                FascistState => OnePartyState,
                OnePartyState => PresidentialRepublic,
                Theocracy => ConstitutionalMonarchy,
                _ => ParliamentaryDemocracy,
            }
        },

        TransitionType::Election => {
            // Democratic transition
            match current {
                ParliamentaryDemocracy => match rng.gen_range(0..4) {
                    0 => DemocraticSocialism,
                    1 => PresidentialRepublic,
                    2 => FederalRepublic,
                    _ => ParliamentaryDemocracy, // Re-elected
                },
                PresidentialRepublic => match rng.gen_range(0..3) {
                    0 => ParliamentaryDemocracy,
                    1 => DemocraticSocialism,
                    _ => PresidentialRepublic,
                },
                _ => ParliamentaryDemocracy,
            }
        },

        TransitionType::ForeignImposed => {
            // Imposed by external power - random based on era
            match rng.gen_range(0..5) {
                0 => PresidentialRepublic,
                1 => ParliamentaryDemocracy,
                2 => MilitaryJunta,
                3 => ProvisionalGovernment,
                _ => current, // Puppet government of same type
            }
        },

        TransitionType::PopularUprising => {
            // People's movement
            if pressure.economic_crisis > 0.6 {
                match rng.gen_range(0..3) {
                    0 => DemocraticSocialism,
                    1 => Syndicalism,
                    _ => MarketSocialism,
                }
            } else {
                ParliamentaryDemocracy
            }
        },

        TransitionType::Succession => {
            // Usually same government with new ruler
            current
        },

        TransitionType::EliteConspiracy => {
            // Palace coup - usually oligarchy
            match rng.gen_range(0..3) {
                0 => Oligarchy,
                1 => Plutocracy,
                _ => MilitaryJunta,
            }
        },
    }
}

/// Process government transitions that have been triggered
pub fn process_government_transitions(
    mut events: EventReader<GovernmentTransition>,
    mut nations: Query<(&mut crate::nations::Nation, &mut Governance, &mut super::history::GovernmentHistory)>,
    mut name_generator: Local<Option<crate::name_generator::NameGenerator>>,
    time: Res<crate::simulation::GameTime>,
) {
    // Initialize name generator if needed
    if name_generator.is_none() {
        *name_generator = Some(crate::name_generator::NameGenerator::new());
    }
    let name_gen = name_generator.as_mut().unwrap();

    for event in events.read() {
        if let Ok((mut nation, mut governance, mut history)) = nations.get_mut(event.nation_entity) {
            // Record the change in history
            history.changes.push(super::history::GovernmentChange {
                from: event.from_government,
                to: event.to_government,
                transition_type: event.transition_type,
                peaceful: event.peaceful,
                game_time: time.current_date as u32,
            });

            // Update governance
            governance.government_type = event.to_government;
            governance.last_transition = Some(time.current_date as u32);
            governance.reform_pressure = 0.0;

            // Reset stability based on transition type
            governance.stability = if event.peaceful { 0.6 } else { 0.3 };

            // Update tradition strength
            governance.tradition_strength = event.to_government.mechanics().reform_resistance;

            // Generate new nation name based on new government
            // Note: In actual integration, we'd need to get the culture from somewhere
            let culture = crate::name_generator::Culture::Western; // Placeholder
            let (new_nation_name, new_ruler_title) = super::naming::generate_governance_aware_name(
                name_gen,
                culture,
                &event.to_government,
            );

            // Update nation name
            nation.name = new_nation_name;

            // Log the transition
            info!(
                "Government transition: {} changed from {:?} to {:?} via {:?}",
                nation.name,
                event.from_government,
                event.to_government,
                event.transition_type
            );
        }
    }
}

/// Check if a transition between two government types is possible
pub fn can_transition(from: GovernmentType, to: GovernmentType) -> bool {
    use GovernmentType::*;

    // Some transitions are impossible or very unlikely
    match (from, to) {
        // Cannot go from hive mind to anything else easily
        (HiveMindCollective, _) => false,
        (_, HiveMindCollective) => false,

        // Primitivism rejects all technology
        (AnarchoPrimitivism, _) if to != TribalFederation => false,

        // Some natural progressions
        (Feudalism, AbsoluteMonarchy) => true,
        (AbsoluteMonarchy, ConstitutionalMonarchy) => true,
        (ConstitutionalMonarchy, ParliamentaryDemocracy) => true,

        // Generally possible
        _ => true,
    }
}

/// Trigger a government transition manually (for events/decisions)
pub fn transition_government(
    governance: &mut Governance,
    to: GovernmentType,
    transition_type: TransitionType,
) {
    governance.government_type = to;
    governance.reform_pressure = 0.0;
    governance.stability = if matches!(transition_type, TransitionType::Reform | TransitionType::Election) {
        0.7
    } else {
        0.4
    };
}