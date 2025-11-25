//! Drama Engine Systems
//!
//! Systems for character lifecycle, relationships, and family generation.

use bevy::prelude::*;
use rand::Rng;
use super::characters::{
    Character, CharacterRole, CharacterRelationshipBundle, FamilyBranch, FamilyMember,
    HasRelationship, RelationshipType,
};
use super::drama::{DramaEvent, EventConsequence};
use super::events::{CharacterBornEvent, CharacterDeathEvent, DeathCause, RelationshipChangedEvent};

/// System to age characters over time
pub fn age_characters(
    mut characters: Query<(Entity, &mut Character)>,
    time: Res<crate::simulation::GameTime>,
    mut death_events: MessageWriter<CharacterDeathEvent>,
) {
    let mut rng = rand::thread_rng();

    // Age characters when a year passes (only on Jan 1st)
    let day_of_year = time.day_of_year() as i32;
    if day_of_year != 0 {
        return;
    }

    for (entity, mut character) in &mut characters {
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
            death_events.write(CharacterDeathEvent {
                character: entity,
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
/// NOTE: Relationship components are immutable in Bevy. To change relationships,
/// we remove the old relationship and insert a new one via Commands.
pub fn update_relationships(
    characters: Query<(Entity, &super::characters::CharacterId)>,
    mut commands: Commands,
    mut drama_events: MessageReader<DramaEvent>,
    mut relationship_events: MessageWriter<RelationshipChangedEvent>,
) {
    // Build lookup map from CharacterId to Entity
    let character_lookup: std::collections::HashMap<super::characters::CharacterId, Entity> =
        characters.iter().map(|(entity, id)| (*id, entity)).collect();

    for event in drama_events.read() {
        for consequence in &event.consequences {
            if let EventConsequence::RelationshipChange {
                character_a,
                character_b,
                new_relationship,
            } = consequence
            {
                // Look up entities from CharacterIds
                let entity_a = character_lookup.get(character_a).copied();
                let entity_b = character_lookup.get(character_b).copied();

                // Only process if both characters exist
                if let (Some(entity_a), Some(entity_b)) = (entity_a, entity_b) {
                    // To change a relationship:
                    // 1. Remove old HasRelationship component (Bevy auto-cleans RelatedTo)
                    // 2. Insert new relationship bundle
                    commands.entity(entity_a).remove::<HasRelationship>();
                    commands.entity(entity_a).insert(CharacterRelationshipBundle::new(
                        entity_b,
                        new_relationship.clone(),
                        0.5, // Default strength for new relationships
                        true,
                    ));

                    // Send change event with resolved entities
                    relationship_events.write(RelationshipChangedEvent {
                        character_a: entity_a,
                        character_b: entity_b,
                        old_relationship: None,
                        new_relationship: new_relationship.clone(),
                    });
                } else {
                    warn!(
                        "Could not find entities for relationship change event: {:?} -> {:?}",
                        character_a, character_b
                    );
                }
            }
        }
    }
}

/// Process character-specific events
pub fn process_character_events(
    characters: Query<&mut Character>,
    mut birth_events: MessageReader<CharacterBornEvent>,
    mut death_events: MessageReader<CharacterDeathEvent>,
    commands: Commands,
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
    culture: crate::name_generator::Culture,
    name_gen: &mut crate::name_generator::NameGenerator,
) -> Vec<Entity> {
    let mut rng = rand::thread_rng();
    let mut family_entities = Vec::new();

    // Create ruler
    let ruler = Character::generate(
        house_entity,
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
            branch: FamilyBranch::MainLine,
        },
    )).id();
    family_entities.push(ruler_entity);

    // Create spouse (50% chance)
    if rng.gen_bool(0.5) {
        let spouse = Character::generate(
            house_entity,
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
                branch: FamilyBranch::MarriedIn,
            },
        )).id();
        family_entities.push(spouse_entity);

        // Create marriage relationship using the bundle (automatic bidirectional tracking)
        commands.entity(ruler_entity).insert(
            CharacterRelationshipBundle::spouse(spouse_entity, rng.gen_range(0.3..1.0))
        );
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
                branch: FamilyBranch::MainLine,
            },
        )).id();
        family_entities.push(child_entity);

        // Create parent-child relationship using the bundle (automatic bidirectional tracking)
        commands.entity(ruler_entity).insert(
            CharacterRelationshipBundle::child(child_entity, rng.gen_range(0.5..1.0))
        );
    }

    // Maybe add an advisor (30% chance)
    if rng.gen_bool(0.3) {
        let advisor = Character::generate(
            house_entity,
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
                branch: FamilyBranch::MainLine,
            },
        )).id();
        family_entities.push(advisor_entity);
    }

    // Maybe add a bastard for drama (10% chance)
    if rng.gen_bool(0.1) {
        let bastard = Character::generate(
            house_entity,
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
                branch: FamilyBranch::BastardLine,
            },
        )).id();
        family_entities.push(bastard_entity);
    }

    family_entities
}