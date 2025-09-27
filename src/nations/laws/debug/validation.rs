//! Law system validation and consistency checking
//!
//! Validates law data integrity and consistency.

use bevy::prelude::*;
use crate::nations::laws::{NationLaws, LawRegistry, LawId};
use std::collections::{HashSet, HashMap};

/// Report of validation results
#[derive(Debug, Default)]
pub struct LawValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub nations_checked: usize,
    pub laws_checked: usize,
    pub conflicts_found: usize,
    pub orphaned_laws: Vec<LawId>,
}

impl LawValidationReport {
    /// Check if validation passed without errors
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Print the report to console
    pub fn print(&self) {
        info!("=== LAW VALIDATION REPORT ===");
        info!("Nations checked: {}", self.nations_checked);
        info!("Laws checked: {}", self.laws_checked);

        if self.errors.is_empty() {
            info!("✓ No errors found");
        } else {
            error!("✗ {} errors found:", self.errors.len());
            for error in &self.errors {
                error!("  - {}", error);
            }
        }

        if !self.warnings.is_empty() {
            warn!("{} warnings:", self.warnings.len());
            for warning in &self.warnings {
                warn!("  - {}", warning);
            }
        }

        if self.conflicts_found > 0 {
            warn!("{} law conflicts detected", self.conflicts_found);
        }

        if !self.orphaned_laws.is_empty() {
            warn!("{} orphaned laws (laws with invalid IDs)", self.orphaned_laws.len());
        }
    }
}

/// Validate law consistency across all nations
pub fn validate_law_consistency(
    nations_query: Query<(Entity, &NationLaws)>,
    registry: Res<LawRegistry>,
) -> LawValidationReport {
    let mut report = LawValidationReport::default();

    for (entity, nation_laws) in &nations_query {
        report.nations_checked += 1;

        // Check active laws
        for &law_id in &nation_laws.active_laws {
            report.laws_checked += 1;

            // Verify law exists in registry
            if registry.get_law(law_id).is_none() {
                report.errors.push(format!(
                    "Nation {:?} has active law {:?} that doesn't exist in registry",
                    entity, law_id
                ));
                report.orphaned_laws.push(law_id);
            }

            // Check for conflicts within active laws
            for &other_law_id in &nation_laws.active_laws {
                if law_id != other_law_id {
                    if let Some(law) = registry.get_law(law_id) {
                        if law.conflicts_with.contains(&other_law_id) {
                            report.conflicts_found += 1;
                            report.errors.push(format!(
                                "Nation {:?} has conflicting active laws: {:?} and {:?}",
                                entity, law_id, other_law_id
                            ));
                        }
                    }
                }
            }
        }

        // Check proposed laws
        for proposal in &nation_laws.proposed_laws {
            // Verify proposed law exists
            if registry.get_law(proposal.law_id).is_none() {
                report.errors.push(format!(
                    "Nation {:?} has proposed law {:?} that doesn't exist in registry",
                    entity, proposal.law_id
                ));
            }

            // Check if proposed law is already active
            if nation_laws.active_laws.contains(&proposal.law_id) {
                report.errors.push(format!(
                    "Nation {:?} has law {:?} both active and proposed",
                    entity, proposal.law_id
                ));
            }

            // Validate support percentage
            if proposal.current_support < 0.0 || proposal.current_support > 1.0 {
                report.errors.push(format!(
                    "Nation {:?} has invalid support percentage {:.2} for law {:?}",
                    entity, proposal.current_support, proposal.law_id
                ));
            }
        }

        // Check cooldowns
        for (&law_id, &cooldown) in &nation_laws.proposal_cooldowns {
            if cooldown < 0.0 {
                report.warnings.push(format!(
                    "Nation {:?} has negative cooldown {:.2} for law {:?}",
                    entity, cooldown, law_id
                ));
            }
        }

        // Validate combined effects match active laws
        // This is complex and would require recalculating, so just check basics
        if nation_laws.active_laws.is_empty() {
            let effects = &nation_laws.combined_effects;
            // Check if effects are non-zero when no laws are active
            if effects.tax_efficiency_modifier.abs() > 0.001 ||
               effects.stability_change.abs() > 0.001 ||
               effects.army_morale_modifier.abs() > 0.001 {
                report.warnings.push(format!(
                    "Nation {:?} has non-zero combined effects but no active laws",
                    entity
                ));
            }
        }
    }

    report
}

/// Validate a single nation's laws
pub fn validate_nation_laws(
    nation_laws: &NationLaws,
    registry: &LawRegistry,
) -> Vec<String> {
    let mut errors = Vec::new();

    // Check for duplicate active laws
    let mut seen_laws = HashSet::new();
    for &law_id in &nation_laws.active_laws {
        if !seen_laws.insert(law_id) {
            errors.push(format!("Duplicate active law: {:?}", law_id));
        }
    }

    // Check for conflicting active laws
    for &law_id in &nation_laws.active_laws {
        if let Some(law) = registry.get_law(law_id) {
            for &conflict_id in &law.conflicts_with {
                if nation_laws.active_laws.contains(&conflict_id) {
                    errors.push(format!(
                        "Conflicting laws active: {:?} conflicts with {:?}",
                        law_id, conflict_id
                    ));
                }
            }
        }
    }

    // Check proposal duplicates
    let mut seen_proposals = HashSet::new();
    for proposal in &nation_laws.proposed_laws {
        if !seen_proposals.insert(proposal.law_id) {
            errors.push(format!("Duplicate proposed law: {:?}", proposal.law_id));
        }
    }

    errors
}

/// Check for law conflicts in a set of laws
pub fn check_law_conflicts(
    law_ids: &[LawId],
    registry: &LawRegistry,
) -> HashMap<LawId, Vec<LawId>> {
    let mut conflicts = HashMap::new();

    for &law_id in law_ids {
        if let Some(law) = registry.get_law(law_id) {
            let mut law_conflicts = Vec::new();

            for &other_id in law_ids {
                if law_id != other_id && law.conflicts_with.contains(&other_id) {
                    law_conflicts.push(other_id);
                }
            }

            if !law_conflicts.is_empty() {
                conflicts.insert(law_id, law_conflicts);
            }
        }
    }

    conflicts
}

/// System to run validation periodically (debug builds only)
#[cfg(debug_assertions)]
pub fn periodic_law_validation(
    time: Res<Time>,
    mut timer: Local<Timer>,
    nations_query: Query<(Entity, &NationLaws)>,
    registry: Res<LawRegistry>,
) {
    // Run validation every 10 seconds in debug builds
    if timer.duration() == std::time::Duration::ZERO {
        *timer = Timer::from_seconds(10.0, TimerMode::Repeating);
    }

    timer.tick(time.delta());

    if timer.finished() {
        let report = validate_law_consistency(nations_query, registry);

        if !report.is_valid() {
            error!("Law validation failed!");
            report.print();
        } else if !report.warnings.is_empty() {
            warn!("Law validation passed with warnings");
            report.print();
        }
    }
}