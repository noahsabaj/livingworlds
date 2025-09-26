//! Law conflict checking
//!
//! Determines which laws conflict with a proposed law,
//! preventing incompatible laws from coexisting.

use crate::nations::laws::{
    registry::{LawRegistry, NationLaws},
    types::LawId,
};

/// Check if adding a law would create conflicts
pub fn check_law_conflicts(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
    proposed_law: LawId,
) -> Vec<LawId> {
    let mut conflicts = Vec::new();

    // Check direct conflicts
    let law_conflicts = registry.get_conflicts(proposed_law);
    for &conflict_id in &law_conflicts {
        if nation_laws.is_active(conflict_id) {
            conflicts.push(conflict_id);
        }
    }

    conflicts
}