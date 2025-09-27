//! Camera keyboard controls using shortcuts registry for rebindable keys

use crate::camera::CameraController;
use crate::constants::*;
use crate::ui::shortcuts::{ShortcutRegistry, ShortcutId, ShortcutEvent};
use bevy::prelude::*;

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
    let any_movement = up_key.map_or(false, |k| keyboard.pressed(k))
        || down_key.map_or(false, |k| keyboard.pressed(k))
        || left_key.map_or(false, |k| keyboard.pressed(k))
        || right_key.map_or(false, |k| keyboard.pressed(k))
        || keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::ArrowRight);

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
    if up_key.map_or(false, |k| keyboard.pressed(k)) || keyboard.pressed(KeyCode::ArrowUp) {
        controller.target_position.y += pan_speed;
    }
    if down_key.map_or(false, |k| keyboard.pressed(k)) || keyboard.pressed(KeyCode::ArrowDown) {
        controller.target_position.y -= pan_speed;
    }
    if left_key.map_or(false, |k| keyboard.pressed(k)) || keyboard.pressed(KeyCode::ArrowLeft) {
        controller.target_position.x -= pan_speed;
    }
    if right_key.map_or(false, |k| keyboard.pressed(k)) || keyboard.pressed(KeyCode::ArrowRight) {
        controller.target_position.x += pan_speed;
    }
}

/// Handle camera reset and zoom shortcuts via events
pub fn handle_camera_shortcuts(
    mut query: Query<&mut CameraController>,
    mut shortcut_events: EventReader<ShortcutEvent>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    for event in shortcut_events.read() {
        match event.shortcut_id {
            ShortcutId::CameraReset => {
                controller.target_position = Vec3::ZERO;
                controller.target_zoom = 1.0;
                info!("Camera reset to origin");
            }
            ShortcutId::CameraZoomIn => {
                controller.target_zoom = (controller.target_zoom * 0.9).max(0.1);
            }
            ShortcutId::CameraZoomOut => {
                controller.target_zoom = (controller.target_zoom * 1.1).min(10.0);
            }
            _ => {}
        }
    }
}
