//! Camera plugin implementation

use super::input;
use super::movement;
use super::setup::setup_camera;
use super::window;
use crate::states::GameState;
use bevy::prelude::{IntoScheduleConfigs, in_state};
use bevy_plugin_builder::define_plugin;

/// Camera control plugin for managing viewport and camera movement using declarative syntax
define_plugin!(CameraPlugin {
    resources: [movement::CameraBounds, window::WindowFocusState],

    startup: [setup_camera],

    update: [
        (
            input::handle_keyboard_movement,
            input::handle_mouse_wheel_zoom,
            input::handle_mouse_drag,
            input::handle_edge_panning,
            input::handle_camera_shortcuts,
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