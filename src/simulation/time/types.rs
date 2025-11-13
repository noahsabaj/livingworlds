//! Type-safe time representation for deterministic simulation
//!
//! This module provides tick-based time types that eliminate float precision issues
//! and enable deterministic simulation regardless of framerate.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A single simulation tick representing 0.001 game days
///
/// This provides millisecond-day precision while using integer math,
/// eliminating float precision degradation over long simulations.
/// At maximum speed (9x), the u64 can run for 20 million years without overflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Reflect)]
pub struct GameTick(pub u64);

impl GameTick {
    /// Number of ticks in one game day
    pub const TICKS_PER_DAY: u64 = 1000;

    /// Number of ticks in one game year (365 days)
    pub const TICKS_PER_YEAR: u64 = 365_000;

    /// Create a new GameTick from a day value
    pub fn from_days(days: u32) -> Self {
        GameTick(days as u64 * Self::TICKS_PER_DAY)
    }

    /// Create a new GameTick from a year value
    pub fn from_years(years: u32) -> Self {
        GameTick(years as u64 * Self::TICKS_PER_YEAR)
    }

    /// Get the total number of complete days
    pub fn to_days(&self) -> u32 {
        (self.0 / Self::TICKS_PER_DAY) as u32
    }

    /// Get the total number of complete years
    pub fn to_years(&self) -> u32 {
        (self.0 / Self::TICKS_PER_YEAR) as u32
    }

    /// Get the day within the current year (0-364)
    pub fn day_of_year(&self) -> u32 {
        self.to_days() % 365
    }

    /// Get the fractional day for smooth visual interpolation
    pub fn fractional_days(&self) -> f32 {
        self.0 as f32 / Self::TICKS_PER_DAY as f32
    }

    /// Add a duration in ticks
    pub fn add_ticks(&mut self, ticks: u64) {
        self.0 = self.0.saturating_add(ticks);
    }

    /// Add a duration in days
    pub fn add_days(&mut self, days: u32) {
        self.add_ticks(days as u64 * Self::TICKS_PER_DAY);
    }

    /// Check if a certain number of days have passed since another tick
    pub fn days_since(&self, other: GameTick) -> Option<u32> {
        if self.0 >= other.0 {
            Some(((self.0 - other.0) / Self::TICKS_PER_DAY) as u32)
        } else {
            None
        }
    }
}

impl Default for GameTick {
    fn default() -> Self {
        GameTick(0)
    }
}

/// Type-safe simulation speed settings
///
/// Using an enum instead of raw floats ensures only valid speeds are possible
/// and makes the code self-documenting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum SimulationSpeed {
    /// Simulation is paused (0 ticks per second)
    Paused,
    /// Normal speed - 1 game day per real second (1000 ticks/sec)
    Normal,
    /// 3x speed - 3 game days per real second (3000 ticks/sec)
    Fast,
    /// 6x speed - 6 game days per real second (6000 ticks/sec)
    Faster,
    /// 9x speed - 9 game days per real second (9000 ticks/sec)
    Fastest,
}

impl SimulationSpeed {
    /// Get the number of simulation ticks per real-world second
    pub fn ticks_per_second(&self) -> u16 {
        match self {
            SimulationSpeed::Paused => 0,
            SimulationSpeed::Normal => 1000,
            SimulationSpeed::Fast => 3000,
            SimulationSpeed::Faster => 6000,
            SimulationSpeed::Fastest => 9000,
        }
    }

    /// Get the multiplier for backwards compatibility
    pub fn multiplier(&self) -> f32 {
        match self {
            SimulationSpeed::Paused => 0.0,
            SimulationSpeed::Normal => 1.0,
            SimulationSpeed::Fast => 3.0,
            SimulationSpeed::Faster => 6.0,
            SimulationSpeed::Fastest => 9.0,
        }
    }

    /// Get the next faster speed level
    pub fn faster(&self) -> Self {
        match self {
            SimulationSpeed::Paused => SimulationSpeed::Normal,
            SimulationSpeed::Normal => SimulationSpeed::Fast,
            SimulationSpeed::Fast => SimulationSpeed::Faster,
            SimulationSpeed::Faster => SimulationSpeed::Fastest,
            SimulationSpeed::Fastest => SimulationSpeed::Fastest,
        }
    }

    /// Get the next slower speed level
    pub fn slower(&self) -> Self {
        match self {
            SimulationSpeed::Paused => SimulationSpeed::Paused,
            SimulationSpeed::Normal => SimulationSpeed::Paused,
            SimulationSpeed::Fast => SimulationSpeed::Normal,
            SimulationSpeed::Faster => SimulationSpeed::Fast,
            SimulationSpeed::Fastest => SimulationSpeed::Faster,
        }
    }

    /// Parse from a number key (1-5)
    pub fn from_number_key(key: u8) -> Option<Self> {
        match key {
            1 => Some(SimulationSpeed::Paused),
            2 => Some(SimulationSpeed::Normal),
            3 => Some(SimulationSpeed::Fast),
            4 => Some(SimulationSpeed::Faster),
            5 => Some(SimulationSpeed::Fastest),
            _ => None,
        }
    }

    /// Get a human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SimulationSpeed::Paused => "Paused",
            SimulationSpeed::Normal => "Normal",
            SimulationSpeed::Fast => "Fast",
            SimulationSpeed::Faster => "Faster",
            SimulationSpeed::Fastest => "Fastest",
        }
    }
}

impl Default for SimulationSpeed {
    fn default() -> Self {
        SimulationSpeed::Normal
    }
}

/// Visual interpolation data for smooth rendering between ticks
///
/// This separates simulation time (discrete ticks) from visual time (smooth interpolation)
/// allowing the simulation to remain deterministic while visuals remain smooth.
#[derive(Resource, Default, Reflect)]
pub struct VisualTime {
    /// Smoothly interpolated day value for display
    pub interpolated_days: f32,
    /// Smoothly interpolated year value for display
    pub interpolated_years: f32,
}