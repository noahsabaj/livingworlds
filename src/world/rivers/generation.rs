//! River system generation with flow accumulation

use bevy::log::info;
use bevy::prelude::Vec2;
use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use rayon::prelude::*;
use std::collections::HashSet;

use super::super::provinces::{Province, ProvinceId};
use super::super::terrain::TerrainType;
use super::RiverSystem;
use crate::constants::*;
use crate::math::get_neighbor_positions;
use crate::resources::MapDimensions;
use crate::world::generation::GenerationUtils;

// River generation constants
const DEFAULT_RIVER_DENSITY: f32 = 1.0;
const RIVER_SOURCE_DIVISOR: usize = 4; // Use 1/4 of potential sources
const MAX_RIVER_LENGTH: usize = 1000; // Maximum tiles a river can traverse
const FLOW_INCREMENT: f32 = 0.1; // Flow increase per tile downstream
const MIN_VISIBLE_FLOW: f32 = 0.3; // Minimum flow to show as river terrain (reduced for better visibility)
const MEANDER_CHANCE: f32 = 0.3; // Chance to pick non-optimal path for natural meandering

/// Error types for river generation
#[derive(Debug, Clone)]
pub enum RiverGenerationError {
    EmptyProvinces,
    InvalidDimensions(String),
    NoValidSources,
    NaNElevation(u32), // Province ID with NaN elevation
}

impl std::fmt::Display for RiverGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProvinces => write!(f, "Cannot generate rivers with empty provinces"),
            Self::InvalidDimensions(msg) => write!(f, "Invalid map dimensions: {}", msg),
            Self::NoValidSources => write!(f, "No valid river sources found"),
            Self::NaNElevation(id) => write!(f, "Province {} has NaN elevation", id),
        }
    }
}

impl std::error::Error for RiverGenerationError {}

/// Builder for generating river systems following the builder pattern
///
/// This builder encapsulates river generation logic with configurable density.
pub struct RiverBuilder<'a> {
    provinces: &'a mut [Province],
    utils: GenerationUtils,
    rng: &'a mut StdRng,
    river_density: f32,
    min_elevation: f32,
}

impl<'a> RiverBuilder<'a> {
    pub fn new(
        provinces: &'a mut [Province],
        dimensions: MapDimensions,
        rng: &'a mut StdRng,
    ) -> Self {
        // Create generation utilities for shared operations
        let utils = GenerationUtils::new(dimensions);

        Self {
            provinces,
            utils,
            rng,
            river_density: DEFAULT_RIVER_DENSITY,
            min_elevation: RIVER_MIN_ELEVATION,
        }
    }

    pub fn with_density(mut self, density: f32) -> Self {
        self.river_density = density.max(0.0);
        self
    }

    /// Set minimum elevation for river sources
    pub fn with_min_elevation(mut self, elevation: f32) -> Self {
        self.min_elevation = elevation;
        self
    }

    pub fn build(self) -> Result<RiverSystem, RiverGenerationError> {
        generate_rivers_internal(
            self.provinces,
            &self.utils,
            self.rng,
            self.river_density,
            self.min_elevation,
        )
    }
}

// Helper functions

/// Apply terrain type to a province if it's not ocean
fn apply_terrain_if_not_ocean(province: &mut Province, terrain: TerrainType) {
    if province.terrain != TerrainType::Ocean {
        province.terrain = terrain;
    }
}

/// Find potential river sources from mountains and high hills
fn find_river_sources(
    provinces: &[Province],
    min_elevation: f32,
) -> Result<Vec<(ProvinceId, Vec2, f32)>, RiverGenerationError> {
    // ENGINEERING FIX: Use percentile-based approach instead of terrain types
    // This adapts to any elevation distribution and guarantees river sources exist

    // First, collect all land provinces with their elevations
    let mut land_provinces: Vec<(ProvinceId, Vec2, f32)> = Vec::new();

    for province in provinces.iter() {
        if province.elevation.value().is_nan() {
            return Err(RiverGenerationError::NaNElevation(province.id.value()));
        }

        // Skip ocean and beach provinces
        if province.terrain != TerrainType::Ocean && province.terrain != TerrainType::Beach {
            land_provinces.push((province.id, province.position, province.elevation.value()));
        }
    }

    if land_provinces.is_empty() {
        return Err(RiverGenerationError::NoValidSources);
    }

    // Sort by elevation to find percentiles
    land_provinces.sort_by(|a, b| a.2.total_cmp(&b.2));

    // FLEXIBLE CRITERIA: Use top 25% of land elevations as potential river sources
    // This guarantees we always have sources regardless of elevation compression
    let percentile_75_index = (land_provinces.len() * 3) / 4;  // Top 25%

    // Also apply the min_elevation as a secondary filter (but much lower now)
    let elevation_threshold = land_provinces[percentile_75_index].2.max(min_elevation);

    // Collect all provinces above the threshold
    let mut potential_sources: Vec<(ProvinceId, Vec2, f32)> = land_provinces
        .into_iter()
        .filter(|&(_, _, elev)| elev >= elevation_threshold)
        .collect();

    if potential_sources.is_empty() {
        return Err(RiverGenerationError::NoValidSources);
    }

    // Sort by elevation (highest first) for better river flow
    potential_sources.sort_by(|a, b| b.2.total_cmp(&a.2));

    Ok(potential_sources)
}

