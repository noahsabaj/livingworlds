//! Law voting mechanics
//!
//! Handles the voting process and passage thresholds for proposed laws.

use rand::Rng;

use super::types::LawVoteResult;
use crate::nations::laws::mechanics::get_government_law_affinity;
use crate::nations::laws::registry::{LawRegistry, ProposedLaw};
use crate::nations::laws::types::LawComplexity;
use crate::nations::{Nation, Governance, GovernmentType, GovernmentCategory};

/// Trigger a vote on a proposed law
pub fn trigger_law_vote(
    proposed_law: &ProposedLaw,
    nation: &Nation,
    governance: &Governance,
    registry: &LawRegistry,
) -> LawVoteResult {
    let law = match registry.get_law(proposed_law.law_id) {
        Some(law) => law,
        None => return LawVoteResult::Failed { reason: "Law not found".to_string() },
    };

    // Calculate final support
    let mut final_support = proposed_law.current_support;

    // Government affinity affects the vote
    let gov_affinity = get_government_law_affinity(law, governance.government_type);
    final_support += gov_affinity * 0.2;

    // Stability affects willingness to change
    if nation.stability > 0.7 {
        final_support -= 0.1; // Stable nations resist change
    } else if nation.stability < 0.3 {
        final_support += 0.1; // Unstable nations try anything
    }

    // Add some randomness
    let mut rng = rand::thread_rng();
    final_support += rng.gen_range(-0.1..0.1);

    // Determine threshold based on government type and law complexity
    let threshold = calculate_passage_threshold(
        governance.government_type,
        law.complexity,
        law.is_constitutional,
    );

    if final_support >= threshold {
        LawVoteResult::Passed {
            final_support,
            margin: final_support - threshold,
        }
    } else {
        LawVoteResult::Failed {
            reason: format!("Insufficient support: {:.1}% < {:.1}%",
                final_support * 100.0, threshold * 100.0),
        }
    }
}

/// Calculate the threshold needed to pass a law
pub fn calculate_passage_threshold(
    government: GovernmentType,
    complexity: LawComplexity,
    is_constitutional: bool,
) -> f32 {
    let base_threshold = match government.category() {
        GovernmentCategory::Autocratic => 0.3,   // Ruler decides
        GovernmentCategory::Democratic => 0.5,   // Majority vote
        GovernmentCategory::Anarchist => 0.75,   // Consensus needed
        GovernmentCategory::Corporate => 0.4,    // Board approval
        GovernmentCategory::Theocratic => 0.45,  // Religious approval
        GovernmentCategory::Socialist => 0.5,    // Worker councils
        GovernmentCategory::Monarchic => 0.35,   // Royal decree
        GovernmentCategory::Technocratic => 0.4, // Expert panel
        GovernmentCategory::Tribal => 0.6,       // Elder consensus
    };

    let complexity_modifier = match complexity {
        LawComplexity::Trivial => -0.1,
        LawComplexity::Simple => -0.05,
        LawComplexity::Moderate => 0.0,
        LawComplexity::Complex => 0.1,
        LawComplexity::Revolutionary => 0.2,
    };

    let constitutional_modifier: f32 = if is_constitutional { 0.15 } else { 0.0 };

    (base_threshold + complexity_modifier + constitutional_modifier).clamp(0.2, 0.9)
}