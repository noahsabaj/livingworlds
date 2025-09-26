//! Time control utilities for testing
//!
//! Functions to advance time in test apps for simulating game progression.

use bevy::prelude::*;
use crate::simulation::time::resources::GameTime;

/// Advance the test app by a number of frames
pub fn advance_frames(app: &mut App, frames: u32) {
    for _ in 0..frames {
        app.update();
    }
}

/// Advance the test app by game days
pub fn advance_days(app: &mut App, days: u32) {
    // Each update represents 1 game day in test mode
    for _ in 0..days {
        if let Some(mut time) = app.world_mut().get_resource_mut::<GameTime>() {
            time.advance_day();
        }
        app.update();
    }
}