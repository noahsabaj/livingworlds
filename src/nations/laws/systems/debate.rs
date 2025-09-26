//! Law debate system
//!
//! System that manages ongoing law debates and updates support levels.

use bevy::prelude::*;
use rand::random;

use crate::nations::laws::registry::NationLaws;
use crate::nations::laws::types::LawStatus;
use crate::nations::Nation;
use crate::simulation::GameTime;

/// System to update ongoing law debates
pub fn update_law_debates_system(
    mut nations: Query<(&Nation, &mut NationLaws)>,
    time: Res<GameTime>,
) {
    let delta_days = 1.0; // Assuming 1 day per update

    for (nation, mut nation_laws) in &mut nations {
        // Update proposed laws
        nation_laws.proposed_laws.retain_mut(|proposed| {
            proposed.debate_days_remaining -= delta_days;

            // Update support based on events (simplified for now)
            proposed.current_support += random::<f32>() * 0.02 - 0.01;
            proposed.current_support = proposed.current_support.clamp(0.0, 1.0);

            // Update status in the map
            nation_laws.law_status.insert(
                proposed.law_id,
                LawStatus::Proposed {
                    support: proposed.current_support,
                    days_remaining: proposed.debate_days_remaining,
                },
            );

            // Keep if debate is ongoing
            proposed.debate_days_remaining > 0.0
        });
    }
}