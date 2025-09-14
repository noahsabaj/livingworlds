//! World builder implementation
//!
//! This module contains the main WorldBuilder that orchestrates all generation steps
//! to create a complete World data structure.

use bevy::prelude::*;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::resources::{WorldSize, MapDimensions};
use crate::constants::*;
use super::super::data::World;

// Import the internal generation modules
use super::{provinces, rivers, agriculture, clouds, erosion, climate, utils};

/// World builder that orchestrates all generation steps
///
/// This is the main entry point for world generation. It coordinates
/// all the individual builders to create a complete World.
pub struct WorldBuilder {
    seed: u32,
    size: WorldSize,
    rng: StdRng,
    dimensions: MapDimensions,
    continent_count: u32,
    ocean_coverage: f32,
    river_density: f32,
}

impl WorldBuilder {
    pub fn new(
        seed: u32,
        size: WorldSize,
        continent_count: u32,
        ocean_coverage: f32,
        river_density: f32,
    ) -> Self {
        let rng = StdRng::seed_from_u64(seed as u64);
        let dimensions = MapDimensions::from_world_size(&size);

        Self {
            seed,
            size,
            rng,
            dimensions,
            continent_count,
            ocean_coverage,
            river_density,
        }
    }

    pub fn build(mut self) -> World {
        let start = std::time::Instant::now();
        println!("═══════════════════════════════════════════════════════");
        println!("Starting world generation with seed: {}", self.seed);
        println!("World size: {:?} ({} provinces)", self.size,
                 self.dimensions.provinces_per_row * self.dimensions.provinces_per_col);
        println!("═══════════════════════════════════════════════════════");

        // Step 1: Generate provinces with Perlin noise elevation
        println!("\n[1/7] Generating provinces with Perlin noise terrain...");
        let mut provinces = provinces::ProvinceBuilder::new(
            self.dimensions,
            &mut self.rng,
            self.seed,
        )
        .with_ocean_coverage(self.ocean_coverage)
        .with_continent_count(self.continent_count)
        .build();

        // Step 2: Apply erosion simulation for realistic terrain
        println!("\n[2/7] Applying erosion simulation...");
        let erosion_iterations = match self.dimensions.provinces_per_row * self.dimensions.provinces_per_col {
            n if n < 400_000 => 3_000,
            n if n < 700_000 => 5_000,
            _ => 8_000,
        };
        println!("  Using {} erosion iterations for world size", erosion_iterations);
        erosion::apply_erosion_to_provinces(
            &mut provinces,
            self.dimensions,
            self.rng.clone(),
            erosion_iterations,
        );

        // Step 3: Calculate ocean depths
        println!("\n[3/7] Calculating ocean depths...");
        utils::calculate_ocean_depths(&mut provinces, self.dimensions);

        // Step 4: Generate climate zones
        println!("\n[4/7] Applying climate zones...");
        climate::apply_climate_to_provinces(&mut provinces, self.dimensions);

        // Step 5: Generate river systems
        println!("\n[5/7] Generating river systems...");
        let river_system = rivers::RiverBuilder::new(
            &provinces,
            self.dimensions,
            self.rng.clone(),
        )
        .with_river_count(self.river_density)
        .build(&mut provinces);

        // Step 6: Calculate agriculture values
        println!("\n[6/7] Calculating agriculture values...");
        agriculture::calculate_agriculture_values(&mut provinces);

        // Step 7: Generate cloud system
        println!("\n[7/7] Generating atmospheric clouds...");
        let cloud_system = clouds::CloudBuilder::new(
            self.dimensions,
            self.rng.clone(),
        ).build();

        println!("\n[*] Building spatial index...");
        let mut spatial_index = HashMap::new();
        for province in &provinces {
            let col = province.id.value() % self.dimensions.provinces_per_row;
            let row = province.id.value() / self.dimensions.provinces_per_row;
            spatial_index.insert((col as i32, row as i32), province.id.value());
        }

        let elapsed = start.elapsed();
        println!("\n═══════════════════════════════════════════════════════");
        println!("World generation completed in {:.2}s", elapsed.as_secs_f32());
        println!("═══════════════════════════════════════════════════════");

        World {
            provinces,
            rivers: river_system,
            spatial_index,
            map_dimensions: self.dimensions,
            clouds: cloud_system,
        }
    }
}