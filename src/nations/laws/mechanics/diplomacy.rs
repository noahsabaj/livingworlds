//! Law diplomatic impact
//!
//! Calculates how law differences affect diplomatic relations between nations.

use crate::nations::laws::{
    registry::{LawRegistry, NationLaws},
    types::LawCategory,
};

/// Calculate the diplomatic impact of law differences between nations
pub fn calculate_law_diplomatic_impact(
    nation1_laws: &NationLaws,
    nation2_laws: &NationLaws,
    registry: &LawRegistry,
) -> f32 {
    let mut impact = 0.0;
    let mut total_laws = 0;

    // Check laws that differ
    for &law_id in &nation1_laws.active_laws {
        if !nation2_laws.is_active(law_id) {
            if let Some(law) = registry.get_law(law_id) {
                // Some laws cause more diplomatic friction
                match law.category {
                    LawCategory::Religious => impact -= 0.02,
                    LawCategory::Military => {
                        if law.effects.expansion_desire_modifier > 0.0 {
                            impact -= 0.03;
                        }
                    }
                    LawCategory::Diplomatic => impact -= 0.01,
                    _ => impact -= 0.005,
                }
            }
        } else {
            // Shared laws improve relations
            impact += 0.01;
        }
        total_laws += 1;
    }

    // Check laws nation2 has that nation1 doesn't
    for &law_id in &nation2_laws.active_laws {
        if !nation1_laws.is_active(law_id) {
            total_laws += 1;
        }
    }

    // Normalize by total laws
    if total_laws > 0 {
        impact / total_laws as f32
    } else {
        0.0
    }
}