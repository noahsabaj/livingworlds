//! Modular world generation system for Living Worlds
//! 
//! This module contains all world generation logic in a clean, organized structure.
//! Each submodule handles a specific aspect of world generation.

// Public module exports
pub mod types;
pub mod tectonics;
pub mod provinces;
pub mod rivers;
pub mod agriculture;
pub mod clouds;
pub mod utils;

// Re-export commonly used types
pub use types::{GeneratedWorld, RiverSystem, CloudSystem, MapDimensions, MapBounds, CloudData, CloudLayer};

use noise::Perlin;
use rand::{SeedableRng, rngs::StdRng};
use std::collections::HashMap;

use crate::resources::WorldSize;
use crate::constants::*;

/// Main world generator that orchestrates all generation steps
pub struct WorldGenerator {
    seed: u32,
    size: WorldSize,
    perlin: Perlin,
    rng: StdRng,
    dimensions: MapDimensions,
    continent_count: u32,
    ocean_coverage: f32,
    river_density: f32,
}

impl WorldGenerator {
    /// Create a new world generator with the given parameters
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
    
    /// Generate a complete world
    pub fn generate(mut self) -> GeneratedWorld {
        let start = std::time::Instant::now();
        println!("═══════════════════════════════════════════════════════");
        println!("Starting world generation with seed: {}", self.seed);
        println!("World size: {:?} ({} provinces)", self.size, 
                 self.dimensions.provinces_per_row * self.dimensions.provinces_per_col);
        println!("═══════════════════════════════════════════════════════");
        
        // Step 1: Generate tectonic plates and continent centers
        println!("\n[1/6] Generating tectonic plates ({} continents)...", self.continent_count);
        let tectonics = tectonics::generate_with_count(
            &mut self.rng, 
            self.dimensions.bounds, 
            self.seed,
            self.continent_count
        );
        
        // Step 2: Generate provinces with terrain
        println!("\n[2/6] Generating provinces and terrain ({:.0}% ocean)...", self.ocean_coverage * 100.0);
        let mut provinces = provinces::generate_with_ocean_coverage(
            &tectonics, 
            self.dimensions, 
            &self.perlin, 
            &mut self.rng,
            self.ocean_coverage
        );
        
        // Step 3: Calculate ocean depths  
        println!("\n[3/6] Calculating ocean depths...");
        provinces::calculate_ocean_depths(&mut provinces, self.dimensions);
        
        // Step 4: Generate river systems
        println!("\n[4/6] Generating river systems (density: {:.1}x)...", self.river_density);
        let rivers = rivers::generate_with_density(
            &mut provinces, 
            self.dimensions, 
            &mut self.rng,
            self.river_density
        );
        
        // Step 5: Calculate agriculture and fresh water
        println!("\n[5/6] Calculating agriculture and fresh water...");
        agriculture::calculate(&mut provinces, &rivers, self.dimensions);
        
        // Step 6: Generate mineral resources using tectonic data for placement
        println!("\n[6/6] Generating mineral resources...");
        let minerals = crate::minerals::generate_world_minerals_with_tectonics(
            self.seed, 
            &provinces,
            &tectonics
        );
        
        // Build spatial index for fast lookups
        let mut spatial_index = HashMap::new();
        let spatial_cell_size = self.dimensions.hex_size * SPATIAL_INDEX_CELL_SIZE_MULTIPLIER;
        
        for province in &provinces {
            let grid_x = (province.position.x / spatial_cell_size).floor() as i32;
            let grid_y = (province.position.y / spatial_cell_size).floor() as i32;
            spatial_index.insert((grid_x, grid_y), province.id);
        }
        
        // Generate cloud system
        let clouds = clouds::generate(&mut self.rng, &self.dimensions);
        
        let elapsed = start.elapsed();
        println!("\n═══════════════════════════════════════════════════════");
        println!("✓ World generation completed in {:.2}s", elapsed.as_secs_f32());
        println!("  {} provinces generated", provinces.len());
        println!("  {} river tiles", rivers.river_tiles.len());
        println!("  {} delta tiles", rivers.delta_tiles.len());
        println!("  {} clouds", clouds.clouds.len());
        println!("═══════════════════════════════════════════════════════");
        
        GeneratedWorld {
            provinces,
            rivers,
            minerals,
            spatial_index,
            map_dimensions: self.dimensions,
            clouds,
        }
    }
}