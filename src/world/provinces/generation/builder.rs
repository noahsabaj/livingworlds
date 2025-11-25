//! Province builder - orchestrates the generation pipeline
//!
//! This is the main entry point for province generation, coordinating all the
//! specialized generation modules to produce the final province map.

use log::info;
use rand::rngs::StdRng;

use crate::parallel::parallel_enumerate;
use crate::resources::MapDimensions;
use crate::world::generation::GenerationUtils;
use crate::world::gpu::extract_province_positions;
use crate::world::provinces::{
    Province, ProvinceId, Elevation, Agriculture, Distance, Abundance,
    ProvinceBundle, ProvinceData, ProvinceMarker, ProvinceNeighbors,
};
use crate::world::terrain::TerrainType;

use super::continents::ContinentGenerator;
use super::ocean_systems::OceanManager;
use super::terrain_classifier::TerrainClassifier;
use super::island_filter::IslandFilter;
use super::climate_effects::ClimateProcessor;
use super::neighbor_calculator::{NeighborCalculator, precompute_neighbor_indices};
use super::gpu_accelerator::GpuAccelerator;

/// Default ocean coverage percentage (0.0 to 1.0)
const DEFAULT_OCEAN_COVERAGE: f32 = 0.6;

/// Province builder that orchestrates the generation pipeline
pub struct ProvinceBuilder<'a> {
    utils: GenerationUtils,
    dimensions: MapDimensions,
    rng: &'a mut StdRng,
    seed: u32,
    ocean_coverage: f32,
    continent_count: u32,
}

impl<'a> ProvinceBuilder<'a> {
    pub fn new(dimensions: MapDimensions, rng: &'a mut StdRng, seed: u32) -> Self {
        let utils = GenerationUtils::new(dimensions);

        Self {
            utils,
            dimensions,
            rng,
            seed,
            ocean_coverage: DEFAULT_OCEAN_COVERAGE,
            continent_count: 7,
        }
    }

    pub fn with_ocean_coverage(mut self, coverage: f32) -> Self {
        self.ocean_coverage = coverage.clamp(0.1, 0.9);
        self
    }

    pub fn with_continent_count(mut self, count: u32) -> Self {
        self.continent_count = count.max(1);
        self
    }

    pub fn build(self) -> Vec<Province> {
        let total_provinces = self.utils.total_provinces();
        info!("  Generating {} hexagonal provinces", total_provinces);

        // Step 1: Generate continent seeds
        let continent_gen = ContinentGenerator::new(self.dimensions, self.continent_count, self.seed);
        let continent_seeds = continent_gen.generate_seeds(self.rng);
        info!("  Generated {} continent seeds", continent_seeds.len());

        // Step 2: Calculate sea level for target ocean coverage
        let ocean_manager = OceanManager::new(self.ocean_coverage, self.dimensions, self.seed);
        let sea_level = ocean_manager.calculate_sea_level(self.rng, self.continent_count);
        info!(
            "  Sea level set to {:.3} for {:.0}% ocean coverage",
            sea_level,
            self.ocean_coverage * 100.0
        );

        // Step 3: Generate provinces with elevation and terrain
        let mut provinces = self.generate_provinces_accelerated(total_provinces, sea_level);

        // Step 4: Filter out small islands
        let island_filter = IslandFilter::new();
        let islands_removed = island_filter.filter(&mut provinces);
        if islands_removed > 0 {
            info!("  Removed {} small island provinces", islands_removed);
        }

        // Step 5: Precompute neighbor indices for O(1) access
        precompute_neighbor_indices(&mut provinces);

        // Step 6: Apply climate effects (rain shadow)
        ClimateProcessor::apply_rain_shadow(&mut provinces);
        info!("  Applied rain shadow effect for desert placement");

        // Log final statistics
        let ocean_count = provinces
            .iter()
            .filter(|p| p.terrain == TerrainType::Ocean)
            .count();
        let land_count = provinces.len() - ocean_count;
        info!(
            "  Generated {} land provinces, {} ocean provinces",
            land_count, ocean_count
        );

        provinces
    }

    /// Generate provinces with GPU/parallel acceleration
    fn generate_provinces_accelerated(
        &self,
        total_provinces: u32,
        sea_level: f32,
    ) -> Vec<Province> {
        // Extract all province positions
        let positions = extract_province_positions(
            self.dimensions.provinces_per_row,
            self.dimensions.provinces_per_col,
            self.dimensions.hex_size,
        );

        // Use GPU acceleration (or parallel CPU) for elevation generation
        let gpu_accel = GpuAccelerator::new(self.dimensions, self.seed);
        let elevations = gpu_accel.try_gpu_elevation_generation(&positions, self.continent_count);

        // Generate provinces from positions and elevations
        self.generate_provinces_from_elevations(positions, elevations, sea_level)
    }

