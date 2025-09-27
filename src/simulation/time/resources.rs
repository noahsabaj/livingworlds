//! Time-related resources

use super::types::{GameTick, SimulationSpeed};
use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

/// Current game time and simulation speed with tick-based precision
///
/// This resource maintains the authoritative game time using integer ticks
/// for perfect precision and determinism. All fields are private to ensure
/// time can only be modified through controlled methods.
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct GameTime {
    // Core time state (private for safety)
    current_tick: GameTick,
    speed: SimulationSpeed,
    speed_before_pause: SimulationSpeed,

    // Frame accumulator for smooth tick generation
    #[serde(skip)]
    accumulated_time: f32,

    // Cached values for performance (updated only when tick changes)
    cached_year: u32,
    cached_day_of_year: u32,
    cached_total_days: u32,
}

impl GameTime {
    /// Get the current simulation tick
    pub fn current_tick(&self) -> GameTick {
        self.current_tick
    }

    /// Get the current year (using cached value)
    pub fn current_year(&self) -> u32 {
        self.cached_year
    }

    /// Get the current day of year (0-364)
    pub fn day_of_year(&self) -> u32 {
        self.cached_day_of_year
    }

    /// Get the total number of days elapsed
    pub fn current_day(&self) -> u32 {
        self.cached_total_days
    }

    /// Get the hour of the day (0.0-24.0) for visual effects
    pub fn hour_of_day(&self) -> f32 {
        // Get fractional part of current day from ticks
        let ticks_in_current_day = self.current_tick.0 % GameTick::TICKS_PER_DAY;
        let fraction_of_day = ticks_in_current_day as f32 / GameTick::TICKS_PER_DAY as f32;
        fraction_of_day * 24.0
    }

    /// Get the current simulation speed
    pub fn get_speed(&self) -> SimulationSpeed {
        self.speed
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.speed == SimulationSpeed::Paused
    }

    /// Set the simulation speed
    pub fn set_speed(&mut self, speed: SimulationSpeed) {
        self.speed = speed;
    }

    /// Pause the simulation
    pub fn pause(&mut self) {
        if self.speed != SimulationSpeed::Paused {
            self.speed_before_pause = self.speed;
            self.set_speed(SimulationSpeed::Paused);
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.speed == SimulationSpeed::Paused {
            self.set_speed(self.speed_before_pause);
        }
    }

    /// Toggle pause state
    pub fn toggle_pause(&mut self) {
        if self.is_paused() {
            self.resume();
        } else {
            self.pause();
        }
    }

    /// Advance the simulation by a number of ticks
    pub fn advance_ticks(&mut self, ticks: u64) {
        let old_day = self.cached_total_days;
        self.current_tick.add_ticks(ticks);
        self.update_cache();

        // Log if we crossed a day boundary
        if self.cached_total_days != old_day {
            trace!("Day {} (Year {})", self.cached_total_days, self.cached_year);
        }
    }

    /// Update cached values after tick change
    fn update_cache(&mut self) {
        self.cached_total_days = self.current_tick.to_days();
        self.cached_year = crate::constants::SIMULATION_STARTING_YEAR + self.current_tick.to_years();
        self.cached_day_of_year = self.current_tick.day_of_year();
    }
}

impl Default for GameTime {
    fn default() -> Self {
        let mut time = Self {
            current_tick: GameTick::default(),
            speed: SimulationSpeed::Normal,
            speed_before_pause: SimulationSpeed::Normal,
            accumulated_time: 0.0,
            cached_year: crate::constants::SIMULATION_STARTING_YEAR,
            cached_day_of_year: 0,
            cached_total_days: 0,
        };
        time.update_cache();
        time
    }
}
