//! Extended diplomatic relationships
//!
//! Relationships for rivalries, vassalage, tribute, and territorial claims.

use bevy::prelude::*;

/// A nation has a rival relationship with another nation
///
/// Creates symmetric rivalry between two nations.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = Rivals)]
pub struct RivalOf(pub Entity);

/// Nations that are rivals with this nation
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = RivalOf, linked_spawn)]
pub struct Rivals(Vec<Entity>);

impl Rivals {
    pub fn rivals(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_rivals(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

/// A nation is a vassal of an overlord
///
/// Creates hierarchical vassalage relationship.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = Overlord)]
pub struct VassalOf(pub Entity);

/// The overlord component tracks all vassals
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = VassalOf, linked_spawn)]
pub struct Overlord(Vec<Entity>);

impl Overlord {
    pub fn vassals(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_vassals(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn vassal_count(&self) -> usize {
        self.0.len()
    }
}

/// A nation pays tribute to another nation
///
/// Creates tributary relationship where one nation provides resources.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = TributeLord)]
pub struct PaysTributeTo(pub Entity);

/// Nations that pay tribute to this nation
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = PaysTributeTo, linked_spawn)]
pub struct TributeLord(Vec<Entity>);

impl TributeLord {
    pub fn tributaries(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_tributaries(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn tributary_count(&self) -> usize {
        self.0.len()
    }
}

/// A nation has a territorial claim on a province or territory
///
/// Points to the Province or Territory entity being claimed.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = ClaimedBy)]
pub struct HasClaimOn(pub Entity);

/// Provinces/Territories claimed by nations
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = HasClaimOn, linked_spawn)]
pub struct ClaimedBy(Vec<Entity>);

impl ClaimedBy {
    pub fn claimants(&self) -> &[Entity] {
        &self.0
    }

    pub fn is_claimed(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn claimant_count(&self) -> usize {
        self.0.len()
    }
}
