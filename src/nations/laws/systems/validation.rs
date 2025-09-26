//! Validation systems for law data consistency
//!
//! Systems that run in debug builds to catch data corruption
//! and inconsistencies early in development.

use bevy::prelude::*;
use std::collections::HashSet;

use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::laws::types::LawId;
use crate::nations::Nation;

/// Validation results for reporting
#[derive(Debug, Default)]
pub struct ValidationResults {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Run comprehensive validation on all nation law data
#[cfg(debug_assertions)]
pub fn validate_law_data_system(
    nations: Query<(Entity, &Nation, &NationLaws)>,
    registry: Res<LawRegistry>,
    mut last_check: Local<f32>,
    time: Res<Time>,
) {
    // Only validate every 5 seconds to avoid performance impact
    *last_check += time.delta_secs();
    if *last_check < 5.0 {
        return;
    }
    *last_check = 0.0;

    let mut total_errors = 0;
    let mut total_warnings = 0;

    for (entity, nation, nation_laws) in &nations {
        let mut results = ValidationResults::default();

        // Run all validations
        validate_active_laws(&nation_laws, &registry, &mut results);
        validate_proposed_laws(&nation_laws, &registry, &mut results);
        validate_law_history(&nation_laws, &mut results);
        validate_effect_consistency(&nation_laws, &registry, &mut results);
        validate_cooldowns(&nation_laws, &mut results);

        // Report issues
        if !results.errors.is_empty() {
            error!(
                "Nation {} ({:?}) has {} validation errors:",
                nation.name, entity, results.errors.len()
            );
            for error in &results.errors {
                error!("  - {}", error);
            }
            total_errors += results.errors.len();
        }

        if !results.warnings.is_empty() {
            warn!(
                "Nation {} ({:?}) has {} validation warnings:",
                nation.name, entity, results.warnings.len()
            );
            for warning in &results.warnings {
                warn!("  - {}", warning);
            }
            total_warnings += results.warnings.len();
        }
    }

    if total_errors > 0 || total_warnings > 0 {
        warn!(
            "Law validation complete: {} errors, {} warnings across all nations",
            total_errors, total_warnings
        );
    }
}

/// Validate active laws consistency
fn validate_active_laws(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
    results: &mut ValidationResults,
) {
    // Check active_laws and active_law_data are in sync
    if nation_laws.active_laws.len() != nation_laws.active_law_data.len() {
        results.errors.push(format!(
            "Active laws mismatch: {} in set, {} in data",
            nation_laws.active_laws.len(),
            nation_laws.active_law_data.len()
        ));
    }

    // Check all active laws have data entries
    for &law_id in &nation_laws.active_laws {
        if !nation_laws.active_law_data.contains_key(&law_id) {
            results.errors.push(format!(
                "Active law {:?} missing from active_law_data",
                law_id
            ));
        }

        // Check the law exists in registry
        if registry.get_law(law_id).is_none() {
            results.errors.push(format!(
                "Active law {:?} not found in registry",
                law_id
            ));
        }
    }

    // Check all data entries are in active set
    for &law_id in nation_laws.active_law_data.keys() {
        if !nation_laws.active_laws.contains(&law_id) {
            results.errors.push(format!(
                "Law data for {:?} but not in active set",
                law_id
            ));
        }
    }

    // Check for conflicting laws
    for &law_id in &nation_laws.active_laws {
        if let Some(law) = registry.get_law(law_id) {
            for &conflict_id in &law.conflicts_with {
                if nation_laws.active_laws.contains(&conflict_id) {
                    results.errors.push(format!(
                        "Conflicting laws both active: {:?} and {:?}",
                        law_id, conflict_id
                    ));
                }
            }
        }
    }
}

/// Validate proposed laws
fn validate_proposed_laws(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
    results: &mut ValidationResults,
) {
    let mut seen = HashSet::new();

    for proposal in &nation_laws.proposed_laws {
        // Check for duplicates
        if !seen.insert(proposal.law_id) {
            results.errors.push(format!(
                "Duplicate proposal for law {:?}",
                proposal.law_id
            ));
        }

        // Check law exists
        if registry.get_law(proposal.law_id).is_none() {
            results.errors.push(format!(
                "Proposed law {:?} not in registry",
                proposal.law_id
            ));
        }

        // Check not already active
        if nation_laws.active_laws.contains(&proposal.law_id) {
            results.errors.push(format!(
                "Law {:?} is both active and proposed",
                proposal.law_id
            ));
        }

        // Validate support range
        if proposal.current_support < 0.0 || proposal.current_support > 1.0 {
            results.errors.push(format!(
                "Invalid support {:.2} for proposal {:?}",
                proposal.current_support, proposal.law_id
            ));
        }

        // Check debate days
        if proposal.debate_days_remaining < 0.0 {
            results.warnings.push(format!(
                "Negative debate days {:.1} for {:?}",
                proposal.debate_days_remaining, proposal.law_id
            ));
        }
    }
}

/// Validate law history
fn validate_law_history(
    nation_laws: &NationLaws,
    results: &mut ValidationResults,
) {
    if nation_laws.history.len() > 100 {
        results.warnings.push(format!(
            "History exceeds limit: {} entries",
            nation_laws.history.len()
        ));
    }

    // Check for impossible sequences
    let mut last_seen: HashSet<LawId> = HashSet::new();
    for change in &nation_laws.history {
        use crate::nations::laws::registry::LawChangeType;

        match change.change_type {
            LawChangeType::Enacted => {
                if !last_seen.insert(change.law_id) {
                    results.warnings.push(format!(
                        "Law {:?} enacted multiple times in history",
                        change.law_id
                    ));
                }
            }
            LawChangeType::Repealed => {
                if !last_seen.remove(&change.law_id) {
                    results.warnings.push(format!(
                        "Law {:?} repealed but was not enacted in history",
                        change.law_id
                    ));
                }
            }
            _ => {}
        }
    }
}

/// Validate effect consistency
fn validate_effect_consistency(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
    results: &mut ValidationResults,
) {
    // Recalculate what effects should be
    let mut expected_effects = crate::nations::laws::types::LawEffects::default();

    for active_law in nation_laws.active_law_data.values() {
        expected_effects.add_with_diminishing_returns(&active_law.original_effects);
    }

    // Compare key fields (allow small floating point differences)
    let epsilon = 0.001;

    if (nation_laws.combined_effects.tax_efficiency_modifier - expected_effects.tax_efficiency_modifier).abs() > epsilon {
        results.warnings.push(format!(
            "Tax efficiency mismatch: {} vs expected {}",
            nation_laws.combined_effects.tax_efficiency_modifier,
            expected_effects.tax_efficiency_modifier
        ));
    }

    if (nation_laws.combined_effects.stability_change - expected_effects.stability_change).abs() > epsilon {
        results.warnings.push(format!(
            "Stability change mismatch: {} vs expected {}",
            nation_laws.combined_effects.stability_change,
            expected_effects.stability_change
        ));
    }

    // Check more fields as needed...
}

/// Validate cooldowns
fn validate_cooldowns(
    nation_laws: &NationLaws,
    results: &mut ValidationResults,
) {
    for (&law_id, &cooldown) in &nation_laws.proposal_cooldowns {
        if cooldown < 0.0 {
            results.errors.push(format!(
                "Negative cooldown {} for law {:?}",
                cooldown, law_id
            ));
        }

        if cooldown > 365.0 {
            results.warnings.push(format!(
                "Excessive cooldown {} days for law {:?}",
                cooldown, law_id
            ));
        }
    }
}

/// System to periodically recalculate effects to prevent drift
#[cfg(debug_assertions)]
pub fn periodic_recalculation_system(
    mut nations: Query<&mut NationLaws>,
    registry: Res<LawRegistry>,
    mut last_recalc: Local<f32>,
    time: Res<Time>,
) {
    // Recalculate every 60 seconds in debug builds
    *last_recalc += time.delta_secs();
    if *last_recalc < 60.0 {
        return;
    }
    *last_recalc = 0.0;

    let mut recalc_count = 0;
    for mut nation_laws in &mut nations {
        nation_laws.recalculate_combined_effects(&registry);
        recalc_count += 1;
    }

    debug!("Recalculated law effects for {} nations", recalc_count);
}