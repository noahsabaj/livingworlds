//! Agriculture and fresh water distance calculation

use bevy::log::info;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

use super::super::rivers::RiverSystem;
use super::super::terrain::TerrainType;
use super::{Agriculture, Distance, Province, ProvinceId};
use crate::resources::MapDimensions;

// Agriculture base values by terrain type
const RIVER_BASE_AGRICULTURE: f32 = 2.5; // River tiles are very fertile
const PLAINS_BASE_AGRICULTURE: f32 = 1.0; // Good farmland
const FOREST_BASE_AGRICULTURE: f32 = 0.8; // Can be cleared for farming
const HILLS_BASE_AGRICULTURE: f32 = 0.5; // Terraced farming possible
const DESERT_BASE_AGRICULTURE: f32 = 0.2; // Limited oasis farming
const TUNDRA_BASE_AGRICULTURE: f32 = 0.1; // Very limited growing season
const MOUNTAINS_BASE_AGRICULTURE: f32 = 0.05; // Minimal agriculture
const OCEAN_BASE_AGRICULTURE: f32 = 0.0; // No agriculture in ocean
const DEFAULT_BASE_AGRICULTURE: f32 = 0.3; // Default for unknown terrain

// Water proximity bonuses (distance in tiles -> multiplier)
const WATER_DISTANCE_BONUSES: [(f32, f32); 5] = [
    (0.0, 1.0),  // On water source (no bonus, already in terrain)
    (1.0, 1.3),  // Adjacent to water
    (3.0, 1.1),  // Near water
    (5.0, 1.05), // Moderate distance
    (10.0, 1.0), // Far from water (no bonus)
];

// Threshold for "high agriculture" provinces
const HIGH_AGRICULTURE_THRESHOLD: f32 = 1.5;

// Maximum distance to calculate from water (for performance)
const MAX_WATER_DISTANCE: f32 = 20.0;

/// Error types for agriculture calculation
#[derive(Debug, Clone)]
pub enum AgricultureError {
    EmptyProvinces,
    InvalidDimensions(String),
    InvalidProvinceId(u32),
}

impl std::fmt::Display for AgricultureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProvinces => write!(f, "Cannot calculate agriculture with empty provinces"),
            Self::InvalidDimensions(msg) => write!(f, "Invalid map dimensions: {}", msg),
            Self::InvalidProvinceId(id) => write!(f, "Invalid province ID: {}", id),
        }
    }
}

impl std::error::Error for AgricultureError {}

/// Calculate agriculture and fresh water distance for all provinces
pub fn calculate(
    provinces: &mut [Province],
    river_system: &RiverSystem,
    dimensions: MapDimensions,
) -> Result<(), AgricultureError> {
    if provinces.is_empty() {
        return Err(AgricultureError::EmptyProvinces);
    }

    if dimensions.provinces_per_row == 0 || dimensions.provinces_per_col == 0 {
        return Err(AgricultureError::InvalidDimensions(
            "provinces_per_row and provinces_per_col must be non-zero".to_string(),
        ));
    }

    let river_set: HashSet<ProvinceId> = river_system
        .river_tiles
        .iter()
        .map(|&id| ProvinceId::new(id))
        .collect();
    let delta_set: HashSet<ProvinceId> = river_system
        .delta_tiles
        .iter()
        .map(|&id| ProvinceId::new(id))
        .collect();

    let water_distances =
        calculate_water_distances(provinces, &river_set, &delta_set, &dimensions)?;

    // Apply agriculture values in parallel
    let agriculture_results: Vec<(usize, Agriculture, Distance)> = provinces
        .par_iter()
        .enumerate()
        .map(|(idx, province)| {
            let base_agriculture = get_base_agriculture(province.terrain);
            let water_dist = water_distances[idx].unwrap_or(MAX_WATER_DISTANCE);
            let water_bonus = calculate_water_bonus(province, &river_set, &delta_set, water_dist);

            let agriculture = Agriculture::new(base_agriculture * water_bonus);
            let fresh_water_distance = Distance::new(water_dist);

            (idx, agriculture, fresh_water_distance)
        })
        .collect();

    // Apply results back to provinces
    let mut high_agriculture_count = 0;
    for (idx, agriculture, fresh_water_distance) in agriculture_results {
        provinces[idx].agriculture = agriculture;
        provinces[idx].fresh_water_distance = fresh_water_distance;

        if agriculture.value() >= HIGH_AGRICULTURE_THRESHOLD {
            high_agriculture_count += 1;
        }
    }

    info!(
        "Agriculture calculation complete: {} provinces with high agriculture (>= {})",
        high_agriculture_count, HIGH_AGRICULTURE_THRESHOLD
    );

    Ok(())
}

/// Get base agriculture value for a terrain type - uses centralized properties
fn get_base_agriculture(terrain: TerrainType) -> f32 {
    terrain.properties().agriculture_base
}

/// Calculate water bonus based on proximity to water sources
fn calculate_water_bonus(
    province: &Province,
    river_set: &HashSet<ProvinceId>,
    delta_set: &HashSet<ProvinceId>,
    water_distance: f32,
) -> f32 {
    // Special handling for water terrain - they don't get extra bonus
    if province.terrain == TerrainType::River || province.terrain == TerrainType::Ocean {
        return 1.0; // Base agriculture already accounts for water
    }

    if river_set.contains(&province.id) || delta_set.contains(&province.id) {
        return 1.0; // Already accounted in terrain type
    }

    // Apply distance-based bonus
    for &(max_dist, bonus) in WATER_DISTANCE_BONUSES.iter() {
        if water_distance <= max_dist {
            return bonus;
        }
    }

    1.0 // No bonus if too far from water
}

/// Calculate actual distances from water sources using BFS
fn calculate_water_distances(
    provinces: &[Province],
    river_set: &HashSet<ProvinceId>,
    delta_set: &HashSet<ProvinceId>,
    _dimensions: &MapDimensions,
) -> Result<Vec<Option<f32>>, AgricultureError> {
    // Use Vec for O(1) indexed access instead of HashMap!
    let mut distances: Vec<Option<f32>> = vec![None; provinces.len()];
    let mut queue = VecDeque::new();

    // Add all water sources to queue with distance 0
    for (idx, province) in provinces.iter().enumerate() {
        if province.terrain == TerrainType::Ocean
            || province.terrain == TerrainType::River
            || river_set.contains(&province.id)
            || delta_set.contains(&province.id)
        {
            queue.push_back((idx, 0.0));
            distances[idx] = Some(0.0);
        }
    }

    // BFS using direct array indexing - no HashMap operations!
    while let Some((current_idx, current_dist)) = queue.pop_front() {
        // Stop if we've reached maximum distance
        if current_dist >= MAX_WATER_DISTANCE {
            continue;
        }

        let current_province = &provinces[current_idx];

        // Use precomputed neighbor indices for O(1) access
        for &neighbor_idx_opt in &current_province.neighbor_indices {
            if let Some(neighbor_idx) = neighbor_idx_opt {
                // Direct array check - no hashing!
                if distances[neighbor_idx].is_none() {
                    let new_dist = current_dist + 1.0;
                    distances[neighbor_idx] = Some(new_dist);
                    queue.push_back((neighbor_idx, new_dist));
                }
            }
        }
    }

    Ok(distances)
}
