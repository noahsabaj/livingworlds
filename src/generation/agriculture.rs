//! Agriculture and fresh water distance calculation

use std::collections::{HashMap, HashSet, VecDeque};
use bevy::prelude::*;
use rayon::prelude::*;

use crate::components::{Province, ProvinceId, Agriculture, Distance};
use crate::world::terrain::TerrainType;
use crate::resources::MapDimensions;
use crate::world::RiverSystem;
use super::utils::hex_neighbors;

// Agriculture base values by terrain type
const DELTA_BASE_AGRICULTURE: f32 = 3.0;     // River deltas are extremely fertile
const RIVER_BASE_AGRICULTURE: f32 = 2.5;     // River tiles are very fertile  
const PLAINS_BASE_AGRICULTURE: f32 = 1.0;    // Good farmland
const FOREST_BASE_AGRICULTURE: f32 = 0.8;    // Can be cleared for farming
const HILLS_BASE_AGRICULTURE: f32 = 0.5;     // Terraced farming possible
const DESERT_BASE_AGRICULTURE: f32 = 0.2;    // Limited oasis farming
const TUNDRA_BASE_AGRICULTURE: f32 = 0.1;    // Very limited growing season
const MOUNTAINS_BASE_AGRICULTURE: f32 = 0.05; // Minimal agriculture
const OCEAN_BASE_AGRICULTURE: f32 = 0.0;     // No agriculture in ocean
const DEFAULT_BASE_AGRICULTURE: f32 = 0.3;   // Default for unknown terrain

// Water proximity bonuses (distance in tiles -> multiplier)
const WATER_DISTANCE_BONUSES: [(f32, f32); 5] = [
    (0.0, 1.0),   // On water source (no bonus, already in terrain)
    (1.0, 1.3),   // Adjacent to water
    (3.0, 1.1),   // Near water
    (5.0, 1.05),  // Moderate distance
    (10.0, 1.0),  // Far from water (no bonus)
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
    // Input validation
    if provinces.is_empty() {
        return Err(AgricultureError::EmptyProvinces);
    }
    
    if dimensions.provinces_per_row == 0 || dimensions.provinces_per_col == 0 {
        return Err(AgricultureError::InvalidDimensions(
            "provinces_per_row and provinces_per_col must be non-zero".to_string()
        ));
    }
    
    // Build water source sets for quick lookups
    let river_set: HashSet<ProvinceId> = river_system.river_tiles
        .iter()
        .map(|&id| ProvinceId::new(id))
        .collect();
    let delta_set: HashSet<ProvinceId> = river_system.delta_tiles
        .iter()
        .map(|&id| ProvinceId::new(id))
        .collect();
    
    // Calculate water distances using BFS
    let water_distances = calculate_water_distances(provinces, &river_set, &delta_set, &dimensions)?;
    
    // Apply agriculture values in parallel
    let agriculture_results: Vec<(usize, Agriculture, Distance)> = provinces
        .par_iter()
        .enumerate()
        .map(|(idx, province)| {
            let base_agriculture = get_base_agriculture(province.terrain);
            let water_bonus = calculate_water_bonus(
                province,
                &river_set,
                &delta_set,
                water_distances.get(&province.id).copied().unwrap_or(MAX_WATER_DISTANCE)
            );
            
            let agriculture = Agriculture::new(base_agriculture * water_bonus);
            let fresh_water_distance = Distance::new(
                water_distances.get(&province.id).copied().unwrap_or(MAX_WATER_DISTANCE)
            );
            
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
        high_agriculture_count,
        HIGH_AGRICULTURE_THRESHOLD
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
    if province.terrain == TerrainType::River || 
       province.terrain == TerrainType::Delta ||
       province.terrain == TerrainType::Ocean {
        return 1.0; // Base agriculture already accounts for water
    }
    
    // Check if directly on a river or delta (shouldn't happen with above check, but safety)
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
    dimensions: &MapDimensions,
) -> Result<HashMap<ProvinceId, f32>, AgricultureError> {
    // Build spatial index for O(1) lookups
    let mut grid_to_province: HashMap<(i32, i32), ProvinceId> = HashMap::new();
    let mut province_to_grid: HashMap<ProvinceId, (i32, i32)> = HashMap::new();
    
    for province in provinces {
        let col = (province.id.value() % dimensions.provinces_per_row) as i32;
        let row = (province.id.value() / dimensions.provinces_per_row) as i32;
        grid_to_province.insert((col, row), province.id);
        province_to_grid.insert(province.id, (col, row));
    }
    
    // Initialize BFS queue with all water sources
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    // Add all water sources to queue with distance 0
    for province in provinces {
        if province.terrain == TerrainType::Ocean ||
           province.terrain == TerrainType::River ||
           province.terrain == TerrainType::Delta ||
           river_set.contains(&province.id) ||
           delta_set.contains(&province.id) {
            queue.push_back((province.id, 0.0));
            distances.insert(province.id, 0.0);
        }
    }
    
    // BFS to calculate distances
    while let Some((current_id, current_dist)) = queue.pop_front() {
        // Stop if we've reached maximum distance
        if current_dist >= MAX_WATER_DISTANCE {
            continue;
        }
        
        // Get grid coordinates
        let (col, row) = province_to_grid.get(&current_id)
            .ok_or_else(|| AgricultureError::InvalidProvinceId(current_id.value()))?;
        
        // Check all hexagonal neighbors
        for (neighbor_col, neighbor_row) in hex_neighbors(*col, *row) {
            if let Some(&neighbor_id) = grid_to_province.get(&(neighbor_col, neighbor_row)) {
                // Only process if not already visited
                if !distances.contains_key(&neighbor_id) {
                    let new_dist = current_dist + 1.0;
                    distances.insert(neighbor_id, new_dist);
                    queue.push_back((neighbor_id, new_dist));
                }
            }
        }
    }
    
    Ok(distances)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_base_agriculture_values() {
        assert_eq!(get_base_agriculture(TerrainType::Delta), DELTA_BASE_AGRICULTURE);
        assert_eq!(get_base_agriculture(TerrainType::River), RIVER_BASE_AGRICULTURE);
        assert_eq!(get_base_agriculture(TerrainType::TemperateGrassland), PLAINS_BASE_AGRICULTURE);
        assert_eq!(get_base_agriculture(TerrainType::Ocean), OCEAN_BASE_AGRICULTURE);
    }
    
    #[test]
    fn test_water_bonus_calculation() {
        let province = Province {
            id: ProvinceId::new(1),
            terrain: TerrainType::TemperateGrassland,
            position: Vec2::ZERO,
            elevation: crate::components::Elevation::new(0.5),
            population: 100,
            agriculture: Agriculture::new(0.0),
            fresh_water_distance: Distance::new(0.0),
        };
        
        let river_set = HashSet::new();
        let delta_set = HashSet::new();
        
        // Test various distances
        assert_eq!(calculate_water_bonus(&province, &river_set, &delta_set, 0.0), 1.0);
        assert_eq!(calculate_water_bonus(&province, &river_set, &delta_set, 0.5), 1.3);
        assert_eq!(calculate_water_bonus(&province, &river_set, &delta_set, 2.0), 1.1);
        assert_eq!(calculate_water_bonus(&province, &river_set, &delta_set, 15.0), 1.0);
    }
}