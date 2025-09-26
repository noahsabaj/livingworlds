//! Elevation generation and processing
//!
//! Handles terrain elevation generation using Perlin noise, redistribution algorithms,
//! and various techniques to create realistic topography.

use bevy::prelude::Vec2;
use log::info;

use crate::math::{PerlinNoise, smooth_falloff};
use crate::world::provinces::elevation::{ElevationParams, MapBounds, compute_elevation};
use super::continents::ContinentSeed;

/// Falloff start distance from center (60% = natural island shapes)
const FALLOFF_START: f32 = 0.6;

/// Histogram equalization bins for elevation redistribution
const HISTOGRAM_BINS: usize = 256;

/// Processes elevation data for province generation
pub struct ElevationProcessor {
    noise: PerlinNoise,
    dimensions: crate::resources::MapDimensions,
    seed: u32,
}

impl ElevationProcessor {
    pub fn new(seed: u32, dimensions: crate::resources::MapDimensions) -> Self {
        Self {
            noise: PerlinNoise::with_seed(seed),
            dimensions,
            seed,
        }
    }

    /// Generate elevation for a single position using the unified elevation system
    pub fn generate_elevation(&self, position: Vec2, continent_seeds: &[(Vec2, f32, f32)]) -> f32 {
        let params = ElevationParams {
            position,
            continent_seeds,
            continent_count: continent_seeds.len() as u32,
            seed: self.seed as u64,
            hex_size: self.dimensions.hex_size,
            map_bounds: MapBounds {
                x_min: self.dimensions.bounds.x_min,
                x_max: self.dimensions.bounds.x_max,
                y_min: self.dimensions.bounds.y_min,
                y_max: self.dimensions.bounds.y_max,
            },
        };

        compute_elevation(&params, &self.noise)
    }

    /// Process all elevations with redistribution
    pub fn process_elevations(&self, positions: &[Vec2], continent_seeds: &[ContinentSeed]) -> Vec<f32> {
        // Convert seeds to tuple format for elevation computation
        let seed_tuples: Vec<(Vec2, f32, f32)> = continent_seeds.iter()
            .map(|s| (s.position, s.strength, s.radius))
            .collect();

        // Generate base elevations
        let mut elevations: Vec<f32> = positions.iter()
            .map(|&pos| self.generate_elevation(pos, &seed_tuples))
            .collect();

        // Apply redistribution for better terrain distribution
        self.apply_adaptive_redistribution(&mut elevations);

        elevations
    }

    /// Calculate distance from map edges for falloff (radial distance)
    pub fn calculate_edge_distance(&self, position: Vec2) -> f32 {
        let width = self.dimensions.bounds.x_max - self.dimensions.bounds.x_min;
        let height = self.dimensions.bounds.y_max - self.dimensions.bounds.y_min;

        // Calculate center of the map
        let center_x = self.dimensions.bounds.x_min + width / 2.0;
        let center_y = self.dimensions.bounds.y_min + height / 2.0;
        let map_center = Vec2::new(center_x, center_y);

        // Use radial distance from center for natural, circular falloff
        let distance_from_center = position.distance(map_center);
        let max_distance = (width.min(height)) / 2.0;

        // Normalize to 0-1 range (0 at center, 1 at edges)
        (distance_from_center / max_distance).clamp(0.0, 1.0)
    }

    /// Calculate falloff based on distance from edge
    pub fn calculate_falloff(&self, distance: f32) -> f32 {
        // Use TWO smooth_falloff calls with .max() for organic landmasses
        // This exact algorithm creates natural island shapes
        smooth_falloff(distance, 0.0, FALLOFF_START)
            .max(1.0 - smooth_falloff(distance, FALLOFF_START, 1.0))
    }

    /// Apply adaptive percentile-based redistribution to elevation data
    pub fn apply_adaptive_redistribution(&self, elevations: &mut [f32]) {
        // Sort to find percentiles
        let mut sorted = elevations.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));

        // Find key percentiles in the distribution
        let len = sorted.len();
        let p10 = sorted[len * 10 / 100];  // 10th percentile
        let p40 = sorted[len * 40 / 100];  // 40th percentile
        let p60 = sorted[len * 60 / 100];  // 60th percentile (target sea level)
        let p70 = sorted[len * 70 / 100];  // 70th percentile
        let p90 = sorted[len * 90 / 100];  // 90th percentile
        let p95 = sorted[len * 95 / 100];  // 95th percentile
        let p99 = sorted[len * 99 / 100];  // 99th percentile

        // Build mapping function based on percentiles
        for elevation in elevations.iter_mut() {
            let val = *elevation;

            *elevation = if val <= p60 {
                // Bottom 60% → Ocean (0.0 to 0.15)
                let t = (val - sorted[0]) / (p60 - sorted[0]).max(0.001);
                t * t * 0.15  // Quadratic for deep ocean basins

            } else if val <= p70 {
                // 60-70% → Coastal transition (0.15 to 0.25)
                let t = (val - p60) / (p70 - p60).max(0.001);
                0.15 + t * 0.10

            } else if val <= p90 {
                // 70-90% → Lowlands and hills (0.25 to 0.45)
                let t = (val - p70) / (p90 - p70).max(0.001);
                0.25 + t * 0.20

            } else if val <= p95 {
                // 90-95% → Mountains (0.45 to 0.65)
                let t = (val - p90) / (p95 - p90).max(0.001);
                0.45 + t * 0.20

            } else {
                // Top 5% → High peaks (0.65 to 1.0)
                let t = (val - p95) / (p99 - p95).max(0.001);
                0.65 + t.powf(0.7) * 0.35
            };
        }

        info!("  Adaptive redistribution: p60={:.3} (sea), p90={:.3} (mountains), p95={:.3} (peaks)",
              p60, p90, p95);
    }

    /// Apply histogram equalization for better distribution
    pub fn histogram_equalization(&self, elevations: &mut [f32]) {
        // Build histogram
        let mut histogram = vec![0u32; HISTOGRAM_BINS];
        for &elev in elevations.iter() {
            let bin = ((elev.clamp(0.0, 1.0) * (HISTOGRAM_BINS - 1) as f32) as usize)
                .min(HISTOGRAM_BINS - 1);
            histogram[bin] += 1;
        }

        // Compute cumulative distribution function (CDF)
        let mut cdf = vec![0.0f32; HISTOGRAM_BINS];
        let total = elevations.len() as f32;
        let mut sum = 0u32;
        for i in 0..HISTOGRAM_BINS {
            sum += histogram[i];
            cdf[i] = sum as f32 / total;
        }

        // Apply equalization
        for elev in elevations.iter_mut() {
            let bin = ((elev.clamp(0.0, 1.0) * (HISTOGRAM_BINS - 1) as f32) as usize)
                .min(HISTOGRAM_BINS - 1);
            *elev = cdf[bin];
        }
    }

    /// Apply simple power redistribution to a single elevation value
    /// Used for sea level calculation sampling
    pub fn apply_hybrid_power_redistribution(&self, elevation: f32) -> f32 {
        // Simple curve for single-value redistribution
        elevation.powf(1.2)
    }
}