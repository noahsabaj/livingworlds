//! Law voting system
//!
//! System that processes completed debates and conducts votes.

use bevy::prelude::*;

use crate::nations::laws::passage::{trigger_law_vote, LawVoteResult};
use crate::nations::laws::registry::{LawRegistry, NationLaws, ActiveLaws};
use crate::nations::laws::types::{LawEnactmentEvent, LawRepealEvent, LawStatus};
use crate::nations::{Nation, Governance};
use crate::simulation::GameTime;

/// System to process law votes when debate ends
pub fn process_law_votes_system(
    mut nations: Query<(Entity, &Nation, &Governance, &mut NationLaws)>,
    registry: Res<LawRegistry>,
    time: Res<GameTime>,
    mut active_laws: ResMut<ActiveLaws>,
    mut enactment_events: MessageWriter<LawEnactmentEvent>,
    repeal_events: MessageWriter<LawRepealEvent>,
) {
    for (entity, nation, governance, mut nation_laws) in &mut nations {
        // Process completed debates
        let mut completed_proposals = Vec::new();
        for (i, proposed) in nation_laws.proposed_laws.iter().enumerate() {
            if proposed.debate_days_remaining <= 0.0 {
                completed_proposals.push(i);
            }
        }

        // Vote on completed proposals (in reverse to maintain indices)
        for &idx in completed_proposals.iter().rev() {
            let proposed = nation_laws.proposed_laws.remove(idx);
            let vote_result = trigger_law_vote(&proposed, nation, governance, &registry);

            match vote_result {
                LawVoteResult::Passed { final_support, margin } => {
                    // Enact the law
                    if let Some(law) = registry.get_law(proposed.law_id) {
                        nation_laws.enact_law(
                            proposed.law_id,
                            &law.effects,
                            (time.current_year()) as i32,
                        );

                        // Update global tracking
                        active_laws.on_law_enacted(nation.id, proposed.law_id);

                        // Fire event
                        enactment_events.write(LawEnactmentEvent {
                            nation_entity: entity,
                            nation_name: nation.name.clone(),
                            law_id: proposed.law_id,
                            law_name: law.name.clone(),
                            category: law.category,
                        });

                        info!(
                            "{} enacted {} (support: {:.1}%, margin: {:.1}%)",
                            nation.name,
                            law.name,
                            final_support * 100.0,
                            margin * 100.0
                        );
                    }
                }
                LawVoteResult::Failed { reason } => {
                    // Law failed, add cooldown
                    nation_laws.proposal_cooldowns.insert(proposed.law_id, 365.0);

                    // Reset status
                    nation_laws.law_status.insert(proposed.law_id, LawStatus::Inactive);

                    debug!(
                        "{} failed to pass law {}: {}",
                        nation.name, proposed.law_id.0, reason
                    );
                }
            }
        }
    }
}