//! Viral scoring algorithms

use crate::nations::{DramaEvent, EventVisibility};

use super::patterns::ViralPattern;

/// Calculate the final viral score for an event
pub fn calculate_viral_score(
    base_score: f32,
    matched_patterns: &[ViralPattern],
    event: &DramaEvent,
) -> f32 {
    let mut final_score = base_score;

    // Add bonus for each matched pattern
    for pattern in matched_patterns {
        final_score += pattern.score_event(event) * 0.3;
    }

    // Visibility multiplier - public events are more viral
    let visibility_multiplier = match event.visibility {
        EventVisibility::Legendary => 1.5,
        EventVisibility::Public => 1.2,
        EventVisibility::CourtGossip => 1.0,
        EventVisibility::Rumor => 0.8,
        EventVisibility::Secret => 0.5,
    };

    final_score *= visibility_multiplier;

    // Multiple patterns create synergy
    if matched_patterns.len() > 2 {
        final_score *= 1.0 + (matched_patterns.len() as f32 * 0.1);
    }

    // Cap at 1.0
    final_score.min(1.0)
}