//! Climate components - weather patterns and seasonal variations
//!
//! Climate changes slowly over centuries and affects all human activity.

use bevy::prelude::*;
use lw_core::Fixed32;
use serde::{Deserialize, Serialize};
use lw_core::shared_types::GameTime;

/// Climate component - long-term weather patterns
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub temperature_range: TemperatureRange,
    pub precipitation: Precipitation,
    pub seasonal_pattern: SeasonalPattern,
    pub extreme_weather_frequency: Fixed32,
    pub growing_season_length: Fixed32,
    pub climate_stability: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureRange {
    pub winter_low: Fixed32,
    pub winter_high: Fixed32,
    pub summer_low: Fixed32,
    pub summer_high: Fixed32,
    pub daily_variation: Fixed32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precipitation {
    pub annual_rainfall: Fixed32,
    pub distribution: RainfallDistribution,
    pub reliability: Fixed32,  // How predictable
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RainfallDistribution {
    EvenlyDistributed,
    WetDrySeason,
    Monsoon,
    Mediterranean,  // Wet winter, dry summer
    Continental,    // Summer rainfall
    Desert,        // Minimal, irregular
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeasonalPattern {
    Tropical,      // Minimal seasonal variation
    Temperate,     // Four distinct seasons
    Mediterranean, // Two main seasons
    Monsoon,       // Dramatic wet/dry
    Arctic,        // Extreme seasonal variation
    Desert,        // Hot days, cold nights
}

/// Climate change component - tracks long-term trends
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ClimateChange {
    pub temperature_trend: Fixed32,    // Warming/cooling per century
    pub precipitation_trend: Fixed32,  // Wetting/drying trend
    pub last_update: GameTime,
    pub rate_of_change: Fixed32,
}

/// Weather event component - temporary weather conditions
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEvent {
    pub event_type: WeatherEventType,
    pub intensity: Fixed32,
    pub duration_remaining: u32,
    pub affected_radius: Fixed32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherEventType {
    Drought,
    Flood,
    Blizzard,
    Heatwave,
    Storm,
    Hurricane,
    Tornado,
}

/// Microclimate component - local climate variations
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Microclimate {
    pub altitude_effect: Fixed32,      // Mountains are colder
    pub water_body_effect: Fixed32,    // Lakes/oceans moderate temperature
    pub urban_heat_island: Fixed32,    // Cities are warmer
    pub forest_cooling: Fixed32,       // Trees provide cooling
}

/// Geological stability of a region
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GeologicalStability {
    pub tectonic_activity: Fixed32,    // 0-1, earthquake risk
    pub volcanic_activity: Fixed32,    // 0-1, eruption risk
    pub soil_stability: Fixed32,       // 0-1, landslide risk
    pub flood_risk: Fixed32,           // 0-1, based on topology
    pub coastal_erosion: Fixed32,      // 0-1, for coastal areas
}

/// Natural disaster event
#[derive(Event, Debug, Clone)]
pub struct NaturalDisaster {
    pub disaster_type: DisasterType,
    pub epicenter: Entity,              // Province at center
    pub affected_provinces: Vec<Entity>,
    pub intensity: Fixed32,             // 0-1, severity
    pub duration: u32,                  // How long it lasts
    pub casualties: u32,
    pub infrastructure_damage: Fixed32, // 0-1, destruction level
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisasterType {
    Earthquake,
    Volcano,
    Tsunami,
    Hurricane,
    Tornado,
    Flood,
    Drought,
    Wildfire,
    Blizzard,
    Landslide,
    Plague,     // Disease outbreak
    Famine,     // Food shortage
}