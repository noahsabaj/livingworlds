//! Window focus management for camera

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Tracks window focus state for cursor confinement
#[derive(Resource, Default)]
pub struct WindowFocusState {
    /// Whether window was focused last frame (for detecting focus changes)
    pub was_focused: bool,
}

/// Automatically handle cursor confinement based on window focus
/// This allows alt-tab and window switching to work properly
///
/// TODO: Re-enable when bevy-ui-builders 0.2.0 is updated to properly support Bevy 0.17 Window API
/// The issue is that bevy-ui-builders re-exports an outdated Window type that doesn't have the cursor field
pub fn handle_window_focus(
    _windows: Query<&mut bevy::window::Window, With<PrimaryWindow>>,
    mut focus_state: ResMut<WindowFocusState>,
) {
    // Temporarily disabled due to bevy-ui-builders type conflict
    // for mut window in &mut windows {
    //     let is_focused = window.focused;
    //
    //     // Detect focus state changes
    //     if is_focused != focus_state.was_focused {
    //         focus_state.was_focused = is_focused;
    //
    //         window.cursor.grab_mode = if is_focused {
    //             CursorGrabMode::Confined // Enable edge panning
    //         } else {
    //             CursorGrabMode::None // Allow alt-tab and window switching
    //         };
    //     }
    // }

    // Keep the state updated even though we're not acting on it
    focus_state.was_focused = false;
}
