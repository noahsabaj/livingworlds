//! Main viral moment detector implementation

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::content_creation::export::captions::generate_caption;
use crate::content_creation::export::platforms::recommend_platforms;
use crate::content_creation::types::{ViralMomentDetected, ViralPotential};
use crate::nations::DramaEvent;

use super::patterns::ViralPattern;
use super::scoring::{calculate_base_viral_score, calculate_viral_score};
use super::types::DetectionConfig;

// Viral potential thresholds
const LEGENDARY_THRESHOLD: f32 = 0.9;
const HIGH_THRESHOLD: f32 = 0.7;
const MEDIUM_THRESHOLD: f32 = 0.5;
const LOW_THRESHOLD: f32 = 0.3;

/// Resource that detects potentially viral moments in gameplay
#[derive(Resource)]
pub struct ViralMomentDetector {
    /// Recent events to analyze for patterns
    event_buffer: VecDeque<DramaEvent>,
    /// Configuration for detection
    config: DetectionConfig,
    /// Patterns to look for
    patterns: Vec<ViralPattern>,
}

impl Default for ViralMomentDetector {
    fn default() -> Self {
        Self {
            event_buffer: VecDeque::with_capacity(100),
            config: DetectionConfig::default(),
            patterns: ViralPattern::all_patterns(),
        }
    }
}

impl ViralMomentDetector {
    /// Add an event to the buffer for analysis
    pub fn add_event(&mut self, event: DramaEvent) {
        self.event_buffer.push_back(event);
        if self.event_buffer.len() > self.config.buffer_size {
            self.event_buffer.pop_front();
        }
    }

    /// Check if an event meets viral threshold
    pub fn is_viral(&self, score: f32) -> bool {
        score >= self.config.viral_threshold
    }

    /// Get viral potential level from score
    pub fn get_potential(&self, score: f32) -> ViralPotential {
        match score {
            s if s >= LEGENDARY_THRESHOLD => ViralPotential::Legendary,
            s if s >= HIGH_THRESHOLD => ViralPotential::High,
            s if s >= MEDIUM_THRESHOLD => ViralPotential::Medium,
            s if s >= LOW_THRESHOLD => ViralPotential::Low,
            _ => ViralPotential::None,
        }
    }
}

/// System to detect viral moments from drama events
pub fn detect_viral_moments(
    mut detector: ResMut<ViralMomentDetector>,
    mut drama_events: MessageReader<DramaEvent>,
    mut viral_events: MessageWriter<ViralMomentDetected>,
    time: Res<Time>,
) {
    for event in drama_events.read() {
        // Add to buffer for pattern analysis
        detector.add_event(event.clone());

        // Calculate base viral score from event type
        let base_score = calculate_base_viral_score(event);

        // Check patterns and calculate final score
        let matched_patterns: Vec<_> = detector.patterns.iter()
            .filter_map(|pattern| {
                let score = pattern.score_event(event);
                if score > 0.0 {
                    Some(pattern.clone())
                } else {
                    None
                }
            })
            .collect();

        let final_score = calculate_viral_score(base_score, &matched_patterns, event);

        // Check if it meets viral threshold
        if detector.is_viral(final_score) {
            let caption = generate_caption(event);
            let platforms = recommend_platforms(event, final_score);

            viral_events.write(ViralMomentDetected {
                event: event.clone(),
                viral_score: final_score,
                potential: detector.get_potential(final_score),
                suggested_caption: caption,
                recommended_platforms: platforms,
                timestamp: time.elapsed_secs(),
            });
        }
    }
}