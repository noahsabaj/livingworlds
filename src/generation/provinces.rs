//! Province generation and ocean depth calculation

use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use bevy::prelude::*;
use noise::Perlin;
use rand::rngs::StdRng;

use crate::components::Province;
use crate::terrain::{TerrainType, classify_terrain_with_climate};
use crate::constants::*;
use super::types::MapDimensions;
use super::tectonics::TectonicSystem;

pub fn generate(
    tectonics: &TectonicSystem,
    dimensions: MapDimensions,
    perlin: &Perlin,
    _rng: &mut StdRng,
) -> Vec<Province> {
    let total_provinces = dimensions.provinces_per_row * dimensions.provinces_per_col;
    
    // Generate all provinces in parallel
    let provinces: Vec<Province> = (0..total_provinces)
        .into_par_iter()
        .map(|idx| {
            let col = idx % dimensions.provinces_per_row;
            let row = idx / dimensions.provinces_per_row;
            let province_id = idx;
            
            // Calculate position using the SINGLE canonical hex position function
            let (pos_x, pos_y) = crate::constants::calculate_hex_position(
                col, row, dimensions.hex_size, 
                dimensions.provinces_per_row, dimensions.provinces_per_col
            );
            
            // Position is already centered by calculate_hex_position
            let x = pos_x;
            let y = pos_y;
            
            // Generate elevation with improved continental generation
            let elevation = crate::terrain::generate_elevation_with_edges(
                x, y, perlin, &tectonics.continent_centers,
                dimensions.bounds.x_max - dimensions.bounds.x_min,
                dimensions.bounds.y_max - dimensions.bounds.y_min,
            );
            
            // Classify terrain based on elevation and climate
            let terrain = classify_terrain_with_climate(
                elevation, x, y,
                dimensions.bounds.y_max - dimensions.bounds.y_min,
            );
            
            // Initial population based on terrain
            let base_pop = if terrain == TerrainType::Ocean {
                0.0
            } else {
                PROVINCE_MIN_POPULATION + (elevation * PROVINCE_MAX_ADDITIONAL_POPULATION)
            };
            
            Province {
                id: province_id,
                position: Vec2::new(pos_x, pos_y),  // Use proper hex grid position
                nation_id: None,
                population: base_pop,
                terrain,
                elevation,
                agriculture: 0.0,  // Will be calculated later
                fresh_water_distance: f32::MAX,  // Will be calculated later
            }
        })
        .collect();
    
    provinces
}

pub fn calculate_ocean_depths(provinces: &mut [Province], dimensions: MapDimensions) {
    let start = std::time::Instant::now();
    println!("Calculating ocean depths for {} provinces...", provinces.len());
    
    // Build spatial grid for land positions for O(1) lookups
    let grid_size = dimensions.hex_size * 3.0;  // Grid cells of 3 hex widths
    let mut land_grid: HashMap<(i32, i32), Vec<Vec2>> = HashMap::new();
    let mut ocean_positions = Vec::new();
    
    // Populate spatial grid with land positions and collect ocean positions
    for (i, province) in provinces.iter().enumerate() {
        if province.terrain == TerrainType::Ocean {
            ocean_positions.push((i, province.position));
        } else {
            let grid_x = (province.position.x / grid_size).floor() as i32;
            let grid_y = (province.position.y / grid_size).floor() as i32;
            land_grid.entry((grid_x, grid_y))
                .or_insert_with(Vec::new)
                .push(province.position);
        }
    }
    
    println!("  {} ocean tiles, {} land grid cells", 
             ocean_positions.len(), land_grid.len());
    
    // Calculate ocean depths in PARALLEL for massive speedup
    let land_grid_arc = Arc::new(land_grid);
    let ocean_depth_updates: Vec<(usize, f32)> = ocean_positions
        .par_iter()  // Parallel iterator for 10-20x speedup!
        .map(|(ocean_idx, ocean_pos)| {
            // Check only nearby grid cells (9-cell neighborhood)
            let grid_x = (ocean_pos.x / grid_size).floor() as i32;
            let grid_y = (ocean_pos.y / grid_size).floor() as i32;
            
            let mut min_dist_to_land = f32::MAX;
            
            // Only check 3x3 grid around this ocean tile
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let Some(land_tiles) = land_grid_arc.get(&(grid_x + dx, grid_y + dy)) {
                        for land_pos in land_tiles {
                            let dist = ocean_pos.distance(*land_pos);
                            min_dist_to_land = min_dist_to_land.min(dist);
                            
                            // Early exit for adjacent land
                            if min_dist_to_land <= dimensions.hex_size * 1.5 {
                                return (*ocean_idx, 0.12); // Shallow water
                            }
                        }
                    }
                }
            }
            
            // Calculate depth based on distance
            let hex_distance = min_dist_to_land / dimensions.hex_size;
            let depth = if hex_distance <= 1.8 {
                0.12  // Shallow coastal waters
            } else if hex_distance <= 5.0 {
                0.07  // Continental shelf  
            } else {
                0.02  // Deep ocean
            };
            
            (*ocean_idx, depth)
        })
        .collect();
    
    // Apply depth updates to provinces
    for (ocean_idx, depth) in ocean_depth_updates {
        provinces[ocean_idx].elevation = depth;
    }
    
    println!("Ocean depth calculation completed in {:.2}s", start.elapsed().as_secs_f32());
}