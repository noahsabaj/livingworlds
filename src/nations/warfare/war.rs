//! War state tracking and management
//!
//! This module defines war goals, active wars, and war outcomes.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// War goal types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum WarGoal {
    /// Conquer specific provinces
    Conquest {
        target_provinces: Vec<u32>,
    },
    /// Force regime change / subjugation
    Subjugation,
    /// Defensive war (restore territory)
    Liberation {
        provinces_to_liberate: Vec<u32>,
    },
    /// Humiliation (reduce enemy prestige/strength)
    Humiliation,
    /// Total annexation
    Annexation,
}

/// Active war between nations
///
/// Participants are tracked via relationships:
/// - ParticipatesInWar: Nations link to this war entity
/// - Attacking: Attacker links to defender
/// - AttackedBy: Defender tracks all attackers
#[derive(Component, Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct War {
    /// War identifier
    pub war_id: u32,
    /// War goal
    pub war_goal: WarGoal,
    /// Casus belli (justification)
    pub casus_belli: CasusBelli,
    /// War start date
    pub start_year: u32,
    /// War score (-100 to +100, positive = attacker winning)
    pub war_score: f32,
    /// Battles fought
    pub battles_fought: u32,
}

/// Casus belli placeholder (will be defined in diplomacy module)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum CasusBelli {
    /// Border dispute (share land border)
    BorderDispute,
    /// Historical claim (once owned provinces)
    HistoricalClaim,
    /// Ideological conflict (opposing government types)
    IdeologicalConflict,
    /// Ally defense (ally was attacked)
    DefensivePact,
    /// Reconquest (reclaim lost territory)
    Reconquest,
    /// Fabricated claim (needs time and resources)
    FabricatedClaim,
    /// No CB (huge diplomatic penalty)
    NoCasusBelli,
}

/// War outcome when war ends
#[derive(Debug, Clone, Copy)]
pub enum WarOutcome {
    /// Attacker achieves war goal
    AttackerVictory,
    /// Defender repels attacker
    DefenderVictory,
    /// White peace (status quo)
    WhitePeace,
}
