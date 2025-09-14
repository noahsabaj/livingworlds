//! Camera controller components and resources

use crate::constants::*;
use bevy::prelude::*;

/// Camera controller component storing camera state and settings
#[derive(Component)]
pub struct CameraController {
    /// Target position for smooth interpolation
    pub target_position: Vec3,
    /// Target zoom level for smooth zooming
    pub target_zoom: f32,
    /// Current zoom level (cached for performance)
    pub current_zoom: f32,

    /// Smoothing factor (0.0 = instant, 1.0 = very smooth)
    pub position_smoothing: f32,
    pub zoom_smoothing: f32,

    /// Mouse drag state
    pub is_dragging: bool,
    pub last_cursor_pos: Option<Vec2>,

    /// Movement speed multipliers
    pub pan_speed_base: f32,
    pub edge_pan_speed_base: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            target_zoom: 1.0,
            current_zoom: 1.0,
            position_smoothing: 8.0, // Nice smooth feeling
            zoom_smoothing: 10.0,    // Slightly faster zoom response
            is_dragging: false,
            last_cursor_pos: None,
            pan_speed_base: CAMERA_PAN_SPEED_BASE,
            edge_pan_speed_base: CAMERA_EDGE_PAN_SPEED_BASE,
        }
    }
}
