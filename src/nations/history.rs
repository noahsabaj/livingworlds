//! Nation history tracking system
//!
//! Tracks historical events, wars, rulers, and statistics that influence
//! nation behavior and decision-making.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Maximum number of historical events to keep in memory per nation
const MAX_HISTORICAL_EVENTS: usize = 100;

/// Tracks the complete history of a nation
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct NationHistory {
    /// Current ruler information
    pub ruler: RulerInfo,

    /// War and peace tracking
    pub war_status: WarStatus,
    pub years_at_peace: u32,
    pub years_at_war: u32,
    pub total_wars: u32,
    pub total_victories: u32,
    pub total_defeats: u32,

    /// Recent battle outcomes (last 10)
    pub recent_battles: VecDeque<BattleOutcome>,

    /// Historical events log
    pub events: VecDeque<HistoricalEvent>,

    /// Economic history
    pub peak_treasury: f32,
    pub lowest_treasury: f32,
    pub tax_changes: u32,

    /// Expansion history
    pub provinces_gained: u32,
    pub provinces_lost: u32,
    pub expansion_attempts: u32,

    /// Internal stability
    pub reforms_enacted: u32,
    pub rebellions_faced: u32,

    /// Founding information
    pub founded_year: u32,
    pub founding_culture: String,
}

impl Default for NationHistory {
    fn default() -> Self {
        Self {
            ruler: RulerInfo::default(),
            war_status: WarStatus::AtPeace,
            years_at_peace: 0,
            years_at_war: 0,
            total_wars: 0,
            total_victories: 0,
            total_defeats: 0,
            recent_battles: VecDeque::with_capacity(10),
            events: VecDeque::with_capacity(MAX_HISTORICAL_EVENTS),
            peak_treasury: 1000.0,
            lowest_treasury: 1000.0,
            tax_changes: 0,
            provinces_gained: 0,
            provinces_lost: 0,
            expansion_attempts: 0,
            reforms_enacted: 0,
            rebellions_faced: 0,
            founded_year: 0,
            founding_culture: String::from("Unknown"),
        }
    }
}

impl NationHistory {
    /// Create a new history for a newly formed nation
    pub fn new(founding_year: u32, culture: String, ruler_name: String) -> Self {
        Self {
            ruler: RulerInfo {
                name: ruler_name,
                age: 25,
                years_ruling: 0,
                legitimacy: 1.0,
                has_heir: false,
                personality: RulerTraits::random(),
            },
            founded_year: founding_year,
            founding_culture: culture,
            years_at_peace: 0,
            ..Default::default()
        }
    }

    /// Record a historical event
    pub fn record_event(&mut self, event: HistoricalEvent) {
        self.events.push_back(event);
        if self.events.len() > MAX_HISTORICAL_EVENTS {
            self.events.pop_front();
        }
    }

    /// Record a battle outcome
    pub fn record_battle(&mut self, outcome: BattleOutcome) {
        match outcome {
            BattleOutcome::Victory(_) => self.total_victories += 1,
            BattleOutcome::Defeat(_) => self.total_defeats += 1,
            BattleOutcome::Stalemate => {}
        }

        self.recent_battles.push_back(outcome);
        if self.recent_battles.len() > 10 {
            self.recent_battles.pop_front();
        }
    }

    /// Get recent victory rate (last 10 battles)
    pub fn recent_victory_rate(&self) -> f32 {
        if self.recent_battles.is_empty() {
            return 0.5;
        }

        let victories = self.recent_battles
            .iter()
            .filter(|b| matches!(b, BattleOutcome::Victory(_)))
            .count() as f32;

        victories / self.recent_battles.len() as f32
    }

    /// Check if nation has been at peace for a long time
    pub fn is_long_peace(&self) -> bool {
        self.years_at_peace > 20
    }

    /// Check if nation has experienced recent defeats
    pub fn has_recent_defeats(&self) -> bool {
        let recent_defeats = self.recent_battles
            .iter()
            .rev()
            .take(3)
            .filter(|b| matches!(b, BattleOutcome::Defeat(_)))
            .count();
        recent_defeats >= 2
    }

    /// Check if the ruler is experienced
    pub fn has_experienced_ruler(&self) -> bool {
        self.ruler.years_ruling > 10
    }

    /// Update yearly statistics
    pub fn yearly_update(&mut self) {
        self.ruler.age += 1;
        self.ruler.years_ruling += 1;

        match self.war_status {
            WarStatus::AtWar(_) => {
                self.years_at_war += 1;
                self.years_at_peace = 0;
            }
            WarStatus::AtPeace => {
                self.years_at_peace += 1;
            }
        }

        // Natural legitimacy decay/recovery
        if self.years_at_peace > 5 {
            self.ruler.legitimacy = (self.ruler.legitimacy + 0.05).min(1.0);
        }

        // Check for heir
        if !self.ruler.has_heir && self.ruler.age > 30 {
            // 10% chance per year to get an heir after age 30
            self.ruler.has_heir = rand::random::<f32>() < 0.1;
        }
    }
}

