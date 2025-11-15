//! Caption generation for social media posts

use crate::nations::{DramaEvent, DramaEventType};

/// Generate an appropriate caption for social media based on the event
pub fn generate_caption(event: &DramaEvent) -> String {
    match &event.event_type {
        DramaEventType::BabyRuler { baby, action, age_months } => {
            format!(
                "{}-month-old ruler {} just {} - LivingWorlds",
                age_months, baby, action
            )
        }
        DramaEventType::DescentIntoMadness { character, first_sign, .. } => {
            format!(
                "{} has lost it! They just {} - LivingWorlds",
                character, first_sign
            )
        }
        DramaEventType::AffairRevealed { lovers, spouse_reaction, .. } => {
            format!(
                "SCANDAL! {} and {} caught having affair! Spouse reaction: {:?} - LivingWorlds",
                lovers.0, lovers.1, spouse_reaction
            )
        }
        DramaEventType::AbsurdEvent { description, perpetrator, reasoning } => {
            format!(
                "{} just {}. Their reasoning? \"{}\" - LivingWorlds",
                perpetrator, description, reasoning
            )
        }
        DramaEventType::SecretExposed { character,  .. } => {
            format!(
                "SECRET REVEALED! {} was hiding something big... - LivingWorlds",
                character
            )
        }
        DramaEventType::Duel { challenger, challenged, reason, winner, .. } => {
            let winner_text = winner.as_ref().map_or("No one survived", |w| w.as_str());
            format!(
                "DUEL! {} challenged {} because of {:?}. Winner: {} - LivingWorlds",
                challenger, challenged, reason, winner_text
            )
        }
        DramaEventType::ChildProdigy { child, age, achievement } => {
            format!(
                "PRODIGY! {} (age {}) just {}! - LivingWorlds",
                child, age, achievement
            )
        }
        _ => "Something incredible just happened in Living Worlds!".to_string(),
    }
}

/// Generate a short caption for platforms with character limits
pub fn generate_short_caption(event: &DramaEvent) -> String {
    match &event.event_type {
        DramaEventType::BabyRuler { age_months, .. } => {
            format!("{}-month-old ruler does politics - LivingWorlds", age_months)
        }
        DramaEventType::DescentIntoMadness { character, .. } => {
            format!("{} goes mad! - LivingWorlds", character)
        }
        _ => "Viral moment in LivingWorlds!".to_string(),
    }
}