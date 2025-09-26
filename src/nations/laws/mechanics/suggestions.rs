//! Law suggestions based on pressures
//!
//! Suggests appropriate laws to address nation pressures and problems.

use std::collections::HashMap;
use crate::simulation::PressureType;
use crate::nations::laws::{
    registry::{LawRegistry, NationLaws},
    types::LawId,
};

/// Determine which laws a nation should consider based on pressures
pub fn suggest_laws_for_pressures(
    pressures: &HashMap<PressureType, f32>,
    registry: &LawRegistry,
    nation_laws: &NationLaws,
) -> Vec<LawId> {
    let mut suggestions = Vec::new();

    for (pressure_type, &pressure_level) in pressures {
        if pressure_level < 0.4 {
            continue; // Only suggest laws for significant pressures
        }

        // Find laws that help with this pressure
        let beneficial_laws = registry.filter_laws(|law| {
            // Check if law addresses this pressure
            if let Some(&modifier) = law.effects.pressure_modifiers.get(pressure_type) {
                // Negative modifiers reduce pressure (good)
                modifier < 0.0 && !nation_laws.is_active(law.id)
            } else {
                // Check indirect effects
                match pressure_type {
                    PressureType::PopulationOvercrowding => {
                        law.effects.population_growth_modifier < 0.0
                            || law.effects.happiness_modifier > 0.2
                    }
                    PressureType::EconomicStrain => {
                        law.effects.tax_efficiency_modifier > 0.1
                            || law.effects.trade_income_modifier > 0.1
                    }
                    PressureType::MilitaryVulnerability => {
                        law.effects.army_morale_modifier > 0.1
                            || law.effects.mobilization_speed_modifier > 0.1
                    }
                    PressureType::LegitimacyCrisis => {
                        law.effects.legitimacy_change > 0.1
                            || law.effects.stability_change > 0.1
                    }
                    _ => false,
                }
            }
        });

        for law in beneficial_laws {
            if !suggestions.contains(&law.id) {
                suggestions.push(law.id);
            }
        }
    }

    suggestions
}