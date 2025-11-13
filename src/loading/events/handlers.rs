//! Event handling systems for loading interactions

use super::types::CancelWorldGeneration;
use crate::loading::ui::CancelGenerationButton;
use crate::states::{GameState, RequestStateTransition};
use bevy::prelude::*;

/// Handle cancel button interactions
pub fn handle_cancel_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CancelGenerationButton>)>,
    mut cancel_events: MessageWriter<CancelWorldGeneration>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            info!("Cancel Generation button pressed");
            cancel_events.write(CancelWorldGeneration);
        }
    }
}

/// Handle cancel world generation events
pub fn handle_cancel_generation(
    mut cancel_events: MessageReader<CancelWorldGeneration>,
    mut state_events: MessageWriter<RequestStateTransition>,
    mut commands: Commands,
) {
    for _event in cancel_events.read() {
        info!("Canceling world generation");

        // Clean up any generation resources
        commands.remove_resource::<crate::world::AsyncWorldGeneration>(); // Dropping this cancels the async task

        // Reset pending world generation flag
        commands.insert_resource(crate::states::PendingWorldGeneration {
            pending: false,
            delay_timer: 0.0,
        });

        // Transition back to world configuration
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::WorldConfiguration,
        });

        info!("Returning to world configuration screen");
    }
}
