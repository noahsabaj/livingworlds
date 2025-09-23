//! Single Source of Truth for Elevation Generation
//!
//! This module provides the ONLY implementation of elevation calculation logic.
//! Both CPU and GPU backends should use this same algorithm.

use bevy::prelude::*;
use crate::math::{PerlinNoise, smooth_falloff};

/// All parameters needed for elevation calculation
#[derive(Debug, Clone)]
pub struct ElevationParams<'a> {
    pub position: Vec2,
    pub continent_seeds: &'a [(Vec2, f32, f32)],
    pub continent_count: u32,
    pub seed: u64,
    pub hex_size: f32,
    pub map_bounds: MapBounds,
}

#[derive(Debug, Clone)]
pub struct MapBounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl MapBounds {
    pub fn width(&self) -> f32 {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> f32 {
        self.y_max - self.y_min
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x_min + self.width() / 2.0,
            self.y_min + self.height() / 2.0,
        )
    }
}

/// The SINGLE elevation calculation function - all elevation generation goes through here
///
/// This is the Single Source of Truth for elevation generation. Any changes to the
/// elevation algorithm should be made here and ONLY here.
pub fn compute_elevation(params: &ElevationParams, noise: &PerlinNoise) -> f32 {
    // Scale position to noise space with proper hexagonal aspect ratio correction
    // Hexagons have x spacing of 1.5 * hex_size and y spacing of SQRT_3 * hex_size
    // We need to normalize by these different factors to get uniform noise sampling
    let x_scale = 1.0 / (params.hex_size * 1.5);
    let y_scale = 1.0 / (params.hex_size * 1.732050808); // SQRT_3
    let x = (params.position.x * x_scale) as f64;
    let y = (params.position.y * y_scale) as f64;

    // Sample base terrain noise
    let base_elevation = noise.sample_terrain(x, y) as f32;

    // Calculate continent influence
    let continent_influence = calculate_continent_influence(
        params.position,
        params.continent_seeds,
        noise,
    );

    // CRITICAL: Different weighting for Pangaea vs normal worlds
    let combined_elevation = if params.continent_count == 1 {
        // Pangaea mode: 95% continent, 5% noise for massive supercontinent
        let pangaea_elevation = base_elevation * 0.05 + continent_influence * 0.95;

        // Add edge suppression for Pangaea
        apply_pangaea_edge_suppression(pangaea_elevation, params.position, &params.map_bounds)
    } else {
        // Normal mode: 50/50 balance for varied continents
        base_elevation * 0.5 + continent_influence * 0.5
    };

    // Apply power redistribution for realistic terrain
    apply_hybrid_power_redistribution(combined_elevation)
}

/// Calculate influence from all continent seeds
fn calculate_continent_influence(
    position: Vec2,
    continent_seeds: &[(Vec2, f32, f32)],
    noise: &PerlinNoise,
) -> f32 {
    let mut max_influence = 0.0_f32;

    for (seed_pos, strength, radius) in continent_seeds {
        let distance = position.distance(*seed_pos);

        // Domain warping for organic continent shapes
        let warp_x = noise.sample_scaled(
            (position.x * 0.005) as f64,
            (position.y * 0.005) as f64,
            0.01,
        ) as f32;
        let warp_y = noise.sample_scaled(
            (position.x * 0.005 + 100.0) as f64,
            (position.y * 0.005 + 100.0) as f64,
            0.01,
        ) as f32;

        let warp_strength = radius * 0.3;
        let warped_distance = distance + (warp_x + warp_y) * warp_strength;

        // Smooth falloff for natural coastlines
        let inner_radius = radius * 0.4;
        let outer_radius = radius * 1.2;
        let influence = smooth_falloff(warped_distance, inner_radius, outer_radius) * strength;

        max_influence = max_influence.max(influence);
    }

    max_influence
}

/// Apply edge suppression for Pangaea to ensure ocean surrounds the supercontinent
fn apply_pangaea_edge_suppression(elevation: f32, position: Vec2, bounds: &MapBounds) -> f32 {
    let center = bounds.center();
    let distance_from_center = position.distance(center);
    let max_distance = (bounds.width().min(bounds.height()) / 2.0) * 0.85;

    if distance_from_center > max_distance {
        0.0 // Force ocean at edges
    } else {
        elevation
    }
}

/// Apply power redistribution for realistic elevation distribution
fn apply_hybrid_power_redistribution(elevation: f32) -> f32 {
    // Simple power curve for now - can be made more sophisticated
    elevation.powf(1.2).clamp(0.0, 1.0)
}

/// Configuration for batch elevation generation
pub struct BatchElevationConfig {
    pub continent_seeds: Vec<(Vec2, f32, f32)>,
    pub continent_count: u32,
    pub seed: u64,
    pub hex_size: f32,
    pub map_bounds: MapBounds,
}

/// Generate elevations for multiple positions at once
/// This can be parallelized with rayon or offloaded to GPU
pub fn compute_batch_elevations(
    positions: &[Vec2],
    config: &BatchElevationConfig,
    noise: &PerlinNoise,
) -> Vec<f32> {
    positions
        .iter()
        .map(|position| {
            let params = ElevationParams {
                position: *position,
                continent_seeds: &config.continent_seeds,
                continent_count: config.continent_count,
                seed: config.seed,
                hex_size: config.hex_size,
                map_bounds: config.map_bounds.clone(),
            };
            compute_elevation(&params, noise)
        })
        .collect()
}