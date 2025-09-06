//! Province templates - helper functions for creating specific geography types
//!
//! These functions demonstrate how to compose entities with appropriate components
//! to create different geographical regions like river valleys or mountain strongholds.

use bevy::prelude::*;
use lw_core::{Fixed32, Vec2fx};
use super::{
    province::*,
    terrain::*,
    climate::*,
    resources::*,
    water::*,
};

/// Create an Egypt-like river valley province bundle
pub fn create_river_valley_bundle(
    id: u32,
    position: Vec2fx,
) -> impl Bundle {
    (
        Province {
            id,
            position,
            coordinates: ProvinceCoordinates {
                x: position.x.integer_part(),
                y: position.y.integer_part(),
                region: GeographicRegion::Continental,
            },
            owner: None,
        },
        TerrainRiver {
            flow_rate: Fixed32::from_float(0.8),
            seasonal_variation: Fixed32::from_float(0.3),
            flood_predictability: Fixed32::from_float(0.9), // Very predictable like Nile
            navigation_capacity: Fixed32::from_float(0.9),
            fish_abundance: Fixed32::from_float(0.6),
        },
        Climate {
            temperature_range: TemperatureRange {
                winter_low: Fixed32::from_num(10),
                winter_high: Fixed32::from_num(20),
                summer_low: Fixed32::from_num(25),
                summer_high: Fixed32::from_num(40),
                daily_variation: Fixed32::from_num(15),
            },
            precipitation: Precipitation {
                annual_rainfall: Fixed32::from_float(0.05), // Very low, desert
                distribution: RainfallDistribution::Desert,
                reliability: Fixed32::from_float(0.1), // Unreliable local rain
            },
            seasonal_pattern: SeasonalPattern::Desert,
            extreme_weather_frequency: Fixed32::from_float(0.1),
            growing_season_length: Fixed32::from_float(0.9), // Year-round growing
            climate_stability: Fixed32::from_float(0.9),
        },
        AgriculturalPotential {
            soil_fertility: Fixed32::from_float(0.95), // Extremely fertile from silt
            irrigation_potential: Fixed32::from_float(1.0),
            crop_suitability: CropSuitability {
                grains: Fixed32::from_float(0.9),
                vegetables: Fixed32::from_float(0.8),
                fruits: Fixed32::from_float(0.6),
                cash_crops: Fixed32::from_float(0.7), // Cotton, etc.
            },
            livestock_capacity: Fixed32::from_float(0.4),
            growing_seasons_per_year: 3,
        },
        WaterAccess {
            access_type: WaterAccessType::Riverine,
            reliability: Fixed32::from_float(0.95),
            quality: Fixed32::from_float(0.8),
            quantity_available: Fixed32::from_float(1.0),
        },
    )
}

/// Create an Alpine mountain stronghold bundle
pub fn create_mountain_stronghold_bundle(
    id: u32,
    position: Vec2fx,
) -> impl Bundle {
    (
        Province {
            id,
            position,
            coordinates: ProvinceCoordinates {
                x: position.x.integer_part(),
                y: position.y.integer_part(),
                region: GeographicRegion::Continental,
            },
            owner: None,
        },
        TerrainMountain {
            elevation: Fixed32::from_num(2000),
            slope_gradient: Fixed32::from_float(0.7),
            passes: Vec::new(), // Would be populated with actual pass entities
            avalanche_risk: Fixed32::from_float(0.4),
            mineral_potential: Fixed32::from_float(0.7),
        },
        Climate {
            temperature_range: TemperatureRange {
                winter_low: Fixed32::from_num(-15),
                winter_high: Fixed32::from_num(0),
                summer_low: Fixed32::from_num(5),
                summer_high: Fixed32::from_num(20),
                daily_variation: Fixed32::from_num(10),
            },
            precipitation: Precipitation {
                annual_rainfall: Fixed32::from_float(0.8),
                distribution: RainfallDistribution::EvenlyDistributed,
                reliability: Fixed32::from_float(0.7),
            },
            seasonal_pattern: SeasonalPattern::Temperate,
            extreme_weather_frequency: Fixed32::from_float(0.3),
            growing_season_length: Fixed32::from_float(0.4), // Short growing season
            climate_stability: Fixed32::from_float(0.7),
        },
        AgriculturalPotential {
            soil_fertility: Fixed32::from_float(0.3), // Poor mountain soil
            irrigation_potential: Fixed32::from_float(0.2),
            crop_suitability: CropSuitability {
                grains: Fixed32::from_float(0.2),
                vegetables: Fixed32::from_float(0.3),
                fruits: Fixed32::from_float(0.1),
                cash_crops: Fixed32::from_float(0.0),
            },
            livestock_capacity: Fixed32::from_float(0.6), // Good for goats, sheep
            growing_seasons_per_year: 1,
        },
        WaterAccess {
            access_type: WaterAccessType::Spring,
            reliability: Fixed32::from_float(0.8),
            quality: Fixed32::from_float(0.95), // Pure mountain water
            quantity_available: Fixed32::from_float(0.6),
        },
    )
}

