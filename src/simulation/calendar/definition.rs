//! Calendar definition and validation

use super::types::{CalendarPeriod, DateDisplay, Season, WeekCycle};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum allowed days per year (sanity limit)
const MAX_DAYS_PER_YEAR: u32 = 1000;

/// Maximum allowed periods (months) per year
const MAX_PERIODS_PER_YEAR: usize = 100;

/// Complete calendar definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarDefinition {
    /// Unique identifier for this calendar (e.g., "gregorian", "french_revolutionary")
    pub id: String,

    /// Display name (e.g., "Gregorian Calendar", "French Revolutionary Calendar")
    pub name: String,

    /// Periods in the year (months, décades, etc.)
    pub periods: Vec<CalendarPeriod>,

    /// Optional week cycle
    pub week_cycle: Option<WeekCycle>,

    /// Optional season definitions
    pub seasons: Vec<Season>,

    /// Era/epoch name (e.g., "AD", "After Landing", "Year of Revolution")
    pub era_name: String,

    /// Display format template string
    /// Uses these placeholders:
    /// - `{year}`: Year number
    /// - `{era}`: Era name
    /// - `{period}`: Period name (month, décade, etc.)
    /// - `{day}`: Day number in period
    /// - `{weekday}`: Weekday name (if week cycle defined)
    pub display_format: String,
}

impl CalendarDefinition {
    /// Calculate total days in a year for this calendar
    pub fn days_per_year(&self) -> u32 {
        self.periods.iter().map(|p| p.days).sum()
    }

    /// Calculate total ticks in a year (days * 1000 ticks/day)
    pub fn ticks_per_year(&self) -> u64 {
        self.days_per_year() as u64 * 1000
    }

    /// Convert day of year (0-indexed) to period and day within period
    pub fn day_to_period_and_day(&self, day_of_year: u32) -> (usize, u32) {
        let mut remaining_days = day_of_year;

        for (idx, period) in self.periods.iter().enumerate() {
            if remaining_days < period.days {
                // Found the period - day is 1-indexed
                return (idx, remaining_days + 1);
            }
            remaining_days -= period.days;
        }

        // Fallback to last day of year
        let last_period_idx = self.periods.len().saturating_sub(1);
        let last_day = self.periods.get(last_period_idx)
            .map(|p| p.days)
            .unwrap_or(1);
        (last_period_idx, last_day)
    }

    /// Convert day of year to DateDisplay struct
    pub fn format_date(&self, year: u32, day_of_year: u32) -> DateDisplay {
        let (period_index, day_in_period) = self.day_to_period_and_day(day_of_year);

        let weekday_index = self.week_cycle.as_ref().map(|cycle| {
            (day_of_year % cycle.cycle_length) as usize
        });

        DateDisplay {
            year,
            period_index,
            day_in_period,
            day_of_year,
            weekday_index,
        }
    }

    /// Format a date as a string using the display template
    pub fn display_date(&self, date: &DateDisplay) -> String {
        let period_name = self.periods.get(date.period_index)
            .map(|p| p.name.as_str())
            .unwrap_or("Unknown");

        let weekday_name = date.weekday_index
            .and_then(|idx| {
                self.week_cycle.as_ref()
                    .and_then(|cycle| cycle.day_names.get(idx))
                    .map(|s| s.as_str())
            })
            .unwrap_or("");

        // Simple template replacement
        self.display_format
            .replace("{year}", &date.year.to_string())
            .replace("{era}", &self.era_name)
            .replace("{period}", period_name)
            .replace("{day}", &date.day_in_period.to_string())
            .replace("{weekday}", weekday_name)
            .trim()
            .to_string()
    }