    /// Generate provinces from pre-computed positions and elevations
    fn generate_provinces_from_elevations(
        &self,
        positions: Vec<bevy::prelude::Vec2>,
        elevations: Vec<f32>,
        sea_level: f32,
    ) -> Vec<Province> {
        assert_eq!(
            positions.len(),
            elevations.len(),
            "Positions and elevations must have same length"
        );

        // Combine positions and elevations for parallel processing
        let position_elevation_pairs: Vec<_> = positions
            .into_iter()
            .zip(elevations.into_iter())
            .collect();

        // Create neighbor calculator
        let neighbor_calc = NeighborCalculator::new(self.dimensions);

        // Generate provinces in parallel
        parallel_enumerate(
            &position_elevation_pairs,
            |index, (position, elevation)| {
                let province_id = ProvinceId::new(index as u32);
                let (col, row) = self.utils.id_to_grid_coords(province_id);

                // Determine terrain type
                let terrain = TerrainClassifier::classify_terrain(*elevation, sea_level);
                let neighbors = neighbor_calc.calculate_hex_neighbors(col as u32, row as u32);
                let neighbor_indices = self.utils.get_neighbor_indices(col, row);

                Province {
                    id: province_id,
                    position: *position,
                    owner_entity: None,
                    culture: None,
                    elevation: Elevation::new(*elevation),
                    terrain,
                    population: 0,
                    max_population: 1000,
                    agriculture: Agriculture::default(),
                    fresh_water_distance: Distance::default(),
                    iron: Abundance::default(),
                    copper: Abundance::default(),
                    tin: Abundance::default(),
                    gold: Abundance::default(),
                    coal: Abundance::default(),
                    stone: Abundance::default(),
                    gems: Abundance::default(),
                    neighbors,
                    neighbor_indices,
                    version: 0,
                    dirty: false,
                }
            },
            "province_generation",
        )
    }
}

// ================================================================================================
// BUNDLE GENERATION - Convert provinces to ECS bundles for spawn_batch
// ================================================================================================

/// Convert a slice of Province structs to ProvinceBundle for ECS spawning
///
/// This creates bundles without neighbor entity references - those must be set
/// in a second pass after all entities are spawned via `set_neighbor_entities`.
pub fn provinces_to_bundles(provinces: &[Province]) -> Vec<ProvinceBundle> {
    provinces
        .iter()
        .map(|province| ProvinceBundle {
            marker: ProvinceMarker,
            data: ProvinceData::from_province(province),
            neighbors: ProvinceNeighbors::default(), // Set in second pass
        })
        .collect()
}

/// Set neighbor entity references after all provinces are spawned
///
/// This is the second pass that populates ProvinceNeighbors with actual Entity references.
/// Must be called after spawn_batch completes and entities are available.
///
/// # Arguments
/// * `world` - Bevy World for entity access
/// * `entity_order` - Ordered list of province entities (index = ProvinceId)
/// * `provinces` - Original province data with neighbor_indices
pub fn set_neighbor_entities(
    world: &mut bevy::prelude::World,
    entity_order: &[bevy::prelude::Entity],
    provinces: &[Province],
) {
    use bevy::prelude::Entity;

    // Build lookup from entity to province index
    let entity_to_index: std::collections::HashMap<Entity, usize> = entity_order
        .iter()
        .enumerate()
        .map(|(idx, &entity)| (entity, idx))
        .collect();

    // Update each province's neighbor entities
    for (idx, province) in provinces.iter().enumerate() {
        if idx >= entity_order.len() {
            log::warn!("Province index {} out of bounds for entity_order", idx);
            continue;
        }

        let entity = entity_order[idx];

        // Convert neighbor indices to entities
        let neighbor_entities: [Option<Entity>; 6] = province
            .neighbor_indices
            .map(|opt_idx| opt_idx.and_then(|i| entity_order.get(i).copied()));

        // Update the ProvinceNeighbors component
        if let Some(mut neighbors) = world.get_mut::<ProvinceNeighbors>(entity) {
            neighbors.neighbors = neighbor_entities;
        }
    }

    info!(
        "Set neighbor entities for {} provinces",
        entity_order.len()
    );
}