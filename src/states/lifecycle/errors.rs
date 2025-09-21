//! Error State Lifecycle Management
//!
//! This module handles the enter/exit lifecycle for error-related states,
//! including WorldGenerationFailed state with dialog management and recovery flows.

use crate::states::definitions::*;
use bevy::prelude::*;

/// System that runs when entering the WorldGenerationFailed state
pub fn enter_world_generation_failed(
    mut commands: Commands,
    error_resource: Res<crate::resources::WorldGenerationError>,
    error_context: Option<Res<crate::diagnostics::ErrorContext>>,
) {
    #[cfg(feature = "debug-states")]
    warn!(
        "Entering WorldGenerationFailed state with error: {}",
        error_resource.error_message
    );

    // Use enhanced dialog with context if available
    if let Some(context) = error_context {
        crate::ui::dialog_presets::world_generation_error_dialog_with_context(
            &mut commands,
            &context,
        );
    } else {
        // Fall back to basic dialog if no context available
        crate::ui::dialog_presets::world_generation_error_dialog(
            &mut commands,
            &error_resource.error_message,
        );
    }
}

/// Handle button clicks in the error dialog
pub fn handle_error_dialog_buttons(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    dialog_query: Query<Entity, With<crate::ui::WorldGenerationErrorDialog>>,
    confirm_button_query: Query<
        &Interaction,
        (Changed<Interaction>, With<crate::ui::ConfirmButton>),
    >,
    cancel_button_query: Query<&Interaction, (Changed<Interaction>, With<crate::ui::CancelButton>)>,
) {
    for interaction in &confirm_button_query {
        if *interaction == Interaction::Pressed {
            // Go back to world configuration
            next_state.set(GameState::WorldConfiguration);

            // Remove the error dialog
            for entity in &dialog_query {
                commands.entity(entity).despawn();
            }
        }
    }

    for interaction in &cancel_button_query {
        if *interaction == Interaction::Pressed {
            // Go back to main menu
            next_state.set(GameState::MainMenu);

            // Remove the error dialog
            for entity in &dialog_query {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Cleanup when exiting the WorldGenerationFailed state
pub fn exit_world_generation_failed(
    mut commands: Commands,
    dialog_query: Query<Entity, With<crate::ui::DialogOverlay>>,
) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting WorldGenerationFailed state");

    // Clean up any remaining dialogs
    for entity in &dialog_query {
        commands.entity(entity).despawn();
    }

    // Remove the error resources
    commands.remove_resource::<crate::resources::WorldGenerationError>();
    commands.remove_resource::<crate::diagnostics::ErrorContext>();
}
