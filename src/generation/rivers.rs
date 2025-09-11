//! River system generation with flow accumulation

use std::collections::{HashMap, HashSet};
use bevy::prelude::*;
use rand::rngs::StdRng;

use crate::components::Province;
use crate::terrain::TerrainType;
use crate::constants::*;
use super::types::{MapDimensions, RiverSystem};
use super::utils::hex_neighbors;

pub fn generate(
    provinces: &mut [Province],
    dimensions: MapDimensions,
    _rng: &mut StdRng,
) -> RiverSystem {
    // Default river density of 1.0
    generate_with_density(provinces, dimensions, _rng, 1.0)
}

/// Generate rivers with specified density multiplier
pub fn generate_with_density(
    provinces: &mut [Province],
    dimensions: MapDimensions,
    _rng: &mut StdRng,
    river_density: f32,
) -> RiverSystem {
    let start = std::time::Instant::now();
    
    // Build spatial index for province lookups using ACTUAL col/row from ID
    let mut position_to_province: HashMap<(i32, i32), &Province> = HashMap::new();
    for province in provinces.iter() {
        // Calculate actual grid coordinates from province ID
        let col = (province.id % dimensions.provinces_per_row) as i32;
        let row = (province.id / dimensions.provinces_per_row) as i32;
        position_to_province.insert((col, row), province);
    }
    
    // Find potential river sources (mountains and hills)
    let mut potential_sources = Vec::new();
    for province in provinces.iter() {
        if province.terrain == TerrainType::Mountains || 
            (province.terrain == TerrainType::Hills && province.elevation >= RIVER_MIN_ELEVATION) {
            potential_sources.push((province.id, province.position, province.elevation));
        }
    }
    
    println!("Found {} potential river sources (mountains/hills)", potential_sources.len());
    
    // Sort by elevation (highest first) for better river flow
    potential_sources.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    // Select river sources based on density
    let base_num_rivers = (RIVER_COUNT / 4).min(potential_sources.len());  // Fewer but longer rivers
    let num_rivers = ((base_num_rivers as f32 * river_density) as usize).min(potential_sources.len());
    let selected_sources: Vec<_> = potential_sources.into_iter().take(num_rivers).collect();
    
    println!("Selected {} river sources to trace", selected_sources.len());
    
    let mut river_tiles = Vec::new();
    let mut delta_tiles = Vec::new();
    let mut flow_accumulation: HashMap<u32, f32> = HashMap::new();
    
    // Trace each river from source to ocean
    for (source_id, _source_pos, _) in selected_sources {
        let mut current_id = source_id;
        let mut river_path = vec![source_id];
        let mut visited = HashSet::new();
        
        // Use grid coordinates for visited set
        let start_col = (source_id % dimensions.provinces_per_row) as i32;
        let start_row = (source_id / dimensions.provinces_per_row) as i32;
        visited.insert((start_col, start_row));
        
        let mut flow = 1.0;  // Start with flow of 1
        const MAX_RIVER_LENGTH: usize = 1000;
        
        for _ in 0..MAX_RIVER_LENGTH {
            // Get current grid coordinates from province ID
            let current_col = (current_id % dimensions.provinces_per_row) as i32;
            let current_row = (current_id / dimensions.provinces_per_row) as i32;
            
            // Get hexagonal neighbors
            let neighbors = hex_neighbors(current_col, current_row);
            
            // Find the lowest unvisited neighbor
            let mut lowest_neighbor: Option<(u32, f32)> = None;
            let mut reached_ocean = false;
            
            for (nx, ny) in neighbors {
                if let Some(province) = position_to_province.get(&(nx, ny)) {
                    // Check if already visited using grid coordinates
                    if visited.contains(&(nx, ny)) {
                        continue;
                    }
                    
                    // Check if we reached ocean
                    if province.terrain == TerrainType::Ocean {
                        delta_tiles.push(province.id);
                        if !river_path.is_empty() {
                            delta_tiles.push(*river_path.last().unwrap());
                        }
                        
                        // Add flow to all tiles in this river
                        for &tile_id in &river_path {
                            *flow_accumulation.entry(tile_id).or_insert(0.0) += flow;
                        }
                        
                        river_tiles.extend(river_path.clone());
                        reached_ocean = true;
                        break;
                    }
                    
                    // Track lowest neighbor for continuing river
                    if lowest_neighbor.is_none() || province.elevation < lowest_neighbor.as_ref().unwrap().1 {
                        lowest_neighbor = Some((province.id, province.elevation));
                    }
                }
            }
            
            // If we reached ocean, stop this river
            if reached_ocean {
                break;
            }
            
            // Continue to lowest neighbor if available
            if let Some((next_id, _)) = lowest_neighbor {
                river_path.push(next_id);
                current_id = next_id;
                
                // Mark this grid cell as visited
                let next_col = (next_id % dimensions.provinces_per_row) as i32;
                let next_row = (next_id / dimensions.provinces_per_row) as i32;
                visited.insert((next_col, next_row));
                
                flow += 0.1;  // Accumulate flow as we go downstream
            } else {
                break;  // No valid path forward
            }
        }
    }
    
    // Build HashMap for O(1) province lookups by ID to avoid O(n²) pattern
    let mut province_id_to_idx: HashMap<u32, usize> = HashMap::new();
    for (idx, province) in provinces.iter().enumerate() {
        province_id_to_idx.insert(province.id, idx);
    }
    
    // Convert high-flow tiles to river terrain - O(n) instead of O(n²)
    for (province_id, flow) in &flow_accumulation {
        if *flow > 0.5 {  // Only tiles with significant flow become visible rivers
            if let Some(&idx) = province_id_to_idx.get(province_id) {
                if provinces[idx].terrain != TerrainType::Ocean {
                    provinces[idx].terrain = TerrainType::River;
                }
            }
        }
    }
    
    // Convert delta tiles to delta terrain - O(n) instead of O(n²)
    for &delta_id in &delta_tiles {
        if let Some(&idx) = province_id_to_idx.get(&delta_id) {
            if provinces[idx].terrain != TerrainType::Ocean {
                provinces[idx].terrain = TerrainType::Delta;
            }
        }
    }
    
    println!("River generation completed in {:.2}s", start.elapsed().as_secs_f32());
    println!("Generated {} river tiles and {} delta tiles", river_tiles.len(), delta_tiles.len());
    
    RiverSystem {
        river_tiles,
        delta_tiles,
        flow_accumulation,
    }
}