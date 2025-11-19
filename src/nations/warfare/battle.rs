//! Battle auto-resolution system
//!
//! This module implements automatic battle resolution using:
//! - Dice roll randomness (30% variance)
//! - Defender advantage (1.2x multiplier)
//! - Magnitude-based casualties

use bevy::prelude::*;
use rand::Rng;
use crate::nations::{Nation, NationHistory, BattleOutcome};

/// Battle resolution configuration
pub struct BattleConfig {
    /// Random variance factor (0.0-1.0, default 0.3 = 30% randomness)
    pub randomness: f32,
    /// Defender advantage multiplier (default 1.2)
    pub defender_bonus: f32,
    /// Casualties multiplier for defeats (default 0.1 = 10% losses)
    pub casualty_rate: f32,
}

impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            randomness: 0.3,
            defender_bonus: 1.2,
            casualty_rate: 0.1,
        }
    }
}

/// Battle between two nations
pub struct Battle {
    pub attacker_entity: Entity,
    pub defender_entity: Entity,
    pub attacker_strength: f32,
    pub defender_strength: f32,
    pub config: BattleConfig,
}

/// Battle result
pub struct BattleResult {
    pub winner: Entity,
    pub loser: Entity,
    pub magnitude: f32, // 0.0 (narrow) to 1.0 (crushing victory)
    pub attacker_casualties: f32,
    pub defender_casualties: f32,
}

impl Battle {
    /// Auto-resolve the battle
    pub fn resolve<R: Rng>(self, rng: &mut R) -> BattleResult {
        // Apply defender bonus
        let effective_defender = self.defender_strength * self.config.defender_bonus;

        // Add randomness via dice roll (uniform distribution)
        let attacker_roll = rng.gen_range(1.0 - self.config.randomness..=1.0 + self.config.randomness);
        let defender_roll = rng.gen_range(1.0 - self.config.randomness..=1.0 + self.config.randomness);

        let final_attacker = self.attacker_strength * attacker_roll;
        let final_defender = effective_defender * defender_roll;

        // Determine winner
        let (winner, loser, winner_strength, loser_strength) = if final_attacker > final_defender {
            (self.attacker_entity, self.defender_entity, final_attacker, final_defender)
        } else {
            (self.defender_entity, self.attacker_entity, final_defender, final_attacker)
        };

        // Calculate victory magnitude (0.0 = narrow, 1.0 = crushing)
        let strength_ratio = winner_strength / loser_strength.max(1.0);
        let magnitude = ((strength_ratio - 1.0) / 2.0).min(1.0); // Max at 3:1 ratio

        // Calculate casualties
        let attacker_casualties = if winner == self.attacker_entity {
            self.attacker_strength * self.config.casualty_rate * (1.0 - magnitude) // Winner takes fewer casualties
        } else {
            self.attacker_strength * self.config.casualty_rate * (1.0 + magnitude) // Loser takes more casualties
        };

        let defender_casualties = if winner == self.defender_entity {
            self.defender_strength * self.config.casualty_rate * (1.0 - magnitude)
        } else {
            self.defender_strength * self.config.casualty_rate * (1.0 + magnitude)
        };

        BattleResult {
            winner,
            loser,
            magnitude,
            attacker_casualties,
            defender_casualties,
        }
    }
}

/// Record battle outcome in nation history
pub fn record_battle_outcome(
    nation: &mut NationHistory,
    outcome: BattleOutcome,
) {
    // Add to recent battles deque (max 10)
    nation.recent_battles.push_back(outcome.clone());
    if nation.recent_battles.len() > 10 {
        nation.recent_battles.pop_front();
    }

    // Update totals
    match outcome {
        BattleOutcome::Victory(_) => nation.total_victories += 1,
        BattleOutcome::Defeat(_) => nation.total_defeats += 1,
        BattleOutcome::Stalemate => {}
    }
}
