//! Agriculture and fresh water distance calculation

use std::collections::HashSet;
use crate::components::Province;
use crate::terrain::TerrainType;
use super::types::{MapDimensions, RiverSystem};

pub fn calculate(
    provinces: &mut [Province],
    river_system: &RiverSystem,
    dimensions: MapDimensions,
) {
    // Build sets for quick lookups
    let river_set: HashSet<u32> = river_system.river_tiles.iter().cloned().collect();
    let delta_set: HashSet<u32> = river_system.delta_tiles.iter().cloned().collect();
    
    let mut high_agriculture_count = 0;
    
    for province in provinces.iter_mut() {
        // Skip ocean tiles
        if province.terrain == TerrainType::Ocean {
            province.agriculture = 0.0;
            continue;
        }
        
        // Base agriculture from terrain type
        let base_agriculture = match province.terrain {
            TerrainType::Delta => 3.0,      // River deltas are extremely fertile
            TerrainType::River => 2.5,      // River tiles are very fertile
            TerrainType::Plains => 1.0,     // Good farmland
            TerrainType::Forest => 0.8,     // Can be cleared for farming
            TerrainType::Hills => 0.5,      // Terraced farming possible
            TerrainType::Desert => 0.2,     // Limited oasis farming
            TerrainType::Tundra => 0.1,     // Very limited growing season
            _ => 0.3,
        };
        
        // Bonus for being near rivers or deltas
        let water_bonus = if river_set.contains(&province.id) {
            1.5  // On a river
        } else if delta_set.contains(&province.id) {
            2.0  // At a river delta
        } else {
            // Check distance to nearest water
            let _grid_x = (province.position.x / dimensions.hex_size).round() as i32;
            let _grid_y = (province.position.y / dimensions.hex_size).round() as i32;
            
            let mut min_water_dist = f32::MAX;
            for &_river_id in &river_system.river_tiles[..river_system.river_tiles.len().min(100)] {
                // Just check first 100 river tiles for performance
                min_water_dist = 5.0;  // Simplified for now
                break;
            }
            
            match min_water_dist {
                d if d <= 1.0 => 1.2,   // Adjacent to water
                d if d <= 3.0 => 0.8,   // Near water
                d if d <= 5.0 => 0.5,   // Moderate distance
                _ => 0.3,               // Far from water
            }
        };
        
        province.agriculture = base_agriculture * water_bonus;
        province.fresh_water_distance = if river_set.contains(&province.id) {
            0.0
        } else {
            10.0  // Simplified for now
        };
        
        if province.agriculture >= 1.5 {
            high_agriculture_count += 1;
        }
    }
    
    println!("Set up {} provinces with high agriculture (>= 1.5)", high_agriculture_count);
}