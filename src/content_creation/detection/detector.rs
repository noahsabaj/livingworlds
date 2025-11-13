//! Main viral moment detector implementation

use bevy::prelude::*;
use std::collections::VecDeque;

use crate::content_creation::types::{ViralMomentDetected, ViralPotential};
use crate::nations::{Character, DramaEvent, EventImportance};

use super::patterns::ViralPattern;
use super::scoring::calculate_viral_score;
use super::types::DetectionConfig;

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
            s if s >= 0.9 => ViralPotential::Legendary,
            s if s >= 0.7 => ViralPotential::High,
            s if s >= 0.5 => ViralPotential::Medium,
            s if s >= 0.3 => ViralPotential::Low,
            _ => ViralPotential::None,
        }
    }
}

/// System to detect viral moments from drama events
pub fn detect_viral_moments(
    mut detector: ResMut<ViralMomentDetector>,
    mut drama_events: MessageReader<DramaEvent>,
    mut viral_events: MessageWriter<ViralMomentDetected>,
    characters: Query<&Character>,
    time: Res<Time>,
) {
    for event in drama_events.read() {
        // Add to buffer for pattern analysis
        detector.add_event(event.clone());

        // Calculate viral score
        let base_score = match event.importance {
            EventImportance::Legendary => 1.0,
            EventImportance::Major => 0.7,
            EventImportance::Significant => 0.4,
            EventImportance::Notable => 0.2,
            EventImportance::Trivial => 0.0,
        };

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
            let caption = generate_caption(event, &characters);
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

// Helper functions for caption and platform generation
fn generate_caption(event: &DramaEvent, characters: &Query<&Character>) -> String {
    use crate::nations::DramaEventType;

    match &event.event_type {
        DramaEventType::BabyRuler { baby, action, age_months } => {
            format!(
                "{}-month-old ruler {} just {} 👑👶 #LivingWorlds #BabyRuler",
                age_months, baby, action
            )
        }
        DramaEventType::DescentIntoMadness { character, first_sign, .. } => {
            format!(
                "{} has lost it! They just {} 🤪 #LivingWorlds #MadRuler",
                character, first_sign
            )
        }
        _ => "Something incredible just happened in Living Worlds! #LivingWorlds".to_string(),
    }
}

fn recommend_platforms(event: &DramaEvent, viral_score: f32) -> Vec<crate::content_creation::types::SocialPlatform> {
    use crate::content_creation::types::SocialPlatform;
    use crate::nations::DramaEventType;

    let mut platforms = Vec::new();

    // TikTok loves absurd short moments
    if viral_score > 0.8 {
        platforms.push(SocialPlatform::TikTok);
    }

    // Reddit loves detailed drama
    if matches!(event.event_type,
        DramaEventType::Betrayal { .. } |
        DramaEventType::SuccessionCrisis { .. } |
        DramaEventType::SecretExposed { .. }
    ) {
        platforms.push(SocialPlatform::Reddit);
    }

    // Twitter for quick viral moments
    if matches!(event.event_type,
        DramaEventType::BabyRuler { .. } |
        DramaEventType::AbsurdEvent { .. }
    ) {
        platforms.push(SocialPlatform::Twitter);
    }

    platforms
}