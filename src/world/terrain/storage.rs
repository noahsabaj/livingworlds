//! Climate data storage for runtime overlay visualization
//!
//! This module provides persistent storage of climate data computed during
//! world generation, enabling efficient runtime visualization through the
//! overlay system without recalculation.

use bevy::prelude::*;
use crate::world::ProvinceId;
use std::collections::HashMap;

/// Simplified climate data for runtime visualization
#[derive(Debug, Clone, Copy, Reflect)]
pub struct ProvinceClimate {
    /// Average annual temperature in Celsius
    pub temperature: f32,
    /// Annual rainfall in mm/year
    pub rainfall: f32,
    /// Derived climate zone for quick classification
    pub zone: ClimateZone,
    /// Humidity factor (0.0 to 1.0)
    pub humidity: f32,
}

/// Climate zone classification for visualization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ClimateZone {
    Arctic,      // < -10°C
    Subarctic,   // -10°C to 0°C
    Temperate,   // 0°C to 20°C
    Subtropical, // 20°C to 25°C
    Tropical,    // 25°C to 30°C
    Desert,      // > 30°C or < 250mm rainfall
    Alpine,      // High elevation cold zones
}

impl ClimateZone {
    /// Classify based on temperature and rainfall
    pub fn from_climate_data(temperature: f32, rainfall: f32, elevation: f32) -> Self {
        // Alpine zones at high elevation
        if elevation > 0.7 && temperature < 0.0 {
            return ClimateZone::Alpine;
        }

        // Desert classification based on low rainfall
        if rainfall < 250.0 {
            return ClimateZone::Desert;
        }

        // Temperature-based classification
        match temperature {
            t if t < -10.0 => ClimateZone::Arctic,
            t if t < 0.0 => ClimateZone::Subarctic,
            t if t < 20.0 => ClimateZone::Temperate,
            t if t < 25.0 => ClimateZone::Subtropical,
            t if t < 30.0 => ClimateZone::Tropical,
            _ => ClimateZone::Desert, // Extreme heat becomes desert-like
        }
    }
}

/// Resource storing climate data for all provinces
#[derive(Resource, Default, Debug, Clone, Reflect)]
pub struct ClimateStorage {
    /// Climate data indexed by province ID
    pub climates: HashMap<ProvinceId, ProvinceClimate>,
    /// Global temperature range for normalization
    pub min_temperature: f32,
    pub max_temperature: f32,
    /// Global rainfall range for normalization
    pub min_rainfall: f32,
    pub max_rainfall: f32,
}

impl ClimateStorage {
    pub fn new() -> Self {
        Self {
            climates: HashMap::new(),
            min_temperature: f32::INFINITY,
            max_temperature: f32::NEG_INFINITY,
            min_rainfall: f32::INFINITY,
            max_rainfall: f32::NEG_INFINITY,
        }
    }

    /// Add climate data for a province
    pub fn insert(&mut self, id: ProvinceId, climate: ProvinceClimate) {
        // Update global ranges for normalization
        self.min_temperature = self.min_temperature.min(climate.temperature);
        self.max_temperature = self.max_temperature.max(climate.temperature);
        self.min_rainfall = self.min_rainfall.min(climate.rainfall);
        self.max_rainfall = self.max_rainfall.max(climate.rainfall);

        self.climates.insert(id, climate);
    }

    /// Get climate data for a province
    pub fn get(&self, id: ProvinceId) -> Option<&ProvinceClimate> {
        self.climates.get(&id)
    }

    /// Get normalized temperature (0.0 to 1.0) for color mapping
    pub fn normalized_temperature(&self, temperature: f32) -> f32 {
        if self.max_temperature <= self.min_temperature {
            return 0.5;
        }
        ((temperature - self.min_temperature) / (self.max_temperature - self.min_temperature))
            .clamp(0.0, 1.0)
    }

    /// Get normalized rainfall (0.0 to 1.0) for color mapping
    pub fn normalized_rainfall(&self, rainfall: f32) -> f32 {
        if self.max_rainfall <= self.min_rainfall {
            return 0.5;
        }
        ((rainfall - self.min_rainfall) / (self.max_rainfall - self.min_rainfall))
            .clamp(0.0, 1.0)
    }
}