//! River system generation with flow accumulation

use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, seq::SliceRandom};
use rayon::prelude::*;

use crate::components::{Province, ProvinceId};
use crate::world::terrain::TerrainType;
use crate::constants::*;
use crate::resources::MapDimensions;
use crate::world::RiverSystem;
use super::utils::hex_neighbors;

// River generation constants
const DEFAULT_RIVER_DENSITY: f32 = 1.0;
const RIVER_SOURCE_DIVISOR: usize = 4;  // Use 1/4 of potential sources
const MAX_RIVER_LENGTH: usize = 1000;   // Maximum tiles a river can traverse
const FLOW_INCREMENT: f32 = 0.1;        // Flow increase per tile downstream
const MIN_VISIBLE_FLOW: f32 = 0.5;      // Minimum flow to show as river terrain
const MEANDER_CHANCE: f32 = 0.3;        // Chance to pick non-optimal path for natural meandering

/// Error types for river generation
#[derive(Debug, Clone)]
pub enum RiverGenerationError {
    EmptyProvinces,
    InvalidDimensions(String),
    NoValidSources,
    NaNElevation(u32),  // Province ID with NaN elevation
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
    dimensions: MapDimensions,
    rng: &'a mut StdRng,
    river_density: f32,
    min_elevation: f32,
}

impl<'a> RiverBuilder<'a> {
    /// Create a new river builder
    pub fn new(
        provinces: &'a mut [Province],
        dimensions: MapDimensions,
        rng: &'a mut StdRng,
    ) -> Self {
        Self {
            provinces,
            dimensions,
            rng,
            river_density: DEFAULT_RIVER_DENSITY,
            min_elevation: RIVER_MIN_ELEVATION,
        }
    }
    
    /// Set the river density multiplier (0.5 = half rivers, 2.0 = double rivers)
    pub fn with_density(mut self, density: f32) -> Self {
        self.river_density = density.max(0.0);
        self
    }
    
    /// Set minimum elevation for river sources
    pub fn with_min_elevation(mut self, elevation: f32) -> Self {
        self.min_elevation = elevation;
        self
    }
    
    /// Build the river system
    pub fn build(self) -> Result<RiverSystem, RiverGenerationError> {
        generate_rivers_internal(
            self.provinces,
            self.dimensions,
            self.rng,
            self.river_density,
            self.min_elevation,
        )
    }
}

// Helper functions to remove DRY violations

/// Convert province ID to grid coordinates
fn id_to_grid_coords(id: ProvinceId, provinces_per_row: u32) -> (i32, i32) {
    let col = (id.value() % provinces_per_row) as i32;
    let row = (id.value() / provinces_per_row) as i32;
    (col, row)
}

/// Apply terrain type to a province if it's not ocean
fn apply_terrain_if_not_ocean(province: &mut Province, terrain: TerrainType) {
    if province.terrain != TerrainType::Ocean {
        province.terrain = terrain;
    }
}

/// Build spatial index for O(1) province lookups by grid position
fn build_spatial_index<'a>(provinces: &'a [Province], dimensions: &MapDimensions) -> HashMap<(i32, i32), &'a Province> {
    let mut position_to_province = HashMap::new();
    for province in provinces.iter() {
        let (col, row) = id_to_grid_coords(province.id, dimensions.provinces_per_row);
        position_to_province.insert((col, row), province);
    }
    position_to_province
}

