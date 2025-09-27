//! Drama Engine Plugin - Integrates character drama into Living Worlds
//!
//! This plugin adds the character-driven narrative system that creates
//! viral moments and emergent storytelling.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use crate::simulation::GameTime;
use super::{
    characters::{
        Character, CharacterId, FamilyMember, HasRelationship, RelatedTo,
        RelationshipMetadata, RelationshipType
    },
    drama::{DramaEvent, generate_drama_events, GlobalRng},
};

// Re-export for convenience
pub use super::characters::{CharacterRole, DetailedPersonality, Quirk, Secret};
pub use super::drama::{DramaEventType, EventImportance};

define_plugin!(DramaEnginePlugin {
    resources: [
        GameTime,
        GlobalRng,
        CharacterRegistry,
    ],

    events: [
        DramaEvent,
        CharacterBornEvent,
        CharacterDeathEvent,
        RelationshipChangedEvent,
    ],

    update: [
        generate_drama_events.run_if(in_state(crate::states::GameState::InGame)),
        age_characters.run_if(in_state(crate::states::GameState::InGame)),
        update_relationships.run_if(in_state(crate::states::GameState::InGame)),
        process_character_events.run_if(in_state(crate::states::GameState::InGame)),
    ],

    custom_init: |app: &mut bevy::app::App| {
        // Register relationship components and metadata
        // NOTE: These types need Reflect derive to be registered
        // app.register_type::<HasRelationship>()
        //    .register_type::<RelatedTo>()
        //    .register_type::<RelationshipMetadata>()
        app.register_type::<RelationshipType>();
    }
});

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
    pub old_relationship: Option<super::characters::RelationshipType>,
    pub new_relationship: super::characters::RelationshipType,
}

/// System to age characters over time
fn age_characters(
    mut characters: Query<&mut Character>,
    time: Res<crate::simulation::GameTime>,
    mut death_events: EventWriter<CharacterDeathEvent>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Age characters when a year passes (only on Jan 1st)
    let day_of_year = time.day_of_year() as i32;
    if day_of_year != 0 {
        return;
    }

    for mut character in &mut characters {
        character.age += 1;

        // Check for natural death (increases with age)
        let death_chance = match character.age {
            0..=40 => 0.001,
            41..=60 => 0.01,
            61..=80 => 0.05,
            81..=100 => 0.15,
            _ => 0.3,
        };

        // Stress and health affect death chance
        let modified_chance = death_chance
            * (2.0 - character.health)
            * (1.0 + character.stress);

        if rng.gen_bool(modified_chance.min(1.0).max(0.0) as f64) {
            death_events.send(CharacterDeathEvent {
                character: Entity::PLACEHOLDER, // Would need entity in real implementation
                cause: DeathCause::Natural,
            });
        }

        // Stress naturally decreases over time
        character.stress = (character.stress - 0.1).max(0.0);

        // Madness can increase with age and stress
        if character.personality.madness > 0.0 {
            character.personality.madness += character.stress * 0.01;
        }
    }
}

/// System to update relationships based on events
fn update_relationships(
    characters: Query<(Entity, &Character)>,
    mut relationships: Query<&mut HasRelationship>,
    mut drama_events: EventReader<DramaEvent>,
    mut relationship_events: EventWriter<RelationshipChangedEvent>,
) {
    use super::drama::EventConsequence;

    for event in drama_events.read() {
        for consequence in &event.consequences {
            if let EventConsequence::RelationshipChange {
                character_a,
                character_b,
                new_relationship,
            } = consequence {
                // Find existing relationship
                for mut relationship in &mut relationships {
                    // Update relationship (would need proper entity lookup in real implementation)
                    // This is simplified for the example
                }

                // Send change event
                relationship_events.send(RelationshipChangedEvent {
                    character_a: Entity::PLACEHOLDER,
                    character_b: Entity::PLACEHOLDER,
                    old_relationship: None,
                    new_relationship: new_relationship.clone(),
                });
            }
        }
    }
}

