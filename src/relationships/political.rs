//! Political Relationships - Core nation/province governance
//!
//! This module defines the fundamental political relationships that form the backbone
//! of Living Worlds' political system: nation control over provinces and capital cities.

use bevy::prelude::*;

// ================================================================================================
// PROVINCE OWNERSHIP RELATIONSHIPS
// ================================================================================================

/// A province is controlled by a nation
/// When applied to a Province entity, automatically creates `Controls` on the Nation entity
///
/// This replaces the manual `Province.owner: Option<NationId>` field with automatic
/// bidirectional relationship tracking.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Controls)]
pub struct ControlledBy(pub Entity);

/// Reverse relationship: A nation controls multiple provinces
/// Automatically maintained by Bevy when `ControlledBy` is added to provinces
///
/// This replaces the manual `ProvinceOwnershipCache` HashMap with automatic tracking.
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ControlledBy, linked_spawn)]
pub struct Controls(Vec<Entity>); // Private for safety - Bevy handles internal access

impl Controls {
    /// Get read-only access to controlled provinces (convenience method)
    pub fn provinces(&self) -> &[Entity] {
        &self.0
    }

    /// Check if nation controls a specific province
    pub fn contains_province(&self, province: Entity) -> bool {
        self.0.contains(&province)
    }

    /// Get number of controlled provinces
    pub fn province_count(&self) -> usize {
        self.0.len()
    }

    /// Check if nation controls any provinces
    pub fn has_provinces(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// CAPITAL CITY RELATIONSHIPS
// ================================================================================================

/// A nation has a specific province as its capital
/// One-to-one relationship from Nation to Province
///
/// This replaces the manual `Nation.capital_province: u32` field with entity relationships.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = CapitalOf)]
pub struct HasCapital(pub Entity);

/// Reverse relationship: A province is the capital of a nation
/// Automatically maintained by Bevy when `HasCapital` is added
/// NOTE: Should typically only contain one nation (validated by validation systems)
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = HasCapital, linked_spawn)]
pub struct CapitalOf(Vec<Entity>); // Private for safety - Bevy handles internal access

impl CapitalOf {
    /// Get read-only access to nations that have this province as capital
    /// Should typically be only one nation
    pub fn nations(&self) -> &[Entity] {
        &self.0
    }

    /// Get the primary nation that has this as capital (first one)
    /// Returns None if no nation has this as capital
    pub fn primary_nation(&self) -> Option<Entity> {
        self.0.first().copied()
    }

    /// Check if this province is capital of a specific nation
    pub fn is_capital_of(&self, nation: Entity) -> bool {
        self.0.contains(&nation)
    }
}

// ================================================================================================
// RULING RELATIONSHIPS - Dynasties and leadership
// ================================================================================================

/// A person/house rules over a nation
/// Enables dynasty and succession mechanics
///
/// This allows for implementing royal families, successions, and political intrigue.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = RuledBy)]
pub struct RulesOver(pub Entity);

/// Reverse relationship: A nation is ruled by a person/house
/// Automatically maintained by Bevy when `RulesOver` is added
/// NOTE: Should typically only contain one ruler (validated by validation systems)
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = RulesOver, linked_spawn)]
pub struct RuledBy(Vec<Entity>); // Private for safety - Bevy handles internal access

impl RuledBy {
    /// Get read-only access to rulers of this nation
    /// Should typically be only one ruler
    pub fn rulers(&self) -> &[Entity] {
        &self.0
    }

    /// Get the current ruler (first one)
    /// Returns None if nation has no ruler
    pub fn current_ruler(&self) -> Option<Entity> {
        self.0.first().copied()
    }

    /// Check if this nation is ruled by a specific ruler
    pub fn is_ruled_by(&self, ruler: Entity) -> bool {
        self.0.contains(&ruler)
    }

    /// Check if nation has any ruler
    pub fn has_ruler(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// QUERY FUNCTIONS - Efficient political queries
// ================================================================================================

/// Query function for territories controlled by nations
/// Replaces ProvinceOwnershipCache.by_nation HashMap
/// Returns debug information about nation territories
pub fn query_nation_territories(
    nations_query: &Query<(Entity, &Controls)>,
) -> Vec<(Entity, usize)> {
    nations_query
        .iter()
        .map(|(nation_entity, controls)| (nation_entity, controls.provinces().len()))
        .collect()
}

/// Query function for province ownership
/// Replaces Province.owner field lookups
/// Returns list of (province, nation) pairs
pub fn query_province_owners(
    provinces_query: &Query<(Entity, Option<&ControlledBy>)>,
) -> Vec<(Entity, Entity)> {
    provinces_query
        .iter()
        .filter_map(|(province_entity, controlled_by)| {
            controlled_by.map(|cb| (province_entity, cb.0))
        })
        .collect()
}

/// Query function for nation capitals
/// Replaces Nation.capital_province field lookups
/// Returns list of (nation, capital_province) pairs
pub fn query_nation_capitals(
    nations_query: &Query<(Entity, Option<&HasCapital>)>,
) -> Vec<(Entity, Entity)> {
    nations_query
        .iter()
        .filter_map(|(nation_entity, has_capital)| has_capital.map(|hc| (nation_entity, hc.0)))
        .collect()
}

// ================================================================================================
// VALIDATION SYSTEMS - Political integrity
// ================================================================================================

/// Validates that all provinces have valid nation owners
/// Replaces manual cache validation with entity relationship validation
pub fn validate_province_ownership(
    provinces_query: Query<(Entity, Option<&ControlledBy>)>,
    nations_query: Query<Entity>,
) {
    let valid_nations: std::collections::HashSet<Entity> = nations_query.iter().collect();

    for (province_entity, controlled_by) in provinces_query.iter() {
        if let Some(controller) = controlled_by {
            if !valid_nations.contains(&controller.0) {
                warn!(
                    "Province {:?} is controlled by invalid nation {:?}",
                    province_entity, controller.0
                );
            }
        }
    }
}

/// Validates that nation capitals are valid provinces
pub fn validate_capital_assignments(
    nations_query: Query<(Entity, Option<&HasCapital>)>,
    provinces_query: Query<Entity>,
) {
    let valid_provinces: std::collections::HashSet<Entity> = provinces_query.iter().collect();

    for (nation_entity, has_capital) in nations_query.iter() {
        if let Some(capital) = has_capital {
            if !valid_provinces.contains(&capital.0) {
                warn!(
                    "Nation {:?} has invalid capital province {:?}",
                    nation_entity, capital.0
                );
            }
        }
    }
}
