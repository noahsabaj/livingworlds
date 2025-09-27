//! Government legitimacy calculations
//!
//! This module calculates how legitimate a government is perceived to be
//! by its population, affecting stability and reform pressure.

use bevy::prelude::*;

use super::types::{Governance, GovernmentType, PoliticalPressure};

/// Factors that affect government legitimacy
#[derive(Debug, Clone, Default)]
pub struct LegitimacyFactors {
    pub military_victories: f32,     // Recent victories boost legitimacy
    pub economic_prosperity: f32,    // Good economy = happy people
    pub time_in_power: f32,          // Longevity can help or hurt
    pub cultural_unity: f32,         // Shared culture strengthens legitimacy
    pub divine_mandate: f32,         // For religious governments
    pub democratic_mandate: f32,     // For elected governments
    pub revolutionary_fervor: f32,   // For new revolutionary governments
    pub foreign_recognition: f32,    // International acceptance
}

/// Update government legitimacy based on various factors
pub fn update_government_legitimacy(
    mut nations: Query<(
        &crate::nations::Nation,
        &mut Governance,
        &PoliticalPressure,
        &crate::nations::house::House,
    )>,
    time: Res<crate::simulation::GameTime>,
) {
    for (nation, mut governance, pressure, house) in &mut nations {
        let factors = calculate_legitimacy_factors(
            &nation,
            &governance,
            pressure,
            house,
            time.current_day(),
        );

        let new_legitimacy = calculate_legitimacy(&governance, &factors);

        // Apply legitimacy decay
        let decay = governance.government_type.mechanics().legitimacy_decay;
        governance.stability = (governance.stability * (1.0 - decay) + new_legitimacy * decay).clamp(0.0, 1.0);

        // Legitimacy affects reform pressure
        if governance.stability < 0.3 {
            governance.reform_pressure += 0.01;
        } else if governance.stability > 0.7 {
            governance.reform_pressure = (governance.reform_pressure - 0.01).max(0.0);
        }
    }
}

/// Calculate legitimacy factors for a nation
fn calculate_legitimacy_factors(
    nation: &crate::nations::Nation,
    governance: &Governance,
    pressure: &PoliticalPressure,
    house: &crate::nations::house::House,
    current_day: u32,
) -> LegitimacyFactors {
    let mut factors = LegitimacyFactors::default();

    // Economic prosperity
    factors.economic_prosperity = if nation.treasury > 1000.0 {
        1.0
    } else if nation.treasury > 500.0 {
        0.7
    } else if nation.treasury > 100.0 {
        0.4
    } else {
        0.2
    };

    // Military strength
    factors.military_victories = nation.military_strength.min(1.0);

    // Time in power (honeymoon period, then decay, then tradition)
    let days_in_power = governance.last_transition
        .map(|transition_day| current_day.saturating_sub(transition_day))
        .unwrap_or(1000);

    factors.time_in_power = if days_in_power < 100 {
        0.8 // Honeymoon period
    } else if days_in_power < 365 {
        0.6
    } else if days_in_power < 1000 {
        0.5
    } else if days_in_power > 3650 {
        0.7 // Traditional legitimacy after 10 years
    } else {
        0.4
    };

    // Cultural unity (simplified for now)
    factors.cultural_unity = 1.0 - pressure.cultural_shift;

    // Government-specific legitimacy
    match governance.government_type.category() {
        super::types::GovernmentCategory::Theocratic => {
            factors.divine_mandate = 0.8;
        },
        super::types::GovernmentCategory::Democratic => {
            factors.democratic_mandate = 0.9;
        },
        super::types::GovernmentCategory::Socialist | super::types::GovernmentCategory::Anarchist => {
            factors.revolutionary_fervor = if days_in_power < 365 { 0.8 } else { 0.4 };
        },
        _ => {},
    }

    // House legitimacy affects overall legitimacy
    factors.foreign_recognition = house.legitimacy;

    factors
}

/// Calculate overall legitimacy from factors
pub fn calculate_legitimacy(governance: &Governance, factors: &LegitimacyFactors) -> f32 {
    let mechanics = governance.government_type.mechanics();

    // Base legitimacy from stability
    let base = mechanics.stability_base;

    // Apply factors with weights based on government type
    let economic_weight = match governance.government_type {
        GovernmentType::CorporateState | GovernmentType::MerchantRepublic => 1.5,
        GovernmentType::Kleptocracy => 0.3,
        _ => 1.0,
    };

    let military_weight = match governance.government_type {
        GovernmentType::MilitaryJunta | GovernmentType::Stratocracy => 1.8,
        GovernmentType::NomadicKhanate => 1.5,
        GovernmentType::AnarchoCommunism => 0.3,
        _ => 1.0,
    };

    let divine_weight = match governance.government_type {
        GovernmentType::Theocracy | GovernmentType::DivineManadate | GovernmentType::Caliphate => 2.0,
        GovernmentType::CultState => 1.5,
        _ => 0.2,
    };

    let democratic_weight = match governance.government_type {
        GovernmentType::ParliamentaryDemocracy | GovernmentType::PresidentialRepublic => 1.8,
        GovernmentType::DirectDemocracy => 2.0,
        _ => 0.1,
    };

    // Calculate weighted sum
    let weighted_sum =
        factors.economic_prosperity * economic_weight * 0.2 +
        factors.military_victories * military_weight * 0.15 +
        factors.time_in_power * 0.1 +
        factors.cultural_unity * 0.15 +
        factors.divine_mandate * divine_weight * 0.15 +
        factors.democratic_mandate * democratic_weight * 0.15 +
        factors.revolutionary_fervor * 0.05 +
        factors.foreign_recognition * 0.05;

    // Normalize and combine with base
    (base + weighted_sum).clamp(0.0, 1.0)
}