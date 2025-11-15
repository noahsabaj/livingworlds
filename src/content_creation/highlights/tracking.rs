//! Highlight tracking systems

use bevy::prelude::*;
use crate::content_creation::detection::scoring::calculate_simple_viral_score;
use crate::content_creation::export::captions::generate_caption;
use crate::nations::DramaEvent;
use super::reel::HighlightReel;

// Tracking thresholds
const TRACKING_VIRAL_THRESHOLD: f32 = 0.5;
const AUTO_EXPORT_VIRAL_THRESHOLD: f32 = 0.9;

/// System to track drama events and add viral ones to the highlight reel
pub fn track_highlights(
    mut reel: ResMut<HighlightReel>,
    mut drama_events: MessageReader<DramaEvent>,
    time: Res<Time>,
) {
    for event in drama_events.read() {
        let viral_score = calculate_simple_viral_score(event);

        // Only track highly viral moments
        if viral_score > TRACKING_VIRAL_THRESHOLD {
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
            if viral_score > AUTO_EXPORT_VIRAL_THRESHOLD && reel.auto_export {
                trigger_auto_export(highlight_id, &event, viral_score);
            }
        }
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