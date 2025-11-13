//! Highlight tracking systems

use bevy::prelude::*;
use crate::content_creation::types::SocialPlatform;
use crate::nations::{DramaEvent, DramaEventType};
use super::reel::HighlightReel;

/// System to track drama events and add viral ones to the highlight reel
pub fn track_highlights(
    mut reel: ResMut<HighlightReel>,
    mut drama_events: MessageReader<DramaEvent>,
    time: Res<Time>,
) {
    for event in drama_events.read() {
        let viral_score = calculate_viral_score(&event);

        // Only track highly viral moments
        if viral_score > 0.5 {
            let caption = generate_caption(&event);
            let timestamp = time.elapsed_secs();

            let highlight_id = reel.add(
                event.clone(),
                viral_score,
                caption.clone(),
                timestamp
            );

            info!(
                "Added viral moment to highlight reel: {} (score: {:.2}, id: {})",
                caption, viral_score, highlight_id
            );

            // Auto-export extremely viral moments
            if viral_score > 0.9 && reel.auto_export {
                trigger_auto_export(highlight_id, &event, viral_score);
            }
        }
    }
}

/// Calculate how viral a drama event is likely to be
fn calculate_viral_score(event: &DramaEvent) -> f32 {
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
    let consequence_boost = (event.consequences.len() as f32 * 0.05).min(0.2);

    // Boost for important characters
    // TODO: Check if event involves ruler when that field is available
    let importance_boost = 0.0;

    (base_score + consequence_boost + importance_boost).min(1.0)
}

/// Generate a caption for social media
fn generate_caption(event: &DramaEvent) -> String {
    match &event.event_type {
        DramaEventType::BabyRuler { baby, age_months, .. } => {
            format!("{}-month-old {} becomes ruler!", age_months, baby)
        }
        DramaEventType::DescentIntoMadness { character, .. } => {
            format!("{} has lost their mind!", character)
        }
        DramaEventType::AffairRevealed { lovers, .. } => {
            format!("SCANDAL: {} and {} caught in affair!", lovers.0, lovers.1)
        }
        DramaEventType::SecretExposed { character, .. } => {
            format!("{}'s dark secret revealed!", character)
        }
        DramaEventType::AbsurdEvent { description, .. } => {
            format!("You won't believe this: {}", description)
        }
        DramaEventType::Duel { challenger, challenged, .. } => {
            format!("DUEL: {} vs {}!", challenger, challenged)
        }
        _ => "Epic moment in Living Worlds!".to_string(),
    }
}

/// Trigger automatic export for highly viral moments
fn trigger_auto_export(highlight_id: u32, event: &DramaEvent, viral_score: f32) {
    info!(
        "Auto-exporting viral highlight {} with score {:.2}",
        highlight_id, viral_score
    );

    // TODO: Trigger actual export pipeline
    // This would integrate with the export subsystem
}

/// Update the highlight reel UI display
pub fn update_highlight_reel(
    reel: Res<HighlightReel>,
    mut ui_query: Query<&mut Text, With<HighlightReelDisplay>>,
) {
    for mut text in &mut ui_query {
        if reel.highlights.is_empty() {
            text.0 = "No highlights captured yet".to_string();
        } else {
            let top_highlights = reel.get_top(3);
            let mut display = format!("Top {} Viral Moments:\n", top_highlights.len());

            for (i, highlight) in top_highlights.iter().enumerate() {
                display.push_str(&format!(
                    "{}. {} (Score: {:.2})\n",
                    i + 1,
                    highlight.caption,
                    highlight.viral_score
                ));
            }

            text.0 = display;
        }
    }
}

/// Marker component for highlight reel UI
#[derive(Component)]
pub struct HighlightReelDisplay;

/// System to check for platform-specific export opportunities
pub fn check_platform_opportunities(
    reel: Res<HighlightReel>,
    time: Res<Time>,
) {
    // Check every 5 seconds
    static mut LAST_CHECK: f32 = 0.0;
    let current_time = time.elapsed_secs();

    unsafe {
        if current_time - LAST_CHECK < 5.0 {
            return;
        }
        LAST_CHECK = current_time;
    }

    // Get unexported highlights
    let unexported = reel.get_unexported();
    if unexported.is_empty() {
        return;
    }

    // Analyze best platforms for each highlight
    for highlight in unexported {
        let best_platforms = recommend_platforms(&highlight.event, highlight.viral_score);

        if !best_platforms.is_empty() {
            trace!(
                "Highlight {} recommended for platforms: {:?}",
                highlight.id,
                best_platforms
            );
        }
    }
}

/// Recommend platforms based on event type and score
fn recommend_platforms(event: &DramaEvent, viral_score: f32) -> Vec<SocialPlatform> {
    use crate::content_creation::recommend_platforms as platform_recommend;
    platform_recommend(event, viral_score)
}