/// Trace a single river from source to ocean using shared utilities
fn trace_river_path(
    source_id: ProvinceId,
    provinces: &[Province],
    utils: &GenerationUtils,
    rng_seed: u64,
) -> (Vec<ProvinceId>, Vec<ProvinceId>, f32) {
    // Create thread-local RNG from seed
    use rand::SeedableRng;
    let mut rng = StdRng::seed_from_u64(rng_seed.wrapping_add(source_id.value() as u64));
    let mut current_id = source_id;
    let mut river_path = vec![source_id];
    let mut delta_tiles = Vec::new();
    let mut visited = HashSet::new();

    let (start_col, start_row) = utils.id_to_grid_coords(source_id);
    visited.insert((start_col, start_row));

    let mut flow = 1.0;

    for _ in 0..MAX_RIVER_LENGTH {
        let (current_col, current_row) = utils.id_to_grid_coords(current_id);
        let neighbors =
            get_neighbor_positions(current_col, current_row, utils.dimensions().hex_size);

        // Collect valid unvisited neighbors using shared utilities
        let mut valid_neighbors: Vec<(&Province, (i32, i32))> = Vec::new();

        for (neighbor_col, neighbor_row) in neighbors {
            if let Some(province) =
                utils.get_province_at_grid(provinces, neighbor_col, neighbor_row)
            {
                if visited.contains(&(neighbor_col, neighbor_row)) {
                    continue;
                }

                if province.terrain == TerrainType::Ocean {
                    delta_tiles.push(province.id);
                    if let Some(&last_province) = river_path.last() {
                        delta_tiles.push(last_province);
                    }
                    return (river_path, delta_tiles, flow);
                }

                valid_neighbors.push((province, (neighbor_col, neighbor_row)));
            }
        }

        if valid_neighbors.is_empty() {
            break; // No valid path forward
        }

        // Sort neighbors by elevation (lowest first)
        // Use total_cmp to handle NaN values deterministically
        valid_neighbors.sort_by(|a, b| a.0.elevation.value().total_cmp(&b.0.elevation.value()));

        // Add randomization for natural meandering
        let next = if !valid_neighbors.is_empty()
            && rng.r#gen::<f32>() < MEANDER_CHANCE
            && valid_neighbors.len() > 1
        {
            // Sometimes pick a suboptimal path for more natural rivers
            let idx = rng.gen_range(0..valid_neighbors.len().min(2));
            &valid_neighbors[idx]
        } else {
            // Usually pick the lowest neighbor
            &valid_neighbors[0]
        };

        river_path.push(next.0.id);
        current_id = next.0.id;
        visited.insert(next.1);
        flow += FLOW_INCREMENT;
    }

    (river_path, delta_tiles, flow)
}

/// Apply terrain changes based on river flow using shared utilities
fn apply_terrain_changes(
    provinces: &mut [Province],
    river_tiles: &[ProvinceId],
    delta_tiles: &[ProvinceId],
    flow_accumulation: &[f32], // Now a Vec indexed by province ID
    utils: &GenerationUtils,
) {
    // First pass: Apply base river terrain using direct indexing
    for (province_idx, flow) in flow_accumulation.iter().enumerate() {
        if *flow > MIN_VISIBLE_FLOW {
            // Direct array access - no HashMap needed!
            apply_terrain_if_not_ocean(&mut provinces[province_idx], TerrainType::River);
        }
    }

    // Second pass: No widening - single-tile rivers look best on hex grids
    // Previous attempts at widening created visual artifacts due to hex grid offset patterns

    // Generate branching deltas using shared utilities
    generate_delta_branches(delta_tiles, provinces, flow_accumulation, utils);
}

