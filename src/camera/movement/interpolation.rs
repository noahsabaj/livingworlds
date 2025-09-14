//! Smooth camera movement interpolation

use crate::camera::CameraController;
use crate::math::{lerp_exp, lerp_exp_vec3};
use bevy::prelude::*;

/// Apply smooth interpolation to camera movement and zoom
pub fn apply_smooth_movement(
    mut query: Query<(&mut Transform, &mut Projection, &mut CameraController)>,
    time: Res<Time>,
) {
    for (mut transform, mut projection, mut controller) in query.iter_mut() {
        // Smooth position interpolation using centralized function
        transform.translation = lerp_exp_vec3(
            transform.translation,
            controller.target_position,
            controller.position_smoothing,
            time.delta_secs(),
        );

        // Smooth zoom interpolation using centralized function
        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            ortho.scale = lerp_exp(
                ortho.scale,
                controller.target_zoom,
                controller.zoom_smoothing,
                time.delta_secs(),
            );
            controller.current_zoom = ortho.scale; // Cache for other systems
        }
    }
}
