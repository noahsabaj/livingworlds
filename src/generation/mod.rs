//! World generation builders and factories for Living Worlds
//! 
//! This module contains all the BUILDERS that create world data.
//! Following the builder pattern, each module provides factories that
//! produce the data structures defined in the world module.

// Public module exports
pub mod provinces;
pub mod rivers;
pub mod agriculture;
pub mod clouds;
pub mod erosion;
pub mod climate;
pub mod utils;

// Import world data types that we build
use crate::world::World;

// Re-export the main builder is not needed since it's defined below

use bevy::prelude::Vec4;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::resources::{WorldSize, MapDimensions};
use crate::constants::*;

/// World builder that orchestrates all generation steps
///
/// This follows the builder pattern to construct a complete World.
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
    /// Create a new world builder with the given parameters
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
    
    /// Build the complete world
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
        // Scale erosion iterations based on world size for reasonable performance
        let erosion_iterations = match self.dimensions.provinces_per_row * self.dimensions.provinces_per_col {
            n if n < 400_000 => 3_000,   // Small worlds: fast generation
            n if n < 700_000 => 5_000,   // Medium worlds: balanced
            _ => 8_000,                  // Large worlds: still reasonable
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
        provinces::calculate_ocean_depths(&mut provinces, self.dimensions);

        // Step 4: Simulate climate system
        println!("\n[4/7] Simulating climate (temperature, moisture, biomes)...");
        let mut climate_system = climate::ClimateSystem::new(self.dimensions);
        climate_system.simulate(&provinces);

        // Apply biomes to provinces based on climate
        for province in &mut provinces {
            let biome = climate_system.get_biome(
                province.id.value(),
                province.elevation.value()
            );
            // Convert biome to terrain type using 1:1 mapping
            province.terrain = crate::world::terrain::biome_to_terrain(biome, province.elevation.value());
        }

        // Step 5: Generate river systems (now follows eroded valleys)
        println!("\n[5/7] Generating river systems (density: {:.1}x)...", self.river_density);
        let rivers = rivers::RiverBuilder::new(
            &mut provinces,
            self.dimensions,
            &mut self.rng,
        )
        .with_density(self.river_density)
        .build()
        .expect("Failed to generate river systems");

        // Step 6: Calculate agriculture and fresh water
        println!("\n[6/7] Calculating agriculture and fresh water...");
        agriculture::calculate(&mut provinces, &rivers, self.dimensions)
            .expect("Failed to calculate agriculture");

        // Step 7: Generate mineral resources using elevation and terrain
        println!("\n[7/7] Generating mineral resources...");
        crate::minerals::generate_world_minerals(
            self.seed,
            &mut provinces
        );
        
        // Build spatial index for fast lookups
        let mut spatial_index = HashMap::new();
        let spatial_cell_size = self.dimensions.hex_size * SPATIAL_INDEX_CELL_SIZE_MULTIPLIER;
        
        for province in &provinces {
            let grid_x = (province.position.x / spatial_cell_size).floor() as i32;
            let grid_y = (province.position.y / spatial_cell_size).floor() as i32;
            spatial_index.insert((grid_x, grid_y), province.id.value());
        }
        
        // Generate cloud system
        let clouds = clouds::CloudBuilder::new(&mut self.rng, &self.dimensions)
            .build();
        
        let elapsed = start.elapsed();
        println!("\n═══════════════════════════════════════════════════════");
        println!("✓ World generation completed in {:.2}s", elapsed.as_secs_f32());
        println!("  {} provinces generated", provinces.len());
        println!("  {} river tiles", rivers.river_tiles.len());
        println!("  {} delta tiles", rivers.delta_tiles.len());
        println!("  {} clouds", clouds.clouds.len());
        println!("═══════════════════════════════════════════════════════");
        
        World {
            provinces,
            rivers,
            spatial_index,
            map_dimensions: self.dimensions,
            clouds,
        }
    }
}