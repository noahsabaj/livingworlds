//! Cursor confinement management for edge panning

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

/// Setup cursor confinement when entering gameplay to keep mouse within window for edge panning
pub fn setup_cursor_confinement(mut cursor_options: Single<&mut CursorOptions>) {
    // Confine cursor to window bounds - perfect for strategy games
    cursor_options.grab_mode = CursorGrabMode::Confined;
}

/// Release cursor confinement when leaving gameplay (entering menus)
pub fn release_cursor_confinement(mut cursor_options: Single<&mut CursorOptions>) {
    // Release cursor for menu navigation and alt-tab
    cursor_options.grab_mode = CursorGrabMode::None;
}
