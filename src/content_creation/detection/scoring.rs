//! Viral scoring algorithms

use crate::nations::{DramaEvent, DramaEventType, EventVisibility};

use super::patterns::ViralPattern;

// Scoring constants
const PATTERN_BONUS_MULTIPLIER: f32 = 0.3;
const SYNERGY_MULTIPLIER: f32 = 0.1;
const CONSEQUENCE_BONUS_PER_ITEM: f32 = 0.05;
const MAX_CONSEQUENCE_BONUS: f32 = 0.2;

// Visibility multipliers
const VISIBILITY_LEGENDARY_MULTIPLIER: f32 = 1.5;
const VISIBILITY_PUBLIC_MULTIPLIER: f32 = 1.2;
const VISIBILITY_COURT_GOSSIP_MULTIPLIER: f32 = 1.0;
const VISIBILITY_RUMOR_MULTIPLIER: f32 = 0.8;
const VISIBILITY_SECRET_MULTIPLIER: f32 = 0.5;

/// Calculate base viral score from event type alone
pub fn calculate_base_viral_score(event: &DramaEvent) -> f32 {
    let base_score = match &event.event_type {
        // Extremely viral events
        DramaEventType::BabyRuler { age_months, .. } => {
            // Younger = more viral
            1.0 - (*age_months as f32 / 24.0).min(0.5)
        }
        DramaEventType::DescentIntoMadness { .. } => 0.95,
        DramaEventType::AbsurdEvent { .. } => 0.9,

        // Very viral events
        DramaEventType::AffairRevealed { .. } => 0.85,
        DramaEventType::SecretExposed { .. } => 0.85,
        DramaEventType::Duel { .. } => 0.8,
        DramaEventType::AnimalIncident { .. } => 0.8,
        DramaEventType::ChildProdigy { age, .. } => {
            // Younger prodigies are more viral
            1.0 - (*age as f32 / 18.0).min(0.2)
        }

        // Moderately viral events
        DramaEventType::Betrayal { .. } => 0.7,
        DramaEventType::SuccessionCrisis { .. } => 0.7,
        DramaEventType::DrunkenIncident { .. } => 0.65,
        DramaEventType::QuirkIncident { .. } => 0.6,
        DramaEventType::FamilyFeud { .. } => 0.6,

        // Standard viral events
        DramaEventType::RomanticProposal { .. } => 0.5,
        DramaEventType::EnemiesAlly { .. } => 0.5,
        DramaEventType::AccidentalDeath { .. } => 0.5,

        // Less viral events
        DramaEventType::DeathbedConfession { .. } => 0.4,
        DramaEventType::PersonalDuel { .. } => 0.4,
        DramaEventType::DisgracedLeader { .. } => 0.45,
        DramaEventType::HeroicSacrifice { .. } => 0.55,

        // Catch-all for any other drama event types
        _ => 0.5,
    };

    // Boost score based on consequences
    let consequence_boost = (event.consequences.len() as f32 * CONSEQUENCE_BONUS_PER_ITEM).min(MAX_CONSEQUENCE_BONUS);

    // Boost for important characters
    // TODO: Check if event involves ruler when that field is available
    let importance_boost = 0.0;

    base_score + consequence_boost + importance_boost
}

/// Calculate the final viral score for an event with pattern matching
pub fn calculate_viral_score(
    base_score: f32,
    matched_patterns: &[ViralPattern],
    event: &DramaEvent,
) -> f32 {
    let mut final_score = base_score;

    // Add bonus for each matched pattern
    for pattern in matched_patterns {
        final_score += pattern.score_event(event) * PATTERN_BONUS_MULTIPLIER;
    }

    // Visibility multiplier - public events are more viral
    let visibility_multiplier = match event.visibility {
        EventVisibility::Legendary => VISIBILITY_LEGENDARY_MULTIPLIER,
        EventVisibility::Public => VISIBILITY_PUBLIC_MULTIPLIER,
        EventVisibility::CourtGossip => VISIBILITY_COURT_GOSSIP_MULTIPLIER,
        EventVisibility::Rumor => VISIBILITY_RUMOR_MULTIPLIER,
        EventVisibility::Secret => VISIBILITY_SECRET_MULTIPLIER,
    };

    final_score *= visibility_multiplier;

    // Multiple patterns create synergy
    if matched_patterns.len() > 2 {
        final_score *= 1.0 + (matched_patterns.len() as f32 * SYNERGY_MULTIPLIER);
    }

    // Cap at 1.0
    final_score.min(1.0)
}

/// Calculate viral score without pattern matching (for simple use cases)
pub fn calculate_simple_viral_score(event: &DramaEvent) -> f32 {
    let base_score = calculate_base_viral_score(event);

    // Apply visibility multiplier
    let visibility_multiplier = match event.visibility {
        EventVisibility::Legendary => VISIBILITY_LEGENDARY_MULTIPLIER,
        EventVisibility::Public => VISIBILITY_PUBLIC_MULTIPLIER,
        EventVisibility::CourtGossip => VISIBILITY_COURT_GOSSIP_MULTIPLIER,
        EventVisibility::Rumor => VISIBILITY_RUMOR_MULTIPLIER,
        EventVisibility::Secret => VISIBILITY_SECRET_MULTIPLIER,
    };

    (base_score * visibility_multiplier).min(1.0)
}