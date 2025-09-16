//! Navigation handling systems
//!
//! This module handles navigation buttons (Generate World, Back).

use super::super::components::{BackButton, GenerateButton};
use super::super::types::WorldGenerationSettings;
use crate::states::{GameState, RequestStateTransition};
use bevy::prelude::*;

pub fn init_default_settings(mut commands: Commands) {
    commands.insert_resource(WorldGenerationSettings::default());
    debug!("Initialized default world generation settings");
}

pub fn handle_generate_button(
    mut commands: Commands,
    interactions: Query<&Interaction, (Changed<Interaction>, With<GenerateButton>)>,
    mut settings: ResMut<WorldGenerationSettings>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            debug!("Generate World button pressed");
            debug!("Using seed: {}", settings.seed);
            debug!("Settings: {:?}", *settings);

            // Signal that we need to generate a world
            commands.insert_resource(crate::states::PendingWorldGeneration {
                pending: true,
                delay_timer: 0.1,
            });

            // Initialize loading screen
            let mut loading_state = crate::loading_screen::LoadingState::default();
            crate::loading_screen::start_world_generation_loading(
                &mut loading_state,
                settings.seed,
                format!("{:?}", settings.world_size),
            );
            commands.insert_resource(loading_state);

            // Transition to loading screen
            state_events.write(RequestStateTransition {
                from: GameState::WorldConfiguration,
                to: GameState::LoadingWorld,
            });
        }
    }
}

pub fn handle_back_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            debug!("Back button pressed");
            state_events.write(RequestStateTransition {
                from: GameState::WorldConfiguration,
                to: GameState::MainMenu,
            });
        }
    }
}
