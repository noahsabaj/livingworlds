//! Tab switching event handlers
//!
//! This module handles events related to switching between different tabs
//! in the mod browser UI.

use crate::modding::manager::ModManager;
use crate::modding::ui::state::ModBrowserState;
use crate::modding::ui::tabs::spawn_tab_content;
use crate::modding::ui::types::{
    ContentArea, ModBrowserTab, ModBrowserTabButton, SwitchModTabEvent,
};
use crate::ui::{colors, ButtonStyle, StyledButton};
use bevy::prelude::*;

/// Handles clicks on tab buttons
pub fn handle_tab_button_clicks(
    interaction_query: Query<
        (&Interaction, &ModBrowserTabButton),
        (Changed<Interaction>, With<ModBrowserTabButton>),
    >,
    mut switch_events: EventWriter<SwitchModTabEvent>,
) {
    for (interaction, tab_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            switch_events.send(SwitchModTabEvent {
                tab: tab_button.tab,
            });
        }
    }
}

/// Handles tab switching events
pub fn handle_tab_switching(
    mut commands: Commands,
    mut switch_events: EventReader<SwitchModTabEvent>,
    mut browser_state: ResMut<ModBrowserState>,
    content_query: Query<Entity, With<ContentArea>>,
    mod_manager: Res<ModManager>,
) {
    for event in switch_events.read() {
        if browser_state.current_tab == event.tab {
            continue;
        }

        debug!("Switching to tab: {:?}", event.tab);
        browser_state.current_tab = event.tab;

        // Clear and rebuild content area
        for entity in &content_query {
            commands.entity(entity).despawn_recursive();
            commands.entity(entity).with_children(|parent| {
                spawn_tab_content(
                    parent,
                    browser_state.current_tab,
                    &mod_manager,
                    &browser_state.search_query,
                );
            });
        }
    }
}

/// Updates tab button visuals based on current state
pub fn update_tab_buttons(
    state: Res<ModBrowserState>,
    mut tab_query: Query<(
        &ModBrowserTabButton,
        &mut StyledButton,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    if !state.is_changed() {
        return;
    }

    for (tab_button, _styled_button, mut bg_color, mut border_color) in &mut tab_query {
        if tab_button.tab == state.current_tab {
            // Style is immutable after creation - update colors directly
            *bg_color = BackgroundColor(colors::PRIMARY);
            *border_color = BorderColor(colors::PRIMARY.lighter(0.2));
        } else {
            *bg_color = BackgroundColor(colors::SECONDARY);
            *border_color = BorderColor(colors::BORDER_DEFAULT);
        }
    }
}