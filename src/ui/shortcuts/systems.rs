//! Systems for processing keyboard shortcuts

use bevy::prelude::*;
use super::registry::*;
use super::types::*;

/// Process keyboard input and trigger shortcuts
pub fn process_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ShortcutRegistry>,
    config: Res<ShortcutConfig>,
    mut events: EventWriter<ShortcutEvent>,
) {
    if !config.enabled {
        return;
    }

    // Check all registered shortcuts
    if let Some(shortcut_id) = registry.check_shortcuts(&keyboard) {
        if let Some(definition) = registry.get(&shortcut_id) {
            events.send(ShortcutEvent {
                shortcut_id: shortcut_id.clone(),
                binding: definition.binding,
                context: definition.context,
            });

            debug!("Shortcut triggered: {:?} ({})", shortcut_id, definition.binding.display_string());
        }
    }
}

/// Update shortcut context based on game state
pub fn update_shortcut_context(
    mut registry: ResMut<ShortcutRegistry>,
    state: Option<Res<State<crate::states::GameState>>>,
    dialog_query: Query<Entity, With<crate::ui::DialogOverlay>>,
) {
    // Determine context based on game state and UI elements
    let context = if !dialog_query.is_empty() {
        ShortcutContext::Dialog
    } else if let Some(state) = state {
        match state.get() {
            crate::states::GameState::MainMenu => ShortcutContext::MainMenu,
            crate::states::GameState::InGame => ShortcutContext::InGame,
            crate::states::GameState::WorldConfiguration => ShortcutContext::WorldConfig,
            _ => ShortcutContext::Global,
        }
    } else {
        ShortcutContext::Global
    };

    registry.set_context(context);
}

/// Update UI hints to show available shortcuts
pub fn update_shortcut_hints(
    registry: Res<ShortcutRegistry>,
    config: Res<ShortcutConfig>,
    mut text_query: Query<&mut Text, With<ShortcutHint>>,
) {
    if !config.show_hints {
        // Hide all hints
        for mut text in &mut text_query {
            text.0 = String::new();
        }
        return;
    }

    // Update hint text with current shortcuts
    for mut text in &mut text_query {
        // This would be customized based on the specific hint component
        // For now, just show a simple list
        let shortcuts = registry.shortcuts_in_context(registry.context());

        let hint_text = shortcuts
            .iter()
            .take(5) // Show first 5 shortcuts
            .map(|def| def.display_string())
            .collect::<Vec<_>>()
            .join("\n");

        text.0 = hint_text;
    }
}

/// Check for shortcut conflicts (runs on startup or when shortcuts change)
pub fn check_shortcut_conflicts(
    registry: Res<ShortcutRegistry>,
    config: Res<ShortcutConfig>,
) {
    if !config.check_conflicts {
        return;
    }

    // Registry already handles conflicts during registration
    // This system could be extended to provide UI feedback about conflicts
}

/// Handle common shortcut actions
pub fn handle_common_shortcuts(
    mut events: EventReader<ShortcutEvent>,
) {
    for event in events.read() {
        match event.shortcut_id {
            // Time controls are handled by time_controls.rs - not here!
            // This prevents duplicate handling and ensures proper separation of concerns

            // Screenshots
            ShortcutId::TakeScreenshot => {
                info!("Taking screenshot...");
                // Would trigger screenshot system
            }

            _ => {
                // Other shortcuts handled by specific systems
            }
        }
    }
}

/// Marker component for UI elements that display shortcut hints
#[derive(Component)]
pub struct ShortcutHint {
    /// Which shortcuts to show hints for
    pub filter: ShortcutHintFilter,
}

/// Filter for which shortcuts to show in hints
#[derive(Debug, Clone)]
pub enum ShortcutHintFilter {
    /// Show all shortcuts in current context
    All,
    /// Show shortcuts from specific group
    Group(String),
    /// Show specific shortcuts
    Specific(Vec<ShortcutId>),
}

/// System to handle shortcut rebinding UI
pub fn handle_rebinding(
    mut registry: ResMut<ShortcutRegistry>,
    config: Res<ShortcutConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
    rebind_query: Query<&RebindingShortcut>,
) {
    if !config.allow_rebinding {
        return;
    }

    for rebinding in &rebind_query {
        // Check if any key was pressed
        for key in keyboard.get_pressed() {
            let modifiers = ModifierKeys {
                ctrl: keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight),
                shift: keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight),
                alt: keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight),
                meta: false,
            };

            // Skip if only modifier keys are pressed
            if matches!(key, KeyCode::ControlLeft | KeyCode::ControlRight |
                           KeyCode::ShiftLeft | KeyCode::ShiftRight |
                           KeyCode::AltLeft | KeyCode::AltRight) {
                continue;
            }

            let new_binding = KeyBinding {
                key: *key,
                modifiers,
            };

            match registry.rebind(rebinding.shortcut_id.clone(), new_binding) {
                Ok(()) => {
                    info!("Rebound {:?} to {}", rebinding.shortcut_id, new_binding.display_string());
                }
                Err(conflict) => {
                    warn!("Cannot rebind: {}", conflict);
                }
            }
        }
    }
}

/// Component for entities that are waiting for rebinding input
#[derive(Component)]
pub struct RebindingShortcut {
    pub shortcut_id: ShortcutId,
}