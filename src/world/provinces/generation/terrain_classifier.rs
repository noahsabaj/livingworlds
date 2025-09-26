//! Terrain type classification
//!
//! Converts elevation data into terrain types, handling beaches, mountains,
//! and various biome classifications.

use crate::math::smoothstep;
use crate::world::terrain::TerrainType;

/// Beach/coast width in elevation units
const BEACH_WIDTH: f32 = 0.01;

/// Smoothing range for coastline transitions
const COASTLINE_SMOOTH_RANGE: f32 = 0.04;

/// Classifies terrain based on elevation and other factors
pub struct TerrainClassifier;

impl TerrainClassifier {
    /// Classify a single elevation into terrain type with smooth coastlines
    pub fn classify_terrain(elevation: f32, sea_level: f32) -> TerrainType {
        // Apply smoothstep near sea level for cleaner coastlines
        let smoothed_elevation = if (elevation - sea_level).abs() < COASTLINE_SMOOTH_RANGE {
            // Near sea level, apply smoothstep to reduce noise
            let t = (elevation - (sea_level - COASTLINE_SMOOTH_RANGE)) / (COASTLINE_SMOOTH_RANGE * 2.0);
            let smooth_t = smoothstep(0.0, 1.0, t);
            (sea_level - COASTLINE_SMOOTH_RANGE) + smooth_t * (COASTLINE_SMOOTH_RANGE * 2.0)
        } else {
            elevation
        };

        // Elevation-based terrain classification
        if smoothed_elevation < sea_level {
            TerrainType::Ocean
        } else if smoothed_elevation < sea_level + BEACH_WIDTH {
            TerrainType::Beach
        } else if smoothed_elevation < sea_level + 0.05 {
            // Coastal lowlands
            TerrainType::TemperateGrassland
        } else if smoothed_elevation < sea_level + 0.15 {
            // Foothills
            TerrainType::TemperateDeciduousForest
        } else if smoothed_elevation < sea_level + 0.25 {
            // Mid-elevation (will become desert with rain shadow)
            TerrainType::Chaparral
        } else if smoothed_elevation < sea_level + 0.4 {
            // Highlands
            TerrainType::Alpine
        } else {
            // Mountain peaks
            TerrainType::Tundra
        }
    }

    /// Classify all elevations in batch
    pub fn classify_all(elevations: &[f32], sea_level: f32) -> Vec<TerrainType> {
        elevations.iter()
            .map(|&elevation| Self::classify_terrain(elevation, sea_level))
            .collect()
    }

    /// Determine biome based on terrain and climate factors
    pub fn determine_biome(terrain: TerrainType, moisture: f32, _temperature: f32) -> TerrainType {
        // Climate-based biome modifications
        match terrain {
            TerrainType::Chaparral if moisture < 0.3 => TerrainType::Desert,
            TerrainType::TemperateGrassland if moisture < 0.2 => TerrainType::Savanna,
            TerrainType::TemperateDeciduousForest if moisture < 0.25 => TerrainType::TemperateGrassland,
            _ => terrain,
        }
    }
}