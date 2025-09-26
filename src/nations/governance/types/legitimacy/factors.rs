//! Legitimacy factors and related types
//!
//! This module contains all the structures for tracking positive and negative
//! factors that affect government legitimacy.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::nations::governance::{CrisisFactors, GovernmentCategory, GovernmentType};

/// Comprehensive legitimacy tracking with positive and negative factors
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct LegitimacyFactors {
    // ========== POSITIVE LEGITIMACY SOURCES ==========
    /// Electoral legitimacy (for democracies)
    pub electoral_mandate: Option<ElectoralMandate>,
    /// Divine right (for theocracies/monarchies)
    pub divine_approval: Option<DivineApproval>,
    /// Revolutionary fervor (for revolutionary states)
    pub revolutionary_fervor: Option<RevolutionaryFervor>,
    /// Economic prosperity (universal)
    pub prosperity_bonus: f32,
    /// Military victories
    pub recent_victories: Vec<MilitaryVictory>,
    /// Effective governance
    pub administrative_efficiency: f32,
    /// Popular approval
    pub public_approval_rating: f32,
    /// Tradition and historical precedent
    pub traditional_authority: f32,
    /// Control over key institutions
    pub institutional_control: InstitutionalControl,
    /// International recognition
    pub diplomatic_recognition: f32,

    // ========== NEGATIVE LEGITIMACY FACTORS ==========
    /// Current crises affecting the state
    pub crisis_factors: CrisisFactors,
    /// Contested succession
    pub succession_legitimacy: f32,
    /// Corruption scandal impact
    pub corruption_scandal: Option<CorruptionScandal>,
    /// Failed promises/policies
    pub broken_promises: Vec<BrokenPromise>,
    /// Regional separatism
    pub separatist_movements: Vec<SeparatistMovement>,
    /// Cultural/religious minority unrest
    pub minority_unrest: f32,
    /// Foreign influence/puppet state perception
    pub foreign_puppet_perception: f32,

    // ========== DYNAMIC FACTORS ==========
    /// Recent legitimacy events (positive or negative)
    pub recent_events: Vec<crate::nations::governance::LegitimacyEvent>,
    /// Stability trend over time
    pub stability_trend: f32,
    /// Time since last major crisis
    pub days_since_crisis: u32,
    /// Government competence perception
    pub competence_rating: f32,
}

/// Electoral mandate for democratic governments
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ElectoralMandate {
    pub vote_percentage: f32,
    pub coalition_strength: f32,
    pub days_until_election: u32,
    pub election_was_contested: bool,
}

/// Divine approval for theocratic/religious governments
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct DivineApproval {
    pub religious_unity: f32,
    pub clergy_support: f32,
    pub holy_sites_controlled: u32,
    pub recent_miracles: u32,
}

/// Revolutionary fervor for revolutionary states
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct RevolutionaryFervor {
    pub ideological_purity: f32,
    pub revolutionary_zeal: f32,
    pub days_since_revolution: u32,
    pub counter_revolutionaries_purged: u32,
}

/// Military victory record
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct MilitaryVictory {
    pub war_name: String,
    pub enemy_name: String,
    pub days_ago: u32,
    pub significance: f32,
}

/// Control over state institutions
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct InstitutionalControl {
    pub military_loyalty: f32,
    pub bureaucracy_control: f32,
    pub judiciary_control: f32,
    pub media_control: f32,
    pub police_loyalty: f32,
}

/// Corruption scandal details
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct CorruptionScandal {
    pub scandal_name: String,
    pub severity: f32,
    pub days_since: u32,
    pub officials_implicated: u32,
}

/// Broken promise tracking
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct BrokenPromise {
    pub promise_type: String,
    pub days_since_broken: u32,
    pub severity: f32,
}

/// Separatist movement details
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct SeparatistMovement {
    pub region_name: String,
    pub strength: f32,
    pub provinces_affected: u32,
    pub demands: Vec<String>,
}

impl Default for LegitimacyFactors {
    fn default() -> Self {
        Self {
            electoral_mandate: None,
            divine_approval: None,
            revolutionary_fervor: None,
            prosperity_bonus: 0.5,
            recent_victories: Vec::new(),
            administrative_efficiency: 0.6,
            public_approval_rating: 0.5,
            traditional_authority: 0.0,
            institutional_control: InstitutionalControl {
                military_loyalty: 0.7,
                bureaucracy_control: 0.7,
                judiciary_control: 0.6,
                media_control: 0.5,
                police_loyalty: 0.7,
            },
            diplomatic_recognition: 0.5,
            crisis_factors: CrisisFactors::default(),
            succession_legitimacy: 1.0,
            corruption_scandal: None,
            broken_promises: Vec::new(),
            separatist_movements: Vec::new(),
            minority_unrest: 0.0,
            foreign_puppet_perception: 0.0,
            recent_events: Vec::new(),
            stability_trend: 0.0,
            days_since_crisis: 100,
            competence_rating: 0.5,
        }
    }
}

impl LegitimacyFactors {
    /// Create legitimacy factors appropriate for the government type
    pub fn for_government_type(gov_type: GovernmentType) -> Self {
        let mut factors = Self::default();

        // Set up government-specific legitimacy sources
        match gov_type.category() {
            GovernmentCategory::Democratic => {
                factors.electoral_mandate = Some(ElectoralMandate {
                    vote_percentage: 0.52,
                    coalition_strength: 0.7,
                    days_until_election: 1460,
                    election_was_contested: false,
                });
            }
            GovernmentCategory::Theocratic | GovernmentCategory::Monarchic => {
                factors.divine_approval = Some(DivineApproval {
                    religious_unity: 0.75,
                    clergy_support: 0.8,
                    holy_sites_controlled: 1,
                    recent_miracles: 0,
                });
                factors.traditional_authority = 0.8;
            }
            GovernmentCategory::Socialist | GovernmentCategory::Anarchist => {
                factors.revolutionary_fervor = Some(RevolutionaryFervor {
                    ideological_purity: 0.9,
                    revolutionary_zeal: 0.8,
                    days_since_revolution: 0,
                    counter_revolutionaries_purged: 0,
                });
            }
            _ => {}
        }

        factors
    }

    /// Calculate total legitimacy based on government type
    pub fn calculate_legitimacy(&self, _gov_type: GovernmentType) -> f32 {
        // Stub implementation - full implementation is complex
        // For now, return a simple average of some factors
        let mut total = 0.0;
        let mut count = 0;

        total += self.prosperity_bonus;
        count += 1;

        total += self.administrative_efficiency;
        count += 1;

        total += self.public_approval_rating;
        count += 1;

        if count > 0 {
            (total / count as f32).clamp(0.0, 1.0)
        } else {
            0.5
        }
    }
}