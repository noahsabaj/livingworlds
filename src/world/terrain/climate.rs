//! Climate simulation for realistic biome distribution
//!
//! This module simulates moisture propagation, temperature gradients,
//! rain shadows, and other climate phenomena to create realistic
//! biome distributions based on physical principles.

use bevy::prelude::Vec2;
use std::collections::{HashMap, VecDeque};
use rayon::prelude::*;
use crate::world::{Province, Elevation};
use crate::world::TerrainType;
use crate::math::{exponential_smooth, euclidean_vec2, PI, sin_cos};

// CONSTANTS - Climate parameters based on Earth

/// Base temperature at equator (Celsius)
const EQUATOR_TEMP: f32 = 30.0;

/// Base temperature at poles (Celsius)
const POLE_TEMP: f32 = -30.0;

/// Temperature lapse rate (degrees C per meter of elevation)
const LAPSE_RATE: f32 = 0.0065;

/// Ocean moderation effect on temperature (max degrees)
const OCEAN_TEMP_MODERATION: f32 = 10.0;

/// Base annual rainfall at ocean (mm/year)
const OCEAN_RAINFALL: f32 = 1500.0;

/// Rainfall decay rate inland (per km)
const RAINFALL_DECAY_INLAND: f32 = 0.995;

/// Rain shadow effect strength
const RAIN_SHADOW_FACTOR: f32 = 0.3;

/// Minimum rainfall (mm/year) for desert threshold
const DESERT_RAINFALL: f32 = 250.0;

/// Maximum distance for ocean influence (km)
const OCEAN_INFLUENCE_DISTANCE: f32 = 500.0;

/// Elevation threshold for alpine climate (meters)
const ALPINE_ELEVATION: f32 = 3000.0;

/// Treeline elevation (meters)
const TREELINE_ELEVATION: f32 = 2500.0;

/// Prevailing wind directions by latitude
const TRADE_WIND_ZONE: f32 = 0.3;     // 0-30° latitude
const WESTERLIES_ZONE: f32 = 0.6;     // 30-60° latitude
const POLAR_EASTERLIES_ZONE: f32 = 1.0; // 60-90° latitude


/// Climate data for a province
#[derive(Debug, Clone)]
pub struct Climate {
    /// Average annual temperature (Celsius)
    pub temperature: f32,
    /// Annual rainfall (mm/year)
    pub rainfall: f32,
    /// Distance to nearest ocean (km)
    pub ocean_distance: f32,
    /// Humidity (0.0 to 1.0)
    pub humidity: f32,
    /// Continentality (0.0 = maritime, 1.0 = continental)
    pub continentality: f32,
    /// Prevailing wind direction (radians)
    pub wind_direction: f32,
    /// Wind strength (0.0 to 1.0)
    pub wind_strength: f32,
}

impl Default for Climate {
    fn default() -> Self {
        Self {
            temperature: 15.0,
            rainfall: 500.0,
            ocean_distance: f32::INFINITY,
            humidity: 0.5,
            continentality: 0.5,
            wind_direction: 0.0,
            wind_strength: 0.5,
        }
    }
}

/// Biome type based on temperature and moisture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    // Polar biomes
    PolarDesert,
    Tundra,

    // Cold biomes
    Taiga,
    BorealForest,

    // Temperate biomes
    TemperateRainforest,
    TemperateDeciduousForest,
    TemperateGrassland,
    ColdDesert,

    // Subtropical biomes
    MediterraneanForest,
    Chaparral,
    SubtropicalDesert,

    // Tropical biomes
    TropicalRainforest,
    TropicalSeasonalForest,
    Savanna,
    TropicalDesert,

    // Special biomes
    Alpine,
    Wetlands,
    Mangrove,
}


/// Main climate simulation system
pub struct ClimateSystem {
    /// Climate data for each province
    pub climates: HashMap<u32, Climate>,
    /// World dimensions
    dimensions: crate::resources::MapDimensions,
}

impl ClimateSystem {
    pub fn new(dimensions: crate::resources::MapDimensions) -> Self {
        Self {
            climates: HashMap::new(),
            dimensions,
        }
    }

