//! Patterns that indicate viral potential in events

use crate::nations::{DramaEvent, DramaEventType};

/// Patterns that make events more likely to go viral
#[derive(Debug, Clone)]
pub enum ViralPattern {
    // Age-based absurdity
    BabyRuler { max_age: u32 },
    ElderlyNewParent { min_age: u32 },

    // Unexpected outcomes
    DavidVsGoliath,     // Weak defeats strong
    IronicDeath,        // Death related to their quirk
    ProphecyFulfilled,  // Something predicted happens

    // Relationship drama
    LoveTriangle,
    ForbiddenLove,
    BetrayalByFamily,
    SecretRevealed,

    // Absurd events
    AnimalInvolvement,   // Animals in governance
    CompletelyAbsurd,    // War on concepts, etc.
    ChainReaction,       // One event causes many

    // Character moments
    QuirkCausesProblem,
    MadnessManifests,
    UnexpectedHeroism,
    TotalCowardice,

    // Meta patterns
    PerfectTiming,       // Happens at crucial moment
    HistoryRepeats,      // Same thing happened before
    Coincidence,         // Multiple related events
}

impl ViralPattern {
    /// Get all default patterns for detection
    pub fn all_patterns() -> Vec<Self> {
        vec![
            ViralPattern::BabyRuler { max_age: 5 },
            ViralPattern::ElderlyNewParent { min_age: 70 },
            ViralPattern::DavidVsGoliath,
            ViralPattern::IronicDeath,
            ViralPattern::LoveTriangle,
            ViralPattern::SecretRevealed,
            ViralPattern::AnimalInvolvement,
            ViralPattern::CompletelyAbsurd,
            ViralPattern::QuirkCausesProblem,
            ViralPattern::MadnessManifests,
        ]
    }

    /// Score how well an event matches this pattern
    pub fn score_event(&self, event: &DramaEvent) -> f32 {
        match self {
            ViralPattern::BabyRuler { max_age } => {
                if let DramaEventType::BabyRuler { age_months, .. } = &event.event_type {
                    if *age_months < max_age * 12 {
                        return 1.0; // Perfect match
                    }
                }
                0.0
            }
            ViralPattern::CompletelyAbsurd => {
                if let DramaEventType::AbsurdEvent { .. } = &event.event_type {
                    return 1.0;
                }
                if let DramaEventType::DescentIntoMadness { first_sign, .. } = &event.event_type {
                    if first_sign.contains("declared war on") || first_sign.contains("banned") {
                        return 0.9;
                    }
                }
                0.0
            }
            ViralPattern::SecretRevealed => {
                if let DramaEventType::SecretExposed { .. } = &event.event_type {
                    return 1.0;
                }
                if let DramaEventType::AffairRevealed { .. } = &event.event_type {
                    return 0.9;
                }
                0.0
            }
            ViralPattern::LoveTriangle => {
                if let DramaEventType::LoveTriangle { .. } = &event.event_type {
                    return 1.0;
                }
                0.0
            }
            ViralPattern::MadnessManifests => {
                if let DramaEventType::DescentIntoMadness { .. } = &event.event_type {
                    return 1.0;
                }
                if let DramaEventType::DrunkenIncident { .. } = &event.event_type {
                    return 0.7;
                }
                0.0
            }
            ViralPattern::QuirkCausesProblem => {
                if let DramaEventType::QuirkIncident { .. } = &event.event_type {
                    return 1.0;
                }
                0.0
            }
            ViralPattern::AnimalInvolvement => {
                if let DramaEventType::AnimalIncident { .. } = &event.event_type {
                    return 1.0;
                }
                0.0
            }
            _ => 0.0, // Pattern not yet implemented
        }
    }

    /// Get a description of this pattern for UI/debugging
    pub fn description(&self) -> &str {
        match self {
            ViralPattern::BabyRuler { .. } => "Baby ruler doing adult things",
            ViralPattern::ElderlyNewParent { .. } => "Very old character having children",
            ViralPattern::DavidVsGoliath => "Underdog victory",
            ViralPattern::IronicDeath => "Death related to character's quirk",
            ViralPattern::ProphecyFulfilled => "Predicted event comes true",
            ViralPattern::LoveTriangle => "Complex romantic entanglement",
            ViralPattern::ForbiddenLove => "Romance between enemies",
            ViralPattern::BetrayalByFamily => "Family member betrayal",
            ViralPattern::SecretRevealed => "Hidden information exposed",
            ViralPattern::AnimalInvolvement => "Animals in human roles",
            ViralPattern::CompletelyAbsurd => "Completely nonsensical event",
            ViralPattern::ChainReaction => "Event causes cascade of consequences",
            ViralPattern::QuirkCausesProblem => "Character quirk causes incident",
            ViralPattern::MadnessManifests => "Character goes insane",
            ViralPattern::UnexpectedHeroism => "Coward becomes hero",
            ViralPattern::TotalCowardice => "Hero becomes coward",
            ViralPattern::PerfectTiming => "Event happens at perfect moment",
            ViralPattern::HistoryRepeats => "History repeating itself",
            ViralPattern::Coincidence => "Multiple unlikely events align",
        }
    }
}