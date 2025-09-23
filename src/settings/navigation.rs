//! Keyboard navigation for the settings menu

use super::components::*;
use bevy::prelude::*;

/// Handle keyboard navigation (Tab/Shift+Tab) and ESC
pub fn handle_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
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
    // Only process input if settings menu is actually open
    let settings_entity = match settings_root.get_single() {
        Ok(entity) => {
            debug!("⚙️ Settings menu open, processing navigation input");
            entity
        },
        Err(_) => {
            trace!("⚙️ No settings menu open, skipping navigation input");
            return; // No settings menu open, skip all input processing
        }
    };

    // ESC to close settings
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.entity(settings_entity).despawn();
        return;
    }

    // Tab navigation
    if keyboard.just_pressed(KeyCode::Tab) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            // Shift+Tab - go backwards
            if focus.index > 0 {
                focus.index -= 1;
            } else {
                focus.index = focus.max_index;
            }
        } else {
            // Tab - go forwards
            if focus.index < focus.max_index {
                focus.index += 1;
            } else {
                focus.index = 0;
            }
        }

        for (_entity, focusable, mut bg_color, interaction) in &mut param_set.p0() {
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

    // Enter/Space to activate focused element
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if keyboard.just_pressed(KeyCode::Space) {
            debug!("⚙️ Space key detected in settings navigation");
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
