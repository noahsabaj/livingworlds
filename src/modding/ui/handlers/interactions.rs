//! Mod interaction event handlers
//!
//! This module handles user interactions with mod toggles, confirmations,
//! and other interactive elements in the mod browser.

use crate::loading::{start_mod_application_loading, LoadingState};
use crate::modding::manager::ModManager;
use crate::modding::ui::state::ModBrowserState;
use crate::modding::ui::types::{ApplyModChangesEvent, ConfirmModsetButton, ModToggle};
use crate::states::{GameState, RequestStateTransition};
use bevy::prelude::*;

/// Handles clicks on the confirm modset button
pub fn handle_confirm_modset_clicks(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ConfirmModsetButton>)>,
    mut apply_events: EventWriter<ApplyModChangesEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            apply_events.send(ApplyModChangesEvent);
        }
    }
}

/// Handles mod toggle interactions
pub fn handle_mod_toggles(
    mut interaction_query: Query<
        (&Interaction, &ModToggle, &Children),
        (Changed<Interaction>, With<ModToggle>),
    >,
    mut text_query: Query<&mut Text>,
    mut mod_manager: ResMut<ModManager>,
    mut browser_state: ResMut<ModBrowserState>,
) {
    for (interaction, toggle, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Toggle the mod's enabled state
            if let Some(loaded_mod) = mod_manager
                .available_mods
                .iter_mut()
                .find(|m| m.manifest.id == toggle.mod_id)
            {
                loaded_mod.enabled = !loaded_mod.enabled;

                // Update active mods set
                if loaded_mod.enabled {
                    browser_state.active_mods.insert(toggle.mod_id.clone());
                } else {
                    browser_state.active_mods.remove(&toggle.mod_id);
                }

                // Update button text
                for &child in children {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = if loaded_mod.enabled { "[X]" } else { "[ ]" }.to_string();
                    }
                }

                info!(
                    "Mod '{}' {}",
                    loaded_mod.manifest.name,
                    if loaded_mod.enabled {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
            }
        }
    }
}

/// Handles applying mod changes with a soft reset
pub fn handle_apply_changes(
    mut apply_events: EventReader<ApplyModChangesEvent>,
    mut loading_state: ResMut<LoadingState>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    for _ in apply_events.read() {
        info!("Applying mod changes and performing soft reset");

        // Set loading state to indicate mod application
        start_mod_application_loading(&mut loading_state);

        // Transition to loading state for soft reset
        state_events.send(RequestStateTransition {
            target: GameState::LoadingWorld,
        });
    }
}