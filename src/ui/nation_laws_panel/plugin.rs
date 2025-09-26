//! Nation laws panel plugin
//!
//! Manages the display of nation-specific laws and proposals.

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;
use crate::states::GameState;
use crate::ui::{despawn_ui_entities, SelectedNation, ViewLawsButton};
use super::types::*;
use super::panel::spawn_nation_laws_panel;
use super::updates::{update_active_laws_list, update_proposed_laws_list};

define_plugin!(NationLawsPanelPlugin {
    resources: [
        NationLawsPanelState
    ],

    update: [
        handle_view_laws_button.run_if(in_state(GameState::InGame)),
        handle_close_button.run_if(in_state(GameState::InGame)),
        update_active_laws_list.run_if(in_state(GameState::InGame)),
        update_proposed_laws_list.run_if(in_state(GameState::InGame)),
        handle_repeal_buttons.run_if(in_state(GameState::InGame)),
        handle_support_oppose_buttons.run_if(in_state(GameState::InGame))
    ]
});

/// Handle view laws button click
fn handle_view_laws_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ViewLawsButton>)>,
    mut state: ResMut<NationLawsPanelState>,
    mut commands: Commands,
    panel_query: Query<Entity, With<NationLawsPanel>>,
    selected_nation: Res<SelectedNation>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed && selected_nation.entity.is_some() {
            state.is_open = !state.is_open;

            if state.is_open {
                // Spawn panel if not exists
                if panel_query.is_empty() {
                    spawn_nation_laws_panel(&mut commands);
                }
            } else {
                // Close panel
                despawn_ui_entities::<NationLawsPanel>(commands, panel_query);
            }
        }
    }
}

/// Handle close button click
fn handle_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClosePanelButton>)>,
    mut state: ResMut<NationLawsPanelState>,
    mut commands: Commands,
    panel_query: Query<Entity, With<NationLawsPanel>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            state.is_open = false;
            despawn_ui_entities::<NationLawsPanel>(commands, panel_query);
        }
    }
}

/// Handle repeal law buttons
fn handle_repeal_buttons(
    mut interaction_query: Query<(&Interaction, &RepealLawButton), Changed<Interaction>>,
    selected_nation: Res<SelectedNation>,
    mut nation_laws_query: Query<&mut crate::nations::laws::NationLaws>,
    mut events: EventWriter<crate::nations::laws::LawRepealEvent>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok(mut nation_laws) = nation_laws_query.get_mut(nation_entity) {
                    // Send repeal event
                    events.send(crate::nations::laws::LawRepealEvent {
                        nation_entity,
                        law_id: button.law_id,
                    });

                    // Immediate UI feedback
                    info!("Repealing law {:?} for nation", button.law_id);
                }
            }
        }
    }
}

/// Handle support/oppose buttons for proposed laws
fn handle_support_oppose_buttons(
    support_query: Query<(&Interaction, &SupportLawButton), Changed<Interaction>>,
    oppose_query: Query<(&Interaction, &OpposeLawButton), Changed<Interaction>>,
    selected_nation: Res<SelectedNation>,
    mut nation_laws_query: Query<&mut crate::nations::laws::NationLaws>,
) {
    // Handle support buttons
    for (interaction, button) in &support_query {
        if *interaction == Interaction::Pressed {
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok(mut nation_laws) = nation_laws_query.get_mut(nation_entity) {
                    if let Some(proposal) = nation_laws.proposed_laws.get_mut(button.proposal_index) {
                        // Increase support (simulate player influence)
                        proposal.support_percentage = (proposal.support_percentage + 0.1).min(1.0);
                        info!("Supporting law proposal at index {}", button.proposal_index);
                    }
                }
            }
        }
    }

    // Handle oppose buttons
    for (interaction, button) in &oppose_query {
        if *interaction == Interaction::Pressed {
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok(mut nation_laws) = nation_laws_query.get_mut(nation_entity) {
                    if let Some(proposal) = nation_laws.proposed_laws.get_mut(button.proposal_index) {
                        // Decrease support (simulate player influence)
                        proposal.support_percentage = (proposal.support_percentage - 0.1).max(0.0);
                        info!("Opposing law proposal at index {}", button.proposal_index);
                    }
                }
            }
        }
    }
}