/// Process character-specific events
fn process_character_events(
    mut characters: Query<&mut Character>,
    mut birth_events: EventReader<CharacterBornEvent>,
    mut death_events: EventReader<CharacterDeathEvent>,
    mut commands: Commands,
) {
    // Handle births
    for birth in birth_events.read() {
        info!("New character born into house!");
        // Would spawn actual character entity here
    }

    // Handle deaths
    for death in death_events.read() {
        info!("Character died: {:?}", death.cause);
        // Would despawn character and handle succession
    }
}

/// Helper function to spawn a character family for a house
pub fn spawn_house_family(
    commands: &mut Commands,
    house_entity: Entity,
    nation_id: crate::nations::NationId,
    culture: crate::name_generator::Culture,
    name_gen: &mut crate::name_generator::NameGenerator,
) -> Vec<Entity> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut family_entities = Vec::new();

    // Create ruler
    let ruler = Character::generate(
        house_entity,
        nation_id,
        culture,
        CharacterRole::Ruler,
        name_gen,
        &mut rng,
    );

    let ruler_entity = commands.spawn((
        ruler.clone(),
        FamilyMember {
            house: house_entity,
            generation: 0,
            branch: super::characters::FamilyBranch::MainLine,
        },
    )).id();
    family_entities.push(ruler_entity);

    // Create spouse (50% chance)
    if rng.gen_bool(0.5) {
        let spouse = Character::generate(
            house_entity,
            nation_id,
            culture,
            CharacterRole::Spouse,
            name_gen,
            &mut rng,
        );

        let spouse_entity = commands.spawn((
            spouse,
            FamilyMember {
                house: house_entity,
                generation: 0,
                branch: super::characters::FamilyBranch::MarriedIn,
            },
        )).id();
        family_entities.push(spouse_entity);

        // Create marriage relationship with metadata
        commands.entity(ruler_entity).insert((
            HasRelationship(spouse_entity),
            super::characters::RelationshipMetadata {
                relationship_type: super::characters::RelationshipType::Spouse,
                strength: rng.gen_range(0.3..1.0),
                public_knowledge: true,
            }
        ));
    }

    // Create 1-4 children
    let num_children = rng.gen_range(1..=4);
    for i in 0..num_children {
        let role = if i == 0 {
            CharacterRole::Heir
        } else {
            CharacterRole::Child
        };

        let child = Character::generate(
            house_entity,
            nation_id,
            culture,
            role,
            name_gen,
            &mut rng,
        );

        let child_entity = commands.spawn((
            child,
            FamilyMember {
                house: house_entity,
                generation: 1,
                branch: super::characters::FamilyBranch::MainLine,
            },
        )).id();
        family_entities.push(child_entity);

        // Create parent-child relationship with metadata
        commands.entity(ruler_entity).insert((
            HasRelationship(child_entity),
            super::characters::RelationshipMetadata {
                relationship_type: super::characters::RelationshipType::Child,
                strength: rng.gen_range(0.5..1.0),
                public_knowledge: true,
            }
        ));
    }

    // Maybe add an advisor (30% chance)
    if rng.gen_bool(0.3) {
        let advisor = Character::generate(
            house_entity,
            nation_id,
            culture,
            CharacterRole::Advisor,
            name_gen,
            &mut rng,
        );

        let advisor_entity = commands.spawn((
            advisor,
            FamilyMember {
                house: house_entity,
                generation: 0,
                branch: super::characters::FamilyBranch::MainLine,
            },
        )).id();
        family_entities.push(advisor_entity);
    }

    // Maybe add a bastard for drama (10% chance)
    if rng.gen_bool(0.1) {
        let bastard = Character::generate(
            house_entity,
            nation_id,
            culture,
            CharacterRole::Bastard,
            name_gen,
            &mut rng,
        );

        let bastard_entity = commands.spawn((
            bastard,
            FamilyMember {
                house: house_entity,
                generation: 1,
                branch: super::characters::FamilyBranch::BastardLine,
            },
        )).id();
        family_entities.push(bastard_entity);
    }

    family_entities
}