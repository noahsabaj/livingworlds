//! Law effect application system
//!
//! Applies the combined effects of enacted laws to nation attributes every frame.

use bevy::prelude::*;
use crate::nations::{Economy, Nation, Governance};
use crate::nations::laws::{NationLaws, LawEffects};
use crate::relationships::{EnactedLaws, LawEntity};

/// Apply law effects to nation attributes every frame
pub fn apply_law_effects_to_nations(
    mut nations_query: Query<(
        &mut Nation,
        Option<&mut Governance>,
        Option<&mut Economy>,
        &NationLaws,
    )>,
    time: Res<Time>,
) {
    for (mut nation, governance, economy, nation_laws) in &mut nations_query {
        let effects = &nation_laws.combined_effects;
        let dt = time.delta_secs();

        // Apply stability changes
        if effects.stability_change.abs() > 0.001 {
            nation.stability = (nation.stability + effects.stability_change * dt)
                .clamp(0.0, 1.0);
        }

        // Apply happiness modifier (affects population growth)
        if let Some(mut gov) = governance {
            // Apply legitimacy changes
            if effects.legitimacy_change.abs() > 0.001 {
                // Modify base legitimacy
                gov.legitimacy_factors.base_legitimacy =
                    (gov.legitimacy_factors.base_legitimacy + effects.legitimacy_change * dt)
                    .clamp(0.0, 1.0);
            }

            // Apply administrative efficiency
            if effects.administrative_efficiency_modifier.abs() > 0.001 {
                gov.legitimacy_factors.administrative_efficiency =
                    (gov.legitimacy_factors.administrative_efficiency
                     * (1.0 + effects.administrative_efficiency_modifier))
                    .clamp(0.0, 2.0);
            }

            // Apply diplomatic reputation
            if effects.diplomatic_reputation_change.abs() > 0.001 {
                // Would affect diplomatic relations if we had a diplomacy component
            }

            // Apply reform resistance
            if effects.reform_resistance_change.abs() > 0.001 {
                // Would affect reform speed if we had reform tracking
            }
        }

        // Apply economic modifiers
        if let Some(mut econ) = economy {
            // Tax efficiency directly affects income
            if effects.tax_efficiency_modifier.abs() > 0.001 {
                econ.tax_efficiency = (econ.tax_efficiency
                    * (1.0 + effects.tax_efficiency_modifier))
                    .clamp(0.1, 2.0);
            }

            // Industrial output modifier
            if effects.industrial_output_modifier.abs() > 0.001 {
                econ.industrial_multiplier = (econ.industrial_multiplier
                    * (1.0 + effects.industrial_output_modifier))
                    .clamp(0.1, 3.0);
            }

            // Agricultural output modifier
            if effects.agricultural_output_modifier.abs() > 0.001 {
                econ.agricultural_multiplier = (econ.agricultural_multiplier
                    * (1.0 + effects.agricultural_output_modifier))
                    .clamp(0.1, 3.0);
            }

            // Trade income modifier
            if effects.trade_income_modifier.abs() > 0.001 {
                econ.trade_multiplier = (econ.trade_multiplier
                    * (1.0 + effects.trade_income_modifier))
                    .clamp(0.1, 3.0);
            }

            // Maintenance cost modifier (negative is good)
            if effects.maintenance_cost_modifier.abs() > 0.001 {
                econ.maintenance_cost = (econ.maintenance_cost
                    * (1.0 + effects.maintenance_cost_modifier))
                    .clamp(0.0, 10000.0);
            }
        }

        // Apply military strength modifier
        if effects.army_morale_modifier.abs() > 0.001 {
            nation.military_strength = (nation.military_strength
                * (1.0 + effects.army_morale_modifier))
                .clamp(0.0, 100000.0);
        }

        // Apply revolt risk changes
        if effects.revolt_risk_change.abs() > 0.001 {
            // Would track revolt risk if we had rebellion system
        }

        // Apply corruption changes
        if effects.corruption_change.abs() > 0.001 {
            // Would track corruption if we had corruption system
        }

        // Apply technology rate modifier
        if effects.technology_rate_modifier.abs() > 0.001 {
            // Would affect tech advancement if we had tech system
        }

        // Apply population growth modifier
        if effects.population_growth_modifier.abs() > 0.001 {
            // Would affect population growth calculations
        }

        // Apply cultural conversion modifier
        if effects.cultural_conversion_modifier.abs() > 0.001 {
            // Would affect cultural spread if we had culture system
        }
    }
}

/// Apply law effects using entity relationships (future migration)
pub fn apply_law_effects_with_relationships(
    mut nations_query: Query<(
        &mut Nation,
        Option<&mut Governance>,
        Option<&mut Economy>,
        &EnactedLaws,
    )>,
    laws_query: Query<&LawEntity>,
    time: Res<Time>,
) {
    for (mut nation, governance, economy, enacted_laws) in &mut nations_query {
        // Calculate combined effects from all enacted law entities
        let mut combined_effects = LawEffects::default();

        for &law_entity in enacted_laws.laws() {
            if let Ok(law) = laws_query.get(law_entity) {
                combined_effects = combined_effects.combine_with(&law.effects);
            }
        }

        let dt = time.delta_secs();

        // Apply stability changes
        if combined_effects.stability_change.abs() > 0.001 {
            nation.stability = (nation.stability + combined_effects.stability_change * dt)
                .clamp(0.0, 1.0);
        }

        // Apply other effects similar to above...
        // (Implementation would mirror the above function but using entity relationships)
    }
}

// NOTE: Economy component is now defined in crate::nations::types and imported above