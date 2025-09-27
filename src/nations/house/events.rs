//! Drama Engine Events
//!
//! Events for character lifecycle and relationship changes.

use bevy::prelude::*;
use super::characters::{CharacterId, RelationshipType};

/// Registry of all characters in the game
#[derive(Resource, Default)]
pub struct CharacterRegistry {
    pub characters: Vec<Entity>,
    pub id_counter: u32,
}

impl CharacterRegistry {
    pub fn next_id(&mut self) -> CharacterId {
        self.id_counter += 1;
        CharacterId(self.id_counter)
    }
}

/// Event when a new character is born
#[derive(Event)]
pub struct CharacterBornEvent {
    pub character: Entity,
    pub parents: Option<(Entity, Entity)>,
    pub house: Entity,
}

/// Event when a character dies
#[derive(Event)]
pub struct CharacterDeathEvent {
    pub character: Entity,
    pub cause: DeathCause,
}

/// Cause of death for characters
#[derive(Debug, Clone)]
pub enum DeathCause {
    Natural,
    Battle,
    Assassination,
    Accident(String),
    Disease,
    Heartbreak,
    Mystery,
}

/// Event when relationships change
#[derive(Event)]
pub struct RelationshipChangedEvent {
    pub character_a: Entity,
    pub character_b: Entity,
    pub old_relationship: Option<RelationshipType>,
    pub new_relationship: RelationshipType,
}