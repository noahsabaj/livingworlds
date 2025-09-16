//! Mouse input handling for camera control

use crate::camera::movement::CameraBounds;
use crate::camera::CameraController;
use crate::constants::CAMERA_ZOOM_SPEED;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Handle mouse wheel zoom with zoom-to-cursor
pub fn handle_mouse_wheel_zoom(
    mut query: Query<(&mut CameraController, &Transform, &Projection)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    bounds: Res<CameraBounds>,
) {
    if mouse_wheel.is_empty() {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((mut controller, transform, projection)) = query.single_mut() else {
        return;
    };

    let Projection::Orthographic(ortho) = projection else {
        return;
    };

    for event in mouse_wheel.read() {
        let zoom_delta = match event.unit {
            MouseScrollUnit::Line => event.y,
            MouseScrollUnit::Pixel => event.y * 0.01,
        };

        let zoom_factor = 1.0 - zoom_delta * CAMERA_ZOOM_SPEED;
        let new_zoom =
            (controller.target_zoom * zoom_factor).clamp(bounds.min_zoom, bounds.max_zoom);

        // Only zoom toward cursor when zooming IN (not when zooming out)
        // This prevents the disorienting "tugging" feeling when trying to see more of the map
        let is_zooming_in = new_zoom < controller.target_zoom;

        if is_zooming_in {
            // Zoom toward cursor position if cursor is over window
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to world coordinates before zoom
                let cursor_ndc = Vec2::new(
                    (cursor_pos.x / window.width()) * 2.0 - 1.0,
                    -((cursor_pos.y / window.height()) * 2.0 - 1.0),
                );

                let world_pos_before = Vec2::new(
                    cursor_ndc.x * ortho.scale * window.width() / 2.0 + transform.translation.x,
                    cursor_ndc.y * ortho.scale * window.height() / 2.0 + transform.translation.y,
                );

                let world_pos_after = Vec2::new(
                    cursor_ndc.x * new_zoom * window.width() / 2.0 + transform.translation.x,
                    cursor_ndc.y * new_zoom * window.height() / 2.0 + transform.translation.y,
                );

                // Adjust target position to keep cursor at same world position
                // Use a factor to make it less aggressive (0.5 = 50% of the movement)
                let zoom_to_cursor_strength = 0.7; // Adjust this to taste
                controller.target_position.x +=
                    (world_pos_before.x - world_pos_after.x) * zoom_to_cursor_strength;
                controller.target_position.y +=
                    (world_pos_before.y - world_pos_after.y) * zoom_to_cursor_strength;
            }
        }
        // When zooming OUT, just zoom from current center (no camera movement)

        controller.target_zoom = new_zoom;
    }
}

/// Handle middle mouse button drag for panning
pub fn handle_mouse_drag(
    mut query: Query<&mut CameraController>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    // Start/stop dragging
    if mouse_button.just_pressed(MouseButton::Middle) {
        controller.is_dragging = true;
        controller.last_cursor_pos = window.cursor_position();
    } else if mouse_button.just_released(MouseButton::Middle) {
        controller.is_dragging = false;
    }

    // Apply drag movement
    if controller.is_dragging && !mouse_motion.is_empty() {
        let mut total_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            total_delta += event.delta;
        }

        // Convert screen delta to world delta (accounting for zoom)
        let world_delta = total_delta * controller.current_zoom;
        controller.target_position.x -= world_delta.x;
        controller.target_position.y += world_delta.y; // Y is inverted
    }
}
