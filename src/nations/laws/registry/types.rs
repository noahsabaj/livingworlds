//! Registry-specific types
//!
//! Types used across registry modules for law proposals and changes.

use serde::{Deserialize, Serialize};
use crate::nations::laws::types::LawId;
use crate::simulation::PressureType;

/// A proposed law being debated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedLaw {
    pub law_id: LawId,
    pub initial_support: f32,
    pub current_support: f32,
    pub debate_days_remaining: f32,
    /// The pressure that triggered this proposal
    pub triggering_pressure: Option<PressureType>,
}

/// Record of a law change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawChange {
    pub law_id: LawId,
    pub change_type: LawChangeType,
    pub year: i32,
}

/// Type of law change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawChangeType {
    Enacted,
    Repealed,
    Modified,
}