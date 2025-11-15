//! Time display component for showing game date/time

use crate::ui::{ChildBuilder, LabelBuilder, LabelStyle};
use crate::simulation::{GameTime, CalendarRegistry};
use bevy::prelude::*;

/// Marker component for the game time display
#[derive(Component, Reflect)]
pub struct GameTimeDisplay;

/// Gregorian calendar month names
const MONTH_NAMES: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

/// Gregorian calendar days per month (non-leap year)
const DAYS_PER_MONTH: [u32; 12] = [
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31
];

/// Convert day of year (0-364) to month and day using proper Gregorian calendar
fn day_of_year_to_month_day(day_of_year: u32) -> (usize, u32) {
    let mut remaining_days = day_of_year;

    for (month_idx, &days_in_month) in DAYS_PER_MONTH.iter().enumerate() {
        if remaining_days < days_in_month {
            // Found the month - day_of_month is 1-indexed
            return (month_idx, remaining_days + 1);
        }
        remaining_days -= days_in_month;
    }

    // Fallback for day 365+ (shouldn't happen in 365-day year)
    (11, 31)
}

/// Spawn the time display UI element
pub fn spawn_time_display(parent: &mut ChildBuilder) {
    let entity = LabelBuilder::new("Year 1000, January 1")
        .style(LabelStyle::Heading)
        .font_size(24.0)
        .color(Color::WHITE)
        .build(parent);

    // Add our marker component
    parent.commands().entity(entity).insert(GameTimeDisplay);
}

/// Update the game time display with Year, Month, and Day
pub fn update_time_display(
    game_time: Res<GameTime>,
    calendar_registry: Option<Res<CalendarRegistry>>,
    time_display_query: Query<&Children, With<GameTimeDisplay>>,
    mut text_query: Query<&mut Text>,
) {
    // Find the time display entity and get its children
    if let Ok(children) = time_display_query.single() {
        // Look for the Text component in the children
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                let year = game_time.current_year();
                let day_of_year = game_time.day_of_year();

                // Try to use calendar system if available
                let display_text = if let Some(ref registry) = calendar_registry {
                    if let Some(calendar) = registry.default_calendar() {
                        // Use calendar system for proper display
                        let date = calendar.format_date(year, day_of_year);
                        calendar.display_date(&date)
                    } else {
                        // Fallback if no default calendar
                        fallback_gregorian_display(year, day_of_year)
                    }
                } else {
                    // Fallback if calendar system not loaded yet
                    fallback_gregorian_display(year, day_of_year)
                };

                **text = display_text;
                break; // Found and updated the text
            }
        }
    }
}

/// Fallback Gregorian display if calendar system isn't loaded
fn fallback_gregorian_display(year: u32, day_of_year: u32) -> String {
    let (month_index, day_of_month) = day_of_year_to_month_day(day_of_year);
    format!("Year {}, {} {}", year, MONTH_NAMES[month_index], day_of_month)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_calendar_conversion() {
        // January 1
        assert_eq!(day_of_year_to_month_day(0), (0, 1));

        // January 31
        assert_eq!(day_of_year_to_month_day(30), (0, 31));

        // February 1
        assert_eq!(day_of_year_to_month_day(31), (1, 1));

        // February 28 (last day, no leap year)
        assert_eq!(day_of_year_to_month_day(58), (1, 28));

        // March 1
        assert_eq!(day_of_year_to_month_day(59), (2, 1));

        // April 1
        assert_eq!(day_of_year_to_month_day(90), (3, 1));

        // December 31 (day 364, last day of year)
        assert_eq!(day_of_year_to_month_day(364), (11, 31));
    }

    #[test]
    fn test_no_february_31() {
        // Day 58 should be February 28, not 31
        let (month, day) = day_of_year_to_month_day(58);
        assert_eq!(month, 1); // February
        assert_eq!(day, 28); // Not 31!
        assert!(day <= DAYS_PER_MONTH[month]);
    }
}
