//! Legitimacy calculation methods
//!
//! This module contains the complex calculations for determining
//! government legitimacy based on various factors.

use super::factors::{
    DivineApproval, ElectoralMandate, InstitutionalControl, LegitimacyFactors, RevolutionaryFervor,
};
use super::weights::LegitimacyWeights;
use crate::nations::governance::{CrisisFactors, GovernmentType};

impl LegitimacyFactors {
    /// Calculate total legitimacy with full implementation
    pub fn calculate_legitimacy_full(&self, gov_type: GovernmentType) -> f32 {
        let weights = LegitimacyWeights::for_government_type(gov_type);

        let mut total = 0.0;
        let mut weight_sum = 0.0;

        // Electoral legitimacy
        if let Some(electoral) = &self.electoral_mandate {
            let electoral_score = calculate_electoral_legitimacy(electoral);
            total += electoral_score * weights.electoral;
            weight_sum += weights.electoral;
        }

        // Divine approval
        if let Some(divine) = &self.divine_approval {
            let divine_score = calculate_divine_legitimacy(divine);
            total += divine_score * weights.divine;
            weight_sum += weights.divine;
        }

        // Revolutionary fervor
        if let Some(revolutionary) = &self.revolutionary_fervor {
            let revolutionary_score = calculate_revolutionary_legitimacy(revolutionary);
            total += revolutionary_score * weights.revolutionary;
            weight_sum += weights.revolutionary;
        }

        // Universal factors
        total += self.prosperity_bonus * weights.prosperity;
        weight_sum += weights.prosperity;

        total += self.administrative_efficiency * weights.efficiency;
        weight_sum += weights.efficiency;

        total += self.public_approval_rating * weights.popularity;
        weight_sum += weights.popularity;

        // Normalize
        if weight_sum > 0.0 {
            (total / weight_sum).clamp(0.0, 1.0)
        } else {
            0.5
        }
    }
}

fn calculate_electoral_legitimacy(electoral: &ElectoralMandate) -> f32 {
    let mut score = 0.0;
    score += (electoral.vote_percentage - 0.5) * 2.0;
    score += electoral.coalition_strength * 0.3;
    let election_proximity = 1.0 - (electoral.days_until_election as f32 / 1460.0);
    score -= election_proximity * 0.2;
    if electoral.election_was_contested {
        score -= 0.3;
    }
    score.clamp(0.0, 1.0)
}

fn calculate_divine_legitimacy(divine: &DivineApproval) -> f32 {
    let mut score = 0.0;
    score += divine.religious_unity * 0.3;
    score += divine.clergy_support * 0.3;
    score += (divine.holy_sites_controlled as f32 / 5.0).min(1.0) * 0.2;
    score += (divine.recent_miracles as f32 / 3.0).min(1.0) * 0.2;
    score.clamp(0.0, 1.0)
}

fn calculate_revolutionary_legitimacy(revolutionary: &RevolutionaryFervor) -> f32 {
    let mut score = 0.0;
    score += revolutionary.ideological_purity * 0.4;
    score += revolutionary.revolutionary_zeal * 0.4;
    let time_decay = 1.0 - (revolutionary.days_since_revolution as f32 / 3650.0).min(1.0);
    score *= time_decay;
    let purge_boost = (revolutionary.counter_revolutionaries_purged as f32 / 100.0).min(0.2);
    score += purge_boost;
    score.clamp(0.0, 1.0)
}