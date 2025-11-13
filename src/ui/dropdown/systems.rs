//! Systems for dropdown functionality

use bevy::prelude::*;
use super::components::*;
use super::types::*;

/// Handle dropdown interactions (click to open/close)
pub fn handle_dropdown_interactions<T: DropdownValue>(
    mut dropdown_query: Query<(&Interaction, &mut Dropdown<T>), Changed<Interaction>>,
) {
    for (interaction, mut dropdown) in &mut dropdown_query {
        if *interaction == Interaction::Pressed {
            dropdown.toggle();
        }
    }
}

/// Update dropdown display based on state
pub fn update_dropdown_display<T: DropdownValue>(
    mut dropdown_query: Query<(&mut Dropdown<T>, &mut BackgroundColor), Changed<Dropdown<T>>>,
) {
    for (dropdown, mut bg_color) in &mut dropdown_query {
        *bg_color = BackgroundColor(match dropdown.state {
            DropdownState::Open | DropdownState::Opening => dropdown.style.background_open,
            _ => dropdown.style.background,
        });
    }
}

/// Handle keyboard navigation for dropdowns
pub fn handle_dropdown_keyboard<T: DropdownValue>(
    mut shortcut_events: MessageReader<crate::ui::ShortcutEvent>,
    mut dropdown_query: Query<&mut Dropdown<T>, With<DropdownOpen>>,
) {
    use crate::ui::ShortcutId;

    // Process shortcut events
    for event in shortcut_events.read() {
        // Check if any dropdown is open and has keyboard navigation enabled
        for mut dropdown in &mut dropdown_query {
            if !dropdown.config.keyboard_nav {
                continue;
            }

            match event.shortcut_id {
                ShortcutId::DropdownUp => {
                    dropdown.highlight_previous();
                }
                ShortcutId::DropdownDown => {
                    dropdown.highlight_next();
                }
                ShortcutId::DropdownSelect => {
                    dropdown.select_highlighted();
                    if dropdown.config.close_on_select && !dropdown.config.multi_select {
                        dropdown.close();
                    }
                }
                ShortcutId::Escape | ShortcutId::OpenMainMenu => {
                    dropdown.close();
                }
                _ => {} // Ignore other shortcuts
            }
        }
    }
}