    /// Run full climate simulation
    pub fn simulate(&mut self, provinces: &[Province]) {
        println!("  Starting climate simulation...");

        // Step 1: Calculate ocean distances
        self.calculate_ocean_distances(provinces);

        // Step 2: Calculate base temperatures
        self.calculate_temperatures(provinces);

        // Step 3: Calculate prevailing winds
        self.calculate_winds(provinces);

        // Step 4: Simulate moisture propagation
        self.propagate_moisture(provinces);

        // Step 5: Apply rain shadows
        self.apply_rain_shadows(provinces);

        // Step 6: Calculate final humidity
        self.calculate_humidity(provinces);

        println!("  Climate simulation complete");
    }

    /// Calculate distance to ocean for each province
    fn calculate_ocean_distances(&mut self, provinces: &[Province]) {
        println!("    Calculating ocean distances...");

        // BFS from ocean provinces
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();

        let province_lookup: HashMap<u32, usize> = provinces.iter()
            .enumerate()
            .map(|(idx, p)| (p.id.value(), idx))
            .collect();

        // Initialize with ocean provinces
        let mut ocean_count = 0;
        for province in provinces {
            if province.terrain == TerrainType::Ocean {
                queue.push_back((province.id.value(), 0.0));
                distances.insert(province.id.value(), 0.0);
                ocean_count += 1;
            }
        }
        println!("      Found {} ocean provinces", ocean_count);

        // BFS to find ocean distances using pre-computed neighbors
        let mut processed = 0;
        while let Some((province_id, distance)) = queue.pop_front() {
            processed += 1;

            // Report progress periodically
            if processed % 10000 == 0 {
                print!("\r      Processing province {} of approximately {}...",
                    processed, provinces.len());
            }

            if let Some(&province_idx) = province_lookup.get(&province_id) {
                let province = &provinces[province_idx];

                for neighbor_opt in &province.neighbors {
                    if let Some(neighbor_id) = neighbor_opt {
                        let neighbor_id_val = neighbor_id.value();

                        // If we haven't visited this neighbor yet
                        if !distances.contains_key(&neighbor_id_val) {
                            let new_distance = distance + self.dimensions.hex_size / 1000.0; // Convert to km
                            distances.insert(neighbor_id_val, new_distance);

                            // Only continue BFS if within influence distance
                            if new_distance < OCEAN_INFLUENCE_DISTANCE {
                                queue.push_back((neighbor_id_val, new_distance));
                            }
                        }
                    }
                }
            }
        }

        println!("\r      Processed {} provinces for ocean distances", processed);

        // Store distances in climate data
        for province in provinces {
            let climate = self.climates.entry(province.id.value())
                .or_insert_with(Climate::default);
            climate.ocean_distance = *distances.get(&province.id.value())
                .unwrap_or(&f32::INFINITY);
        }
    }

    /// Calculate temperature based on latitude and elevation (PARALLELIZED)
    fn calculate_temperatures(&mut self, provinces: &[Province]) {
        println!("    Calculating temperatures...");

        let temps: Vec<(u32, f32)> = provinces
            .par_iter()
            .map(|province| {
                let ocean_distance = self.climates.get(&province.id.value())
                    .map(|c| c.ocean_distance)
                    .unwrap_or(f32::INFINITY);

                let latitude = (province.position.y - self.dimensions.bounds.y_min) /
                              (self.dimensions.bounds.y_max - self.dimensions.bounds.y_min);

                let lat_from_equator = (latitude - 0.5).abs() * 2.0;
                let base_temp = EQUATOR_TEMP * (1.0 - lat_from_equator) +
                               POLE_TEMP * lat_from_equator;

                // Apply elevation cooling
                let elevation_m = province.elevation.value() * 5000.0;
                let elevation_cooling = elevation_m * LAPSE_RATE;

                // Apply ocean moderation
                let ocean_moderation = if ocean_distance < OCEAN_INFLUENCE_DISTANCE {
                    let factor = 1.0 - (ocean_distance / OCEAN_INFLUENCE_DISTANCE);
                    OCEAN_TEMP_MODERATION * factor
                } else {
                    0.0
                };

                let temperature = base_temp - elevation_cooling + ocean_moderation;
                (province.id.value(), temperature)
            })
            .collect();

        for (id, temp) in temps {
            self.climates.entry(id)
                .or_insert_with(Climate::default)
                .temperature = temp;
        }
    }

