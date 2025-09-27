//! Legislative Relationships - Nation law enactment and governance
//!
//! This module defines relationships between nations and their laws,
//! enabling full entity-based law tracking with automatic bidirectional relationships.

use bevy::prelude::*;

// ================================================================================================
// LAW ENACTMENT RELATIONSHIPS
// ================================================================================================

/// A law is enacted by a nation
/// When applied to a Law entity, automatically creates `EnactedLaws` on the Nation entity
///
/// This replaces manual HashSet<LawId> tracking with entity relationships.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = EnactedLaws)]
pub struct EnactedBy(pub Entity);

/// Reverse relationship: A nation has enacted multiple laws
/// Automatically maintained by Bevy when `EnactedBy` is added to law entities
///
/// This replaces the manual `NationLaws.active_laws: HashSet<LawId>` with automatic tracking.
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = EnactedBy, linked_spawn)]
pub struct EnactedLaws(Vec<Entity>); // Private for safety

impl EnactedLaws {
    /// Get read-only access to enacted law entities
    pub fn laws(&self) -> &[Entity] {
        &self.0
    }

    /// Check if nation has enacted a specific law
    pub fn has_law(&self, law_entity: Entity) -> bool {
        self.0.contains(&law_entity)
    }

    /// Get number of enacted laws
    pub fn law_count(&self) -> usize {
        self.0.len()
    }

    /// Check if nation has any enacted laws
    pub fn has_any_laws(&self) -> bool {
        !self.0.is_empty()
    }
}

// ================================================================================================
// LAW PROPOSAL RELATIONSHIPS
// ================================================================================================

/// A law is being proposed for a nation
/// Represents laws under debate but not yet enacted
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = ProposedLaws)]
pub struct ProposedFor(pub Entity);

/// Reverse relationship: A nation has laws being proposed
/// Automatically maintained by Bevy when `ProposedFor` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ProposedFor, linked_spawn)]
pub struct ProposedLaws(Vec<Entity>); // Private for safety

impl ProposedLaws {
    /// Get read-only access to proposed law entities
    pub fn proposals(&self) -> &[Entity] {
        &self.0
    }

    /// Check if nation has a specific law proposed
    pub fn has_proposal(&self, law_entity: Entity) -> bool {
        self.0.contains(&law_entity)
    }

    /// Get number of proposed laws
    pub fn proposal_count(&self) -> usize {
        self.0.len()
    }
}

// ================================================================================================
// LAW REPEAL RELATIONSHIPS
// ================================================================================================

/// A law was repealed by a nation (historical tracking)
/// Useful for tracking law history and preventing immediate re-enactment
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = RepealedLaws)]
pub struct RepealedBy(pub Entity);

/// Reverse relationship: A nation has repealed laws (historical)
/// Tracks laws that were previously active but have been repealed
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = RepealedBy, linked_spawn)]
pub struct RepealedLaws(Vec<Entity>); // Private for safety

impl RepealedLaws {
    /// Get read-only access to repealed law entities
    pub fn repealed(&self) -> &[Entity] {
        &self.0
    }

    /// Check if nation has repealed a specific law
    pub fn has_repealed(&self, law_entity: Entity) -> bool {
        self.0.contains(&law_entity)
    }
}

// ================================================================================================
// LAW CONFLICT RELATIONSHIPS
// ================================================================================================

/// A law conflicts with another law
/// Used to prevent conflicting laws from being enacted simultaneously
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = ConflictedBy)]
pub struct ConflictsWith(pub Entity);

/// Reverse relationship: Laws that conflict with this one
/// Automatically maintained bidirectional conflict tracking
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = ConflictsWith, linked_spawn)]
pub struct ConflictedBy(Vec<Entity>); // Private for safety

impl ConflictedBy {
    /// Get laws that conflict with this one
    pub fn conflicting_laws(&self) -> &[Entity] {
        &self.0
    }

    /// Check if this law conflicts with another
    pub fn conflicts_with(&self, law_entity: Entity) -> bool {
        self.0.contains(&law_entity)
    }
}

// ================================================================================================
// LAW ENTITY COMPONENTS
// ================================================================================================

/// Component for law entities containing law data
#[derive(Component, Debug, Clone)]
pub struct LawEntity {
    pub law_id: crate::nations::LawId,
    pub name: String,
    pub category: crate::nations::LawCategory,
    pub effects: crate::nations::LawEffects,
    pub enacted_date: Option<i32>, // Game year when enacted
}

/// Component tracking proposal status for proposed laws
#[derive(Component, Debug, Clone)]
pub struct ProposalStatus {
    pub support_percentage: f32,
    pub debate_duration: f32,
    pub sponsor_entity: Option<Entity>, // Who proposed it
    pub pressure_type: Option<crate::simulation::PressureType>,
}

/// Component tracking repeal information
#[derive(Component, Debug, Clone)]
pub struct RepealInfo {
    pub repeal_date: i32, // Game year when repealed
    pub repeal_reason: String,
    pub cooldown_remaining: f32, // Prevents immediate re-enactment
}

// ================================================================================================
// EVENTS
// ================================================================================================

/// Event when a law is enacted
#[derive(Event, Debug)]
pub struct LawEnactedEvent {
    pub nation_entity: Entity,
    pub law_entity: Entity,
}

/// Event when a law is proposed
#[derive(Event, Debug)]
pub struct LawProposedEvent {
    pub nation_entity: Entity,
    pub law_entity: Entity,
    pub pressure_type: crate::simulation::PressureType,
}

/// Event when a law is repealed
#[derive(Event, Debug)]
pub struct LawRepealedEvent {
    pub nation_entity: Entity,
    pub law_entity: Entity,
    pub reason: String,
}