/// Find potential river sources from mountains and high hills
fn find_river_sources(
    provinces: &[Province],
    min_elevation: f32,
) -> Result<Vec<(ProvinceId, Vec2, f32)>, RiverGenerationError> {
    let mut potential_sources = Vec::new();
    
    for province in provinces.iter() {
        // Check for NaN elevations
        if province.elevation.value().is_nan() {
            return Err(RiverGenerationError::NaNElevation(province.id.value()));
        }
        
        // Alpine terrain and other high-elevation areas can be river sources
        if province.terrain == TerrainType::Alpine ||
            (province.elevation.value() >= min_elevation &&
             matches!(province.terrain, TerrainType::Chaparral | TerrainType::TemperateGrassland)) {
            potential_sources.push((province.id, province.position, province.elevation.value()));
        }
    }
    
    if potential_sources.is_empty() {
        return Err(RiverGenerationError::NoValidSources);
    }
    
    // Sort by elevation (highest first) for better river flow
    potential_sources.sort_by(|a, b| {
        b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    Ok(potential_sources)
}

/// Trace a single river from source to ocean
fn trace_river_path(
    source_id: ProvinceId,
    position_to_province: &HashMap<(i32, i32), &Province>,
    dimensions: &MapDimensions,
    rng_seed: u64,
) -> (Vec<ProvinceId>, Vec<ProvinceId>, f32) {
    // Create thread-local RNG from seed
    use rand::SeedableRng;
    let mut rng = StdRng::seed_from_u64(rng_seed.wrapping_add(source_id.value() as u64));
    let mut current_id = source_id;
    let mut river_path = vec![source_id];
    let mut delta_tiles = Vec::new();
    let mut visited = HashSet::new();
    
    let (start_col, start_row) = id_to_grid_coords(source_id, dimensions.provinces_per_row);
    visited.insert((start_col, start_row));
    
    let mut flow = 1.0;
    
    for _ in 0..MAX_RIVER_LENGTH {
        let (current_col, current_row) = id_to_grid_coords(current_id, dimensions.provinces_per_row);
        let neighbors = hex_neighbors(current_col, current_row);
        
        // Collect valid unvisited neighbors
        let mut valid_neighbors: Vec<(&Province, (i32, i32))> = Vec::new();
        
        for (neighbor_col, neighbor_row) in neighbors {
            if let Some(province) = position_to_province.get(&(neighbor_col, neighbor_row)) {
                if visited.contains(&(neighbor_col, neighbor_row)) {
                    continue;
                }
                
                // Check if we reached ocean
                if province.terrain == TerrainType::Ocean {
                    delta_tiles.push(province.id);
                    if !river_path.is_empty() {
                        delta_tiles.push(*river_path.last().unwrap());
                    }
                    return (river_path, delta_tiles, flow);
                }
                
                valid_neighbors.push((province, (neighbor_col, neighbor_row)));
            }
        }
        
        if valid_neighbors.is_empty() {
            break;  // No valid path forward
        }
        
        // Sort neighbors by elevation (lowest first)
        valid_neighbors.sort_by(|a, b| {
            a.0.elevation.value()
                .partial_cmp(&b.0.elevation.value())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // Add randomization for natural meandering
        let next = if !valid_neighbors.is_empty() && rng.gen::<f32>() < MEANDER_CHANCE && valid_neighbors.len() > 1 {
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

/// Apply terrain changes based on river flow
fn apply_terrain_changes(
    provinces: &mut [Province],
    river_tiles: &[ProvinceId],
    delta_tiles: &[ProvinceId],
    flow_accumulation: &HashMap<u32, f32>,
) {
    // Build HashMap for O(1) province lookups by ID
    let mut province_id_to_idx: HashMap<u32, usize> = HashMap::new();
    for (idx, province) in provinces.iter().enumerate() {
        province_id_to_idx.insert(province.id.value(), idx);
    }
    
    // Convert high-flow tiles to river terrain
    for (province_id, flow) in flow_accumulation {
        if *flow > MIN_VISIBLE_FLOW {
            if let Some(&idx) = province_id_to_idx.get(province_id) {
                apply_terrain_if_not_ocean(&mut provinces[idx], TerrainType::River);
            }
        }
    }
    
    // Convert delta tiles to delta terrain
    for &delta_id in delta_tiles {
        if let Some(&idx) = province_id_to_idx.get(&delta_id.value()) {
            apply_terrain_if_not_ocean(&mut provinces[idx], TerrainType::Delta);
        }
    }
}

/// Internal river generation implementation
fn generate_rivers_internal(
    provinces: &mut [Province],
    dimensions: MapDimensions,
    rng: &mut StdRng,
    river_density: f32,
    min_elevation: f32,
) -> Result<RiverSystem, RiverGenerationError> {
    let start = std::time::Instant::now();
    
    // Input validation
    if provinces.is_empty() {
        return Err(RiverGenerationError::EmptyProvinces);
    }
    
    if dimensions.provinces_per_row == 0 {
        return Err(RiverGenerationError::InvalidDimensions(
            "provinces_per_row cannot be zero".to_string()
        ));
    }
    
    // Build spatial index for province lookups
    let position_to_province = build_spatial_index(provinces, &dimensions);
    
    // Find potential river sources
    let potential_sources = find_river_sources(provinces, min_elevation)?;
    
    info!("Found {} potential river sources", potential_sources.len());
    
    // Select river sources based on density with randomization
    let base_num_rivers = (RIVER_COUNT / RIVER_SOURCE_DIVISOR).min(potential_sources.len());
    let num_rivers = ((base_num_rivers as f32 * river_density) as usize).min(potential_sources.len());
    
    // Randomly select sources for variety
    let mut shuffled_sources = potential_sources;
    shuffled_sources.shuffle(rng);
    let selected_sources: Vec<_> = shuffled_sources.into_iter().take(num_rivers).collect();
    
    info!("Selected {} river sources to trace", selected_sources.len());
    
    // Get RNG seed for thread-local RNGs
    let base_seed = rng.gen::<u64>();
    
    // Trace rivers in parallel for better performance
    let river_results: Vec<(Vec<ProvinceId>, Vec<ProvinceId>, HashMap<u32, f32>)> = selected_sources
        .par_iter()
        .map(|(source_id, _source_pos, _elevation)| {
            let (river_path, delta_tiles, flow) = trace_river_path(
                *source_id,
                &position_to_province,
                &dimensions,
                base_seed,
            );
            
            // Build flow map for this river
            let mut river_flow_map = HashMap::new();
            for &tile_id in &river_path {
                *river_flow_map.entry(tile_id.value()).or_insert(0.0) += flow;
            }
            
            (river_path, delta_tiles, river_flow_map)
        })
        .collect();
    
    // Merge results from parallel execution
    let mut all_river_tiles = Vec::new();
    let mut all_delta_tiles = Vec::new();
    let mut flow_accumulation: HashMap<u32, f32> = HashMap::new();
    
    for (river_path, delta_tiles, river_flow) in river_results {
        all_river_tiles.extend(river_path);
        all_delta_tiles.extend(delta_tiles);
        
        // Merge flow accumulation maps
        for (tile_id, flow) in river_flow {
            *flow_accumulation.entry(tile_id).or_insert(0.0) += flow;
        }
    }
    
    // Apply terrain changes based on flow
    apply_terrain_changes(provinces, &all_river_tiles, &all_delta_tiles, &flow_accumulation);
    
    info!(
        "River generation completed in {:.2}s: {} river tiles, {} delta tiles",
        start.elapsed().as_secs_f32(),
        all_river_tiles.len(),
        all_delta_tiles.len()
    );
    
    Ok(RiverSystem {
        river_tiles: all_river_tiles.into_iter().map(|id| id.value()).collect(),
        delta_tiles: all_delta_tiles.into_iter().map(|id| id.value()).collect(),
        flow_accumulation,
    })
}