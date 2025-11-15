//! Camera keyboard controls using shortcuts registry for rebindable keys

use crate::camera::movement::CameraBounds;
use crate::camera::CameraController;
use crate::constants::*;
use crate::ui::{ShortcutRegistry, ShortcutId, ShortcutEvent};
use bevy::prelude::*;

/// Zoom factor when zooming in via keyboard (multiply by this)
const KEYBOARD_ZOOM_IN_FACTOR: f32 = 0.9;
/// Zoom factor when zooming out via keyboard (multiply by this)
const KEYBOARD_ZOOM_OUT_FACTOR: f32 = 1.1;

/// Helper function to check if a key from registry or fallback is pressed
fn is_key_pressed(
    keyboard: &ButtonInput<KeyCode>,
    registry_key: Option<KeyCode>,
    fallback_key: KeyCode,
) -> bool {
    registry_key.map_or(false, |k| keyboard.pressed(k)) || keyboard.pressed(fallback_key)
}

/// Handle camera movement with continuous key press detection
/// Movement keys are rebindable through the shortcuts registry
pub fn handle_keyboard_movement(
    mut query: Query<&mut CameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<ShortcutRegistry>,
    time: Res<Time>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    // Get the actual key bindings from the registry
    let up_key = registry.get(&ShortcutId::CameraUp).map(|def| def.binding.key);
    let down_key = registry.get(&ShortcutId::CameraDown).map(|def| def.binding.key);
    let left_key = registry.get(&ShortcutId::CameraLeft).map(|def| def.binding.key);
    let right_key = registry.get(&ShortcutId::CameraRight).map(|def| def.binding.key);

    // Check if any movement keys are pressed (including arrow keys as fallback)
    let any_movement = is_key_pressed(&keyboard, up_key, KeyCode::ArrowUp)
        || is_key_pressed(&keyboard, down_key, KeyCode::ArrowDown)
        || is_key_pressed(&keyboard, left_key, KeyCode::ArrowLeft)
        || is_key_pressed(&keyboard, right_key, KeyCode::ArrowRight);

    if !any_movement {
        return;
    }

    let speed_multiplier =
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            CAMERA_SPEED_MULTIPLIER
        } else {
            1.0
        };

    let pan_speed =
        controller.pan_speed_base * controller.current_zoom * time.delta_secs() * speed_multiplier;

    // Handle movement based on registered keys or arrow fallbacks
    if is_key_pressed(&keyboard, up_key, KeyCode::ArrowUp) {
        controller.target_position.y += pan_speed;
    }
    if is_key_pressed(&keyboard, down_key, KeyCode::ArrowDown) {
        controller.target_position.y -= pan_speed;
    }
    if is_key_pressed(&keyboard, left_key, KeyCode::ArrowLeft) {
        controller.target_position.x -= pan_speed;
    }
    if is_key_pressed(&keyboard, right_key, KeyCode::ArrowRight) {
        controller.target_position.x += pan_speed;
    }
}

/// Handle camera reset and zoom shortcuts via events
pub fn handle_camera_shortcuts(
    mut query: Query<&mut CameraController>,
    mut shortcut_events: MessageReader<ShortcutEvent>,
    bounds: Res<CameraBounds>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    for event in shortcut_events.read() {
        match event.shortcut_id {
            ShortcutId::CameraReset => {
                controller.target_position = Vec3::ZERO;
                controller.target_zoom = 1.0;
            }
            ShortcutId::CameraZoomIn => {
                controller.target_zoom =
                    (controller.target_zoom * KEYBOARD_ZOOM_IN_FACTOR).clamp(bounds.min_zoom, bounds.max_zoom);
            }
            ShortcutId::CameraZoomOut => {
                controller.target_zoom =
                    (controller.target_zoom * KEYBOARD_ZOOM_OUT_FACTOR).clamp(bounds.min_zoom, bounds.max_zoom);
            }
            _ => {}
        }
    }
}
