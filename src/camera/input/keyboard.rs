//! Keyboard input handling for camera movement

use crate::camera::CameraController;
use crate::constants::*;
use bevy::prelude::*;

/// Handle keyboard input for camera panning (WASD and arrow keys)
/// NOTE: ESC key handling has been moved to main.rs where game state management belongs
pub fn handle_keyboard_movement(
    mut query: Query<&mut CameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Early return if no movement keys pressed
    if !keyboard.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyA,
        KeyCode::KeyD,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]) {
        return;
    }

    let Ok(mut controller) = query.get_single_mut() else {
        return;
    };

    let speed_multiplier =
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            CAMERA_SPEED_MULTIPLIER
        } else {
            1.0
        };

    let pan_speed =
        controller.pan_speed_base * controller.current_zoom * time.delta_secs() * speed_multiplier;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        controller.target_position.y += pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        controller.target_position.y -= pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        controller.target_position.x -= pan_speed;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        controller.target_position.x += pan_speed;
    }
}

/// Handle camera reset with Home key
pub fn handle_camera_reset(
    mut query: Query<&mut CameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Home) {
        if let Ok(mut controller) = query.get_single_mut() {
            controller.target_position = Vec3::ZERO;
            controller.target_zoom = 1.0;
        }
    }
}
