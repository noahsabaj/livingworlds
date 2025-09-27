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

    // Base pressures - reduced to realistic values that require MULTIPLE crisis factors
    // These multipliers mean individual pressures rarely trigger transitions alone
    let economic = pressure.economic_crisis * 0.3;      // Was 2.0 - now needs severe crisis
    let military = pressure.military_defeat * 0.25;     // Was 1.5 - military alone won't topple
    let cultural = pressure.cultural_shift * 0.15;      // Was 1.0 - cultural shifts are slow
    let external = pressure.external_influence * 0.2;   // Was 1.2 - foreign influence is limited
    let tech = pressure.technological_change * 0.1;     // Was 0.8 - tech changes are gradual
    let religious = pressure.religious_fervor * 0.15;   // Was 1.0 - religious pressure builds slowly
    let revolutionary = pressure.revolutionary_ideas * 0.25; // Was 1.5 - ideas alone don't topple

    // Sum and apply resistance
    let raw_pressure = economic + military + cultural + external + tech + religious + revolutionary;

    // Apply both tradition AND institutional strength as resistance
    // Strong institutions (1.0) make transitions very hard
    // Weak institutions (0.1) make the state a powder keg
    let institutional_resistance = governance.institution_strength.max(0.1); // Never fully zero
    raw_pressure * (1.0 - governance.tradition_strength * mechanics.reform_resistance) / institutional_resistance
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
                    GovernmentCategory::Monarchic => PresidentialRepublic,
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
                GovernmentCategory::Monarchic => TribalFederation,
                GovernmentCategory::Corporate => Kleptocracy,
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
    mut nations: Query<(&mut crate::nations::Nation, &mut Governance, &mut super::history::GovernmentHistory, &mut PoliticalPressure)>,
    mut name_generator: Local<Option<crate::name_generator::NameGenerator>>,
    time: Res<crate::simulation::GameTime>,
) {
    // Initialize name generator if needed
    if name_generator.is_none() {
        *name_generator = Some(crate::name_generator::NameGenerator::new());
    }
    let name_gen = name_generator.as_mut().unwrap();

    for event in events.read() {
        if let Ok((mut nation, mut governance, mut history, mut pressure)) = nations.get_mut(event.nation_entity) {
            // Record the change in history
            history.changes.push(super::history::GovernmentChange {
                from: event.from_government,
                to: event.to_government,
                transition_type: event.transition_type,
                peaceful: event.peaceful,
                game_time: time.current_day(),
            });

            // Apply transition costs based on type - THIS NATURALLY PREVENTS RAPID COUPS
            // Each transition depletes resources, making subsequent transitions harder
            match event.transition_type {
                TransitionType::Coup => {
                    // Military coups cost military loyalty and treasury
                    nation.military_strength *= 0.5;  // Half the military is purged/divided
                    nation.treasury *= 0.7;            // Bribes and restructuring costs
                    governance.stability = 0.25;       // Very unstable after coup
                }
                TransitionType::Revolution => {
                    // Revolutions devastate everything
                    nation.military_strength *= 0.3;   // Army shattered
                    nation.treasury *= 0.4;            // Economy in chaos
                    governance.stability = 0.2;        // Near collapse
                }
                TransitionType::Collapse => {
                    // Total state failure
                    nation.military_strength *= 0.2;   // Military dissolved
                    nation.treasury *= 0.2;            // Economic devastation
                    governance.stability = 0.1;        // Failed state
                }
                TransitionType::Reform | TransitionType::Election => {
                    // Peaceful transitions have minimal cost
                    nation.treasury *= 0.9;            // Election/reform costs
                    governance.stability = if event.peaceful { 0.6 } else { 0.4 };
                }
                TransitionType::PopularUprising => {
                    // Popular movements disrupt economy
                    nation.military_strength *= 0.6;   // Some military defects
                    nation.treasury *= 0.5;            // General strikes, disruption
                    governance.stability = 0.3;
                }
                TransitionType::ForeignImposed => {
                    // Puppet government has foreign backing
                    nation.military_strength *= 0.8;   // Some resistance
                    nation.treasury *= 0.9;            // Foreign aid offsets costs
                    governance.stability = 0.4;
                }
                TransitionType::EliteConspiracy => {
                    // Palace coups preserve most structures
                    nation.military_strength *= 0.7;   // Some purges
                    nation.treasury *= 0.8;            // Elite wealth preserved
                    governance.stability = 0.35;
                }
                TransitionType::Succession => {
                    // Orderly succession has minimal impact
                    governance.stability = 0.7;        // Smooth transition
                }
            }

            // Update governance
            governance.government_type = event.to_government;
            governance.last_transition = Some(time.current_day());
            governance.reform_pressure = 0.0;

            // Update tradition strength
            governance.tradition_strength = event.to_government.mechanics().reform_resistance;

            // INSTITUTIONAL DECAY - Each transition weakens state apparatus
            // This creates natural cascades: weak states transition more easily
            match event.transition_type {
                TransitionType::Collapse => {
                    governance.institution_strength *= 0.2;  // Near-total institutional failure
                }
                TransitionType::Revolution => {
                    governance.institution_strength *= 0.4;  // Institutions shattered
                }
                TransitionType::Coup | TransitionType::EliteConspiracy => {
                    governance.institution_strength *= 0.6;  // Military/elite structures damaged
                }
                TransitionType::PopularUprising => {
                    governance.institution_strength *= 0.5;  // Mass disruption
                }
                TransitionType::ForeignImposed => {
                    governance.institution_strength *= 0.7;  // Some continuity with foreign backing
                }
                TransitionType::Reform | TransitionType::Election => {
                    governance.institution_strength *= 0.85; // Minimal institutional damage
                }
                TransitionType::Succession => {
                    governance.institution_strength *= 0.95; // Almost no damage
                }
            }

            // REVOLUTIONARY EXHAUSTION - Deplete pressures that caused the transition
            // This naturally prevents the same pressures from triggering again immediately
            match event.transition_type {
                TransitionType::Revolution | TransitionType::PopularUprising => {
                    // Revolutionary energy is spent
                    pressure.revolutionary_ideas *= 0.1;  // 90% exhausted
                    pressure.economic_crisis *= 0.5;      // Partially addressed
                }
                TransitionType::Coup | TransitionType::EliteConspiracy => {
                    // Military is divided and exhausted
                    pressure.military_defeat *= 0.2;      // Military reorganizing
                    pressure.external_influence *= 0.3;   // Foreign backers step back
                }
                TransitionType::Collapse => {
                    // Total exhaustion of all pressures
                    pressure.economic_crisis *= 0.3;
                    pressure.military_defeat *= 0.3;
                    pressure.revolutionary_ideas *= 0.2;
                    pressure.cultural_shift *= 0.5;
                }
                TransitionType::Reform | TransitionType::Election => {
                    // Peaceful change relieves some pressure
                    pressure.revolutionary_ideas *= 0.3;  // Ideas partially satisfied
                    pressure.economic_crisis *= 0.7;      // Some reforms help
                }
                TransitionType::ForeignImposed => {
                    // External pressure satisfied
                    pressure.external_influence *= 0.1;
                    pressure.military_defeat *= 0.4;
                }
                _ => {
                    // General pressure relief
                    pressure.revolutionary_ideas *= 0.5;
                    pressure.economic_crisis *= 0.6;
                }
            }

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