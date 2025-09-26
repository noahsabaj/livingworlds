//! Law effects calculation
//!
//! Calculates combined effects of all active laws for a nation,
//! including diminishing returns to prevent extreme stacking.

use crate::nations::laws::{
    registry::{LawRegistry, NationLaws},
    types::LawEffects,
};
use crate::simulation::PressureType;

/// Calculate the combined effects of all active laws for a nation
pub fn calculate_law_effects(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
) -> LawEffects {
    let mut combined = LawEffects::default();

    for &law_id in &nation_laws.active_laws {
        if let Some(law) = registry.get_law(law_id) {
            apply_single_law_effects(&mut combined, &law.effects);
        }
    }

    // Apply diminishing returns to prevent extreme values
    apply_diminishing_returns(&mut combined);

    combined
}

/// Apply a single law's effects to the combined total
fn apply_single_law_effects(combined: &mut LawEffects, effects: &LawEffects) {
    // Economic effects
    combined.tax_efficiency_modifier += effects.tax_efficiency_modifier;
    combined.trade_income_modifier += effects.trade_income_modifier;
    combined.industrial_output_modifier += effects.industrial_output_modifier;
    combined.agricultural_output_modifier += effects.agricultural_output_modifier;
    combined.wealth_inequality_change += effects.wealth_inequality_change;

    // Military effects
    combined.mobilization_speed_modifier += effects.mobilization_speed_modifier;
    combined.army_morale_modifier += effects.army_morale_modifier;
    combined.naval_tradition_modifier += effects.naval_tradition_modifier;
    combined.defensive_bonus_modifier += effects.defensive_bonus_modifier;
    combined.expansion_desire_modifier += effects.expansion_desire_modifier;

    // Social effects
    combined.stability_change += effects.stability_change;
    combined.legitimacy_change += effects.legitimacy_change;
    combined.happiness_modifier += effects.happiness_modifier;
    combined.population_growth_modifier += effects.population_growth_modifier;
    combined.technology_rate_modifier += effects.technology_rate_modifier;
    combined.cultural_conversion_modifier += effects.cultural_conversion_modifier;

    // Political effects
    combined.corruption_change += effects.corruption_change;
    combined.centralization_change += effects.centralization_change;
    combined.reform_resistance_change += effects.reform_resistance_change;
    combined.diplomatic_reputation_change += effects.diplomatic_reputation_change;

    // Merge pressure modifiers
    for (pressure_type, modifier) in &effects.pressure_modifiers {
        *combined.pressure_modifiers.entry(*pressure_type).or_insert(0.0) += modifier;
    }

    // Handle special flags (take most restrictive)
    if let Some(allows) = effects.allows_slavery {
        combined.allows_slavery = Some(
            combined.allows_slavery.unwrap_or(true) && allows
        );
    }
    if let Some(allows) = effects.allows_free_speech {
        combined.allows_free_speech = Some(
            combined.allows_free_speech.unwrap_or(true) && allows
        );
    }
    if let Some(allows) = effects.allows_private_property {
        combined.allows_private_property = Some(
            combined.allows_private_property.unwrap_or(true) && allows
        );
    }
    if let Some(allows) = effects.allows_religious_freedom {
        combined.allows_religious_freedom = Some(
            combined.allows_religious_freedom.unwrap_or(true) && allows
        );
    }
}

/// Apply diminishing returns to prevent extreme modifier stacking
pub fn apply_diminishing_returns(effects: &mut LawEffects) {
    // Apply soft caps to modifiers
    effects.tax_efficiency_modifier = soft_cap(effects.tax_efficiency_modifier, 0.5, 0.8);
    effects.trade_income_modifier = soft_cap(effects.trade_income_modifier, 0.5, 0.8);
    effects.industrial_output_modifier = soft_cap(effects.industrial_output_modifier, 0.5, 0.8);
    effects.agricultural_output_modifier = soft_cap(effects.agricultural_output_modifier, 0.5, 0.8);

    effects.army_morale_modifier = soft_cap(effects.army_morale_modifier, 0.5, 0.8);
    effects.mobilization_speed_modifier = soft_cap(effects.mobilization_speed_modifier, 0.5, 0.8);

    effects.happiness_modifier = soft_cap(effects.happiness_modifier, 0.5, 0.8);
    effects.technology_rate_modifier = soft_cap(effects.technology_rate_modifier, 0.5, 0.8);

    // Clamp extreme values
    effects.stability_change = effects.stability_change.clamp(-0.5, 0.5);
    effects.legitimacy_change = effects.legitimacy_change.clamp(-0.5, 0.5);
    effects.wealth_inequality_change = effects.wealth_inequality_change.clamp(-0.5, 0.5);
    effects.corruption_change = effects.corruption_change.clamp(-0.5, 0.5);
}

/// Apply a soft cap to a modifier value
fn soft_cap(value: f32, threshold: f32, max_value: f32) -> f32 {
    if value.abs() <= threshold {
        value
    } else {
        let excess = value.abs() - threshold;
        let capped_excess = excess * 0.5; // Halve effectiveness beyond threshold
        let total = threshold + capped_excess.min(max_value - threshold);
        total.copysign(value)
    }
}