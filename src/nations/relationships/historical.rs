//! Historical relationships
//!
//! Relationships for tracking former rulers, dynasties, and historical
//! territorial claims. These provide context for diplomatic tensions,
//! claims, and narrative events.

use bevy::prelude::*;

/// A nation formerly ruled a province or territory
///
/// Points to the Province/Territory entity that was historically controlled.
/// Used for historical claims and narrative context.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = FormerRulers)]
pub struct FormerlyRuled(pub Entity);

/// Nations that formerly ruled this territory
///
/// Automatically maintained by Bevy's relationship system.
/// Preserves historical context even after territory changes hands.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = FormerlyRuled, linked_spawn)]
pub struct FormerRulers(Vec<Entity>);

impl FormerRulers {
    pub fn rulers(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_history(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn ruler_count(&self) -> usize {
        self.0.len()
    }
}

/// A territory is considered historical homeland by a nation
///
/// Points to the Province/Territory entity. Stronger than HasClaimOn,
/// represents deep cultural/historical connection.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = FormerlyOwnedBy)]
pub struct HistoricalTerritory(pub Entity);

/// Nations that consider this their historical territory
///
/// Automatically maintained by Bevy's relationship system.
/// Used for casus belli generation and cultural identity.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = HistoricalTerritory, linked_spawn)]
pub struct FormerlyOwnedBy(Vec<Entity>);

impl FormerlyOwnedBy {
    pub fn claimants(&self) -> &[Entity] {
        &self.0
    }

    pub fn is_contested_homeland(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn claimant_count(&self) -> usize {
        self.0.len()
    }
}
