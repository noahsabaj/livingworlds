//! System for updating nation history over time
//!
//! Tracks historical events and updates ruler/war status

use bevy::prelude::*;
use crate::nations::{Nation, NationHistory};
use crate::nations::relationships::AttackedBy;
use crate::simulation::GameTime;

/// Update nation histories each game year
pub fn update_nation_histories(
    mut nations_query: Query<(Entity, &Nation, &mut NationHistory, Option<&AttackedBy>)>,
    game_time: Res<GameTime>,
    mut last_year: Local<u32>,
) {
    let current_year = game_time.current_year();

    // Only update once per year
    if current_year <= *last_year {
        return;
    }
    *last_year = current_year;

    for (entity, nation, mut history, attacked_by) in &mut nations_query {
        // Check if at war via AttackedBy relationship
        let is_at_war = attacked_by.is_some() && attacked_by.unwrap().is_under_attack();

        // Update yearly statistics
        history.yearly_update(is_at_war);

        // Update treasury tracking
        if nation.treasury > history.peak_treasury {
            history.peak_treasury = nation.treasury;
        }
        if nation.treasury < history.lowest_treasury {
            history.lowest_treasury = nation.treasury;
        }

        // Check for economic events
        if nation.treasury < 100.0 && history.lowest_treasury > 500.0 {
            history.record_event(crate::nations::HistoricalEvent::EconomicCrisis {
                year: current_year,
                severity: 1.0 - (nation.treasury / 1000.0),
            });
        }

        // Check for golden age
        if nation.stability > 0.9 && nation.treasury > history.peak_treasury * 0.9 {
            history.record_event(crate::nations::HistoricalEvent::GoldenAge {
                year: current_year,
                prosperity: nation.stability,
            });
        }

        // Ruler succession check (simplified)
        if history.ruler.age > 70 && rand::random::<f32>() < 0.1 {
            let old_ruler = history.ruler.name.clone();
            let new_ruler = format!(
                "{} II",
                crate::name_generator::NameGenerator::new()
                    .generate(crate::name_generator::NameType::Person {
                        gender: if rand::random::<bool>() {
                            crate::name_generator::Gender::Male
                        } else {
                            crate::name_generator::Gender::Female
                        },
                        culture: crate::name_generator::Culture::Western, // TODO: Use actual culture
                        role: crate::name_generator::PersonRole::Noble,
                    })
            );

            history.ruler.name = new_ruler.clone();
            history.ruler.age = 25;
            history.ruler.years_ruling = 0;
            history.ruler.legitimacy = if history.ruler.has_heir { 0.8 } else { 0.5 };
            history.ruler.has_heir = false;
            history.ruler.personality = crate::nations::RulerTraits::random();

            let has_heir = history.ruler.has_heir;
            history.record_event(crate::nations::HistoricalEvent::RulerChanged {
                year: current_year,
                old_ruler,
                new_ruler,
                reason: if has_heir {
                    crate::nations::SuccessionType::Natural
                } else {
                    crate::nations::SuccessionType::Death
                },
            });
        }
    }
}

/// System to track battles and update history
pub fn track_battle_outcomes(
    mut battle_events: MessageReader<BattleEvent>,
    mut nations_query: Query<&mut NationHistory>,
) {
    for event in battle_events.read() {
        // Update victor's history
        if let Ok(mut history) = nations_query.get_mut(event.victor) {
            history.record_battle(crate::nations::BattleOutcome::Victory(event.magnitude));
        }

        // Update loser's history
        if let Ok(mut history) = nations_query.get_mut(event.loser) {
            history.record_battle(crate::nations::BattleOutcome::Defeat(event.magnitude));
        }
    }
}

/// Event for tracking battle results
#[derive(Message)]
pub struct BattleEvent {
    pub victor: Entity,
    pub loser: Entity,
    pub magnitude: f32, // 0.0 to 1.0, how decisive the battle was
}

/// System to track war declarations and peace
///
/// NOTE: War status is now managed via AttackedBy relationship components.
/// This system only records historical events.
pub fn track_war_status(
    mut war_events: MessageReader<WarStatusEvent>,
    mut nations_query: Query<(&Nation, &mut NationHistory)>,
) {
    for event in war_events.read() {
        match event {
            WarStatusEvent::WarDeclared { aggressor, defender, year } => {
                // Collect nation names first to avoid borrow conflicts
                let aggressor_name = nations_query.get(*aggressor).map(|(n, _)| n.name.clone()).ok();
                let defender_name = nations_query.get(*defender).map(|(n, _)| n.name.clone()).ok();

                // Update aggressor's history
                if let Ok((nation, mut history)) = nations_query.get_mut(*aggressor) {
                    history.total_wars += 1;
                    history.years_at_war = 0;

                    if let Some(enemy_name) = defender_name.clone() {
                        history.record_event(crate::nations::HistoricalEvent::WarDeclared {
                            year: *year,
                            enemy: enemy_name,
                            aggressor: true,
                        });
                    }
                }

                // Update defender's history
                if let Ok((nation, mut history)) = nations_query.get_mut(*defender) {
                    history.total_wars += 1;
                    history.years_at_war = 0;

                    if let Some(enemy_name) = aggressor_name {
                        history.record_event(crate::nations::HistoricalEvent::WarDeclared {
                            year: *year,
                            enemy: enemy_name,
                            aggressor: false,
                        });
                    }
                }
            }
            WarStatusEvent::PeaceDeclared { nation_a, nation_b, year, result } => {
                // Collect nation names first
                let nation_a_name = nations_query.get(*nation_a).map(|(n, _)| n.name.clone()).ok();
                let nation_b_name = nations_query.get(*nation_b).map(|(n, _)| n.name.clone()).ok();

                // Update nation A's history
                if let Ok((nation, mut history)) = nations_query.get_mut(*nation_a) {
                    history.years_at_peace = 0;

                    if let Some(enemy_name) = nation_b_name.clone() {
                        history.record_event(crate::nations::HistoricalEvent::WarEnded {
                            year: *year,
                            enemy: enemy_name,
                            result: *result,
                        });
                    }
                }

                // Update nation B's history
                if let Ok((nation, mut history)) = nations_query.get_mut(*nation_b) {
                    history.years_at_peace = 0;

                    if let Some(enemy_name) = nation_a_name {
                        history.record_event(crate::nations::HistoricalEvent::WarEnded {
                            year: *year,
                            enemy: enemy_name,
                            result: *result,
                        });
                    }
                }
            }
        }
    }
}

/// Event for tracking war status changes
#[derive(Message)]
pub enum WarStatusEvent {
    WarDeclared {
        aggressor: Entity,
        defender: Entity,
        year: u32,
    },
    PeaceDeclared {
        nation_a: Entity,
        nation_b: Entity,
        year: u32,
        result: crate::nations::WarResult,
    },
}