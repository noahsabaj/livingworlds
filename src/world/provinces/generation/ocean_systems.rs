//! Ocean and sea level management
//!
//! Handles sea level calculation, ocean depth determination, and water-related
//! province properties.

use log::info;
use rand::Rng;
use std::collections::{HashMap, VecDeque};

use crate::resources::MapDimensions;
use crate::world::provinces::{Province, Elevation};
use crate::world::terrain::TerrainType;
use crate::world::generation::GenerationUtils;
use super::elevation_processor::ElevationProcessor;
use super::continents::ContinentGenerator;

/// Number of sample points for sea level calculation
const SAMPLE_COUNT: usize = 4000;

/// Manages ocean-related calculations and properties
pub struct OceanManager {
    ocean_coverage: f32,
    dimensions: MapDimensions,
    seed: u32,
}

impl OceanManager {
    pub fn new(ocean_coverage: f32, dimensions: MapDimensions, seed: u32) -> Self {
        Self {
            ocean_coverage,
            dimensions,
            seed,
        }
    }

    /// Calculate sea level for target ocean coverage using adaptive sampling
    pub fn calculate_sea_level<R: Rng>(
        &self,
        rng: &mut R,
        continent_count: u32,
    ) -> f32 {
        info!("  Calculating sea level for {:.0}% ocean coverage", self.ocean_coverage * 100.0);

        // Generate continent seeds for sampling
        let continent_gen = ContinentGenerator::new(self.dimensions, continent_count, self.seed);
        let continent_seeds = continent_gen.generate_seeds_for_gpu();

        // Create utilities for random position generation
        let utils = GenerationUtils::new(self.dimensions);

        // Create elevation processor
        let elevation_processor = ElevationProcessor::new(self.seed, self.dimensions);

        // Generate sample elevations
        let mut elevations = Vec::with_capacity(SAMPLE_COUNT);
        for _ in 0..SAMPLE_COUNT {
            let position = utils.random_position(rng);
            let elevation = elevation_processor.generate_elevation(position, &continent_seeds);
            elevations.push(elevation);
        }

        // Apply the same adaptive redistribution that will be used for actual generation
        elevation_processor.apply_adaptive_redistribution(&mut elevations);

        // Find the percentile for ocean coverage
        elevations.sort_by(|a, b| a.total_cmp(b));
        let ocean_index = (self.ocean_coverage * SAMPLE_COUNT as f32) as usize;

        // The sea level is the elevation at the desired percentile
        let sea_level = elevations[ocean_index.min(SAMPLE_COUNT - 1)];

        info!("  Sea level set to {:.3}", sea_level);
        sea_level
    }
}

/// Calculate ocean depths for underwater provinces using BFS from coastlines
/// This is a public function exported from the module
pub fn calculate_ocean_depths(provinces: &mut [Province], _dimensions: MapDimensions) {
    info!("  Calculating ocean depths...");

    // Use BFS to calculate distance from land
    let mut ocean_distances: HashMap<u32, u32> = HashMap::new();
    let mut queue = VecDeque::new();

    // Initialize with coastal provinces - use precomputed indices!
    for province in provinces.iter() {
        if province.terrain != TerrainType::Ocean {
            // This is land, check neighbors for ocean using precomputed indices
            for &neighbor_idx_opt in &province.neighbor_indices {
                if let Some(neighbor_idx) = neighbor_idx_opt {
                    if provinces[neighbor_idx].terrain == TerrainType::Ocean {
                        let neighbor_id = provinces[neighbor_idx].id.value();
                        ocean_distances.insert(neighbor_id, 1);
                        queue.push_back((neighbor_idx, 1));
                    }
                }
            }
        }
    }

    // BFS to calculate distances using indices directly
    while let Some((current_idx, distance)) = queue.pop_front() {
        for &neighbor_idx_opt in &provinces[current_idx].neighbor_indices {
            if let Some(neighbor_idx) = neighbor_idx_opt {
                let neighbor_id = provinces[neighbor_idx].id.value();
                if provinces[neighbor_idx].terrain == TerrainType::Ocean
                    && !ocean_distances.contains_key(&neighbor_id)
                {
                    ocean_distances.insert(neighbor_id, distance + 1);
                    queue.push_back((neighbor_idx, distance + 1));
                }
            }
        }
    }

    // Apply depth based on distance from land
    for province in provinces.iter_mut() {
        if province.terrain == TerrainType::Ocean {
            let distance = ocean_distances
                .get(&province.id.value())
                .copied()
                .unwrap_or(100);

            // Adjust elevation based on distance from land
            if distance <= 2 {
                province.elevation = Elevation::new(0.12); // Shallow water
            } else if distance <= 5 {
                province.elevation = Elevation::new(0.07); // Continental shelf
            } else {
                province.elevation = Elevation::new(0.02); // Deep ocean
            }
        }
    }

    info!("  Ocean depths calculated");
}