/// Information about the current ruler
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct RulerInfo {
    pub name: String,
    pub age: u32,
    pub years_ruling: u32,
    pub legitimacy: f32,  // 0.0 to 1.0
    pub has_heir: bool,
    pub personality: RulerTraits,
}

impl Default for RulerInfo {
    fn default() -> Self {
        Self {
            name: String::from("Unknown Ruler"),
            age: 35,
            years_ruling: 5,
            legitimacy: 0.8,
            has_heir: true,
            personality: RulerTraits::default(),
        }
    }
}

/// Ruler personality traits that affect decision-making
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub struct RulerTraits {
    pub martial: f32,      // Military prowess (-1.0 to 1.0)
    pub diplomatic: f32,   // Diplomatic skill (-1.0 to 1.0)
    pub administrative: f32, // Economic management (-1.0 to 1.0)
    pub ambitious: f32,    // Expansion desire (-1.0 to 1.0)
}

impl RulerTraits {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            martial: rng.gen_range(-1.0..1.0),
            diplomatic: rng.gen_range(-1.0..1.0),
            administrative: rng.gen_range(-1.0..1.0),
            ambitious: rng.gen_range(-1.0..1.0),
        }
    }
}

impl Default for RulerTraits {
    fn default() -> Self {
        Self {
            martial: 0.0,
            diplomatic: 0.0,
            administrative: 0.0,
            ambitious: 0.0,
        }
    }
}

/// Current war status of a nation
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum WarStatus {
    AtPeace,
    AtWar(Vec<super::NationId>),  // List of enemies
}

/// Outcome of a battle
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum BattleOutcome {
    Victory(f32),  // Magnitude of victory (0.0 to 1.0)
    Defeat(f32),   // Magnitude of defeat (0.0 to 1.0)
    Stalemate,
}

/// Types of historical events
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum HistoricalEvent {
    Founded {
        year: u32,
        culture: String,
    },
    WarDeclared {
        year: u32,
        enemy: String,
        aggressor: bool,
    },
    WarEnded {
        year: u32,
        enemy: String,
        result: WarResult,
    },
    RulerChanged {
        year: u32,
        old_ruler: String,
        new_ruler: String,
        reason: SuccessionType,
    },
    ProvinceGained {
        year: u32,
        province_name: String,
        method: AcquisitionMethod,
    },
    ProvinceLost {
        year: u32,
        province_name: String,
        reason: LossReason,
    },
    EconomicCrisis {
        year: u32,
        severity: f32,
    },
    GoldenAge {
        year: u32,
        prosperity: f32,
    },
    ReformEnacted {
        year: u32,
        reform_type: ReformType,
    },
    RebellionFaced {
        year: u32,
        suppressed: bool,
    },
}

/// Result of a war
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum WarResult {
    Victory,
    Defeat,
    WhitePeace,
    Stalemate,
}

/// How a ruler came to power
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum SuccessionType {
    Natural,      // Normal succession
    Death,        // Previous ruler died
    Coup,         // Military takeover
    Revolution,   // Popular uprising
    Abdication,   // Voluntary step down
}

/// How a province was acquired
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum AcquisitionMethod {
    Conquest,
    Diplomatic,
    Settlement,
    Inheritance,
}

/// Why a province was lost
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum LossReason {
    Conquered,
    Rebellion,
    Diplomatic,
    Economic,
}

/// Types of reforms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
pub enum ReformType {
    Military,
    Economic,
    Administrative,
    Cultural,
    Religious,
}

/// Helper function to create initial history for a new nation
pub fn create_initial_history(
    nation_name: &str,
    culture: String,
    founding_year: u32,
    rng: &mut impl rand::Rng,
) -> NationHistory {
    let gender = if rng.gen::<bool>() {
        crate::name_generator::Gender::Male
    } else {
        crate::name_generator::Gender::Female
    };

    let ruler_name = format!("{} I", crate::name_generator::NameGenerator::new()
        .generate(crate::name_generator::NameType::Person {
            gender,
            culture: crate::name_generator::Culture::Western, // TODO: Map culture string to enum
            role: crate::name_generator::PersonRole::Ruler,
        }));

    let mut history = NationHistory::new(founding_year, culture.clone(), ruler_name.clone());

    // Record founding event
    history.record_event(HistoricalEvent::Founded {
        year: founding_year,
        culture,
    });

    // Start with some initial legitimacy based on government type
    history.ruler.legitimacy = rng.gen_range(0.6..0.9);

    history
}