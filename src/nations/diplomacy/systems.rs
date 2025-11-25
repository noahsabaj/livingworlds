//! Diplomatic systems for CB evaluation
//!
//! Systems for evaluating available casus belli options.

use bevy::prelude::*;
use crate::nations::{Nation, Governance};
use crate::nations::warfare::CasusBelli;
use super::casus_belli::CasusBelliExt;

/// Evaluate available casus belli for a nation against all neighbors
pub fn evaluate_available_casus_belli(
    nation_id: crate::nations::NationId,
    nation: &Nation,
    governance: &Governance,
    target_id: crate::nations::NationId,
    target: &Nation,
    target_governance: &Governance,
    is_neighbor: bool,
) -> Vec<CasusBelli> {
    let mut available = Vec::new();

    // Check each CB type
    if CasusBelli::can_justify(
        CasusBelli::BorderDispute,
        nation,
        target,
        governance,
        target_governance,
        is_neighbor,
    ) {
        available.push(CasusBelli::BorderDispute);
    }

    if CasusBelli::can_justify(
        CasusBelli::IdeologicalConflict,
        nation,
        target,
        governance,
        target_governance,
        is_neighbor,
    ) {
        available.push(CasusBelli::IdeologicalConflict);
    }

    // Always available (but costly)
    available.push(CasusBelli::NoCasusBelli);

    available
}