/// Internal river generation implementation
fn generate_rivers_internal(
    provinces: &mut [Province],
    utils: &GenerationUtils,
    rng: &mut StdRng,
    river_density: f32,
    min_elevation: f32,
) -> Result<RiverSystem, RiverGenerationError> {
    let start = std::time::Instant::now();

    if provinces.is_empty() {
        return Err(RiverGenerationError::EmptyProvinces);
    }

    if utils.dimensions().provinces_per_row == 0 {
        return Err(RiverGenerationError::InvalidDimensions(
            "provinces_per_row cannot be zero".to_string(),
        ));
    }

    // No spatial index needed - we'll use direct array indexing

    let potential_sources = find_river_sources(provinces, min_elevation)?;

    info!("Found {} potential river sources", potential_sources.len());

    // Select river sources based on density with randomization
    let base_num_rivers = (RIVER_COUNT / RIVER_SOURCE_DIVISOR).min(potential_sources.len());
    let num_rivers =
        ((base_num_rivers as f32 * river_density) as usize).min(potential_sources.len());

    // Randomly select sources for variety
    let mut shuffled_sources = potential_sources;
    shuffled_sources.shuffle(rng);
    let selected_sources: Vec<_> = shuffled_sources.into_iter().take(num_rivers).collect();

    info!("Selected {} river sources to trace", selected_sources.len());

    let base_seed = rng.r#gen::<u64>();

    // Trace rivers in parallel for better performance using direct indexing
    let river_results: Vec<(Vec<ProvinceId>, Vec<ProvinceId>, Vec<(u32, f32)>)> = selected_sources
        .par_iter()
        .map(|(source_id, _source_pos, _elevation)| {
            let (river_path, delta_tiles, flow) =
                trace_river_path(*source_id, provinces, utils, base_seed);

            // Build flow list without HashMap overhead
            let mut river_flow_list = Vec::new();
            for &tile_id in &river_path {
                river_flow_list.push((tile_id.value(), flow));
            }

            (river_path, delta_tiles, river_flow_list)
        })
        .collect();

    // Merge results from parallel execution into Vec for O(1) access
    let mut all_river_tiles = Vec::new();
    let mut all_delta_tiles = Vec::new();
    let mut flow_accumulation_vec = vec![0.0; provinces.len()]; // Direct indexing!

    for (river_path, delta_tiles, river_flow_list) in river_results {
        all_river_tiles.extend(river_path);
        all_delta_tiles.extend(delta_tiles);

        // Accumulate flow using direct array indexing
        for (tile_id, flow) in river_flow_list {
            if (tile_id as usize) < provinces.len() {
                flow_accumulation_vec[tile_id as usize] += flow;
            }
        }
    }

    // Apply terrain changes based on flow using shared utilities
    apply_terrain_changes(
        provinces,
        &all_river_tiles,
        &all_delta_tiles,
        &flow_accumulation_vec,
        utils,
    );

    info!(
        "River generation completed in {:.2}s: {} river tiles, {} delta tiles",
        start.elapsed().as_secs_f32(),
        all_river_tiles.len(),
        all_delta_tiles.len()
    );

    // Flow accumulation is already a Vec - no conversion needed!

    Ok(RiverSystem {
        river_tiles: all_river_tiles.into_iter().map(|id| id.value()).collect(),
        delta_tiles: all_delta_tiles.into_iter().map(|id| id.value()).collect(),
        flow_accumulation: flow_accumulation_vec,
        flow_direction: vec![None; provinces.len()], // Initialize as empty - not calculated in this version
    })
}

/// Generate branching delta distributaries using shared utilities
fn generate_delta_branches(
    delta_tiles: &[ProvinceId],
    provinces: &mut [Province],
    flow_accumulation: &[f32], // Now a Vec indexed by province ID
    utils: &GenerationUtils,
) {
    for &delta_id in delta_tiles {
        let delta_idx = delta_id.value() as usize;

        // Bounds check and direct array access
        if delta_idx < provinces.len() && delta_idx < flow_accumulation.len() {
            // Get flow at delta point with direct indexing
            let flow = flow_accumulation[delta_idx].max(1.0);

            // Number of distributary channels based on flow
            let num_branches = if flow > 10.0 {
                3 // Major delta - 3 branches
            } else if flow > 5.0 {
                2 // Medium delta - 2 branches
            } else {
                1 // Small delta - single channel
            };

            // Create distributary branches
            create_delta_distributaries(delta_idx, num_branches, provinces, utils);
        }
    }
}

