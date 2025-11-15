//! Calendar loading from RON files

use super::CalendarDefinition;
use bevy::log::{error, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Load a single calendar from a RON file
pub fn load_calendar(path: impl AsRef<Path>) -> Result<CalendarDefinition, CalendarLoadError> {
    let path = path.as_ref();

    // Read file
    let contents = fs::read_to_string(path)
        .map_err(|e| CalendarLoadError::IoError {
            path: path.to_path_buf(),
            error: e.to_string(),
        })?;

    // Parse RON
    let calendar: CalendarDefinition = ron::from_str(&contents)
        .map_err(|e| CalendarLoadError::ParseError {
            path: path.to_path_buf(),
            error: e.to_string(),
        })?;

    // Validate
    calendar.validate()
        .map_err(|e| CalendarLoadError::ValidationError {
            path: path.to_path_buf(),
            calendar_name: calendar.name.clone(),
            error: e,
        })?;

    info!("Loaded calendar: {} ({} days/year)", calendar.name, calendar.days_per_year());
    Ok(calendar)
}

/// Load all calendars from a directory
pub fn load_all_calendars(dir: impl AsRef<Path>) -> Vec<CalendarDefinition> {
    let dir = dir.as_ref();

    if !dir.exists() {
        warn!("Calendar directory does not exist: {:?}", dir);
        return Vec::new();
    }

    let mut calendars = Vec::new();

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();

                // Only process .ron files
                if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                    match load_calendar(&path) {
                        Ok(calendar) => calendars.push(calendar),
                        Err(e) => error!("Failed to load calendar from {:?}: {}", path, e),
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to read calendar directory {:?}: {}", dir, e);
        }
    }

    info!("Loaded {} calendar(s) from {:?}", calendars.len(), dir);
    calendars
}

/// Calendar loading errors
#[derive(Debug)]
pub enum CalendarLoadError {
    IoError {
        path: PathBuf,
        error: String,
    },
    ParseError {
        path: PathBuf,
        error: String,
    },
    ValidationError {
        path: PathBuf,
        calendar_name: String,
        error: super::definition::CalendarValidationError,
    },
}

impl std::fmt::Display for CalendarLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError { path, error } => {
                write!(f, "Failed to read calendar file {:?}: {}", path, error)
            }
            Self::ParseError { path, error } => {
                write!(f, "Failed to parse calendar file {:?}: {}", path, error)
            }
            Self::ValidationError { path, calendar_name, error } => {
                write!(f, "Calendar '{}' from {:?} failed validation: {}", calendar_name, path, error)
            }
        }
    }
}

impl std::error::Error for CalendarLoadError {}
