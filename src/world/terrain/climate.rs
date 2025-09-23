//! Climate simulation for realistic biome distribution
//!
//! This module simulates moisture propagation, temperature gradients,
//! rain shadows, and other climate phenomena to create realistic
//! biome distributions based on physical principles.

use super::super::provinces::Province;
use super::types::TerrainType;
use crate::world::ClimateType;
use crate::math::{exponential_smooth, lerp};
use crate::parallel::{parallel_map, parallel_zip_mutate, parallel_enumerate};
use bevy::log::{debug, info};
use bevy::prelude::Vec2;
use std::collections::{HashMap, VecDeque};
use std::f32::consts::PI;

// Climate temperature parameters

/// Get base temperatures for a given climate type
fn get_climate_temperatures(climate_type: ClimateType) -> (f32, f32) {
    // Returns (equator_temp, pole_temp) in Celsius
    match climate_type {
        ClimateType::Desert => (40.0, -10.0),      // Hot overall, less polar cold
        ClimateType::Arctic => (15.0, -45.0),      // Cold overall
        ClimateType::Tropical => (35.0, -20.0),    // Warm overall
        ClimateType::Temperate => (25.0, -35.0),   // Mild
        ClimateType::Mixed => (30.0, -30.0),       // Earth-like default
    }
}

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
const TRADE_WIND_ZONE: f32 = 0.3; // 0-30° latitude
const WESTERLIES_ZONE: f32 = 0.6; // 30-60° latitude
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
    /// Climate data for each province - Vec for O(1) indexed access
    pub climates: Vec<Climate>,
    /// World dimensions
    dimensions: crate::resources::MapDimensions,
    /// Climate type for temperature calculations
    climate_type: ClimateType,
}

impl ClimateSystem {
    pub fn new(dimensions: crate::resources::MapDimensions, province_count: usize, climate_type: ClimateType) -> Self {
        Self {
            climates: vec![Climate::default(); province_count],
            dimensions,
            climate_type,
        }
    }

    /// Run full climate simulation with unified passes for memory efficiency
    pub fn simulate(&mut self, provinces: &[Province]) {
        info!("Starting climate simulation with unified passes...");

        // Step 1: Calculate ocean distances
        self.calculate_ocean_distances(provinces);

        // Step 2 & 3 UNIFIED: Calculate temperatures AND winds in single pass
        self.calculate_temperatures_and_winds(provinces);

        // Step 4: Simulate moisture propagation
        self.propagate_moisture(provinces);

        // Step 5: Apply rain shadows
        self.apply_rain_shadows(provinces);

        // Step 6: Calculate final humidity
        self.calculate_humidity(provinces);

        info!("Climate simulation complete (memory optimized)");
    }

    /// Calculate distance to ocean for each province
    fn calculate_ocean_distances(&mut self, provinces: &[Province]) {
        debug!("Calculating ocean distances...");

        // Use Vec for O(1) indexed access instead of HashMap!
        let mut distances: Vec<Option<f32>> = vec![None; provinces.len()];
        let mut queue = VecDeque::new();

        // Initialize with ocean provinces
        let mut ocean_count = 0;
        for (idx, province) in provinces.iter().enumerate() {
            if province.terrain == TerrainType::Ocean {
                queue.push_back((idx, 0.0));
                distances[idx] = Some(0.0);
                ocean_count += 1;
            }
        }
        debug!("Found {} ocean provinces", ocean_count);

        // BFS using direct array indexing - no HashMap operations!
        while let Some((province_idx, distance)) = queue.pop_front() {
            let province = &provinces[province_idx];

            // Use precomputed neighbor indices for O(1) neighbor access
            for &neighbor_idx_opt in &province.neighbor_indices {
                if let Some(neighbor_idx) = neighbor_idx_opt {
                    // Direct array access - no hashing!
                    if distances[neighbor_idx].is_none() {
                        let new_distance = distance + self.dimensions.hex_size / 1000.0;
                        distances[neighbor_idx] = Some(new_distance);

                        // Only continue BFS if within influence distance
                        if new_distance < OCEAN_INFLUENCE_DISTANCE {
                            queue.push_back((neighbor_idx, new_distance));
                        }
                    }
                }
            }
        }

        // Store distances in climate data - single pass using direct indexing
        for idx in 0..provinces.len() {
            self.climates[idx].ocean_distance = distances[idx].unwrap_or(f32::INFINITY);
        }
    }