/// Create individual distributary channels for a delta using shared utilities
fn create_delta_distributaries(
    delta_idx: usize,
    num_branches: usize,
    provinces: &mut [Province],
    utils: &GenerationUtils,
) {
    // Extract needed values before mutation
    let delta_id = provinces[delta_idx].id;
    let delta_elevation = provinces[delta_idx].elevation.value();
    let (delta_col, delta_row) = utils.id_to_grid_coords(delta_id);

    // Apply River terrain to the delta tile (no more Delta type)
    apply_terrain_if_not_ocean(&mut provinces[delta_idx], TerrainType::River);

    // Get all neighbors
    let neighbors = get_neighbor_positions(delta_col, delta_row, 50.0);

    // Create branches radiating from delta
    let mut branch_count = 0;
    for (i, (neighbor_col, neighbor_row)) in neighbors.iter().enumerate() {
        if branch_count >= num_branches {
            break;
        }

        // Skip every other neighbor for spacing
        if i % 2 != 0 && num_branches < 3 {
            continue;
        }

        if *neighbor_col < 0 || *neighbor_row < 0 {
            continue;
        }
        let neighbor_id = (*neighbor_row as u32) * PROVINCES_PER_ROW + (*neighbor_col as u32);

        let neighbor_idx = neighbor_id as usize;
        if neighbor_idx < provinces.len() {
            // Extract needed values before potential mutation
            let neighbor_terrain = provinces[neighbor_idx].terrain;
            let neighbor_elevation = provinces[neighbor_idx].elevation.value();

            // Only create distributary if heading toward ocean
            if neighbor_terrain == TerrainType::Ocean || neighbor_elevation < delta_elevation {
                // Create a short distributary channel using shared utilities
                create_distributary_channel(
                    neighbor_idx,
                    3, // Max length of 3 tiles
                    provinces,
                    utils,
                );

                branch_count += 1;
            }
        }
    }
}

/// Create a single distributary channel using shared utilities
fn create_distributary_channel(
    start_idx: usize,
    max_length: usize,
    provinces: &mut [Province],
    utils: &GenerationUtils,
) {
    let mut current_idx = start_idx;
    let mut visited = HashSet::new();

    for _ in 0..max_length {
        // Extract needed values before mutation
        let current_terrain = provinces[current_idx].terrain;
        let current_id = provinces[current_idx].id;
        let current_elevation = provinces[current_idx].elevation.value();

        // Stop if we hit ocean
        if current_terrain == TerrainType::Ocean {
            break;
        }

        // Apply river terrain to create the distributary
        apply_terrain_if_not_ocean(&mut provinces[current_idx], TerrainType::River);
        visited.insert(current_id.value());

        // Find lowest neighbor to continue channel using shared utilities
        let (current_col, current_row) = utils.id_to_grid_coords(current_id);
        let neighbors =
            get_neighbor_positions(current_col, current_row, utils.dimensions().hex_size);

        let mut lowest_neighbor: Option<usize> = None;
        let mut lowest_elevation = current_elevation;

        for (neighbor_col, neighbor_row) in neighbors {
            if neighbor_col < 0 || neighbor_row < 0 {
                continue;
            }
            let neighbor_id = (neighbor_row as u32) * PROVINCES_PER_ROW + (neighbor_col as u32);

            if visited.contains(&neighbor_id) {
                continue;
            }

            let neighbor_idx = neighbor_id as usize;
            if neighbor_idx < provinces.len() {
                let neighbor_province = &provinces[neighbor_idx];

                // Prefer ocean tiles, then lower elevation
                if neighbor_province.terrain == TerrainType::Ocean {
                    lowest_neighbor = Some(neighbor_idx);
                    break;
                } else if neighbor_province.elevation.value() < lowest_elevation {
                    lowest_elevation = neighbor_province.elevation.value();
                    lowest_neighbor = Some(neighbor_idx);
                }
            }
        }

        // Continue to the lowest neighbor if found
        if let Some(next_idx) = lowest_neighbor {
            current_idx = next_idx;
        } else {
            break;
        }
    }
}
