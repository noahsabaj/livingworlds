//! Casus belli justification and validation
//!
//! This module extends the CasusBelli enum with justification logic,
//! diplomatic penalties, and legitimacy costs.

use bevy::prelude::*;
use crate::nations::{Nation, Governance};
use crate::nations::warfare::CasusBelli;

/// CB fabrication state (attached as component to nation entity)
#[derive(Component, Debug, Clone)]
pub struct FabricatingClaim {
    pub target_nation: Entity,
    pub progress: f32,       // 0.0 to 1.0
    pub cost_paid: f32,
}

/// Extension trait for CasusBelli with justification logic
pub trait CasusBelliExt {
    /// Check if nation has valid CB against target
    fn can_justify(
        cb_type: CasusBelli,
        aggressor: &Nation,
        target: &Nation,
        aggressor_gov: &Governance,
        target_gov: &Governance,
        neighbors: bool,
    ) -> bool;

    /// Aggressive expansion penalty multiplier
    fn aggression_penalty(&self) -> f32;

    /// Legitimacy cost for declaring war
    fn legitimacy_cost(&self) -> f32;
}

impl CasusBelliExt for CasusBelli {
    fn can_justify(
        cb_type: CasusBelli,
        _aggressor: &Nation,
        _target: &Nation,
        aggressor_gov: &Governance,
        target_gov: &Governance,
        neighbors: bool,
    ) -> bool {
        match cb_type {
            CasusBelli::BorderDispute => neighbors,
            CasusBelli::IdeologicalConflict => {
                aggressor_gov.government_type.category() != target_gov.government_type.category()
            }
            CasusBelli::NoCasusBelli => true, // Always available but costly
            // Others require historical/relationship data
            _ => false, // TODO: Implement when history tracking exists
        }
    }

    fn aggression_penalty(&self) -> f32 {
        match self {
            CasusBelli::DefensivePact => 0.0,      // Defensive wars have no penalty
            CasusBelli::Reconquest => 0.25,        // Reclaiming own land is justified
            CasusBelli::BorderDispute => 0.5,
            CasusBelli::HistoricalClaim => 0.5,
            CasusBelli::IdeologicalConflict => 0.75,
            CasusBelli::FabricatedClaim => 1.0,
            CasusBelli::NoCasusBelli => 2.0,       // Massive penalty
        }
    }

    fn legitimacy_cost(&self) -> f32 {
        match self {
            CasusBelli::DefensivePact => 0.0,
            CasusBelli::Reconquest => 0.05,
            CasusBelli::BorderDispute => 0.1,
            CasusBelli::HistoricalClaim => 0.1,
            CasusBelli::IdeologicalConflict => 0.15,
            CasusBelli::FabricatedClaim => 0.2,
            CasusBelli::NoCasusBelli => 0.4,       // Huge legitimacy hit
        }
    }
}
