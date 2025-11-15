//! Calendar system resources

use super::CalendarDefinition;
use bevy::prelude::*;
use std::collections::HashMap;

/// Global calendar registry - holds all loaded calendars
#[derive(Resource, Default)]
pub struct CalendarRegistry {
    /// All loaded calendars indexed by ID
    pub calendars: HashMap<String, CalendarDefinition>,
    /// ID of the default calendar used in UI
    pub default_calendar_id: String,
}

impl CalendarRegistry {
    /// Create a new registry with loaded calendars
    pub fn new(calendars: Vec<CalendarDefinition>, default_id: String) -> Self {
        let calendar_map = calendars
            .into_iter()
            .map(|cal| (cal.id.clone(), cal))
            .collect();

        Self {
            calendars: calendar_map,
            default_calendar_id: default_id,
        }
    }

    /// Get the default calendar
    pub fn default_calendar(&self) -> Option<&CalendarDefinition> {
        self.calendars.get(&self.default_calendar_id)
    }

    /// Get a calendar by ID
    pub fn get_calendar(&self, id: &str) -> Option<&CalendarDefinition> {
        self.calendars.get(id)
    }

    /// List all available calendar IDs
    pub fn calendar_ids(&self) -> Vec<&str> {
        self.calendars.keys().map(|s| s.as_str()).collect()
    }
}

/// Per-nation calendar selection (for future multi-calendar support)
#[derive(Component, Clone)]
pub struct NationCalendar {
    /// Calendar ID this nation uses
    pub calendar_id: String,
}

impl NationCalendar {
    pub fn new(calendar_id: impl Into<String>) -> Self {
        Self {
            calendar_id: calendar_id.into(),
        }
    }
}
