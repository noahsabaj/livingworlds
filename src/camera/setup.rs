//! Camera setup systems

use super::controller::CameraController;
use crate::constants::COLOR_OCEAN_BACKGROUND;
use bevy::prelude::*;

/// Setup the main game camera with initial position and projection
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera, // Required for UI text to render
        Camera {
            clear_color: ClearColorConfig::Custom(COLOR_OCEAN_BACKGROUND),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        CameraController::default(),
        Name::new("Main Camera"),
    ));
}