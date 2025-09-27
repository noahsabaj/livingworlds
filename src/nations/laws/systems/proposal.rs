//! Law proposal system
//!
//! System that evaluates nation pressures and proposes new laws.

use bevy::prelude::*;

use crate::nations::laws::passage::evaluate_law_passage;
use crate::nations::laws::registry::{LawRegistry, NationLaws};
use crate::nations::{Nation, Governance};
use crate::simulation::{GameTime, PressureVector};

/// System to propose new laws based on nation pressures
pub fn propose_laws_system(
    mut nations: Query<(
        Entity,
        &Nation,
        &Governance,
        &PressureVector,
        &mut NationLaws,
    )>,
    registry: Res<LawRegistry>,
    time: Res<GameTime>,
) {
    for (entity, nation, governance, pressures, mut nation_laws) in &mut nations {
        // Check if it's time to consider new laws (every 30 days)
        if (time.current_day() as i32) % 30 != 0 {
            continue;
        }

        // Evaluate potential law proposals
        if let Some(proposal) = evaluate_law_passage(
            nation,
            governance,
            pressures,
            &nation_laws,
            &registry,
            (time.current_year()) as i32,
        ) {
            // Propose the law with pressure tracking
            nation_laws.propose_law(
                proposal.law_id,
                proposal.initial_support,
                proposal.debate_days,
                Some(proposal.pressure_motivation),
            );

            // Log the proposal
            if let Some(law) = registry.get_law(proposal.law_id) {
                debug!(
                    "{} proposes {} due to {:?} pressure",
                    nation.name, law.name, proposal.pressure_motivation
                );
            }
        }
    }
}