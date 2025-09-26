//! Types for law passage and reforms
//!
//! Core data structures used throughout the passage system.

use crate::nations::laws::types::LawId;
use crate::simulation::PressureType;

/// Proposal for a new law
#[derive(Debug, Clone)]
pub struct LawProposal {
    pub law_id: LawId,
    pub initial_support: f32,
    pub debate_days: f32,
    pub pressure_motivation: PressureType,
    pub conflicts_to_repeal: Vec<LawId>,
}

/// Result of a law vote
#[derive(Debug, Clone)]
pub enum LawVoteResult {
    Passed {
        final_support: f32,
        margin: f32,
    },
    Failed {
        reason: String,
    },
}

/// Emergency powers available during crisis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergencyPower {
    MartialLaw,
    SuspendElections,
    EmergencyCouncils,
    Purge,
    TotalMobilization,
}

/// Type of law action during revolution or major reform
#[derive(Debug, Clone, Copy)]
pub enum RevolutionLawAction {
    Enact(LawId),
    Repeal(LawId),
}