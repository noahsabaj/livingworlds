//! Time simulation systems

use super::events::{NewYearEvent, SimulationSpeedChanged};
use super::resources::GameTime;
use crate::constants::{SIMULATION_DAYS_PER_YEAR_F32, SIMULATION_STARTING_YEAR};
use bevy::prelude::*;

/// Advance the game time based on real time and speed multiplier
pub fn advance_game_time(mut game_time: ResMut<GameTime>, time: Res<Time>) {
    // Don't advance if paused
    if game_time.paused {
        return;
    }

    // Advance game time (in days) based on real time and speed multiplier
    // 1 real second = 1 game day at 1x speed
    game_time.current_date += time.delta_secs() * game_time.speed;
}

/// Track year changes and send events
pub fn track_year_changes(
    game_time: Res<GameTime>,
    mut last_year: Local<u32>,
    mut year_events: EventWriter<NewYearEvent>,
) {
    let current_year =
        SIMULATION_STARTING_YEAR + (game_time.current_date / SIMULATION_DAYS_PER_YEAR_F32) as u32;

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
    if game_time.paused {
        game_time.paused = false;
        game_time.speed = game_time.speed_before_pause;

        speed_events.write(SimulationSpeedChanged {
            new_speed: game_time.speed,
            is_paused: false,
        });

        #[cfg(feature = "debug-simulation")]
        info!("Resumed from pause menu at speed: {}x", game_time.speed);
    }
}
