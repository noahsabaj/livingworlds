//! Law proposal evaluation
//!
//! Evaluates whether nations should propose laws based on pressures and conditions.

use std::collections::HashMap;

use super::types::LawProposal;
use crate::nations::laws::mechanics::{
    calculate_popularity_weights, evaluate_law_popularity, get_government_law_affinity,
    check_law_conflicts, suggest_laws_for_pressures,
};
use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::laws::types::{LawPrerequisite, LawComplexity};
use crate::nations::{Nation, Governance, GovernmentType, GovernmentCategory};
use crate::simulation::{PressureType, PressureVector};

/// Evaluate whether a nation should propose a new law
pub fn evaluate_law_passage(
    nation: &Nation,
    governance: &Governance,
    pressures: &PressureVector,
    nation_laws: &NationLaws,
    registry: &LawRegistry,
    current_year: i32,
) -> Option<LawProposal> {
    // Don't propose new laws if too many are already being debated
    if nation_laws.proposed_laws.len() >= 3 {
        return None;
    }

    // Get highest pressure
    let (pressure_type, pressure_level) = pressures.highest_pressure()?;

    // Only consider laws if pressure is significant
    if pressure_level.value() < 0.4 {
        return None;
    }

    // Get suggested laws for current pressures
    let pressure_map: HashMap<PressureType, f32> = pressures.pressures
        .iter()
        .map(|(k, v)| (*k, v.value()))
        .collect();

    let suggested_laws = suggest_laws_for_pressures(&pressure_map, registry, nation_laws);

    // Find the best law to propose
    let mut best_law = None;
    let mut best_score = 0.0;

    for law_id in suggested_laws {
        let law = registry.get_law(law_id)?;

        // Check if law is available
        if law.available_from_year > current_year {
            continue;
        }

        // Check for conflicts
        let conflicts = check_law_conflicts(nation_laws, registry, law_id);
        if !conflicts.is_empty() {
            continue; // Skip laws that conflict with existing ones
        }

        // Check prerequisites
        if !check_prerequisites(&law.prerequisites, nation, governance, current_year) {
            continue;
        }

        // Calculate proposal score
        let popularity = evaluate_law_popularity(law, nation, governance, current_year);
        let weights = calculate_popularity_weights(governance.current_government);
        let weighted_support = popularity.weighted_support(&weights);

        let gov_affinity = get_government_law_affinity(law, governance.current_government);
        let pressure_urgency = pressure_level.value();

        let score = weighted_support * 0.4
            + gov_affinity * 0.3
            + pressure_urgency * 0.3;

        if score > best_score {
            best_score = score;
            best_law = Some((law_id, law, popularity));
        }
    }

    best_law.map(|(law_id, law, popularity)| {
        LawProposal {
            law_id,
            initial_support: popularity.weighted_support(
                &calculate_popularity_weights(governance.current_government)
            ),
            debate_days: calculate_debate_duration(law.complexity, governance.current_government),
            pressure_motivation: pressure_type,
            conflicts_to_repeal: Vec::new(),
        }
    })
}

/// Check if all prerequisites for a law are met
pub fn check_prerequisites(
    prerequisites: &[LawPrerequisite],
    nation: &Nation,
    governance: &Governance,
    current_year: i32,
) -> bool {
    for prereq in prerequisites {
        match prereq {
            LawPrerequisite::GovernmentCategory(required_category) => {
                if governance.current_government.category() != *required_category {
                    return false;
                }
            }
            LawPrerequisite::RequiresLaw(_law_id) => {
                // TODO: Check if nation has this law active
                // For now, skip this check
            }
            LawPrerequisite::TechnologyLevel(_level) => {
                // TODO: Check nation's tech level when tech system is implemented
                // For now, assume all tech requirements are met
            }
            LawPrerequisite::MinimumStability(min_stability) => {
                if nation.stability < *min_stability {
                    return false;
                }
            }
            LawPrerequisite::MinimumLegitimacy(min_legitimacy) => {
                if governance.legitimacy < *min_legitimacy {
                    return false;
                }
            }
            LawPrerequisite::YearReached(year) => {
                if current_year < *year {
                    return false;
                }
            }
            LawPrerequisite::MinimumProvinces(_count) => {
                // TODO: Check nation's province count when available
                // For now, assume met
            }
            LawPrerequisite::Custom(_description) => {
                // Custom prerequisites need special handling
                // For now, assume met
            }
        }
    }
    true
}

/// Calculate how long a law should be debated
pub fn calculate_debate_duration(complexity: LawComplexity, government: GovernmentType) -> f32 {
    let base_duration = complexity.implementation_time();

    // Modify based on government type
    let modifier = match government.category() {
        GovernmentCategory::Autocratic => 0.5,  // Quick decisions
        GovernmentCategory::Democratic => 1.5,  // Lengthy debate
        GovernmentCategory::Anarchist => 2.0,   // Consensus takes time
        GovernmentCategory::Corporate => 0.7,   // Board decisions
        GovernmentCategory::Theocratic => 0.8,  // Religious deliberation
        _ => 1.0,
    };

    base_duration * modifier
}