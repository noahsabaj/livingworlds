//! Input handling systems
//!
//! This module handles text input changes and random button interactions.

use super::super::components::*;
use super::super::types::WorldGenerationSettings;
use crate::name_generator::{NameGenerator, NameType};
use bevy::prelude::*;
use crate::ui::TextBuffer;
use rand::Rng;

pub fn handle_text_input_changes(
    // Submit events handled internally by bevy-ui-builders
    mut settings: ResMut<WorldGenerationSettings>,
    name_inputs: Query<&TextBuffer, (With<WorldNameInput>, Changed<TextBuffer>)>,
    seed_inputs: Query<
        &TextBuffer,
        (
            With<SeedInput>,
            Without<WorldNameInput>,
            Changed<TextBuffer>,
        ),
    >,
) {
    for buffer in &name_inputs {
        if !buffer.content.is_empty() {
            settings.world_name = buffer.content.clone();
            debug!("World name changed to: {}", settings.world_name);
        }
    }

    for buffer in &seed_inputs {
        if !buffer.content.is_empty() {
            if let Ok(seed) = buffer.content.parse::<u32>() {
                settings.seed = seed;
                debug!("Seed changed to: {}", settings.seed);
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
    mut name_inputs: Query<&mut TextBuffer, With<WorldNameInput>>,
    mut seed_inputs: Query<&mut TextBuffer, (With<SeedInput>, Without<WorldNameInput>)>,
) {
    // Random name button
    for interaction in &name_interactions {
        if *interaction == Interaction::Pressed {
            let mut name_gen = NameGenerator::new();
            settings.world_name = name_gen.generate(NameType::World);
            for mut text_buffer in &mut name_inputs {
                text_buffer.content = settings.world_name.clone();
            }
            debug!("Generated random name: {}", settings.world_name);
        }
    }

    // Random seed button
    for interaction in &seed_interactions {
        if *interaction == Interaction::Pressed {
            settings.seed = rand::thread_rng().r#gen();
            for mut text_buffer in &mut seed_inputs {
                text_buffer.content = settings.seed.to_string();
            }
            debug!("Generated random seed: {}", settings.seed);
        }
    }
}
