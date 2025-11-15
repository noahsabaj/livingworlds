//! Calendar system startup and initialization

use super::{load_all_calendars, CalendarRegistry};
use crate::simulation::GameTime;
use crate::world::WorldGenerationSettings;
use bevy::prelude::*;

/// System to load calendars and initialize GameTime on startup
/// This runs during app startup to load all available calendars
pub fn setup_calendar_system(mut commands: Commands) {
    // Load calendars from the calendars/ directory
    let calendars = load_all_calendars("calendars");

    // Create registry with Gregorian as default (will be updated when world gen starts)
    let registry = if !calendars.is_empty() {
        // Find Gregorian calendar or use first available
        let default_id = calendars
            .iter()
            .find(|cal| cal.id == "gregorian")
            .map(|cal| cal.id.clone())
            .unwrap_or_else(|| calendars[0].id.clone());

        CalendarRegistry::new(calendars, default_id)
    } else {
        warn!("No calendars loaded! UI will use fallback display");
        CalendarRegistry::default()
    };

    info!(
        "Calendar system initialized with {} calendar(s), default: {}",
        registry.calendars.len(),
        registry.default_calendar_id
    );

    commands.insert_resource(registry);
}

/// System to configure calendar and time based on world generation settings
/// This runs when entering InGame state after world generation
pub fn apply_world_time_settings(
    mut commands: Commands,
    settings: Option<Res<WorldGenerationSettings>>,
    mut calendar_registry: ResMut<CalendarRegistry>,
) {
    if let Some(settings) = settings {
        // Update calendar registry default to match world settings
        if calendar_registry.calendars.contains_key(&settings.calendar_id) {
            info!("Setting world calendar to: {}", settings.calendar_id);
            calendar_registry.default_calendar_id = settings.calendar_id.clone();
        } else {
            warn!(
                "Calendar '{}' not found, using default: {}",
                settings.calendar_id, calendar_registry.default_calendar_id
            );
        }

        // Initialize GameTime with the configured starting year
        let game_time = GameTime::new(settings.starting_year);
        info!("Initializing game time with starting year: {}", settings.starting_year);
        commands.insert_resource(game_time);
    } else {
        // Fallback if no settings available
        warn!("No WorldGenerationSettings found, using default GameTime");
        commands.insert_resource(GameTime::default());
    }
}
