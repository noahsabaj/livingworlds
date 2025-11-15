//! Keyboard navigation for the settings menu

use super::components::*;
use bevy::prelude::*;

/// Handle keyboard navigation (Tab/Shift+Tab) and ESC
pub fn handle_keyboard_navigation(
    mut shortcut_events: MessageReader<crate::ui::ShortcutEvent>,
    mut shortcut_registry: ResMut<crate::ui::ShortcutRegistry>,
    mut focus: ResMut<FocusedElement>,
    mut param_set: ParamSet<(
        Query<(
            Entity,
            &Focusable,
            &mut BackgroundColor,
            Option<&Interaction>,
        )>,
        Query<&mut Interaction>,
    )>,
    settings_root: Query<Entity, With<SettingsMenuRoot>>,
    mut commands: Commands,
) {
    use crate::ui::{ShortcutId, ShortcutContext};

    // Only process input if settings menu is actually open
    let settings_entity = match settings_root.single() {
        Ok(entity) => {
            debug!("Settings menu open, processing navigation input");
            // Set the shortcut context to Settings
            shortcut_registry.set_context(ShortcutContext::Settings);
            entity
        },
        Err(_) => {
            trace!("No settings menu open, skipping navigation input");
            return; // No settings menu open, skip all input processing
        }
    };

    for event in shortcut_events.read() {
        match event.shortcut_id {
            // ESC to close settings
            ShortcutId::Escape | ShortcutId::OpenMainMenu => {
                commands.entity(settings_entity).despawn();
                return;
            }

            // Tab navigation forwards
            ShortcutId::SettingsNavigateNext => {
                if focus.index < focus.max_index {
                    focus.index += 1;
                } else {
                    focus.index = 0;
                }
                update_focus_highlight(&mut param_set.p0(), &focus);
            }

            // Shift+Tab navigation backwards
            ShortcutId::SettingsNavigatePrevious => {
                if focus.index > 0 {
                    focus.index -= 1;
                } else {
                    focus.index = focus.max_index;
                }
                update_focus_highlight(&mut param_set.p0(), &focus);
            }

            // Enter/Space to activate focused element
            ShortcutId::SettingsActivate | ShortcutId::SettingsActivateSpace => {
                if event.shortcut_id == ShortcutId::SettingsActivateSpace {
                    debug!("Space key detected in settings navigation");
                }
                // First find the focused entity
                let mut focused_entity = None;
                for (entity, focusable, _, interaction) in &param_set.p0() {
                    if focusable.order as usize == focus.index && interaction.is_some() {
                        focused_entity = Some(entity);
                        break;
                    }
                }

                // Then simulate the button press on that entity
                if let Some(entity) = focused_entity {
                    if let Ok(mut interaction) = param_set.p1().get_mut(entity) {
                        *interaction = Interaction::Pressed;
                    }
                }
            }

            _ => {} // Ignore other shortcuts
        }
    }
}

/// Helper function to update focus highlighting
fn update_focus_highlight(
    query: &mut Query<(Entity, &Focusable, &mut BackgroundColor, Option<&Interaction>)>,
    focus: &FocusedElement,
) {
    for (_entity, focusable, mut bg_color, interaction) in query {
        if focusable.order as usize == focus.index {
            // Highlight focused element
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.25));
        } else if let Some(interaction) = interaction {
            // Reset to normal state
            match interaction {
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.18, 0.18, 0.2))
                }
                _ => *bg_color = BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
            }
        }
    }
}

/// Update the maximum index for focused elements
pub fn update_max_focus_index(focusables: Query<&Focusable>, mut focus: ResMut<FocusedElement>) {
    let max = focusables
        .iter()
        .map(|f| f.order as usize)
        .max()
        .unwrap_or(0);
    if focus.max_index != max {
        focus.max_index = max;
    }
}
