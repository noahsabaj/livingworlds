//! World generation builders and factories for Living Worlds
//! 
//! This module contains all the BUILDERS that create world data.
//! Following the builder pattern, each module provides factories that
//! produce the data structures defined in the world module.

// Public module exports
pub mod tectonics;
pub mod provinces;
pub mod rivers;
pub mod agriculture;
pub mod clouds;
pub mod utils;

// Import world data types that we build
use crate::world::{World, RiverSystem, CloudSystem, CloudData, CloudLayer};

// Re-export the main builder is not needed since it's defined below

use noise::Perlin;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::resources::{WorldSize, MapDimensions, MapBounds};
use crate::constants::*;

/// World builder that orchestrates all generation steps
/// 
/// This follows the builder pattern to construct a complete World.
pub struct WorldBuilder {
    seed: u32,
    size: WorldSize,
    perlin: Perlin,
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
        let perlin = Perlin::new(seed);
        let rng = StdRng::seed_from_u64(seed as u64);
        let dimensions = MapDimensions::from_world_size(&size);
        
        Self {
            seed,
            size,
            perlin,
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
        
        // Step 1: Generate tectonic plates and continent centers
        println!("\n[1/6] Generating tectonic plates ({} continents)...", self.continent_count);
        let tectonics = tectonics::TectonicsBuilder::new(
            &mut self.rng, 
            self.dimensions.bounds, 
            self.seed,
        )
        .with_continent_count(self.continent_count)
        .build();
        
        // Step 2: Generate provinces with terrain
        println!("\n[2/6] Generating provinces and terrain ({:.0}% ocean)...", self.ocean_coverage * 100.0);
        let mut provinces = provinces::ProvinceBuilder::new(
            &tectonics, 
            self.dimensions, 
            &self.perlin, 
            &mut self.rng,
        )
        .with_ocean_coverage(self.ocean_coverage)
        .build();
        
        // Step 3: Calculate ocean depths  
        println!("\n[3/6] Calculating ocean depths...");
        provinces::calculate_ocean_depths(&mut provinces, self.dimensions);
        
        // Step 4: Generate river systems
        println!("\n[4/6] Generating river systems (density: {:.1}x)...", self.river_density);
        let rivers = rivers::RiverBuilder::new(
            &mut provinces, 
            self.dimensions, 
            &mut self.rng,
        )
        .with_density(self.river_density)
        .build();
        
        // Step 5: Calculate agriculture and fresh water
        println!("\n[5/6] Calculating agriculture and fresh water...");
        agriculture::calculate(&mut provinces, &rivers, self.dimensions);
        
        // Step 6: Generate mineral resources using tectonic data for placement
        println!("\n[6/6] Generating mineral resources...");
        crate::minerals::generate_world_minerals_with_tectonics(
            self.seed, 
            &mut provinces,
            &tectonics
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