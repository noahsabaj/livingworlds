//! Time control input handling (refactored to eliminate duplication)

use crate::resources::GameTime;
use bevy::prelude::*;
// Access sibling modules through parent gateway
use super::speed_mapping::SPEED_PAUSED;
use super::speed_mapping::{get_next_speed_level, get_previous_speed_level, handle_speed_keys};
use crate::simulation::SimulationSpeedChanged;

/// Handle keyboard input for time control
/// Space for pause/resume, number keys 1-5 for speed control, +/- for speed increment/decrement
pub fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    // Debug: System is being called
    trace!("🎛️ handle_time_controls system running");

    let old_speed = game_time.speed;
    let was_paused = game_time.paused;
    let mut speed_changed = false;

    // Debug: Check for any speed-related key presses
    if keyboard.just_pressed(KeyCode::Space) {
        debug!("🎛️ Space key detected in time controls");
    }
    if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Digit2) ||
       keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Digit4) ||
       keyboard.just_pressed(KeyCode::Digit5) {
        debug!("🎛️ Number key detected in time controls");
    }

    // Handle direct speed selection (keys 1-5)
    if let Some((new_speed, _speed_name)) = handle_speed_keys(&keyboard) {
        if new_speed == SPEED_PAUSED {
            if !game_time.paused {
                game_time.speed_before_pause = game_time.speed;
            }
            game_time.paused = true;
        } else {
            game_time.paused = false;
            game_time.speed_before_pause = new_speed;
        }
        game_time.speed = new_speed;
        speed_changed = true;

        #[cfg(feature = "debug-simulation")]
        debug!("Simulation speed: {}", speed_name);
    }

    // Handle speed increment (+ or numpad +)
    // Note: Plus key on main keyboard is usually KeyCode::Equal (since it's Shift+=)
    // But we also check for explicit Plus in case of different keyboard layouts
    let plus_pressed = keyboard.just_pressed(KeyCode::Equal) ||
                       keyboard.just_pressed(KeyCode::NumpadAdd) ||
                       (keyboard.pressed(KeyCode::ShiftLeft) && keyboard.just_pressed(KeyCode::Equal)) ||
                       (keyboard.pressed(KeyCode::ShiftRight) && keyboard.just_pressed(KeyCode::Equal));

    if plus_pressed {
        debug!("🎛️ Plus key detected in time controls");
        let new_speed = get_next_speed_level(game_time.speed, game_time.paused);

        if new_speed != game_time.speed || game_time.paused {
            game_time.paused = false;
            game_time.speed = new_speed;
            game_time.speed_before_pause = new_speed;
            speed_changed = true;

            info!("🎛️ Speed increased to: {}x", new_speed);
        }
    }

    // Handle speed decrement (- or numpad -)
    let minus_pressed = keyboard.just_pressed(KeyCode::Minus) ||
                        keyboard.just_pressed(KeyCode::NumpadSubtract);

    if minus_pressed {
        debug!("🎛️ Minus key detected in time controls");
        let new_speed = get_previous_speed_level(game_time.speed);

        if new_speed != game_time.speed {
            if new_speed == SPEED_PAUSED {
                game_time.paused = true;
                game_time.speed_before_pause = 1.0; // Default to normal when unpausing
            } else {
                game_time.paused = false;
                game_time.speed_before_pause = new_speed;
            }
            game_time.speed = new_speed;
            speed_changed = true;

            info!("🎛️ Speed decreased to: {}x", new_speed);
        }
    }

    // Handle pause toggle (Space key)
    if keyboard.just_pressed(KeyCode::Space) {
        debug!("🎛️ Space key processing in time controls");
        game_time.paused = !game_time.paused;
        if game_time.paused {
            game_time.speed_before_pause = game_time.speed;
            game_time.speed = SPEED_PAUSED;
        } else {
            // Restore the speed we had before pausing
            game_time.speed = game_time.speed_before_pause;
        }
        speed_changed = true;

        info!(
            "🎛️ Simulation {} (speed: {}x)",
            if game_time.paused {
                "paused"
            } else {
                "resumed"
            },
            if game_time.paused {
                0.0
            } else {
                game_time.speed
            }
        );
    }

    // Send event if speed changed
    if speed_changed && (old_speed != game_time.speed || was_paused != game_time.paused) {
        info!("🎛️ Sending speed change event: {} -> {}, paused: {}", old_speed, game_time.speed, game_time.paused);
        speed_events.write(SimulationSpeedChanged {
            new_speed: game_time.speed,
            is_paused: game_time.paused,
        });
    }
}
