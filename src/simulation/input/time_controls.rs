//! Time control input handling (refactored to eliminate duplication)

use crate::resources::GameTime;
use bevy::prelude::*;
// Access sibling modules through parent gateway
use super::speed_mapping::SPEED_PAUSED;
use super::speed_mapping::{
    get_next_speed_level, get_previous_speed_level, get_speed_name, handle_speed_keys,
};
use crate::simulation::SimulationSpeedChanged;

/// Handle keyboard input for time control
/// Space for pause/resume, number keys 1-5 for speed control, +/- for speed increment/decrement
pub fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    let old_speed = game_time.speed;
    let was_paused = game_time.paused;
    let mut speed_changed = false;

    // Handle direct speed selection (keys 1-5)
    if let Some((new_speed, speed_name)) = handle_speed_keys(&keyboard) {
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
        println!("Simulation speed: {}", speed_name);
    }

    // Handle speed increment (+ or numpad +)
    if keyboard.just_pressed(KeyCode::Equal) || keyboard.just_pressed(KeyCode::NumpadAdd) {
        let new_speed = get_next_speed_level(game_time.speed, game_time.paused);

        if new_speed != game_time.speed || game_time.paused {
            game_time.paused = false;
            game_time.speed = new_speed;
            game_time.speed_before_pause = new_speed;
            speed_changed = true;

            #[cfg(feature = "debug-simulation")]
            println!("Speed increased to: {}", get_speed_name(new_speed));
        }
    }

    // Handle speed decrement (- or numpad -)
    if keyboard.just_pressed(KeyCode::Minus) || keyboard.just_pressed(KeyCode::NumpadSubtract) {
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

            #[cfg(feature = "debug-simulation")]
            println!("Speed decreased to: {}", get_speed_name(new_speed));
        }
    }

    // Handle pause toggle (Space key)
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.paused = !game_time.paused;
        if game_time.paused {
            game_time.speed_before_pause = game_time.speed;
            game_time.speed = SPEED_PAUSED;
        } else {
            // Restore the speed we had before pausing
            game_time.speed = game_time.speed_before_pause;
        }
        speed_changed = true;

        #[cfg(feature = "debug-simulation")]
        println!(
            "Simulation {} (speed: {}x)",
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
        speed_events.send(SimulationSpeedChanged {
            new_speed: game_time.speed,
            is_paused: game_time.paused,
        });
    }
}
