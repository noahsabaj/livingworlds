//! Camera plugin implementation

use super::controller::CameraController;
use super::input;
use super::movement;
use super::window;
use crate::states::GameState;
use bevy::prelude::*;

/// Camera control plugin for managing viewport and camera movement
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<movement::CameraBounds>()
            .init_resource::<window::WindowFocusState>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    // Input systems
                    input::handle_keyboard_movement,
                    input::handle_mouse_wheel_zoom,
                    input::handle_mouse_drag,
                    input::handle_edge_panning,
                    input::handle_camera_reset,
                    // Window management
                    window::handle_window_focus,
                    // Movement and interpolation
                    movement::apply_smooth_movement,
                    movement::apply_camera_bounds,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    movement::calculate_camera_bounds,
                    window::setup_cursor_confinement,
                ),
            )
            .add_systems(
                OnExit(GameState::InGame),
                window::release_cursor_confinement,
            );
    }
}

/// Setup the main game camera with initial position and projection
pub fn setup_camera(mut commands: Commands) {
    use crate::constants::COLOR_OCEAN_BACKGROUND;

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
