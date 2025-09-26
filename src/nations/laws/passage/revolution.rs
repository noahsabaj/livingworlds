//! Revolutionary law changes
//!
//! Handles dramatic law changes during government transitions and revolutions.

use super::types::RevolutionLawAction;
use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::{GovernmentType, GovernmentCategory};

/// Handle revolutionary law changes during government transition
pub fn revolutionary_law_changes(
    old_government: GovernmentType,
    new_government: GovernmentType,
    nation_laws: &NationLaws,
    registry: &LawRegistry,
) -> Vec<RevolutionLawAction> {
    let mut changes = Vec::new();

    let old_category = old_government.category();
    let new_category = new_government.category();

    // Dramatic changes between opposite government types
    if is_revolutionary_transition(old_category, new_category) {
        // Identify laws to repeal (those strongly opposed by new government)
        for &law_id in &nation_laws.active_laws {
            if let Some(law) = registry.get_law(law_id) {
                let affinity = law.government_affinity
                    .get(&new_category)
                    .copied()
                    .unwrap_or(0.0);

                if affinity < -0.5 {
                    changes.push(RevolutionLawAction::Repeal(law_id));
                }
            }
        }

        // Suggest new laws aligned with new government
        let aligned_laws = registry.filter_laws(|law| {
            law.government_affinity
                .get(&new_category)
                .map(|&aff| aff > 0.7)
                .unwrap_or(false)
        });

        for law in aligned_laws.into_iter().take(5) {
            if !nation_laws.is_active(law.id) {
                changes.push(RevolutionLawAction::Enact(law.id));
            }
        }
    }

    changes
}

/// Check if transition is revolutionary
pub fn is_revolutionary_transition(old: GovernmentCategory, new: GovernmentCategory) -> bool {
    matches!((old, new),
        (GovernmentCategory::Democratic, GovernmentCategory::Autocratic) |
        (GovernmentCategory::Autocratic, GovernmentCategory::Democratic) |
        (GovernmentCategory::Socialist, GovernmentCategory::Corporate) |
        (GovernmentCategory::Corporate, GovernmentCategory::Socialist) |
        (GovernmentCategory::Theocratic, GovernmentCategory::Anarchist) |
        (GovernmentCategory::Anarchist, GovernmentCategory::Theocratic)
    )
}