//! Calendar system for Living Worlds
//!
//! This module provides a moddable calendar system that allows different ways to
//! represent and display time while maintaining constant tick-based simulation.
//!
//! # Design Philosophy
//!
//! - **Constant Tick Rate**: 1 tick = constant real-time duration (1ms game time)
//! - **1000 ticks = 1 game day** (always, regardless of calendar)
//! - Calendars are different ways to group and display these ticks
//! - A 400-day calendar year = 400,000 ticks (longer real-time than 365-day year)
//!
//! # Calendar Components
//!
//! - **Periods** (months, décades, etc.): Named divisions of the year
//! - **Week Cycles**: Optional weekday names and lengths
//! - **Seasons**: Seasonal divisions for gameplay/display
//! - **Era/Epoch**: Name for the time period ("AD", "Year of Revolution", etc.)

mod definition;
mod loader;
mod resources;
mod startup;
mod types;

pub use definition::CalendarDefinition;
pub use loader::{load_calendar, load_all_calendars, CalendarLoadError};
pub use resources::{CalendarRegistry, NationCalendar};
pub use startup::{setup_calendar_system, apply_world_time_settings};
pub use types::{CalendarPeriod, WeekCycle, Season, DateDisplay};
