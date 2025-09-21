//! Legitimacy pressure calculations
//!
//! Legitimacy pressure represents how accepted a ruler is by their people.
//! Peaceful rulers gain legitimacy through prosperity, stability, and positive change.
//! Warlike rulers might gain it through victories, but lose it through defeats.

use super::types::PressureLevel;
use crate::nations::Nation;

/// Legitimacy pressures affecting a ruler
#[derive(Debug, Clone)]
pub struct LegitimacyPressure {
    /// General dissatisfaction with rule
    pub popular_discontent: PressureLevel,
    /// Noble/elite opposition
    pub elite_opposition: PressureLevel,
    /// Religious disapproval
    pub religious_disapproval: PressureLevel,
    /// Succession crisis or unclear succession
    pub succession_crisis: PressureLevel,
}

/// Sources of legitimacy for different ruler types
#[derive(Debug, Clone)]
pub struct LegitimacySources {
    /// From military victories (for warlike rulers)
    pub military_glory: f32,
    /// From economic prosperity (for all rulers, especially peaceful)
    pub prosperity: f32,
    /// From maintaining stability (crucial for peaceful rulers)
    pub stability: f32,
    /// From cultural achievements (for peaceful/scholarly rulers)
    pub cultural_prestige: f32,
    /// From religious authority
    pub divine_mandate: f32,
    /// From just governance and fair laws
    pub justice: f32,
}

/// Calculate legitimacy pressures based on ruler performance
pub fn calculate_legitimacy_pressure(
    nation: &Nation,
    ruler_personality: RulerPersonality,
    economic_health: f32,
    recent_events: &RecentEvents,
) -> LegitimacyPressure {
    // Calculate legitimacy sources
    let sources =
        calculate_legitimacy_sources(nation, ruler_personality, economic_health, recent_events);

    // Total legitimacy score (0.0 to 1.0)
    let total_legitimacy = calculate_total_legitimacy(&sources, ruler_personality);

    // Popular discontent inversely related to legitimacy
    let popular_discontent = PressureLevel::new(1.0 - total_legitimacy);

    // Elite opposition based on power distribution
    let elite_opposition = if nation.stability < 0.5 {
        PressureLevel::new((0.5 - nation.stability) * 2.0)
    } else {
        PressureLevel::NONE
    };

    // Religious disapproval (simplified for now)
    let religious_disapproval = if sources.divine_mandate < 0.3 {
        PressureLevel::new(1.0 - sources.divine_mandate * 3.33)
    } else {
        PressureLevel::NONE
    };

    // Succession crisis (increases with ruler age, decreases with clear heir)
    let succession_crisis = if recent_events.ruler_age > 60 && !recent_events.has_heir {
        PressureLevel::new((recent_events.ruler_age as f32 - 60.0) / 20.0) // Scale 60-80 to 0-1
    } else {
        PressureLevel::NONE
    };

    LegitimacyPressure {
        popular_discontent,
        elite_opposition,
        religious_disapproval,
        succession_crisis,
    }
}

fn calculate_legitimacy_sources(
    nation: &Nation,
    ruler_personality: RulerPersonality,
    economic_health: f32,
    recent_events: &RecentEvents,
) -> LegitimacySources {
    // Military glory from victories
    let military_glory = recent_events.victories as f32 / 3.0; // Max at 3 victories

    // Prosperity from economic health
    let prosperity = economic_health;

    // Stability bonus
    let stability = nation.stability;

    // Cultural prestige (simplified - would expand with culture system)
    let cultural_prestige = if ruler_personality == RulerPersonality::Peaceful {
        stability * 0.5 + prosperity * 0.5 // Peaceful rulers invest in culture
    } else {
        stability * 0.2
    };

    // Divine mandate (simplified - would expand with religion system)
    let divine_mandate = 0.5; // Neutral for now

    // Justice from fair governance
    let justice = if ruler_personality == RulerPersonality::Peaceful {
        stability * 0.7 + 0.3 // Peaceful rulers emphasize justice
    } else if ruler_personality == RulerPersonality::Ambitious {
        0.4 // Ambitious rulers may bend rules
    } else {
        0.3 // Warlike rulers focus less on justice
    };

    LegitimacySources {
        military_glory,
        prosperity,
        stability,
        cultural_prestige,
        divine_mandate,
        justice,
    }
}

