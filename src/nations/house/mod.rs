//! Noble houses and ruling families - Gateway Module
//!
//! Houses represent the dynasties that rule nations. Each house has its own
//! personality traits, history, and relationships that persist across generations.
//!
//! This is a pure gateway module - all implementation lives in submodules.

// Private submodules - implementation details hidden from external code
mod influence;
mod mottos;
mod traits;
mod types;

// New drama engine modules
mod characters;
mod drama;
mod plugin;

// Public re-exports - carefully controlled API surface

// Core types
pub use types::{House, Ruler, RulerPersonality};

// Trait system
pub use traits::{DominantTrait, HouseArchetype, HouseTraits};

// Motto generation (only the public function, not internals)
pub use mottos::generate_motto;

// Character and drama system exports
pub use characters::{
    Character, CharacterId, CharacterRole, DetailedPersonality,
    Quirk, Secret, Scandal, LifeEvent, Achievement,
    FamilyMember, FamilyBranch, RelationshipType,
    HasRelationship, RelatedTo, RelationshipMetadata
};

pub use drama::{
    DramaEvent, DramaEventId, DramaEventType, EventImportance,
    EventVisibility, EventConsequence, generate_drama_events
};

// Plugin exports
pub use plugin::{DramaEnginePlugin, spawn_house_family, CharacterRegistry};