    /// Calculate prevailing wind patterns
    fn calculate_winds(&mut self, provinces: &[Province]) {
        println!("    Calculating wind patterns...");

        for province in provinces {
            let climate = self.climates.entry(province.id.value())
                .or_insert_with(Climate::default);

            let latitude = (province.position.y - self.dimensions.bounds.y_min) /
                          (self.dimensions.bounds.y_max - self.dimensions.bounds.y_min);

            // Determine wind zone and direction
            let lat_from_equator = (latitude - 0.5).abs();

            if lat_from_equator < TRADE_WIND_ZONE {
                // Trade winds - blow from east to west
                climate.wind_direction = PI; // West
                climate.wind_strength = 0.8;
            } else if lat_from_equator < WESTERLIES_ZONE {
                // Westerlies - blow from west to east
                climate.wind_direction = 0.0; // East
                climate.wind_strength = 1.0;
            } else {
                // Polar easterlies - blow from east to west
                climate.wind_direction = PI; // West
                climate.wind_strength = 0.6;
            }

            // Reduce wind strength over land
            if province.terrain != TerrainType::Ocean {
                climate.wind_strength *= 0.7;
            }
        }
    }

    /// Propagate moisture from oceans inland
    fn propagate_moisture(&mut self, provinces: &[Province]) {
        println!("    Propagating moisture from oceans...");

        let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        let grid_size = self.dimensions.hex_size * 5.0;

        for (idx, province) in provinces.iter().enumerate() {
            let grid_x = (province.position.x / grid_size).floor() as i32;
            let grid_y = (province.position.y / grid_size).floor() as i32;
            grid.entry((grid_x, grid_y))
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Reduced iterations and parallelized processing
        const MOISTURE_ITERATIONS: usize = 3; // Reduced from 10

        for iteration in 0..MOISTURE_ITERATIONS {
            println!("      Moisture propagation iteration {}/{}", iteration + 1, MOISTURE_ITERATIONS);

            // Clone current climate data for parallel read access
            let current_climates = self.climates.clone();

            let batch_size = 10000;
            let num_batches = (provinces.len() + batch_size - 1) / batch_size;

            for batch_idx in 0..num_batches {
                let start_idx = batch_idx * batch_size;
                let end_idx = (start_idx + batch_size).min(provinces.len());

                let batch_rainfall: Vec<(u32, f32)> = (start_idx..end_idx)
                    .into_par_iter()
                    .map(|idx| {
                        let province = &provinces[idx];
                        let climate = current_climates.get(&province.id.value())
                            .cloned()
                            .unwrap_or_default();

                        // Start with base rainfall
                        let mut rainfall = if province.terrain == TerrainType::Ocean {
                            OCEAN_RAINFALL
                        } else {
                            // Decay based on distance from ocean
                            let decay = RAINFALL_DECAY_INLAND.powf(climate.ocean_distance.min(500.0));
                            OCEAN_RAINFALL * decay
                        };

                        // Only check immediate neighbors using the pre-computed neighbor array
                        for neighbor_opt in &province.neighbors {
                            if let Some(neighbor_id) = neighbor_opt {
                                if let Some(neighbor_climate) = current_climates.get(&neighbor_id.value()) {
                                    let transfer = neighbor_climate.rainfall * 0.05 * climate.wind_strength;
                                    rainfall += transfer;
                                }
                            }
                        }

                        // Cap rainfall at reasonable maximum
                        rainfall = rainfall.min(3000.0);

                        (province.id.value(), rainfall)
                    })
                    .collect();

                for (id, rainfall) in batch_rainfall {
                    if let Some(climate) = self.climates.get_mut(&id) {
                        // Blend with existing value for smoother propagation
                        climate.rainfall = exponential_smooth(climate.rainfall, rainfall, 0.3);
                    }
                }

                // Progress report
                if batch_idx % 10 == 0 {
                    print!("\r        Processing batch {}/{}", batch_idx + 1, num_batches);
                }
            }
            println!(); // New line after progress
        }

        println!("      Moisture propagation complete");
    }

    /// Apply rain shadow effects from mountains (PARALLELIZED)
    fn apply_rain_shadows(&mut self, provinces: &[Province]) {
        println!("    Applying rain shadow effects...");

        let mut spatial_grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        let grid_size = self.dimensions.hex_size * 2.0;

        for (idx, province) in provinces.iter().enumerate() {
            let grid_x = (province.position.x / grid_size).floor() as i32;
            let grid_y = (province.position.y / grid_size).floor() as i32;
            spatial_grid.entry((grid_x, grid_y))
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Clone for parallel access
        let current_climates = self.climates.clone();
        let hex_size = self.dimensions.hex_size;

        let rain_shadows: Vec<u32> = provinces
            .par_iter()
            .filter_map(|province| {
                let climate = current_climates.get(&province.id.value())?;

                let (cos_wind, sin_wind) = sin_cos(climate.wind_direction);
                let wind_source = Vec2::new(-cos_wind, -sin_wind);

                // Sample fewer points for performance
                for i in 1..=5 {
                    let check_pos = province.position + wind_source * (i as f32 * 40.0);
                    let grid_x = (check_pos.x / grid_size).floor() as i32;
                    let grid_y = (check_pos.y / grid_size).floor() as i32;

                    if let Some(indices) = spatial_grid.get(&(grid_x, grid_y)) {
                        for &idx in indices {
                            let other = &provinces[idx];
                            if euclidean_vec2(other.position, check_pos) < hex_size * 1.5 {
                                if other.elevation.value() > 0.6 {
                                    return Some(province.id.value());
                                }
                            }
                        }
                    }
                }
                None
            })
            .collect();

        // Apply rain shadow reduction
        let shadow_count = rain_shadows.len();
        for id in rain_shadows {
            if let Some(climate) = self.climates.get_mut(&id) {
                climate.rainfall *= RAIN_SHADOW_FACTOR;
            }
        }

        println!("      Applied rain shadow to {} provinces", shadow_count);
    }

    /// Calculate humidity from rainfall and temperature (PARALLELIZED)
    fn calculate_humidity(&mut self, provinces: &[Province]) {
        println!("    Calculating humidity levels...");

        let humidity_values: Vec<(u32, f32, f32)> = provinces
            .par_iter()
            .filter_map(|province| {
                let climate = self.climates.get(&province.id.value())?;
                // Humidity based on rainfall and temperature
                let base_humidity = (climate.rainfall / 2000.0).min(1.0);

                // Temperature affects humidity capacity
                let temp_factor = ((climate.temperature + 40.0) / 70.0).clamp(0.0, 1.0);

                // Ocean proximity increases humidity
                let ocean_factor = if climate.ocean_distance < 100.0 {
                    1.0
                } else {
                    (200.0 / climate.ocean_distance).min(1.0)
                };

                let humidity = (base_humidity * temp_factor * ocean_factor).clamp(0.0, 1.0);

                let continentality = (climate.ocean_distance / 1000.0).min(1.0);

                Some((province.id.value(), humidity, continentality))
            })
            .collect();

        for (id, humidity, continentality) in humidity_values {
            if let Some(climate) = self.climates.get_mut(&id) {
                climate.humidity = humidity;
                climate.continentality = continentality;
            }
        }
    }

    /// Determine biome from climate data
    pub fn get_biome(&self, province_id: u32, elevation: f32) -> Biome {
        let climate = self.climates.get(&province_id)
            .cloned()
            .unwrap_or_default();

        let temp = climate.temperature;
        let rainfall = climate.rainfall;
        let elevation_m = elevation * 5000.0;

        // Special cases first
        if elevation_m > ALPINE_ELEVATION {
            return Biome::Alpine;
        }

        if rainfall < 50.0 {
            if temp < -10.0 {
                return Biome::PolarDesert;
            } else {
                return Biome::TropicalDesert;
            }
        }

        // Temperature-based classification
        if temp < -10.0 {
            // Polar
            if rainfall < 100.0 {
                Biome::PolarDesert
            } else {
                Biome::Tundra
            }
        } else if temp < 0.0 {
            // Subpolar
            if elevation_m > TREELINE_ELEVATION {
                Biome::Alpine
            } else if rainfall > 400.0 {
                Biome::Taiga
            } else {
                Biome::Tundra
            }
        } else if temp < 10.0 {
            // Cold temperate
            if rainfall > 800.0 {
                Biome::BorealForest
            } else if rainfall > 400.0 {
                Biome::Taiga
            } else {
                Biome::ColdDesert
            }
        } else if temp < 20.0 {
            // Temperate
            if rainfall > 1500.0 {
                Biome::TemperateRainforest
            } else if rainfall > 600.0 {
                Biome::TemperateDeciduousForest
            } else if rainfall > 250.0 {
                Biome::TemperateGrassland
            } else {
                Biome::ColdDesert
            }
        } else if temp < 25.0 {
            // Subtropical
            if rainfall > 1200.0 {
                Biome::TropicalSeasonalForest
            } else if rainfall > 600.0 {
                Biome::MediterraneanForest
            } else if rainfall > 250.0 {
                Biome::Chaparral
            } else {
                Biome::SubtropicalDesert
            }
        } else {
            // Tropical
            if rainfall > 2000.0 {
                Biome::TropicalRainforest
            } else if rainfall > 1200.0 {
                Biome::TropicalSeasonalForest
            } else if rainfall > 600.0 {
                Biome::Savanna
            } else {
                Biome::TropicalDesert
            }
        }
    }
}

/// Apply climate data to provinces during world generation
pub fn apply_climate_to_provinces(provinces: &mut [crate::world::Province], dimensions: crate::resources::MapDimensions) {
    let mut climate_system = ClimateSystem::new(dimensions);
    climate_system.simulate(provinces);

    // Apply climate results to provinces by setting terrain types based on biomes
    for (i, province) in provinces.iter_mut().enumerate() {
        let biome = climate_system.get_biome(province.id.value(), province.elevation.value());
        // Update terrain based on biome if not already set by other systems
        if province.terrain == crate::world::TerrainType::Ocean {
            province.terrain = biome_to_terrain(biome, province.elevation.value());
        }
    }
}

fn biome_to_terrain(biome: Biome, elevation: f32) -> crate::world::TerrainType {
    use crate::world::TerrainType;
    match biome {
        // Polar biomes
        Biome::PolarDesert => TerrainType::PolarDesert,
        Biome::Tundra => TerrainType::Tundra,

        // Cold biomes
        Biome::Taiga => TerrainType::Taiga,
        Biome::BorealForest => TerrainType::BorealForest,

        // Temperate biomes
        Biome::TemperateRainforest => TerrainType::TemperateRainforest,
        Biome::TemperateDeciduousForest => TerrainType::TemperateDeciduousForest,
        Biome::TemperateGrassland => TerrainType::TemperateGrassland,
        Biome::ColdDesert => TerrainType::ColdDesert,

        // Subtropical biomes
        Biome::MediterraneanForest => TerrainType::MediterraneanForest,
        Biome::Chaparral => TerrainType::Chaparral,
        Biome::SubtropicalDesert => TerrainType::SubtropicalDesert,

        // Tropical biomes
        Biome::TropicalRainforest => TerrainType::TropicalRainforest,
        Biome::TropicalSeasonalForest => TerrainType::TropicalSeasonalForest,
        Biome::Savanna => TerrainType::Savanna,
        Biome::TropicalDesert => if elevation > 0.6 { TerrainType::PolarDesert } else { TerrainType::TropicalDesert },

        // Special biomes
        Biome::Alpine => TerrainType::Alpine,
        Biome::Wetlands => TerrainType::Wetlands,
        Biome::Mangrove => TerrainType::Mangrove,
    }
}