fn calculate_total_legitimacy(
    sources: &LegitimacySources,
    ruler_personality: RulerPersonality,
) -> f32 {
    match ruler_personality {
        RulerPersonality::Peaceful => {
            // Peaceful rulers gain legitimacy from prosperity, stability, culture, and justice
            (sources.prosperity * 0.3
                + sources.stability * 0.25
                + sources.cultural_prestige * 0.2
                + sources.justice * 0.15
                + sources.divine_mandate * 0.1)
                .min(1.0)
        }
        RulerPersonality::Warlike => {
            // Warlike rulers need military success but also some stability
            (sources.military_glory * 0.4
                + sources.stability * 0.2
                + sources.prosperity * 0.15
                + sources.divine_mandate * 0.15
                + sources.justice * 0.1)
                .min(1.0)
        }
        RulerPersonality::Ambitious => {
            // Ambitious rulers balance multiple sources
            (sources.prosperity * 0.25
                + sources.military_glory * 0.2
                + sources.stability * 0.2
                + sources.cultural_prestige * 0.15
                + sources.divine_mandate * 0.1
                + sources.justice * 0.1)
                .min(1.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RulerPersonality {
    Peaceful,
    Warlike,
    Ambitious,
}

#[derive(Debug, Clone)]
pub struct RecentEvents {
    pub victories: u32,
    pub defeats: u32,
    pub ruler_age: u32,
    pub has_heir: bool,
    pub years_of_peace: u32,
    pub years_of_war: u32,
}

/// Actions to address legitimacy pressures
#[derive(Debug, Clone)]
pub enum LegitimacyAction {
    /// Hold festivals and public works to gain popularity
    PublicWorks { investment: f32 },
    /// Grant privileges to elites
    AppeasElites { concessions: f32 },
    /// Religious ceremonies and temple building
    ReligiousInvestment { piety_display: f32 },
    /// Secure succession through marriage or designation
    SecureSuccession { urgency: f32 },
    /// Distraction through foreign adventure
    ForeignAdventure { risk_tolerance: f32 },
    /// Reform government for better acceptance
    GovernmentReform { reform_depth: f32 },
}

/// Determine legitimacy action based on pressures
pub fn resolve_legitimacy_pressure(
    pressure: &LegitimacyPressure,
    ruler_personality: RulerPersonality,
) -> Option<LegitimacyAction> {
    // Critical succession crisis - top priority
    if pressure.succession_crisis.is_critical() {
        return Some(LegitimacyAction::SecureSuccession {
            urgency: pressure.succession_crisis.value(),
        });
    }

    // High popular discontent
    if pressure.popular_discontent.is_high() {
        match ruler_personality {
            RulerPersonality::Peaceful => {
                // Peaceful rulers use public works
                return Some(LegitimacyAction::PublicWorks {
                    investment: pressure.popular_discontent.value(),
                });
            }
            RulerPersonality::Warlike => {
                // Warlike rulers might use foreign adventures as distraction
                return Some(LegitimacyAction::ForeignAdventure {
                    risk_tolerance: pressure.popular_discontent.value() * 0.7,
                });
            }
            RulerPersonality::Ambitious => {
                // Ambitious rulers try reforms
                return Some(LegitimacyAction::GovernmentReform {
                    reform_depth: pressure.popular_discontent.value() * 0.5,
                });
            }
        }
    }

    // Elite opposition
    if pressure.elite_opposition.is_high() {
        return Some(LegitimacyAction::AppeasElites {
            concessions: pressure.elite_opposition.value() * 0.6,
        });
    }

    // Religious disapproval
    if pressure.religious_disapproval.is_moderate() {
        return Some(LegitimacyAction::ReligiousInvestment {
            piety_display: pressure.religious_disapproval.value(),
        });
    }

    None
}
