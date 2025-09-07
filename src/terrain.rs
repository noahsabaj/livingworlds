//! Terrain generation system for Living Worlds
//! 
//! Handles procedural generation of elevation, continents, climate zones,
//! and terrain types using Perlin noise and tectonic simulation.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;

/// TerrainType represents the physical terrain of a province
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Ocean,
    Beach,
    Plains,
    Hills,
    Mountains,
    Ice,      // Polar ice caps
    Tundra,   // Cold plains
    Desert,   // Hot dry areas
    Forest,   // Temperate forests
    Jungle,   // Tropical rainforests
    River,    // Rivers flowing from mountains to ocean
}

/// ClimateZone represents the climate of a province
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClimateZone {
    Arctic,      // Ice and tundra
    Subarctic,   // Mostly tundra  
    Temperate,   // Normal terrain
    Subtropical, // Warmer, some deserts
    Tropical,    // Hot and humid
}

/// Get climate zone based on latitude
pub fn get_climate_zone(y: f32, map_height: f32) -> ClimateZone {
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

/// Classify terrain based on elevation and climate
pub fn classify_terrain_with_climate(elevation: f32, y: f32, map_height: f32) -> TerrainType {
    let climate = get_climate_zone(y, map_height);
    
    // Arctic zones are ice or tundra
    if matches!(climate, ClimateZone::Arctic) {
        if elevation < 0.15 {
            return TerrainType::Ocean;
        } else if elevation < 0.25 {
            return TerrainType::Ice;
        } else {
            return TerrainType::Tundra;
        }
    }
    
    // Subarctic has tundra and some forests
    if matches!(climate, ClimateZone::Subarctic) {
        if elevation < 0.15 {
            return TerrainType::Ocean;
        } else if elevation < 0.22 {
            return TerrainType::Tundra;
        } else if elevation < 0.35 {
            // Boreal forests in subarctic regions
            let forest_factor = ((y * 0.007).sin() * (y * 0.004).cos()).abs();
            if forest_factor > 0.4 {
                return TerrainType::Forest;
            }
        }
    }
    
    // Temperate zones have mixed forests and plains
    if matches!(climate, ClimateZone::Temperate) {
        if elevation < 0.15 {
            return TerrainType::Ocean;
        } else if elevation < 0.18 {
            return TerrainType::Beach;
        } else if elevation < 0.35 {
            // Mix of forests and plains based on moisture patterns
            let moisture = ((y * 0.006).sin() * (y * 0.008).cos() + (y * 0.003).sin()).abs();
            if moisture > 0.55 {
                return TerrainType::Forest;
            } else {
                return TerrainType::Plains;
            }
        } else if elevation < 0.5 {
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
        if elevation > 0.2 && elevation < 0.35 {
            // Desert bands based on position
            let desert_factor = ((y * 0.005).sin() * (y * 0.003).cos()).abs();
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
        if elevation < 0.15 {
            return TerrainType::Ocean;
        } else if elevation < 0.18 {
            return TerrainType::Beach;
        } else if elevation < 0.4 {
            // Most tropical land is jungle
            let jungle_factor = ((y * 0.004).sin() * (y * 0.006).cos()).abs();
            if jungle_factor > 0.2 {
                return TerrainType::Jungle;
            } else {
                return TerrainType::Plains;
            }
        } else if elevation < 0.5 {
            // Higher tropical elevations might be jungle or hills
            let jungle_chance = ((y * 0.005).cos()).abs();
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
    classify_terrain(elevation)
}

/// Simple terrain classification based on elevation
fn classify_terrain(elevation: f32) -> TerrainType {
    if elevation < 0.15 {
        TerrainType::Ocean
    } else if elevation < 0.20 {
        TerrainType::Beach
    } else if elevation < 0.45 {
        TerrainType::Plains
    } else if elevation < 0.65 {
        TerrainType::Hills
    } else {
        TerrainType::Mountains
    }
}

/// Get smooth color gradient based on terrain and elevation
pub fn get_terrain_color_gradient(terrain: TerrainType, elevation: f32) -> Color {
    // Define base colors with smoother transitions
    let color = match terrain {
        TerrainType::Ocean => {
            // Three distinct ocean depth colors based on elevation
            if elevation >= 0.10 {
                // Shallow water (coastal)
                Color::srgb(0.15, 0.35, 0.55)
            } else if elevation >= 0.05 {
                // Medium depth
                Color::srgb(0.08, 0.25, 0.45)
            } else {
                // Deep ocean
                Color::srgb(0.02, 0.15, 0.35)
            }
        },
        TerrainType::Beach => {
            // Sandy beach with slight variation
            let sand_var = elevation * 2.0;
            Color::srgb(0.9 + sand_var * 0.05, 0.85 + sand_var * 0.05, 0.65 + sand_var * 0.1)
        },
        TerrainType::Plains => {
            // Lush green plains with elevation-based variation
            let green_factor = (elevation - 0.2) / 0.25;
            let r = 0.25 + green_factor * 0.1;
            let g = 0.55 + green_factor * 0.1;
            let b = 0.25 + green_factor * 0.05;
            Color::srgb(r, g, b)
        },
        TerrainType::Hills => {
            // Brown hills transitioning to grey at higher elevations
            let hill_factor = (elevation - 0.45) / 0.2;
            let r = 0.45 + hill_factor * 0.1;
            let g = 0.4 + hill_factor * 0.05;
            let b = 0.3 + hill_factor * 0.15;
            Color::srgb(r, g, b)
        },
        TerrainType::Mountains => {
            // Rocky grey to snow white based on height
            let snow_factor = ((elevation - 0.65) / 0.35).clamp(0.0, 1.0);
            let grey = 0.6 + snow_factor * 0.35;
            Color::srgb(grey, grey, grey + snow_factor * 0.05)
        },
        TerrainType::Ice => {
            // Polar ice - bright white with blue tint
            Color::srgb(0.92, 0.95, 1.0)
        },
        TerrainType::Tundra => {
            // Cold barren land - gray-brown
            Color::srgb(0.65, 0.6, 0.55)
        },
        TerrainType::Desert => {
            // Sandy desert - warm tan
            let variation = (elevation * 3.0).sin() * 0.05;
            Color::srgb(0.9 + variation, 0.8 + variation, 0.6)
        },
        TerrainType::Forest => {
            // Temperate forest - rich green with variation
            let forest_var = (elevation - 0.3) / 0.2;
            let r = 0.15 + forest_var * 0.05;
            let g = 0.35 + forest_var * 0.1;
            let b = 0.12 + forest_var * 0.03;
            Color::srgb(r, g, b)
        },
        TerrainType::Jungle => {
            // Tropical jungle - deep vibrant green
            let jungle_var = ((elevation * 5.0).sin() * 0.1).abs();
            let r = 0.05 + jungle_var;
            let g = 0.3 + jungle_var * 1.5;
            let b = 0.08 + jungle_var * 0.5;
            Color::srgb(r, g, b)
        },
        TerrainType::River => {
            // River - lighter freshwater blue-green, shallower than ocean
            Color::srgb(0.2, 0.45, 0.55)
        },
    };
    
    color
}

/// Generate elevation with continents and tectonic plates
pub fn generate_elevation(
    x: f32,
    y: f32,
    perlin: &Perlin,
    continent_centers: &[(f32, f32)],
) -> f32 {
    // Base noise for terrain variation
    let scale = 0.001;
    let mut elevation = perlin.get([x as f64 * scale, y as f64 * scale]) as f32;
    
    // Add multiple octaves for detail
    elevation += 0.5 * perlin.get([x as f64 * scale * 2.0, y as f64 * scale * 2.0]) as f32;
    elevation += 0.25 * perlin.get([x as f64 * scale * 4.0, y as f64 * scale * 4.0]) as f32;
    
    // Normalize to 0-1 range
    elevation = (elevation + 1.0) / 2.0;
    
    // Apply continent influence - land masses form around continent centers
    let mut continent_influence: f32 = 0.0;
    for &(cx, cy) in continent_centers {
        let distance = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        let max_influence_distance = 3000.0; // How far continents extend
        
        if distance < max_influence_distance {
            // Influence decreases with distance, creating natural coastlines
            let influence = 1.0 - (distance / max_influence_distance);
            // Use smoothstep for better coastline shapes
            let smooth_influence = influence * influence * (3.0 - 2.0 * influence);
            continent_influence = continent_influence.max(smooth_influence * 0.6);
        }
    }
    
    // Combine base elevation with continent influence
    elevation = (elevation * 0.4 + continent_influence * 0.6).clamp(0.0, 1.0);
    
    // Add some noise to break up perfect circles
    let coastline_noise = perlin.get([x as f64 * 0.002, y as f64 * 0.002]) as f32 * 0.15;
    elevation = (elevation + coastline_noise).clamp(0.0, 1.0);
    
    elevation
}

/// Generate tectonic plates and continent centers
pub fn generate_continent_centers(
    seed: u32,
    num_plates: usize,
    map_width: f32,
    map_height: f32,
    edge_buffer: f32,
) -> Vec<(f32, f32)> {
    let mut rng = StdRng::seed_from_u64(seed as u64);
    let mut continent_centers = Vec::new();
    
    // Create major continental landmasses
    for _ in 0..num_plates {
        let x = rng.gen_range((-map_width + edge_buffer)..(map_width - edge_buffer));
        let y = rng.gen_range((-map_height + edge_buffer)..(map_height - edge_buffer));
        continent_centers.push((x, y));
        
        // Sometimes add smaller nearby landmasses (islands)
        if rng.gen_bool(0.3) {
            let island_x = x + rng.gen_range(-1000.0..1000.0);
            let island_y = y + rng.gen_range(-1000.0..1000.0);
            if island_x.abs() < map_width - edge_buffer && 
               island_y.abs() < map_height - edge_buffer {
                continent_centers.push((island_x, island_y));
            }
        }
    }
    
    continent_centers
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