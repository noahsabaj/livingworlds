//! Core calendar types

use serde::{Deserialize, Serialize};

/// A period in the calendar (month, décade, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarPeriod {
    /// Name of this period (e.g., "January", "Vendémiaire")
    pub name: String,
    /// Number of days in this period
    pub days: u32,
}

/// Week cycle definition for weekday names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekCycle {
    /// Names of weekdays (e.g., ["Monday", "Tuesday", ...])
    pub day_names: Vec<String>,
    /// Length of week cycle
    pub cycle_length: u32,
}

/// Season definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    /// Season name (e.g., "Spring", "Summer")
    pub name: String,
    /// Day of year when this season starts (0-indexed)
    pub start_day: u32,
    /// Day of year when this season ends (0-indexed, inclusive)
    pub end_day: u32,
}

/// Represents a displayed date in this calendar
#[derive(Debug, Clone)]
pub struct DateDisplay {
    /// Year number
    pub year: u32,
    /// Period index (month/décade/etc.)
    pub period_index: usize,
    /// Day within period (1-indexed)
    pub day_in_period: u32,
    /// Day of year (0-indexed)
    pub day_of_year: u32,
    /// Optional weekday index
    pub weekday_index: Option<usize>,
}
