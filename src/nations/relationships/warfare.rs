//! Warfare-related entity relationships
//!
//! Relationships for war participation, battle sides, and military conflicts.

use bevy::prelude::*;

/// A nation participates in a war
///
/// Points to the War entity that this nation is involved in.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = WarParticipants)]
pub struct ParticipatesInWar(pub Entity);

/// War entity tracks all participating nations
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = ParticipatesInWar, linked_spawn)]
pub struct WarParticipants(Vec<Entity>);

impl WarParticipants {
    pub fn participants(&self) -> &[Entity] {
        &self.0
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

/// Attacker in a war (points to defender)
///
/// Creates asymmetric attack relationship between nations.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = AttackedBy)]
pub struct Attacking(pub Entity);

/// Nations being attacked
///
/// Tracks all nations that are attacking this nation.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = Attacking, linked_spawn)]
pub struct AttackedBy(Vec<Entity>);

impl AttackedBy {
    pub fn attackers(&self) -> &[Entity] {
        &self.0
    }

    pub fn is_under_attack(&self) -> bool {
        !self.0.is_empty()
    }
}
