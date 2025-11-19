//! Nation neighbor relationships
//!
//! Relationships for tracking land borders and naval range connections
//! between nations. These replace the old NationNeighborCache.

use bevy::prelude::*;

/// A nation shares a land border with another nation
///
/// Created when nations have provinces that are directly adjacent.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = LandNeighbors)]
pub struct LandNeighborOf(pub Entity);

/// Nations that share land borders with this nation
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = LandNeighborOf, linked_spawn)]
pub struct LandNeighbors(Vec<Entity>);

impl LandNeighbors {
    pub fn neighbors(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_land_neighbors(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

/// A nation is within naval strike range of another nation
///
/// Created when nations have coastal provinces projecting power across water.
#[derive(Component, Debug, Clone, Reflect)]
#[relationship(relationship_target = NavalNeighbors)]
pub struct NavalNeighborOf(pub Entity);

/// Nations within naval range of this nation
///
/// Automatically maintained by Bevy's relationship system.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[relationship_target(relationship = NavalNeighborOf, linked_spawn)]
pub struct NavalNeighbors(Vec<Entity>);

impl NavalNeighbors {
    pub fn neighbors(&self) -> &[Entity] {
        &self.0
    }

    pub fn has_naval_neighbors(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}
