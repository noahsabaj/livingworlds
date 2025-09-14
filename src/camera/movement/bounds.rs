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
    let Ok(window) = windows.get_single() else {
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
    windows: Query<&Window, With<PrimaryWindow>>,
    map_dimensions: Res<MapDimensions>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    for (mut transform, mut controller, projection) in query.iter_mut() {
        let Projection::Orthographic(ortho) = projection else {
            continue;
        };

        let visible_width = window.width() * ortho.scale;
        let visible_height = window.height() * ortho.scale;

        // Y-axis clamping with margin
        let margin_factor = 0.3;
        let max_y = (bounds.max_y - visible_height / 2.0
            + map_dimensions.height_pixels * margin_factor)
            .max(0.0);

        transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
        controller.target_position.y = controller.target_position.y.clamp(-max_y, max_y);

        // X-axis clamping (same as Y-axis, no wrapping)
        let max_x = (bounds.half_map_width - visible_width / 2.0
            + map_dimensions.width_pixels * margin_factor)
            .max(0.0);

        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
        controller.target_position.x = controller.target_position.x.clamp(-max_x, max_x);
    }
}
