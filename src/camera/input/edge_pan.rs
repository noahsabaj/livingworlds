//! Edge panning for RTS-style camera movement

use crate::camera::CameraController;
use crate::constants::CAMERA_EDGE_PAN_THRESHOLD;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Handle edge panning when cursor is near screen edges
pub fn handle_edge_panning(
    mut query: Query<&mut CameraController>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(mut controller) = query.single_mut() else {
        return;
    };

    // Only edge pan if not dragging
    if controller.is_dragging {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let edge_threshold = CAMERA_EDGE_PAN_THRESHOLD;
    let edge_speed = controller.edge_pan_speed_base * controller.current_zoom * time.delta_secs();

    if cursor_pos.x <= edge_threshold {
        controller.target_position.x -= edge_speed;
    }
    if cursor_pos.x >= window.width() - edge_threshold {
        controller.target_position.x += edge_speed;
    }
    if cursor_pos.y <= edge_threshold {
        controller.target_position.y += edge_speed;
    }
    if cursor_pos.y >= window.height() - edge_threshold {
        controller.target_position.y -= edge_speed;
    }
}
