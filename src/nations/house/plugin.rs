//! Drama Engine Plugin - Integrates character drama into Living Worlds
//!
//! This plugin adds the character-driven narrative system that creates
//! viral moments and emergent storytelling.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::characters::RelationshipType;
use super::drama::{generate_drama_events, GlobalRng};
use super::events::{CharacterBornEvent, CharacterDeathEvent, CharacterRegistry, RelationshipChangedEvent};
use super::systems::{age_characters, process_character_events, update_relationships};
use crate::simulation::GameTime;

define_plugin!(DramaEnginePlugin {
    resources: [
        GameTime,
        GlobalRng,
        CharacterRegistry,
    ],

    events: [
        super::drama::DramaEvent,
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