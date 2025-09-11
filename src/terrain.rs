//! Terrain generation system for Living Worlds
//! 
//! Handles procedural generation of elevation, continents, climate zones,
//! and terrain types using Perlin noise and tectonic simulation.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use crate::constants::*;

/// TerrainType represents the physical terrain of a province
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Ocean,
    Beach,
    Plains,
    Hills,
    Mountains,
    Ice,
    Tundra,
    Desert,
    Forest,
    Jungle,
    River,
    Delta,
}

/// ClimateZone represents the climate of a province
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClimateZone {
    Arctic,
    Subarctic,
    Temperate,
    Subtropical,
    Tropical,
}

/// Determine climate zone based on latitude (internal use only during generation)
fn get_climate_zone(y: f32, map_height: f32) -> ClimateZone {
    let latitude = (y / map_height + 0.5).clamp(0.0, 1.0);
    
    if latitude < 0.1 || latitude > 0.9 {
        ClimateZone::Arctic
    } else if latitude < 0.2 || latitude > 0.8 {
        ClimateZone::Subarctic
    } else if latitude < 0.35 || latitude > 0.65 {
        ClimateZone::Temperate
    } else if latitude < 0.45 || latitude > 0.55 {
        ClimateZone::Subtropical
    } else {
        ClimateZone::Tropical
    }
}

/// Classify terrain based on elevation and climate with default sea level
pub fn classify_terrain_with_climate(elevation: f32, x: f32, y: f32, map_height: f32) -> TerrainType {
    // Default sea level of 0.15 (approximately 60% ocean coverage)
    classify_terrain_with_sea_level(elevation, x, y, map_height, 0.15)
}

/// Classify terrain based on elevation, climate, and custom sea level
pub fn classify_terrain_with_sea_level(elevation: f32, x: f32, y: f32, map_height: f32, sea_level: f32) -> TerrainType {
    let climate = get_climate_zone(y, map_height);
    
    // Arctic zones are ice or tundra
    if matches!(climate, ClimateZone::Arctic) {
        if elevation < sea_level {
            return TerrainType::Ocean;
        } else if elevation < sea_level + 0.10 {
            return TerrainType::Ice;
        } else {
            return TerrainType::Tundra;
        }
    }
    
    // Subarctic has tundra and some forests
    if matches!(climate, ClimateZone::Subarctic) {
        if elevation < sea_level {
            return TerrainType::Ocean;
        } else if elevation < sea_level + 0.07 {
            return TerrainType::Tundra;
        } else if elevation < sea_level + 0.20 {
            // Boreal forests in subarctic regions
            let forest_factor = ((y * 0.007).sin() * (y * 0.004).cos()).abs();
            if forest_factor > 0.4 {
                return TerrainType::Forest;
            }
        }
    }
    
    // Temperate zones have mixed forests and plains
    if matches!(climate, ClimateZone::Temperate) {
        if elevation < sea_level {
            return TerrainType::Ocean;
        } else if elevation < sea_level + 0.03 {
            return TerrainType::Beach;
        } else if elevation < sea_level + 0.20 {
            // Mix of forests and plains based on moisture patterns (use both x and y to avoid banding)
            let moisture = ((y * 0.006).sin() * (x * 0.005).cos() + (x * 0.004).sin() * (y * 0.003).cos()).abs();
            if moisture > 0.55 {
                return TerrainType::Forest;
            } else {
                return TerrainType::Plains;
            }
        } else if elevation < sea_level + 0.35 {
            // Higher elevations are hills with some forests
            let forest_chance = ((y * 0.005).cos() * (y * 0.007).sin()).abs();
            if forest_chance > 0.6 {
                return TerrainType::Forest;
            } else {
                return TerrainType::Hills;
            }
        } else {
            return TerrainType::Mountains;
        }
    }
    
    // Subtropical can have deserts and dry forests
    if matches!(climate, ClimateZone::Subtropical) {
        if elevation > sea_level + 0.05 && elevation < sea_level + 0.20 {
            // Desert bands based on position (use x and y to avoid horizontal banding)
            let desert_factor = ((x * 0.004).sin() * (y * 0.005).cos() + (y * 0.003).sin() * (x * 0.003).cos()).abs();
            if desert_factor > 0.6 {
                return TerrainType::Desert;
            } else if desert_factor < 0.3 {
                // Dry subtropical forests
                return TerrainType::Forest;
            }
        }
    }
    
    // Tropical zones have jungles
    if matches!(climate, ClimateZone::Tropical) {
        if elevation < sea_level {
            return TerrainType::Ocean;
        } else if elevation < sea_level + 0.03 {
            return TerrainType::Beach;
        } else if elevation < sea_level + 0.25 {
            // Most tropical land is jungle (use x and y to avoid banding)
            let jungle_factor = ((x * 0.003).sin() * (y * 0.004).cos() + (y * 0.006).sin() * (x * 0.005).cos()).abs();
            if jungle_factor > 0.2 {
                return TerrainType::Jungle;
            } else {
                return TerrainType::Plains;
            }
        } else if elevation < sea_level + 0.35 {
            // Higher tropical elevations might be jungle or hills
            let jungle_chance = ((x * 0.004).sin() * (y * 0.005).cos()).abs();
            if jungle_chance > 0.5 {
                return TerrainType::Jungle;
            } else {
                return TerrainType::Hills;
            }
        } else {
            return TerrainType::Mountains;
        }
    }
    
    // Default terrain classification
    classify_terrain_with_sea_level_simple(elevation, sea_level)
}

