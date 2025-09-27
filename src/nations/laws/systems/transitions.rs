//! Government transition and cooldown systems
//!
//! Handles law changes during government transitions and manages cooldowns.

use bevy::prelude::*;

use crate::nations::laws::passage::{revolutionary_law_changes, RevolutionLawAction};
use crate::nations::laws::registry::{LawRegistry, NationLaws, ActiveLaws};
use crate::nations::laws::types::{LawEnactmentEvent, LawRepealEvent, LawStatus};
use crate::nations::{Nation, GovernmentTransition};
use crate::simulation::GameTime;

/// System to handle law changes during government transitions
pub fn handle_government_transitions_system(
    mut transitions: EventReader<GovernmentTransition>,
    mut nations: Query<(Entity, &Nation, &mut NationLaws)>,
    registry: Res<LawRegistry>,
    time: Res<GameTime>,
    mut active_laws: ResMut<ActiveLaws>,
    mut enactment_events: EventWriter<LawEnactmentEvent>,
    mut repeal_events: EventWriter<LawRepealEvent>,
) {
    for transition in transitions.read() {
        // Find the nation matching the entity
        for (entity, nation, mut nation_laws) in nations.iter_mut() {
            if entity != transition.nation_entity {
                continue;
            }

            // Get revolutionary law changes
            let changes = revolutionary_law_changes(
                transition.from_government,
                transition.to_government,
                &nation_laws,
                &registry,
            );

            // Apply changes
            for change in changes {
                match change {
                    RevolutionLawAction::Enact(law_id) => {
                        if let Some(law) = registry.get_law(law_id) {
                            nation_laws.enact_law(law_id, &law.effects, time.current_year() as i32);
                            active_laws.on_law_enacted(nation.id, law_id);

                            enactment_events.send(LawEnactmentEvent {
                                nation_id: nation.id,
                                nation_name: nation.name.clone(),
                                law_id,
                                law_name: law.name.clone(),
                                category: law.category,
                            });

                            info!(
                                "{} enacts {} after revolution",
                                nation.name, law.name
                            );
                        }
                    }
                    RevolutionLawAction::Repeal(law_id) => {
                        if let Some(law) = registry.get_law(law_id) {
                            let years_active = if let LawStatus::Active { enacted_date, .. } =
                                nation_laws.get_status(law_id)
                            {
                                time.current_year() as i32 - enacted_date
                            } else {
                                0
                            };

                            nation_laws.repeal_law(law_id, time.current_year() as i32);
                            active_laws.on_law_repealed(nation.id, law_id);

                            repeal_events.send(LawRepealEvent {
                                nation_id: nation.id,
                                nation_name: nation.name.clone(),
                                law_id,
                                law_name: law.name.clone(),
                                category: law.category,
                                years_active,
                            });

                            info!(
                                "{} repeals {} after revolution",
                                nation.name, law.name
                            );
                        }
                    }
                }
            }
        }
    }
}

/// System to update law proposal cooldowns
pub fn update_law_cooldowns_system(
    mut nations: Query<&mut NationLaws>,
    time: Res<GameTime>,
) {
    let delta_days = 1.0; // Assuming 1 day per update

    for mut nation_laws in &mut nations {
        nation_laws.update_cooldowns(delta_days);
    }
}