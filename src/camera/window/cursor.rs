//! Cursor confinement management for edge panning

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

/// Setup cursor confinement when entering gameplay to keep mouse within window for edge panning
///
/// TODO: Re-enable when bevy-ui-builders 0.2.0 is updated to properly support Bevy 0.17 Window API
/// The issue is that bevy-ui-builders re-exports an outdated Window type that doesn't have the cursor field
pub fn setup_cursor_confinement(_windows: Query<&mut bevy::window::Window, With<PrimaryWindow>>) {
    // Temporarily disabled due to bevy-ui-builders type conflict
    // for mut window in &mut windows {
    //     // Confine cursor to window bounds - perfect for strategy games
    //     window.cursor.grab_mode = CursorGrabMode::Confined;
    // }
}

/// Release cursor confinement when leaving gameplay (entering menus)
///
/// TODO: Re-enable when bevy-ui-builders 0.2.0 is updated to properly support Bevy 0.17 Window API
/// The issue is that bevy-ui-builders re-exports an outdated Window type that doesn't have the cursor field
pub fn release_cursor_confinement(_windows: Query<&mut bevy::window::Window, With<PrimaryWindow>>) {
    // Temporarily disabled due to bevy-ui-builders type conflict
    // for mut window in &mut windows {
    //     // Release cursor for menu navigation and alt-tab
    //     window.cursor.grab_mode = CursorGrabMode::None;
    // }
}
