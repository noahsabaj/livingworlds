//! Nation-specific law tracking
//!
//! Manages laws for individual nations including enactment,
//! repeal, proposals, and effect tracking.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::nations::laws::types::{LawId, LawStatus, LawEffects};
use crate::simulation::PressureType;
use super::types::{ProposedLaw, LawChange, LawChangeType};

/// Active law with its original effects at enactment time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveLaw {
    /// The law identifier
    pub law_id: LawId,
    /// The effects that were applied when this law was enacted
    /// Stored to ensure proper removal on repeal
    pub original_effects: LawEffects,
    /// Year the law was enacted
    pub enacted_year: i32,
}

/// Tracks laws for an individual nation
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct NationLaws {
    /// Currently active laws (for quick lookup)
    pub active_laws: HashSet<LawId>,

    /// Detailed active law data with original effects
    pub active_law_data: HashMap<LawId, ActiveLaw>,

    /// Status of each law
    pub law_status: HashMap<LawId, LawStatus>,

    /// Combined effects of all active laws
    pub combined_effects: LawEffects,

    /// History of law changes
    pub history: VecDeque<LawChange>,

    /// Laws currently being debated
    pub proposed_laws: Vec<ProposedLaw>,

    /// Cooldown before same law can be proposed again
    pub proposal_cooldowns: HashMap<LawId, f32>,
}

impl NationLaws {
    /// Check if a law is currently active
    pub fn is_active(&self, law_id: LawId) -> bool {
        self.active_laws.contains(&law_id)
    }

    /// Get the status of a specific law
    pub fn get_status(&self, law_id: LawId) -> LawStatus {
        self.law_status
            .get(&law_id)
            .copied()
            .unwrap_or(LawStatus::Inactive)
    }

    /// Enact a new law
    pub fn enact_law(&mut self, law_id: LawId, effects: &LawEffects, year: i32) {
        // Store in quick lookup set
        self.active_laws.insert(law_id);

        // Store detailed data with original effects for proper repeal
        self.active_law_data.insert(
            law_id,
            ActiveLaw {
                law_id,
                original_effects: effects.clone(),
                enacted_year: year,
            },
        );

        self.law_status.insert(
            law_id,
            LawStatus::Active {
                enacted_date: year,
                popularity: 0.5, // Start at neutral popularity
            },
        );

        // Add to history
        self.history.push_back(LawChange {
            law_id,
            change_type: LawChangeType::Enacted,
            year,
        });

        // Keep history size limited
        if self.history.len() > 100 {
            self.history.pop_front();
        }

        // Recalculate combined effects
        self.add_effects(effects);
    }

    /// Repeal an active law
    pub fn repeal_law(&mut self, law_id: LawId, year: i32) {
        // Remove from quick lookup
        self.active_laws.remove(&law_id);

        // Get and remove the active law data to properly reverse effects
        if let Some(active_law) = self.active_law_data.remove(&law_id) {
            // Use the ORIGINAL effects that were applied, not current definition
            self.subtract_effects(&active_law.original_effects);
        } else {
            warn!("Attempted to repeal law {:?} that has no active data", law_id);
        }

        self.law_status.insert(law_id, LawStatus::Inactive);

        // Add to history
        self.history.push_back(LawChange {
            law_id,
            change_type: LawChangeType::Repealed,
            year,
        });

        // Keep history size limited
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Propose a new law for consideration
    pub fn propose_law(&mut self, law_id: LawId, support: f32, debate_days: f32, pressure: Option<PressureType>) {
        // Check if already active
        if self.is_active(law_id) {
            warn!("Cannot propose law {:?} - already active", law_id);
            return;
        }

        // Check if already proposed (prevent duplicates)
        if self.proposed_laws.iter().any(|p| p.law_id == law_id) {
            warn!("Cannot propose law {:?} - already under debate", law_id);
            return;
        }

        // Check cooldown
        if let Some(&cooldown) = self.proposal_cooldowns.get(&law_id) {
            if cooldown > 0.0 {
                return; // Still on cooldown
            }
        }

        self.proposed_laws.push(ProposedLaw {
            law_id,
            initial_support: support,
            current_support: support,
            debate_days_remaining: debate_days,
            triggering_pressure: pressure,
        });

        self.law_status.insert(
            law_id,
            LawStatus::Proposed {
                support,
                days_remaining: debate_days,
            },
        );
    }

    /// Update cooldowns (called each day)
    pub fn update_cooldowns(&mut self, delta: f32) {
        self.proposal_cooldowns.retain(|_, cooldown| {
            *cooldown -= delta;
            *cooldown > 0.0
        });
    }

    /// Add law effects to combined total with diminishing returns
    fn add_effects(&mut self, effects: &LawEffects) {
        self.combined_effects.add_with_diminishing_returns(effects);
    }

    /// Subtract law effects from combined total
    fn subtract_effects(&mut self, effects: &LawEffects) {
        self.combined_effects.subtract(effects);
    }

    /// Fully recalculate combined effects from active laws
    /// Use this to recover from any calculation drift
    pub fn recalculate_combined_effects(&mut self, registry: &crate::nations::laws::registry::LawRegistry) {
        // Start fresh
        self.combined_effects = LawEffects::default();

        // Rebuild from active laws using stored original effects
        for active_law in self.active_law_data.values() {
            self.combined_effects.add_with_diminishing_returns(&active_law.original_effects);
        }

        info!("Recalculated combined effects for nation with {} active laws",
              self.active_laws.len());
    }

    /// Validate law data consistency
    pub fn validate_consistency(&self) -> Result<(), String> {
        // Check that active_laws and active_law_data are in sync
        if self.active_laws.len() != self.active_law_data.len() {
            return Err(format!("Active laws set has {} entries but active_law_data has {}",
                             self.active_laws.len(), self.active_law_data.len()));
        }

        // Check all active laws have data
        for &law_id in &self.active_laws {
            if !self.active_law_data.contains_key(&law_id) {
                return Err(format!("Active law {:?} has no data entry", law_id));
            }
        }

        // Check all data entries are in active set
        for &law_id in self.active_law_data.keys() {
            if !self.active_laws.contains(&law_id) {
                return Err(format!("Law data for {:?} but not in active set", law_id));
            }
        }

        Ok(())
    }
}