/// Simple terrain classification based on elevation
fn classify_terrain(elevation: f32) -> TerrainType {
    classify_terrain_with_sea_level_simple(elevation, 0.15)
}

/// Simple terrain classification with custom sea level
fn classify_terrain_with_sea_level_simple(elevation: f32, sea_level: f32) -> TerrainType {
    if elevation < sea_level {
        TerrainType::Ocean
    } else if elevation < sea_level + 0.05 {
        TerrainType::Beach
    } else if elevation < sea_level + 0.30 {
        TerrainType::Plains
    } else if elevation < sea_level + 0.50 {
        TerrainType::Hills
    } else {
        TerrainType::Mountains
    }
}


/// Generate elevation using advanced noise techniques with map edge handling
/// This version includes map edge handling to force ocean at the boundaries
pub fn generate_elevation_with_edges(x: f32, y: f32, perlin: &Perlin, continent_centers: &[(f32, f32)], map_width: f32, map_height: f32) -> f32 {
    // Use dynamic map dimensions for bounds calculation
    let map_bound_x = map_width / 2.0;
    let map_bound_y = map_height / 2.0;
    let edge_buffer = EDGE_BUFFER;
    
    // Force ocean at map edges
    let dist_from_edge_x = map_bound_x - x.abs();
    let dist_from_edge_y = map_bound_y - y.abs();
    let min_edge_dist = dist_from_edge_x.min(dist_from_edge_y);
    
    if min_edge_dist < edge_buffer {
        // Smooth transition to ocean at edges
        let edge_factor = (min_edge_dist / edge_buffer).max(0.0);
        if edge_factor < 0.3 {
            return 0.0; // Deep ocean at very edge
        }
        // Will apply this factor at the end
    }
    
    // Domain warping for organic shapes
    let warp_scale = 0.002;
    let warp_x = perlin.get([x as f64 * warp_scale, y as f64 * warp_scale]) as f32 * 150.0;
    let warp_y = perlin.get([x as f64 * warp_scale + 100.0, y as f64 * warp_scale]) as f32 * 150.0;
    
    // Apply warping to coordinates
    let wx = x + warp_x;
    let wy = y + warp_y;
    
    // Normalize warped coordinates
    let nx = wx / 1000.0;
    let ny = wy / 1000.0;
    
    // Layered octaves with different characteristics
    let base = perlin.get([nx as f64 * 0.7, ny as f64 * 0.7]) as f32;
    let detail = perlin.get([nx as f64 * 2.0, ny as f64 * 2.0]) as f32 * 0.5;
    let fine = perlin.get([nx as f64 * 4.0, ny as f64 * 4.0]) as f32 * 0.25;
    
    // Tectonic plate boundaries for realistic mountain chains
    // Simulate plate tectonics with voronoi-like regions
    let plate_scale = 0.0008;
    let plate_x = ((wx as f64 * plate_scale).sin() + (wy as f64 * plate_scale * 0.7).cos()) as f32;
    let plate_y = ((wx as f64 * plate_scale * 0.9).cos() + (wy as f64 * plate_scale).sin()) as f32;
    let plate_boundary = 1.0 - (plate_x * plate_y).abs().min(1.0);
    
    // Ridge noise along plate boundaries for mountain chains
    let ridge_scale = 0.003;
    let ridge = 1.0 - (perlin.get([wx as f64 * ridge_scale, wy as f64 * ridge_scale]) as f32 * 2.0).abs();
    let ridge_contribution = ridge * 0.3 * plate_boundary; // Mountains form along plate boundaries
    
    // Combine noise layers
    let mut elevation = (base + detail + fine + ridge_contribution) / 2.0 + 0.5;
    
    // Multiple continent masks with fractal distortion for natural coastlines  
    let mut continent_influence: f32 = 0.0;
    
    for (idx, &(cx, cy)) in continent_centers.iter().enumerate() {
        // Add elongation and rotation to continents for varied shapes
        let continent_seed = (idx as u32).wrapping_mul(2654435761);
        let angle = (continent_seed % 360) as f32 * 0.0174533; // Random rotation
        let elongation = 1.0 + (continent_seed % 100) as f32 / 50.0; // 1.0 to 3.0 elongation
        
        // Rotate and elongate the coordinate space
        let dx = x - cx;
        let dy = y - cy;
        let rotated_x = dx * angle.cos() - dy * angle.sin();
        let rotated_y = (dx * angle.sin() + dy * angle.cos()) * elongation;
        
        let dist = (rotated_x.powi(2) + rotated_y.powi(2)).sqrt();
        
        // Add multi-scale noise distortion for realistic coastlines (but less breakup)
        let distortion_scale = 0.001;
        // Large scale features (continents/peninsulas) - further reduced to prevent holes
        let distortion1 = perlin.get([
            (x + cx * 0.1) as f64 * distortion_scale * 0.3, 
            (y + cy * 0.1) as f64 * distortion_scale * 0.3
        ]) as f32 * 150.0;  // Reduced from 200 to prevent inland lakes
        // Medium scale features (bays/capes)
        let distortion2 = perlin.get([
            x as f64 * distortion_scale * 1.5, 
            y as f64 * distortion_scale * 1.5
        ]) as f32 * 80.0;  // Reduced from 100
        // Fine detail (rough coastline)
        let distortion3 = perlin.get([
            x as f64 * distortion_scale * 6.0, 
            y as f64 * distortion_scale * 6.0
        ]) as f32 * 25.0;  // Reduced from 30
        
        // Apply fractal distortion
        let distorted_dist = dist + distortion1 + distortion2 * 0.5 + distortion3 * 0.25;
        
        // Vary continent sizes dramatically based on index
        let continent_seed = (idx as u32).wrapping_mul(2654435761) % 1000;
        let size_factor = continent_seed as f32 / 1000.0;
        
        let base_radius = if idx >= 30 {
            // Tiny islands (fewer of these)
            CONTINENT_TINY_BASE + size_factor * CONTINENT_TINY_VARIATION
        } else if idx >= 18 {
            // Archipelagos and island chains
            CONTINENT_ARCHIPELAGO_BASE + size_factor * CONTINENT_ARCHIPELAGO_VARIATION
        } else if idx >= 8 {
            // Medium continents (Australia-sized) - more of these
            CONTINENT_MEDIUM_BASE + size_factor * CONTINENT_MEDIUM_VARIATION
        } else {
            // Massive continents (Eurasia-sized) - more of these too
            CONTINENT_MASSIVE_BASE + size_factor * CONTINENT_MASSIVE_VARIATION
        };
        
        // Apply size multiplier for more land
        let adjusted_radius = base_radius * CONTINENT_SIZE_MULTIPLIER;
        
        // Smooth falloff with varying sharpness for different edge types
        let falloff = 1.0 - (distorted_dist / adjusted_radius).clamp(0.0, 1.0);
        let shaped_falloff = falloff.powf(CONTINENT_FALLOFF_BASE + size_factor * CONTINENT_FALLOFF_VARIATION);
        
        // Allow overlapping continents to merge naturally
        continent_influence = continent_influence.max(shaped_falloff);
    }
    
    let mask = continent_influence;
    
    // Apply continent mask
    elevation *= mask;
    
    // Apply edge fade if near map boundary
    if min_edge_dist < edge_buffer {
        let edge_factor = (min_edge_dist / edge_buffer).clamp(0.0, 1.0);
        elevation *= edge_factor * edge_factor; // Quadratic falloff to ocean
    }
    
    // Continental shelf effect - create gradual depth transitions near land
    if elevation < 0.15 && elevation > 0.08 {
        // Near-shore areas become shallow continental shelf
        let shelf_gradient = (elevation - 0.08) / 0.07; // Normalize to 0-1
        elevation = 0.10 + shelf_gradient * 0.08; // Compress to shallow water range (0.10-0.18)
    }
    
    // Volcanic island chains - EXTREMELY RARE to eliminate polka-dot effect
    if elevation < 0.08 {
        // Create hotspot island chains - extremely rare
        let hotspot_scale = 0.0002;  // Even larger scale = even fewer islands
        let hotspot = perlin.get([x as f64 * hotspot_scale + 500.0, y as f64 * hotspot_scale]) as f32;
        
        // Only the most extreme hotspots create islands (extremely rare)
        if hotspot > 0.98 {
            elevation = 0.20 + (hotspot - 0.98) * 5.0; // Extremely rare volcanic islands
        }
        // Seamounts virtually eliminated
        else if hotspot > 0.95 {
            elevation = 0.08 + (hotspot - 0.95) * 0.4; // Very rare seamounts
        }
    }
    
    // For very deep ocean, ensure it stays deep
    if elevation < 0.01 {
        elevation = 0.02; // Deep ocean floor
    }
    
    elevation
}

/// Get terrain population multiplier
pub fn get_terrain_population_multiplier(terrain: TerrainType) -> f32 {
    match terrain {
        TerrainType::Plains => 1.5,   // More population in plains
        TerrainType::Beach => 1.2,    // Coastal areas attract people
        TerrainType::Forest => 1.0,   // Moderate population in forests
        TerrainType::Jungle => 0.6,   // Dense jungle is harder to settle
        TerrainType::Hills => 0.8,    // Less in hills
        TerrainType::Mountains => 0.3, // Few in mountains
        TerrainType::Desert => 0.4,   // Low in deserts
        TerrainType::Tundra => 0.2,   // Very low in tundra
        TerrainType::Ice => 0.0,      // No permanent population on ice
        TerrainType::Ocean => 0.0,    // No population in ocean
        TerrainType::River => 2.0,    // Rivers attract high population
        TerrainType::Delta => 3.0,    // River deltas are extremely fertile
    }
}

/// Bevy plugin for terrain generation
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, _app: &mut App) {
        // Terrain generation happens during world setup
        // No continuous systems needed
    }
}