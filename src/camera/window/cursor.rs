//! Cursor confinement management for edge panning

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

/// Setup cursor confinement when entering gameplay to keep mouse within window for edge panning
pub fn setup_cursor_confinement(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        // Confine cursor to window bounds - perfect for strategy games
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
}

/// Release cursor confinement when leaving gameplay (entering menus)
pub fn release_cursor_confinement(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        // Release cursor for menu navigation and alt-tab
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
