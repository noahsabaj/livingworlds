//! Law modifier application
//!
//! Applies law effects to nation's government mechanics,
//! modifying all aspects of governance based on active laws.

use crate::nations::governance::GovernmentMechanics;
use crate::nations::laws::types::LawEffects;

/// Apply law modifiers to nation mechanics
pub fn apply_law_modifiers(
    base_mechanics: &GovernmentMechanics,
    law_effects: &LawEffects,
) -> GovernmentMechanics {
    let mut modified = base_mechanics.clone();

    // Apply economic modifiers
    modified.tax_efficiency *= 1.0 + law_effects.tax_efficiency_modifier;
    modified.trade_modifier *= 1.0 + law_effects.trade_income_modifier;
    modified.industrial_output *= 1.0 + law_effects.industrial_output_modifier;
    modified.agricultural_output *= 1.0 + law_effects.agricultural_output_modifier;
    modified.inequality += law_effects.wealth_inequality_change;

    // Apply military modifiers
    modified.mobilization_speed *= 1.0 + law_effects.mobilization_speed_modifier;
    modified.army_morale *= 1.0 + law_effects.army_morale_modifier;
    modified.naval_tradition *= 1.0 + law_effects.naval_tradition_modifier;
    modified.defensive_bonus *= 1.0 + law_effects.defensive_bonus_modifier;
    modified.expansion_desire *= 1.0 + law_effects.expansion_desire_modifier;

    // Apply social modifiers
    modified.stability_base += law_effects.stability_change;
    modified.legitimacy_decay -= law_effects.legitimacy_change * 0.1; // Convert to decay rate
    modified.population_growth *= 1.0 + law_effects.population_growth_modifier;
    modified.citizen_happiness *= 1.0 + law_effects.happiness_modifier;
    modified.technology_rate *= 1.0 + law_effects.technology_rate_modifier;
    modified.cultural_conversion *= 1.0 + law_effects.cultural_conversion_modifier;

    // Apply political modifiers
    modified.corruption += law_effects.corruption_change;
    modified.centralization += law_effects.centralization_change;
    modified.reform_resistance += law_effects.reform_resistance_change;
    modified.diplomatic_weight *= 1.0 + law_effects.diplomatic_reputation_change;

    // Clamp final values to reasonable ranges
    modified.tax_efficiency = modified.tax_efficiency.clamp(0.1, 2.0);
    modified.inequality = modified.inequality.clamp(0.0, 1.0);
    modified.corruption = modified.corruption.clamp(0.0, 1.0);
    modified.centralization = modified.centralization.clamp(0.0, 1.0);
    modified.stability_base = modified.stability_base.clamp(-1.0, 1.0);

    modified
}