    /// Validate calendar definition
    pub fn validate(&self) -> Result<(), CalendarValidationError> {
        // Require ID
        if self.id.is_empty() {
            return Err(CalendarValidationError::MissingId);
        }

        // Require name
        if self.name.is_empty() {
            return Err(CalendarValidationError::MissingName);
        }

        // Require at least one period
        if self.periods.is_empty() {
            return Err(CalendarValidationError::NoPeriods);
        }

        // Check period count limit
        if self.periods.len() > MAX_PERIODS_PER_YEAR {
            return Err(CalendarValidationError::TooManyPeriods(self.periods.len()));
        }

        // Validate each period
        for (idx, period) in self.periods.iter().enumerate() {
            if period.name.is_empty() {
                return Err(CalendarValidationError::PeriodMissingName(idx));
            }
            if period.days == 0 {
                return Err(CalendarValidationError::PeriodZeroDays(idx));
            }
        }

        // Check total days per year
        let total_days = self.days_per_year();
        if total_days == 0 {
            return Err(CalendarValidationError::ZeroDaysPerYear);
        }
        if total_days > MAX_DAYS_PER_YEAR {
            return Err(CalendarValidationError::TooManyDaysPerYear(total_days));
        }

        // Validate week cycle if present
        if let Some(ref cycle) = self.week_cycle {
            if cycle.day_names.is_empty() {
                return Err(CalendarValidationError::WeekCycleNoNames);
            }
            if cycle.cycle_length == 0 {
                return Err(CalendarValidationError::WeekCycleZeroLength);
            }
            if cycle.day_names.len() != cycle.cycle_length as usize {
                return Err(CalendarValidationError::WeekCycleMismatch {
                    names: cycle.day_names.len(),
                    length: cycle.cycle_length,
                });
            }
        }

        // Validate seasons
        for (idx, season) in self.seasons.iter().enumerate() {
            if season.name.is_empty() {
                return Err(CalendarValidationError::SeasonMissingName(idx));
            }
            if season.start_day >= total_days {
                return Err(CalendarValidationError::SeasonOutOfRange {
                    season_idx: idx,
                    day: season.start_day,
                    total_days,
                });
            }
            if season.end_day >= total_days {
                return Err(CalendarValidationError::SeasonOutOfRange {
                    season_idx: idx,
                    day: season.end_day,
                    total_days,
                });
            }
        }

        Ok(())
    }
}

/// Calendar validation errors
#[derive(Debug, Clone)]
pub enum CalendarValidationError {
    MissingId,
    MissingName,
    NoPeriods,
    TooManyPeriods(usize),
    PeriodMissingName(usize),
    PeriodZeroDays(usize),
    ZeroDaysPerYear,
    TooManyDaysPerYear(u32),
    WeekCycleNoNames,
    WeekCycleZeroLength,
    WeekCycleMismatch { names: usize, length: u32 },
    SeasonMissingName(usize),
    SeasonOutOfRange { season_idx: usize, day: u32, total_days: u32 },
}

impl fmt::Display for CalendarValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingId => write!(f, "Calendar missing required 'id' field"),
            Self::MissingName => write!(f, "Calendar missing required 'name' field"),
            Self::NoPeriods => write!(f, "Calendar must have at least one period (month/décade/etc.)"),
            Self::TooManyPeriods(count) => write!(f, "Calendar has too many periods: {} (max {})", count, MAX_PERIODS_PER_YEAR),
            Self::PeriodMissingName(idx) => write!(f, "Period {} missing name", idx),
            Self::PeriodZeroDays(idx) => write!(f, "Period {} has zero days", idx),
            Self::ZeroDaysPerYear => write!(f, "Calendar has zero total days per year"),
            Self::TooManyDaysPerYear(days) => write!(f, "Calendar has too many days per year: {} (max {})", days, MAX_DAYS_PER_YEAR),
            Self::WeekCycleNoNames => write!(f, "Week cycle has no day names"),
            Self::WeekCycleZeroLength => write!(f, "Week cycle has zero length"),
            Self::WeekCycleMismatch { names, length } => write!(f, "Week cycle mismatch: {} day names but cycle length is {}", names, length),
            Self::SeasonMissingName(idx) => write!(f, "Season {} missing name", idx),
            Self::SeasonOutOfRange { season_idx, day, total_days } => write!(f, "Season {} has day {} which exceeds total days ({})", season_idx, day, total_days),
        }
    }
}

impl std::error::Error for CalendarValidationError {}
