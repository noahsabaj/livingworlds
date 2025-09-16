//! Input handling systems
//!
//! This module handles text input changes and random button interactions.

use super::super::components::*;
use super::super::types::WorldGenerationSettings;
use crate::name_generator::{NameGenerator, NameType};
use bevy::prelude::*;
use bevy_simple_text_input::{TextInputSubmitEvent, TextInputValue};
use rand::Rng;

pub fn handle_text_input_changes(
    mut name_events: EventReader<TextInputSubmitEvent>,
    mut settings: ResMut<WorldGenerationSettings>,
    name_inputs: Query<&TextInputValue, (With<WorldNameInput>, Changed<TextInputValue>)>,
    seed_inputs: Query<
        &TextInputValue,
        (
            With<SeedInput>,
            Without<WorldNameInput>,
            Changed<TextInputValue>,
        ),
    >,
) {
    for value in &name_inputs {
        if !value.0.is_empty() {
            settings.world_name = value.0.clone();
            debug!("World name changed to: {}", settings.world_name);
        }
    }

    for value in &seed_inputs {
        if !value.0.is_empty() {
            if let Ok(seed) = value.0.parse::<u32>() {
                settings.seed = seed;
                debug!("Seed changed to: {}", settings.seed);
            }
        }
    }

    // Also handle submit events
    for event in name_events.read() {
        if let Ok(value) = name_inputs.get(event.entity) {
            settings.world_name = value.0.clone();
            debug!("World name submitted: {}", settings.world_name);
        }
        if let Ok(value) = seed_inputs.get(event.entity) {
            if let Ok(seed) = value.0.parse::<u32>() {
                settings.seed = seed;
                debug!("Seed submitted: {}", settings.seed);
            }
        }
    }
}

pub fn handle_random_buttons(
    name_interactions: Query<&Interaction, (Changed<Interaction>, With<RandomNameButton>)>,
    seed_interactions: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<RandomSeedButton>,
            Without<RandomNameButton>,
        ),
    >,
    mut settings: ResMut<WorldGenerationSettings>,
    mut name_inputs: Query<&mut TextInputValue, With<WorldNameInput>>,
    mut seed_inputs: Query<&mut TextInputValue, (With<SeedInput>, Without<WorldNameInput>)>,
) {
    // Random name button
    for interaction in &name_interactions {
        if *interaction == Interaction::Pressed {
            let mut name_gen = NameGenerator::new();
            settings.world_name = name_gen.generate(NameType::World);
            for mut input_value in &mut name_inputs {
                input_value.0 = settings.world_name.clone();
            }
            debug!("Generated random name: {}", settings.world_name);
        }
    }

    // Random seed button
    for interaction in &seed_interactions {
        if *interaction == Interaction::Pressed {
            settings.seed = rand::thread_rng().r#gen();
            for mut input_value in &mut seed_inputs {
                input_value.0 = settings.seed.to_string();
            }
            debug!("Generated random seed: {}", settings.seed);
        }
    }
}