    /// Calculate temperature and wind patterns in a unified pass (OPTIMIZED)
    fn calculate_temperatures_and_winds(&mut self, provinces: &[Province]) {
        debug!("Calculating temperatures and wind patterns in unified pass...");

        // Use par_iter_mut to update climates directly without temporary allocations
        let bounds = self.dimensions.bounds;
        let hex_size = self.dimensions.hex_size;

        // Get temperature parameters for the climate type
        let (equator_temp, pole_temp) = get_climate_temperatures(self.climate_type);

        // Use safe parallel mutation with monitoring
        parallel_zip_mutate(
            &mut self.climates,
            provinces,
            |climate, province| {
                // Calculate temperature
                let latitude = (province.position.y - bounds.y_min) / (bounds.y_max - bounds.y_min);
                let lat_from_equator = (latitude - 0.5).abs() * 2.0;
                let base_temp = lerp(equator_temp, pole_temp, lat_from_equator);

                // Apply elevation cooling
                let elevation_m = province.elevation.value() * 5000.0;
                let elevation_cooling = elevation_m * LAPSE_RATE;

                // Apply ocean moderation
                let ocean_moderation = if climate.ocean_distance < OCEAN_INFLUENCE_DISTANCE {
                    let factor = 1.0 - (climate.ocean_distance / OCEAN_INFLUENCE_DISTANCE);
                    OCEAN_TEMP_MODERATION * factor
                } else {
                    0.0
                };

                climate.temperature = base_temp - elevation_cooling + ocean_moderation;

                // Calculate wind patterns in same pass
                let lat_from_equator_wind = (latitude - 0.5).abs();

                if lat_from_equator_wind < TRADE_WIND_ZONE {
                    // Trade winds - blow from east to west
                    climate.wind_direction = PI; // West
                    climate.wind_strength = 0.8;
                } else if lat_from_equator_wind < WESTERLIES_ZONE {
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
            },
            "Temperature and wind calculation"
        );
    }

    /// Propagate moisture from oceans inland
    fn propagate_moisture(&mut self, provinces: &[Province]) {
        debug!("Propagating moisture from oceans...");

        let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        let grid_size = self.dimensions.hex_size * 5.0;

        for (idx, province) in provinces.iter().enumerate() {
            let grid_x = (province.position.x / grid_size).floor() as i32;
            let grid_y = (province.position.y / grid_size).floor() as i32;
            grid.entry((grid_x, grid_y))
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Reduced iterations for better performance
        const MOISTURE_ITERATIONS: usize = 2; // Further reduced from 3

        for iteration in 0..MOISTURE_ITERATIONS {
            debug!(
                "      Moisture propagation iteration {}/{}",
                iteration + 1,
                MOISTURE_ITERATIONS
            );

            // Clone rainfall data only (not entire Climate structs!) for parallel read
            let current_rainfall: Vec<f32> = self.climates.iter().map(|c| c.rainfall).collect();

            let batch_size = 10000;
            let num_batches = (provinces.len() + batch_size - 1) / batch_size;

            for batch_idx in 0..num_batches {
                let start_idx = batch_idx * batch_size;
                let end_idx = (start_idx + batch_size).min(provinces.len());

                // Use safe parallel operation for batch processing
                let batch_indices: Vec<usize> = (start_idx..end_idx).collect();
                let batch_rainfall: Vec<(usize, f32)> = parallel_map(
                    &batch_indices,
                    |&idx| {
                        let province = &provinces[idx];
                        let climate = &self.climates[idx];

                        // Start with base rainfall
                        let mut rainfall = if province.terrain == TerrainType::Ocean {
                            OCEAN_RAINFALL
                        } else {
                            // Decay based on distance from ocean
                            let decay =
                                RAINFALL_DECAY_INLAND.powf(climate.ocean_distance.min(500.0));
                            OCEAN_RAINFALL * decay
                        };

                        // Use precomputed neighbor indices for O(1) access!
                        for &neighbor_idx_opt in &province.neighbor_indices {
                            if let Some(neighbor_idx) = neighbor_idx_opt {
                                let neighbor_rainfall = current_rainfall[neighbor_idx];
                                let transfer = neighbor_rainfall * 0.05 * climate.wind_strength;
                                rainfall += transfer;
                            }
                        }

                        // Cap rainfall at reasonable maximum
                        rainfall = rainfall.min(3000.0);

                        (idx, rainfall)
                    },
                    "Rainfall batch calculation"
                );

                for (idx, rainfall) in batch_rainfall {
                    // Blend with existing value for smoother propagation
                    self.climates[idx].rainfall =
                        exponential_smooth(self.climates[idx].rainfall, rainfall, 0.3);
                }

                // Progress report - reduced frequency
                if batch_idx % 50 == 0 {
                    debug!("Processing batch {}/{}", batch_idx + 1, num_batches);
                }
            }
        }

        info!("      Moisture propagation complete");
    }

    /// Apply rain shadow effects from mountains (PARALLELIZED)
    fn apply_rain_shadows(&mut self, provinces: &[Province]) {
        info!("    Applying rain shadow effects...");

        let mut spatial_grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
        let grid_size = self.dimensions.hex_size * 2.0;

        for (idx, province) in provinces.iter().enumerate() {
            let grid_x = (province.position.x / grid_size).floor() as i32;
            let grid_y = (province.position.y / grid_size).floor() as i32;
            spatial_grid
                .entry((grid_x, grid_y))
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Extract wind data only for parallel access (not cloning entire climates!)
        let wind_data: Vec<(f32, f32)> = self
            .climates
            .iter()
            .map(|c| (c.wind_direction, c.wind_strength))
            .collect();
        let hex_size = self.dimensions.hex_size;

        // Use safe parallel enumeration for rain shadow detection
        let rain_shadow_results = parallel_enumerate(
            provinces,
            |idx, province| {
                let (wind_dir, _wind_strength) = wind_data[idx];

                let (sin_wind, cos_wind) = wind_dir.sin_cos();
                let wind_source = Vec2::new(-cos_wind, -sin_wind);

                // Sample fewer points for performance
                for i in 1..=5 {
                    let check_pos = province.position + wind_source * (i as f32 * 40.0);
                    let grid_x = (check_pos.x / grid_size).floor() as i32;
                    let grid_y = (check_pos.y / grid_size).floor() as i32;

                    if let Some(indices) = spatial_grid.get(&(grid_x, grid_y)) {
                        for &other_idx in indices {
                            let other = &provinces[other_idx];
                            if other.position.distance(check_pos) < hex_size * 1.5 {
                                if other.elevation.value() > 0.6 {
                                    return Some(idx);
                                }
                            }
                        }
                    }
                }
                None
            },
            "Rain shadow detection"
        );

        // Filter out None values to get actual rain shadow indices
        let rain_shadows: Vec<usize> = rain_shadow_results
            .into_iter()
            .flatten()
            .collect();

        // Apply rain shadow reduction
        let shadow_count = rain_shadows.len();
        for idx in rain_shadows {
            self.climates[idx].rainfall *= RAIN_SHADOW_FACTOR;
        }

        debug!("Applied rain shadow to {} provinces", shadow_count);
    }

    /// Calculate humidity from rainfall and temperature (PARALLELIZED)
    fn calculate_humidity(&mut self, provinces: &[Province]) {
        info!("    Calculating humidity levels...");

        // Use safe parallel enumeration for humidity calculation
        let humidity_results = parallel_enumerate(
            provinces,
            |idx, _province| {
                let climate = &self.climates[idx];
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

                Some((idx, humidity, continentality))
            },
            "Humidity calculation"
        );

        // Filter out None values and collect humidity data
        let humidity_values: Vec<(usize, f32, f32)> = humidity_results
            .into_iter()
            .flatten()
            .collect();

        for (idx, humidity, continentality) in humidity_values {
            self.climates[idx].humidity = humidity;
            self.climates[idx].continentality = continentality;
        }
    }

    /// Determine biome from climate data
    pub fn get_biome(&self, idx: usize, elevation: f32) -> Biome {
        let climate = &self.climates[idx];

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
pub fn apply_climate_to_provinces(
    provinces: &mut [crate::world::Province],
    dimensions: crate::resources::MapDimensions,
    climate_type: ClimateType,
) {
    // Count how many provinces actually need climate calculations
    let land_provinces = provinces
        .iter()
        .filter(|p| p.terrain != crate::world::TerrainType::Ocean)
        .count();
    info!(
        "    Calculating climate for {} land provinces (skipping {} ocean provinces) with {:?} climate",
        land_provinces,
        provinces.len() - land_provinces,
        climate_type
    );

    let mut climate_system = ClimateSystem::new(dimensions, provinces.len(), climate_type);
    climate_system.simulate(provinces);

    // Apply climate results to provinces by setting terrain types based on biomes
    // LAZY: Skip ocean provinces entirely for biome calculations
    for (idx, province) in provinces.iter_mut().enumerate() {
        // Skip expensive biome calculations for ocean provinces
        if province.terrain == crate::world::TerrainType::Ocean {
            continue;
        }

        let biome = climate_system.get_biome(idx, province.elevation.value());
        // Update terrain based on biome for land provinces only (preserve River/Beach)
        if province.terrain != crate::world::TerrainType::River
            && province.terrain != crate::world::TerrainType::Beach
        {
            province.terrain = biome_to_terrain(biome, province.elevation.value());
        }
    }
}

fn biome_to_terrain(biome: Biome, elevation: f32) -> crate::world::TerrainType {
    use super::types::TerrainType;
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
        Biome::TropicalDesert => {
            if elevation > 0.6 {
                TerrainType::PolarDesert
            } else {
                TerrainType::TropicalDesert
            }
        }

        // Special biomes
        Biome::Alpine => TerrainType::Alpine,
        Biome::Wetlands => TerrainType::Wetlands,
        Biome::Mangrove => TerrainType::Mangrove,
    }
}
