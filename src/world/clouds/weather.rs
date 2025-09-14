//! Weather system types and state management
//!
//! This module contains weather states and the dynamic weather system
//! that controls atmospheric conditions and cloud coverage.

use bevy::prelude::*;
use bevy::math::Vec2;

/// Weather states representing different atmospheric conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherState {
    Clear,      // 0-10% cloud coverage - bright sunny day
    Fair,       // 10-30% cloud coverage - pleasant with some clouds
    Partly,     // 30-60% cloud coverage - mix of sun and clouds
    Cloudy,     // 60-80% cloud coverage - mostly cloudy
    Overcast,   // 80-100% cloud coverage - completely grey sky
    Storm,      // 90-100% coverage + dark clouds and rain
}

impl WeatherState {
    pub fn coverage_range(&self) -> (f32, f32) {
        match self {
            WeatherState::Clear => (0.0, 0.1),
            WeatherState::Fair => (0.1, 0.3),
            WeatherState::Partly => (0.3, 0.6),
            WeatherState::Cloudy => (0.6, 0.8),
            WeatherState::Overcast => (0.8, 1.0),
            WeatherState::Storm => (0.9, 1.0),
        }
    }

    /// Get a descriptive name for the weather
    pub fn description(&self) -> &str {
        match self {
            WeatherState::Clear => "Clear skies",
            WeatherState::Fair => "Fair weather",
            WeatherState::Partly => "Partly cloudy",
            WeatherState::Cloudy => "Cloudy",
            WeatherState::Overcast => "Overcast",
            WeatherState::Storm => "Stormy",
        }
    }
}

/// Dynamic weather system controlling cloud coverage and atmospheric conditions
#[derive(Resource)]
pub struct WeatherSystem {
    /// Current weather state
    pub current_state: WeatherState,
    /// Target weather state we're transitioning to
    pub target_state: WeatherState,
    /// Progress of transition (0.0 = start, 1.0 = complete)
    pub transition_progress: f32,
    /// Current cloud coverage (0.0 = clear, 1.0 = overcast)
    pub cloud_coverage: f32,
    /// Wind speed and direction
    pub wind_speed: Vec2,
    /// Time since last weather change in seconds
    pub time_since_change: f32,
    /// Minimum time before next weather change
    pub min_weather_duration: f32,
    /// Random weather change chance per second
    pub weather_change_chance: f32,
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self {
            current_state: WeatherState::Partly,  // More clouds initially
            target_state: WeatherState::Partly,
            transition_progress: 1.0,
            cloud_coverage: 0.5,  // Start with 50% coverage instead of 20%
            wind_speed: Vec2::new(5.0, 1.0),
            time_since_change: 0.0,
            min_weather_duration: 60.0,  // At least 1 minute per weather
            weather_change_chance: 0.01, // 1% chance per second after min duration
        }
    }
}