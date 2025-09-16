//! Window focus management for camera

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

/// Tracks window focus state for cursor confinement
#[derive(Resource, Default)]
pub struct WindowFocusState {
    /// Whether window was focused last frame (for detecting focus changes)
    pub was_focused: bool,
}

/// Automatically handle cursor confinement based on window focus
/// This allows alt-tab and window switching to work properly
pub fn handle_window_focus(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut focus_state: ResMut<WindowFocusState>,
) {
    if let Ok(mut window) = windows.single_mut() {
        let is_focused = window.focused;

        // Detect focus state changes
        if is_focused != focus_state.was_focused {
            focus_state.was_focused = is_focused;

            window.cursor_options.grab_mode = if is_focused {
                CursorGrabMode::Confined // Enable edge panning
            } else {
                CursorGrabMode::None // Allow alt-tab and window switching
            };
        }
    }
}
