//! Nation laws panel event handlers

use super::panel::spawn_nation_laws_panel;
use super::types::*;
use crate::ui::{SelectedNation, ViewLawsButton};
use bevy::prelude::*;

/// Handle view laws button click
pub fn handle_view_laws_button(
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
                for entity in &panel_query {
                    commands.entity(entity).despawn();
                }
                break; // Only handle the first interaction
            }
        }
    }
}

/// Handle close button click
pub fn handle_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClosePanelButton>)>,
    mut state: ResMut<NationLawsPanelState>,
    mut commands: Commands,
    panel_query: Query<Entity, With<NationLawsPanel>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            state.is_open = false;
            for entity in &panel_query {
                commands.entity(entity).despawn();
            }
            break; // Only handle the first pressed button
        }
    }
}

/// Handle repeal law buttons
pub fn handle_repeal_buttons(
    mut interaction_query: Query<(&Interaction, &RepealLawButton), Changed<Interaction>>,
    selected_nation: Res<SelectedNation>,
    mut nation_laws_query: Query<&mut crate::nations::NationLaws>,
    registry: Res<crate::nations::LawRegistry>,
    mut messages: MessageWriter<crate::nations::LawRepealEvent>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok(nation_laws) = nation_laws_query.get_mut(nation_entity) {
                    // Get law details from registry for the event
                    if let Some(law) = registry.get_law(button.law_id) {
                        // Send repeal event with all required fields
                        messages.write(crate::nations::LawRepealEvent {
                            nation_id: crate::nations::NationId(0), // TODO: Get actual nation ID
                            nation_name: "Selected Nation".to_string(), // TODO: Get actual nation name
                            law_id: button.law_id,
                            law_name: law.name.clone(),
                            category: law.category,
                            years_active: 0, // TODO: Track years active
                        });

                        // Immediate UI feedback
                        info!("Repealing law {:?} for nation", button.law_id);
                    }
                }
            }
        }
    }
}

/// Handle support/oppose buttons for proposed laws
pub fn handle_support_oppose_buttons(
    support_query: Query<(&Interaction, &SupportLawButton), Changed<Interaction>>,
    oppose_query: Query<(&Interaction, &OpposeLawButton), Changed<Interaction>>,
    selected_nation: Res<SelectedNation>,
    mut nation_laws_query: Query<&mut crate::nations::NationLaws>,
) {
    // Handle support buttons
    for (interaction, button) in &support_query {
        if *interaction == Interaction::Pressed {
            if let Some(nation_entity) = selected_nation.entity {
                if let Ok(mut nation_laws) = nation_laws_query.get_mut(nation_entity) {
                    if let Some(proposal) = nation_laws.proposed_laws.get_mut(button.proposal_index) {
                        // Increase support (simulate player influence)
                        proposal.current_support = (proposal.current_support + 0.1).min(1.0);
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
                        proposal.current_support = (proposal.current_support - 0.1).max(0.0);
                        info!("Opposing law proposal at index {}", button.proposal_index);
                    }
                }
            }
        }
    }
}