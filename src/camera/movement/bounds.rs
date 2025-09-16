//! Camera boundary constraints and calculations

use crate::camera::CameraController;
use crate::constants::*;
use crate::resources::MapDimensions;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Cached camera bounds to avoid recalculation every frame
#[derive(Resource, Default)]
pub struct CameraBounds {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub max_y: f32,
    pub half_map_width: f32,
}

/// Calculate camera bounds once after startup
pub fn calculate_camera_bounds(
    mut bounds: ResMut<CameraBounds>,
    map_dimensions: Res<MapDimensions>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let min_zoom_x = map_dimensions.width_pixels / window.width();
    let min_zoom_y = map_dimensions.height_pixels / window.height();
    let min_zoom = min_zoom_x.max(min_zoom_y) * CAMERA_MAP_PADDING_FACTOR;

    bounds.min_zoom = CAMERA_MIN_ZOOM;
    bounds.max_zoom = min_zoom.max(CAMERA_MAX_ZOOM);
    bounds.half_map_width = map_dimensions.width_pixels / 2.0;
    bounds.max_y = map_dimensions.height_pixels / 2.0;
}

/// Apply camera bounds with clamping
pub fn apply_camera_bounds(
    mut query: Query<(&mut Transform, &mut CameraController, &Projection)>,
    bounds: Res<CameraBounds>,
    _windows: Query<&Window, With<PrimaryWindow>>,
    _map_dimensions: Res<MapDimensions>,
) {
    for (mut transform, mut controller, _projection) in query.iter_mut() {
        // Simple center-only constraints
        // Allow camera center to go 50% beyond map edges for better panning when zoomed out
        let margin_factor = 0.5;
        let max_x = bounds.half_map_width * (1.0 + margin_factor);
        let max_y = bounds.max_y * (1.0 + margin_factor);

        // Clamp camera center position
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);

        // Also clamp target position for smooth interpolation
        controller.target_position.x = controller.target_position.x.clamp(-max_x, max_x);
        controller.target_position.y = controller.target_position.y.clamp(-max_y, max_y);
    }
}
