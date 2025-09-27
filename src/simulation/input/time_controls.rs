//! Time control input handling (refactored to eliminate duplication)

use bevy::prelude::*;
use crate::simulation::{GameTime, SimulationSpeed, SimulationSpeedChanged};
use super::speed_mapping::{get_next_speed_level, get_previous_speed_level, handle_speed_keys};

/// Handle keyboard input for time control
/// Space for pause/resume, number keys 1-5 for speed control, +/- for speed increment/decrement
pub fn handle_time_controls(
    mut game_time: ResMut<GameTime>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    // Debug: System is being called
    trace!("🎛️ handle_time_controls system running");

    let old_speed = game_time.get_speed();
    let was_paused = game_time.is_paused();
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
    if let Some(new_speed) = handle_speed_keys(&keyboard) {
        game_time.set_speed(new_speed);
        speed_changed = true;

        #[cfg(feature = "debug-simulation")]
        debug!("Simulation speed: {}", new_speed.name());
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
        let current_speed = game_time.get_speed();
        let new_speed = get_next_speed_level(current_speed);

        if new_speed != current_speed {
            game_time.set_speed(new_speed);
            speed_changed = true;

            info!("🎛️ Speed increased to: {}", new_speed.name());
        }
    }

    // Handle speed decrement (- or numpad -)
    let minus_pressed = keyboard.just_pressed(KeyCode::Minus) ||
                        keyboard.just_pressed(KeyCode::NumpadSubtract);

    if minus_pressed {
        debug!("🎛️ Minus key detected in time controls");
        let current_speed = game_time.get_speed();
        let new_speed = get_previous_speed_level(current_speed);

        if new_speed != current_speed {
            game_time.set_speed(new_speed);
            speed_changed = true;

            info!("🎛️ Speed decreased to: {}", new_speed.name());
        }
    }

    // Handle pause toggle (Space key)
    if keyboard.just_pressed(KeyCode::Space) {
        debug!("🎛️ Space key processing in time controls");
        game_time.toggle_pause();
        speed_changed = true;

        info!(
            "🎛️ Simulation {} (speed: {})",
            if game_time.is_paused() {
                "paused"
            } else {
                "resumed"
            },
            game_time.get_speed().name()
        );
    }

    // Send event if speed changed
    if speed_changed && (old_speed != game_time.get_speed() || was_paused != game_time.is_paused()) {
        info!("🎛️ Sending speed change event: {} -> {}, paused: {}",
            old_speed.name(), game_time.get_speed().name(), game_time.is_paused());
        speed_events.write(SimulationSpeedChanged {
            new_speed: game_time.get_speed().multiplier(),
            is_paused: game_time.is_paused(),
        });
    }
}
