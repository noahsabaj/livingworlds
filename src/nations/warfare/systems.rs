//! War lifecycle systems
//!
//! Systems for war declaration, battle resolution, and peace.

use bevy::prelude::*;
use rand::thread_rng;
use crate::nations::{Nation, NationHistory, BattleOutcome, ParticipatesInWar, Attacking};
use super::{War, WarGoal, CasusBelli, Battle, BattleConfig, record_battle_outcome, WarOutcome};

/// Event: Nation declares war
#[derive(Debug, Clone, Message)]
pub struct DeclareWarEvent {
    pub attacker: Entity,
    pub defender: Entity,
    pub war_goal: WarGoal,
    pub casus_belli: CasusBelli,
}

/// Event: Battle occurs in active war
#[derive(Debug, Clone, Message)]
pub struct BattleEvent {
    pub war_id: u32,
    pub attacker: Entity,
    pub defender: Entity,
}

/// Event: War ends
#[derive(Debug, Clone, Message)]
pub struct WarEndEvent {
    pub war_id: u32,
    pub outcome: WarOutcome,
}

/// Declare war system
pub fn process_war_declarations(
    mut commands: Commands,
    mut war_events: MessageReader<DeclareWarEvent>,
    nations_query: Query<&Nation>,
    game_time: Res<crate::simulation::GameTime>,
    mut next_war_id: Local<u32>,
) {
    for event in war_events.read() {
        let Ok(attacker_nation) = nations_query.get(event.attacker) else {
            continue;
        };
        let Ok(defender_nation) = nations_query.get(event.defender) else {
            continue;
        };

        *next_war_id += 1;

        // Spawn war entity
        let war_entity = commands.spawn(War {
            war_id: *next_war_id,
            war_goal: event.war_goal.clone(),
            casus_belli: event.casus_belli,
            start_year: game_time.current_year(),
            war_score: 0.0,
            battles_fought: 0,
        }).id();

        // Create relationships
        commands.entity(event.attacker)
            .insert(Attacking(event.defender))
            .insert(ParticipatesInWar(war_entity));

        commands.entity(event.defender)
            .insert(ParticipatesInWar(war_entity));

        info!(
            "War declared: {} vs {} (CB: {:?})",
            attacker_nation.name, defender_nation.name, event.casus_belli
        );
    }
}

/// Process battles in active wars
pub fn process_battle_events(
    mut battle_events: MessageReader<BattleEvent>,
    mut wars_query: Query<&mut War>,
    nations_query: Query<&Nation>,
    mut histories_query: Query<&mut NationHistory>,
    attacking_query: Query<&Attacking>,
) {
    for event in battle_events.read() {
        // Find the war
        let mut war_opt = None;
        for war in &mut wars_query {
            if war.war_id == event.war_id {
                war_opt = Some(war);
                break;
            }
        }

        let Some(mut war) = war_opt else {
            continue;
        };

        let Ok(attacker) = nations_query.get(event.attacker) else {
            continue;
        };
        let Ok(defender) = nations_query.get(event.defender) else {
            continue;
        };

        // Resolve battle
        let battle = Battle {
            attacker_entity: event.attacker,
            defender_entity: event.defender,
            attacker_strength: attacker.military_strength,
            defender_strength: defender.military_strength,
            config: BattleConfig::default(),
        };

        let result = battle.resolve(&mut thread_rng());

        // Update war score based on which side is attacking
        // Check if attacker in battle is the attacker in war
        let is_war_attacker = attacking_query.get(event.attacker).is_ok();
        let score_change = result.magnitude * 10.0; // Max 10 points per battle
        if result.winner == event.attacker {
            if is_war_attacker {
                war.war_score += score_change;
            } else {
                war.war_score -= score_change;
            }
        } else {
            if is_war_attacker {
                war.war_score -= score_change;
            } else {
                war.war_score += score_change;
            }
        }
        war.battles_fought += 1;

        // Record in histories
        if let Ok(mut attacker_history) = histories_query.get_mut(event.attacker) {
            let outcome = if result.winner == event.attacker {
                BattleOutcome::Victory(result.magnitude)
            } else {
                BattleOutcome::Defeat(result.magnitude)
            };
            record_battle_outcome(&mut attacker_history, outcome);
        }

        if let Ok(mut defender_history) = histories_query.get_mut(event.defender) {
            let outcome = if result.winner == event.defender {
                BattleOutcome::Victory(result.magnitude)
            } else {
                BattleOutcome::Defeat(result.magnitude)
            };
            record_battle_outcome(&mut defender_history, outcome);
        }

        info!(
            "Battle in war {}: {} won (magnitude: {:.2}, war score: {:.1})",
            war.war_id,
            if result.winner == event.attacker {
                &attacker.name
            } else {
                &defender.name
            },
            result.magnitude,
            war.war_score
        );
    }
}

/// Check for war resolution (automatic peace at certain thresholds)
pub fn check_war_resolution(
    wars_query: Query<&War>,
    mut war_end_events: MessageWriter<WarEndEvent>,
) {
    for war in &wars_query {
        let outcome = if war.war_score >= 100.0 {
            Some(WarOutcome::AttackerVictory)
        } else if war.war_score <= -100.0 {
            Some(WarOutcome::DefenderVictory)
        } else if war.battles_fought >= 20 && war.war_score.abs() < 10.0 {
            Some(WarOutcome::WhitePeace) // Stalemate after many battles
        } else {
            None
        };

        if let Some(outcome) = outcome {
            war_end_events.write(WarEndEvent {
                war_id: war.war_id,
                outcome,
            });
        }
    }
}
