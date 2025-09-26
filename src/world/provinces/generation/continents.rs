//! Continent generation and seed placement
//!
//! Handles the creation of continental landmasses through strategic seed placement
//! and strength distribution for realistic continent shapes.

use bevy::prelude::Vec2;
use log::info;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::f32::consts::TAU;

use crate::resources::MapDimensions;

/// Represents a continent seed point with influence radius
pub struct ContinentSeed {
    pub position: Vec2,
    pub strength: f32,
    pub radius: f32,
}

/// Generates and manages continent seeds for landmass formation
pub struct ContinentGenerator {
    dimensions: MapDimensions,
    continent_count: u32,
    seed: u32,
}

impl ContinentGenerator {
    pub fn new(dimensions: MapDimensions, continent_count: u32, seed: u32) -> Self {
        Self {
            dimensions,
            continent_count,
            seed,
        }
    }

    /// Generate continent seeds with appropriate spacing and strength
    pub fn generate_seeds(&self, rng: &mut StdRng) -> Vec<ContinentSeed> {
        let mut seeds = Vec::new();

        // Vary continent count for more diverse worlds
        let min_continents = 3.min(self.continent_count);
        let num_continents = if min_continents == self.continent_count {
            // If continent_count < 3, use exact count (no variation)
            self.continent_count
        } else {
            // Otherwise, vary between 3 and the specified count
            rng.gen_range(min_continents..=self.continent_count)
        };

        let map_width = self.dimensions.bounds.x_max - self.dimensions.bounds.x_min;
        let map_height = self.dimensions.bounds.y_max - self.dimensions.bounds.y_min;
        let center_x = self.dimensions.bounds.x_min + map_width / 2.0;
        let center_y = self.dimensions.bounds.y_min + map_height / 2.0;

        // Special handling for Pangaea (single supercontinent)
        if self.continent_count == 1 {
            // Create one ENORMOUS continent in the center
            let position = Vec2::new(center_x, center_y);
            let strength = 1.0;  // Maximum strength for total dominance
            let radius = 0.9 * map_width.min(map_height);  // 90% of map size

            info!("  Generating Pangaea supercontinent: radius={:.0}, strength={:.2}", radius, strength);
            seeds.push(ContinentSeed { position, strength, radius });
        } else {
            // Normal multi-continent generation
            for i in 0..num_continents {
                // Place some continents near center, others more randomly
                let (x, y) = if i < 2 && num_continents > 4 {
                    // First couple continents closer to center for larger worlds
                    let angle = rng.gen_range(0.0..TAU);
                    let dist = rng.gen_range(0.2..0.5) * map_width.min(map_height) / 2.0;
                    let (sin_angle, cos_angle) = angle.sin_cos();
                    (center_x + cos_angle * dist, center_y + sin_angle * dist)
                } else {
                    // Others more randomly distributed
                    let x = self.dimensions.bounds.x_min + rng.gen::<f32>() * map_width;
                    let y = self.dimensions.bounds.y_min + rng.gen::<f32>() * map_height;
                    (x, y)
                };

                let position = Vec2::new(x, y);
                let strength = rng.gen_range(0.6..1.0);
                let radius = rng.gen_range(0.15..0.35) * map_width.min(map_height);

                seeds.push(ContinentSeed { position, strength, radius });
            }
        }

        seeds
    }

    /// Generate seeds optimized for GPU processing with deterministic seeding
    pub fn generate_seeds_for_gpu(&self) -> Vec<(Vec2, f32, f32)> {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        let mut seeds = Vec::new();

        let world_width = self.dimensions.provinces_per_row as f32 * self.dimensions.hex_size;
        let world_height = self.dimensions.provinces_per_col as f32 * self.dimensions.hex_size;

        // Earth-like continent distribution with varied sizes
        for i in 0..self.continent_count {
            // Distribute continents across entire map
            let angle = (i as f32 / self.continent_count as f32) * TAU + rng.gen::<f32>() * 0.5;
            let distance = (0.2 + rng.gen::<f32>() * 0.6) * world_width.min(world_height) * 0.4;

            let x = angle.cos() * distance + (rng.gen::<f32>() - 0.5) * world_width * 0.3;
            let y = angle.sin() * distance + (rng.gen::<f32>() - 0.5) * world_height * 0.3;

            // Vary continent sizes for realism
            let size_roll = rng.gen::<f32>();
            let (strength, radius) = if size_roll < 0.3 {
                // Small islands
                (0.15 + rng.gen::<f32>() * 0.15, 50.0 + rng.gen::<f32>() * 100.0)
            } else if size_roll < 0.7 {
                // Medium continents
                (0.3 + rng.gen::<f32>() * 0.25, 150.0 + rng.gen::<f32>() * 150.0)
            } else {
                // Large continents
                (0.45 + rng.gen::<f32>() * 0.3, 250.0 + rng.gen::<f32>() * 200.0)
            };

            seeds.push((Vec2::new(x, y), strength, radius));
        }

        seeds
    }
}