//! Time simulation systems

use super::events::{NewYearEvent, SimulationSpeedChanged};
use super::resources::GameTime;
use super::types::VisualTime;
use crate::constants::{SIMULATION_DAYS_PER_YEAR_F32, SIMULATION_STARTING_YEAR};
use bevy::prelude::*;

/// Advance the game time using deterministic ticks (runs in FixedUpdate)
///
/// This system accumulates real time and generates simulation ticks
/// based on the current speed setting. Running in FixedUpdate ensures
/// deterministic simulation regardless of framerate.
pub fn advance_simulation_ticks(mut game_time: ResMut<GameTime>, fixed_time: Res<Time<Fixed>>) {
    // Don't advance if paused
    if game_time.is_paused() {
        return;
    }

    // Get ticks per second based on current speed
    let ticks_per_second = game_time.get_speed().ticks_per_second();
    if ticks_per_second == 0 {
        return;
    }

    // Calculate how many ticks to advance this fixed update
    let ticks_to_advance = (fixed_time.delta_secs() * ticks_per_second as f32) as u64;

    if ticks_to_advance > 0 {
        game_time.advance_ticks(ticks_to_advance);
    }
}

/// Interpolate visual time for smooth display (runs in Update)
///
/// This provides smooth visual interpolation between discrete simulation ticks,
/// giving the appearance of continuous time while maintaining deterministic simulation.
pub fn interpolate_visual_time(
    game_time: Res<GameTime>,
    mut visual_time: ResMut<VisualTime>,
    time: Res<Time>,
) {
    // Smoothly interpolate the visual representation
    let current_days = game_time.current_day() as f32;
    let target_days = current_days + (time.delta_secs() * game_time.get_speed().multiplier());

    // Use exponential interpolation for smooth transitions
    visual_time.interpolated_days = visual_time.interpolated_days
        + (target_days - visual_time.interpolated_days) * (1.0 - (-10.0 * time.delta_secs()).exp());

    visual_time.interpolated_years = visual_time.interpolated_days / 365.0;
}

/// Track year changes and send events
pub fn track_year_changes(
    game_time: Res<GameTime>,
    mut last_year: Local<u32>,
    mut year_events: EventWriter<NewYearEvent>,
) {
    let current_year = game_time.current_year();

    if current_year != *last_year && *last_year > 0 {
        year_events.write(NewYearEvent { year: current_year });

        #[cfg(feature = "debug-simulation")]
        info!("Year {}", current_year);

        *last_year = current_year;
    } else if *last_year == 0 {
        // Initialize on first run
        *last_year = current_year;
    }
}

/// Resume from pause menu - restore the game speed
pub fn resume_from_pause_menu(
    mut game_time: ResMut<GameTime>,
    mut speed_events: EventWriter<SimulationSpeedChanged>,
) {
    // When transitioning from Paused to InGame via the menu, restore the speed
    if game_time.is_paused() {
        game_time.resume();

        speed_events.write(SimulationSpeedChanged {
            new_speed: game_time.get_speed().multiplier(),
            is_paused: false,
        });

        #[cfg(feature = "debug-simulation")]
        info!(
            "Resumed from pause menu at speed: {}",
            game_time.get_speed().name()
        );
    }
}
