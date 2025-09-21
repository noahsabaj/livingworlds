//! Speed mapping helpers to eliminate duplication

#![allow(dead_code)] // Preserve utility functions for future use

use bevy::prelude::*;
// Speed constants - re-exported for use within input module
pub const SPEED_PAUSED: f32 = 0.0;
pub const SPEED_NORMAL: f32 = 1.0;
pub const SPEED_FAST: f32 = 3.0;
pub const SPEED_FASTER: f32 = 6.0;
pub const SPEED_FASTEST: f32 = 9.0;

/// Speed level mappings for direct key selection
pub const SPEED_LEVELS: &[(KeyCode, f32, &str)] = &[
    (KeyCode::Digit1, SPEED_PAUSED, "Paused"),
    (KeyCode::Digit2, SPEED_NORMAL, "Normal (1x)"),
    (KeyCode::Digit3, SPEED_FAST, "Fast (3x)"),
    (KeyCode::Digit4, SPEED_FASTER, "Faster (6x)"),
    (KeyCode::Digit5, SPEED_FASTEST, "Fastest (9x)"),
];

pub fn handle_speed_keys(keyboard: &ButtonInput<KeyCode>) -> Option<(f32, &'static str)> {
    SPEED_LEVELS
        .iter()
        .find(|(key, _, _)| keyboard.just_pressed(*key))
        .map(|(_, speed, name)| (*speed, *name))
}

pub fn get_next_speed_level(current_speed: f32, is_paused: bool) -> f32 {
    if is_paused {
        // If paused, go to normal speed
        SPEED_NORMAL
    } else if current_speed < SPEED_NORMAL + 0.1 {
        SPEED_FAST
    } else if current_speed < SPEED_FAST + 0.1 {
        SPEED_FASTER
    } else if current_speed < SPEED_FASTER + 0.1 {
        SPEED_FASTEST
    } else {
        current_speed // Already at max
    }
}

pub fn get_previous_speed_level(current_speed: f32) -> f32 {
    if current_speed > SPEED_FASTEST - 0.1 {
        SPEED_FASTER
    } else if current_speed > SPEED_FASTER - 0.1 {
        SPEED_FAST
    } else if current_speed > SPEED_FAST - 0.1 {
        SPEED_NORMAL
    } else if current_speed > SPEED_NORMAL - 0.1 {
        SPEED_PAUSED
    } else {
        current_speed // Already at min (paused)
    }
}

/// Get the display name for a speed level
pub fn get_speed_name(speed: f32) -> &'static str {
    if speed < 0.1 {
        "Paused"
    } else if speed < 1.1 {
        "Normal (1x)"
    } else if speed < 3.1 {
        "Fast (3x)"
    } else if speed < 6.1 {
        "Faster (6x)"
    } else {
        "Fastest (9x)"
    }
}
