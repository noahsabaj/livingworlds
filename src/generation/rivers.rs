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
    let start = std::time::Instant::now();
    
    // Build spatial index for province lookups
    let mut position_to_province: HashMap<(i32, i32), &Province> = HashMap::new();
    for province in provinces.iter() {
        let grid_x = (province.position.x / dimensions.hex_size).round() as i32;
        let grid_y = (province.position.y / dimensions.hex_size).round() as i32;
        position_to_province.insert((grid_x, grid_y), province);
    }
    
    // Find potential river sources (mountains and hills)
    let mut potential_sources = Vec::new();
    for province in provinces.iter() {
        if province.terrain == TerrainType::Mountains || 
            (province.terrain == TerrainType::Hills && province.elevation >= RIVER_MIN_ELEVATION) {
            potential_sources.push((province.id, province.position, province.elevation));
        }
    }
    
    // Sort by elevation (highest first) for better river flow
    potential_sources.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    // Select river sources
    let num_rivers = (RIVER_COUNT / 4).min(potential_sources.len());  // Fewer but longer rivers
    let selected_sources: Vec<_> = potential_sources.into_iter().take(num_rivers).collect();
    
    let mut river_tiles = Vec::new();
    let mut delta_tiles = Vec::new();
    let mut flow_accumulation: HashMap<u32, f32> = HashMap::new();
    
    // Trace each river from source to ocean
    for (source_id, source_pos, _) in selected_sources {
        let mut current_pos = source_pos;
        let mut river_path = vec![source_id];
        let mut visited = HashSet::new();
        visited.insert((current_pos.x as i32, current_pos.y as i32));
        
        let mut flow = 1.0;  // Start with flow of 1
        const MAX_RIVER_LENGTH: usize = 1000;
        
        for _ in 0..MAX_RIVER_LENGTH {
            let current_grid_x = (current_pos.x / dimensions.hex_size).round() as i32;
            let current_grid_y = (current_pos.y / dimensions.hex_size).round() as i32;
            
            // Get hexagonal neighbors
            let neighbors = hex_neighbors(current_grid_x, current_grid_y);
            
            // Find the lowest unvisited neighbor
            let mut lowest_neighbor: Option<(Vec2, f32, u32)> = None;
            
            for (nx, ny) in neighbors {
                if let Some(province) = position_to_province.get(&(nx, ny)) {
                    let grid_pos = (province.position.x as i32, province.position.y as i32);
                    
                    if visited.contains(&grid_pos) {
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
                        break;
                    }
                    
                    // Track lowest neighbor for continuing river
                    if lowest_neighbor.is_none() || province.elevation < lowest_neighbor.as_ref().unwrap().1 {
                        lowest_neighbor = Some((province.position, province.elevation, province.id));
                    }
                }
            }
            
            // Continue to lowest neighbor if we didn't reach ocean
            if let Some((next_pos, _, next_id)) = lowest_neighbor {
                river_path.push(next_id);
                current_pos = next_pos;
                visited.insert((next_pos.x as i32, next_pos.y as i32));
                flow += 0.1;  // Accumulate flow as we go downstream
            } else {
                break;  // No valid path forward
            }
        }
    }
    
    // Convert high-flow tiles to river terrain
    for (province_id, flow) in &flow_accumulation {
        if *flow > 0.5 {  // Only tiles with significant flow become visible rivers
            if let Some(province) = provinces.iter_mut().find(|p| p.id == *province_id) {
                if province.terrain != TerrainType::Ocean {
                    province.terrain = TerrainType::River;
                }
            }
        }
    }
    
    // Convert delta tiles to delta terrain
    for &delta_id in &delta_tiles {
        if let Some(province) = provinces.iter_mut().find(|p| p.id == delta_id) {
            if province.terrain != TerrainType::Ocean {
                province.terrain = TerrainType::Delta;
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