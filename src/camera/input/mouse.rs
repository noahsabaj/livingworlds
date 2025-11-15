//! Mouse input handling for camera control

use crate::camera::movement::CameraBounds;
use crate::camera::CameraController;
use crate::constants::CAMERA_ZOOM_SPEED;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Conversion factor for pixel-based scroll events
const PIXEL_SCROLL_FACTOR: f32 = 0.01;
/// How aggressively the camera moves toward cursor when zooming in (0.0-1.0)
const ZOOM_TO_CURSOR_STRENGTH: f32 = 0.7;

/// Calculate camera position offset to zoom toward cursor position
fn calculate_zoom_offset_for_cursor(
    cursor_pos: Vec2,
    window: &Window,
    current_transform: &Transform,
    current_zoom: f32,
    new_zoom: f32,
) -> Vec2 {
    // Convert cursor position to normalized device coordinates
    let cursor_ndc = Vec2::new(
        (cursor_pos.x / window.width()) * 2.0 - 1.0,
        -((cursor_pos.y / window.height()) * 2.0 - 1.0),
    );

    // Calculate world position before zoom
    let world_pos_before = Vec2::new(
        cursor_ndc.x * current_zoom * window.width() / 2.0 + current_transform.translation.x,
        cursor_ndc.y * current_zoom * window.height() / 2.0 + current_transform.translation.y,
    );

    // Calculate world position after zoom
    let world_pos_after = Vec2::new(
        cursor_ndc.x * new_zoom * window.width() / 2.0 + current_transform.translation.x,
        cursor_ndc.y * new_zoom * window.height() / 2.0 + current_transform.translation.y,
    );

    // Return offset scaled by zoom-to-cursor strength
    (world_pos_before - world_pos_after) * ZOOM_TO_CURSOR_STRENGTH
}

/// Handle mouse wheel zoom with zoom-to-cursor
pub fn handle_mouse_wheel_zoom(
    mut query: Query<(&mut CameraController, &Transform, &Projection)>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    bounds: Res<CameraBounds>,
) {
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
            MouseScrollUnit::Pixel => event.y * PIXEL_SCROLL_FACTOR,
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
                let offset = calculate_zoom_offset_for_cursor(
                    cursor_pos,
                    window,
                    transform,
                    ortho.scale,
                    new_zoom,
                );
                controller.target_position.x += offset.x;
                controller.target_position.y += offset.y;
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
    mut mouse_motion: MessageReader<MouseMotion>,
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