/// Create a coastal trading port bundle
pub fn create_coastal_port_bundle(
    id: u32,
    position: Vec2fx,
) -> impl Bundle {
    (
        Province {
            id,
            position,
            coordinates: ProvinceCoordinates {
                x: position.x.integer_part(),
                y: position.y.integer_part(),
                region: GeographicRegion::Coastal,
            },
            owner: None,
        },
        TerrainCoastal {
            shore_type: ShoreType::Sandy,
            tidal_range: Fixed32::from_float(0.3),
            storm_exposure: Fixed32::from_float(0.4),
            harbor_potential: Fixed32::from_float(0.8),
            fishing_grounds: Fixed32::from_float(0.7),
        },
        Climate {
            temperature_range: TemperatureRange {
                winter_low: Fixed32::from_num(5),
                winter_high: Fixed32::from_num(15),
                summer_low: Fixed32::from_num(15),
                summer_high: Fixed32::from_num(30),
                daily_variation: Fixed32::from_num(8),
            },
            precipitation: Precipitation {
                annual_rainfall: Fixed32::from_float(0.6),
                distribution: RainfallDistribution::Mediterranean,
                reliability: Fixed32::from_float(0.8),
            },
            seasonal_pattern: SeasonalPattern::Mediterranean,
            extreme_weather_frequency: Fixed32::from_float(0.2),
            growing_season_length: Fixed32::from_float(0.7),
            climate_stability: Fixed32::from_float(0.8),
        },
        MarineResources {
            fish_stocks: Fixed32::from_float(0.7),
            regeneration_rate: Fixed32::from_float(0.6),
            commercial_species: vec![
                FishSpecies {
                    name: "Cod".to_string(),
                    population: Fixed32::from_float(0.6),
                    commercial_value: Fixed32::from_float(0.8),
                },
                FishSpecies {
                    name: "Herring".to_string(),
                    population: Fixed32::from_float(0.8),
                    commercial_value: Fixed32::from_float(0.6),
                },
            ],
            overfishing_risk: Fixed32::from_float(0.3),
        },
        WaterAccess {
            access_type: WaterAccessType::Coastal,
            reliability: Fixed32::from_float(1.0),
            quality: Fixed32::from_float(0.0), // Salt water
            quantity_available: Fixed32::from_float(1.0),
        },
    )
}

/// Create a fertile plains bundle
pub fn create_fertile_plains_bundle(
    id: u32,
    position: Vec2fx,
) -> impl Bundle {
    (
        Province {
            id,
            position,
            coordinates: ProvinceCoordinates {
                x: position.x.integer_part(),
                y: position.y.integer_part(),
                region: GeographicRegion::Continental,
            },
            owner: None,
        },
        TerrainPlains {
            fertility: Fixed32::from_float(0.8),
            rainfall: Fixed32::from_float(0.7),
            flood_risk: Fixed32::from_float(0.2),
            wind_exposure: Fixed32::from_float(0.6),
        },
        Climate {
            temperature_range: TemperatureRange {
                winter_low: Fixed32::from_num(-5),
                winter_high: Fixed32::from_num(10),
                summer_low: Fixed32::from_num(15),
                summer_high: Fixed32::from_num(35),
                daily_variation: Fixed32::from_num(12),
            },
            precipitation: Precipitation {
                annual_rainfall: Fixed32::from_float(0.7),
                distribution: RainfallDistribution::Continental,
                reliability: Fixed32::from_float(0.7),
            },
            seasonal_pattern: SeasonalPattern::Temperate,
            extreme_weather_frequency: Fixed32::from_float(0.2),
            growing_season_length: Fixed32::from_float(0.6),
            climate_stability: Fixed32::from_float(0.75),
        },
        AgriculturalPotential {
            soil_fertility: Fixed32::from_float(0.8),
            irrigation_potential: Fixed32::from_float(0.6),
            crop_suitability: CropSuitability {
                grains: Fixed32::from_float(0.9), // Excellent for wheat, corn
                vegetables: Fixed32::from_float(0.7),
                fruits: Fixed32::from_float(0.5),
                cash_crops: Fixed32::from_float(0.6),
            },
            livestock_capacity: Fixed32::from_float(0.8), // Good grazing
            growing_seasons_per_year: 1,
        },
    )
}