//! Familial Relationships - Character family and social bonds
//!
//! This module defines character-to-character relationships including family ties,
//! romantic relationships, and social connections. Uses Bevy's automatic bidirectional
//! relationship tracking for efficient querying.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ================================================================================================
// CHARACTER RELATIONSHIPS
// ================================================================================================

/// A character has a relationship with another character
/// When applied to a Character entity, automatically creates `RelatedTo` on the target entity
///
/// This replaces manual bidirectional relationship tracking with automatic Bevy relationships.
#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = RelatedTo)]
pub struct HasRelationship(pub Entity);

/// Reverse relationship: A character is related to by other characters
/// Automatically maintained by Bevy when `HasRelationship` is added
#[derive(Component, Debug, Clone, Default)]
#[relationship_target(relationship = HasRelationship, linked_spawn)]
pub struct RelatedTo(Vec<Entity>);

impl RelatedTo {
    /// Get read-only access to characters that have relationships with this one
    pub fn characters(&self) -> &[Entity] {
        &self.0
    }

    /// Check if a specific character has a relationship with this one
    pub fn has_relationship_with(&self, character: Entity) -> bool {
        self.0.contains(&character)
    }

    /// Get the number of relationships
    pub fn relationship_count(&self) -> usize {
        self.0.len()
    }
}

// ================================================================================================
// RELATIONSHIP METADATA
// ================================================================================================

/// Additional relationship metadata stored on the source character
/// This component accompanies `HasRelationship` to provide relationship details
#[derive(Component, Debug, Clone)]
pub struct RelationshipMetadata {
    /// The target entity this metadata applies to
    pub target: Entity,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Relationship strength: -1.0 (hatred) to 1.0 (deep bond)
    pub strength: f32,
    /// Whether this relationship is publicly known
    pub public_knowledge: bool,
}

/// Types of relationships between characters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum RelationshipType {
    // Family
    Parent,
    Child,
    Sibling,
    Spouse,
    ExSpouse,

    // Romance
    Lover,
    SecretLover,
    Betrothed,
    Crush, // One-sided

    // Social
    BestFriend,
    Friend,
    Rival,
    Nemesis,
    Mentor,
    Student,

    // Political
    Ally,
    Conspirator,
    Blackmailer,
    Puppet,
    PuppetMaster,
}

// ================================================================================================
// RELATIONSHIP BUNDLE - Convenience for spawning relationships with metadata
// ================================================================================================

/// Bundle for creating a character relationship with metadata
/// Use this when adding relationships to ensure both components are added together
#[derive(Bundle, Clone)]
pub struct CharacterRelationshipBundle {
    /// The relationship component (Bevy handles bidirectional tracking)
    pub relationship: HasRelationship,
    /// Metadata about the relationship
    pub metadata: RelationshipMetadata,
}

impl CharacterRelationshipBundle {
    /// Create a new relationship bundle
    pub fn new(
        target: Entity,
        relationship_type: RelationshipType,
        strength: f32,
        public_knowledge: bool,
    ) -> Self {
        Self {
            relationship: HasRelationship(target),
            metadata: RelationshipMetadata {
                target,
                relationship_type,
                strength,
                public_knowledge,
            },
        }
    }

    /// Create a family relationship (public, positive strength)
    pub fn family(target: Entity, relationship_type: RelationshipType, strength: f32) -> Self {
        Self::new(target, relationship_type, strength, true)
    }

    /// Create a spouse relationship
    pub fn spouse(target: Entity, strength: f32) -> Self {
        Self::family(target, RelationshipType::Spouse, strength)
    }

    /// Create a parent-child relationship (from parent's perspective)
    pub fn child(target: Entity, strength: f32) -> Self {
        Self::family(target, RelationshipType::Child, strength)
    }

    /// Create a parent-child relationship (from child's perspective)
    pub fn parent(target: Entity, strength: f32) -> Self {
        Self::family(target, RelationshipType::Parent, strength)
    }

    /// Create a sibling relationship
    pub fn sibling(target: Entity, strength: f32) -> Self {
        Self::family(target, RelationshipType::Sibling, strength)
    }

    /// Create a secret lover relationship (not public)
    pub fn secret_lover(target: Entity, strength: f32) -> Self {
        Self::new(target, RelationshipType::SecretLover, strength, false)
    }

    /// Create a rival relationship
    pub fn rival(target: Entity, intensity: f32) -> Self {
        Self::new(target, RelationshipType::Rival, -intensity.abs(), true)
    }

    /// Create a nemesis relationship (intense rivalry)
    pub fn nemesis(target: Entity) -> Self {
        Self::new(target, RelationshipType::Nemesis, -1.0, true)
    }
}

// ================================================================================================
// QUERY HELPERS
// ================================================================================================

/// Find all characters that have a specific relationship type with a target
pub fn find_relationships_of_type<'a>(
    source_entity: Entity,
    relationship_type: &RelationshipType,
    metadata_query: &'a Query<&RelationshipMetadata>,
) -> Vec<Entity> {
    metadata_query
        .iter()
        .filter(|meta| meta.relationship_type == *relationship_type)
        .map(|meta| meta.target)
        .collect()
}

/// Get relationship metadata between two specific characters
pub fn get_relationship_between(
    source: Entity,
    target: Entity,
    metadata_query: &Query<&RelationshipMetadata>,
) -> Option<RelationshipMetadata> {
    metadata_query
        .iter()
        .find(|meta| meta.target == target)
        .cloned()
}

// ================================================================================================
// FAMILY TREE HELPERS
// ================================================================================================

/// Find all children of a character
pub fn find_children(
    parent_entity: Entity,
    characters_with_metadata: &Query<(Entity, &RelationshipMetadata)>,
) -> Vec<Entity> {
    characters_with_metadata
        .iter()
        .filter(|(entity, meta)| {
            *entity == parent_entity && meta.relationship_type == RelationshipType::Child
        })
        .map(|(_, meta)| meta.target)
        .collect()
}

/// Find all parents of a character
pub fn find_parents(
    child_entity: Entity,
    characters_with_metadata: &Query<(Entity, &RelationshipMetadata)>,
) -> Vec<Entity> {
    characters_with_metadata
        .iter()
        .filter(|(entity, meta)| {
            *entity == child_entity && meta.relationship_type == RelationshipType::Parent
        })
        .map(|(_, meta)| meta.target)
        .collect()
}

/// Find spouse(s) of a character
pub fn find_spouses(
    character_entity: Entity,
    characters_with_metadata: &Query<(Entity, &RelationshipMetadata)>,
) -> Vec<Entity> {
    characters_with_metadata
        .iter()
        .filter(|(entity, meta)| {
            *entity == character_entity && meta.relationship_type == RelationshipType::Spouse
        })
        .map(|(_, meta)| meta.target)
        .collect()
}

/// Find all siblings of a character
pub fn find_siblings(
    character_entity: Entity,
    characters_with_metadata: &Query<(Entity, &RelationshipMetadata)>,
) -> Vec<Entity> {
    characters_with_metadata
        .iter()
        .filter(|(entity, meta)| {
            *entity == character_entity && meta.relationship_type == RelationshipType::Sibling
        })
        .map(|(_, meta)| meta.target)
        .collect()
}
