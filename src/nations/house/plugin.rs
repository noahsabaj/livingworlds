//! Drama Engine Plugin - Integrates character drama into Living Worlds
//!
//! This plugin adds the character-driven narrative system that creates
//! viral moments and emergent storytelling.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

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

    messages: [
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
        // NOTE: Bevy 0.17 auto-registers all #[derive(Reflect)] types!
        // Manual registration no longer needed:
        // app.register_type::<HasRelationship>()
        // app.register_type::<RelatedTo>()
        // app.register_type::<RelationshipMetadata>()
        // app.register_type::<RelationshipType>()

        // custom_init kept for potential future use
        let _ = app;
    }
});