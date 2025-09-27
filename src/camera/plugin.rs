//! Camera plugin implementation

use super::controller::CameraController;
use super::input;
use super::movement;
use super::window;
use crate::states::GameState;
use bevy::prelude::{
    Camera, Camera2d, ClearColorConfig, Commands, IntoScheduleConfigs, IsDefaultUiCamera, Name, Transform,
    default, in_state,
};
use bevy_plugin_builder::define_plugin;

// Camera control plugin for managing viewport and camera movement using declarative syntax
define_plugin!(CameraPlugin {
    resources: [movement::CameraBounds, window::WindowFocusState],

    startup: [setup_camera],

    update: [
        (
            input::handle_keyboard_movement,
            input::handle_mouse_wheel_zoom,
            input::handle_mouse_drag,
            input::handle_edge_panning,
            input::handle_camera_reset,
            window::handle_window_focus,
            movement::apply_smooth_movement,
            movement::apply_camera_bounds,
        ).chain().run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [
            movement::calculate_camera_bounds,
            window::setup_cursor_confinement
        ]
    },

    on_exit: {
        GameState::InGame => [window::release_cursor_confinement]
    }
});

// Setup the main game camera with initial position and projection
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
