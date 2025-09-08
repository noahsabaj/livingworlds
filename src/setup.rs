//! World setup and generation module
//! 
//! This module handles all one-time initialization and procedural world
//! generation. It creates the game world, spawns provinces, generates
//! terrain, places nations, and initializes all game systems.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use noise::Perlin;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::HashMap;

use crate::components::{Province, Nation, ProvinceInfrastructure};
use crate::resources::{WorldSeed, WorldSize, ProvincesSpatialIndex, SelectedProvinceInfo};
use crate::terrain::{TerrainType, classify_terrain_with_climate, generate_elevation_with_edges, get_terrain_population_multiplier};
use crate::colors::get_terrain_color_gradient;
use crate::clouds::spawn_clouds;
use crate::constants::*;
use crate::minerals::generate_world_minerals;

// ============================================================================
// TEXTURE GENERATION
// ============================================================================

/// Create a hexagon texture for sprite rendering with antialiasing
/// This is intentionally called only once to create a shared texture for ALL sprites
/// This enables sprite batching for massive performance gains
pub fn create_hexagon_texture(size: f32) -> Image {
    // FLAT-TOP hexagon dimensions: width = 2*radius, height = sqrt(3)*radius
    let texture_width = (size * 2.0) as u32;
    let texture_height = (size * SQRT3) as u32;
    let mut pixels = vec![0u8; (texture_width * texture_height * 4) as usize];
    
    let center_x = texture_width as f32 / 2.0;
    let center_y = texture_height as f32 / 2.0;
    let radius = size; // Full size to touch borders
    
    // Draw hexagon with antialiased edges
    for y in 0..texture_height {
        for x in 0..texture_width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            
            // Check distance to FLAT-TOP hexagon boundaries
            let abs_x = dx.abs();
            let abs_y = dy.abs();
            // Calculate distance from hexagon edge (negative = inside, positive = outside)
            let dist_horizontal = abs_y - radius * SQRT3 / 2.0; // Distance from horizontal (flat) sides
            let dist_diagonal = (abs_y + SQRT3 * abs_x) / 2.0 - radius * SQRT3 / 2.0; // Distance from diagonal
            
            // Take the maximum distance (closest to being outside)
            let distance_from_edge = dist_horizontal.max(dist_diagonal);
            
            // Apply antialiasing using smooth transition
            let aa_width = HEXAGON_AA_WIDTH;
            let alpha = if distance_from_edge <= -aa_width {
                TEXTURE_ALPHA_OPAQUE // Fully inside
            } else if distance_from_edge >= aa_width {
                TEXTURE_ALPHA_TRANSPARENT // Fully outside
            } else {
                // Smooth transition zone
                let t = (aa_width - distance_from_edge) / (aa_width * 2.0);
                (t * TEXTURE_ALPHA_OPAQUE as f32) as u8
            };
            
            let idx = ((y * texture_width + x) * 4) as usize;
            pixels[idx] = TEXTURE_ALPHA_OPAQUE;     // R (white, will be tinted)
            pixels[idx + 1] = TEXTURE_ALPHA_OPAQUE; // G
            pixels[idx + 2] = TEXTURE_ALPHA_OPAQUE; // B
            pixels[idx + 3] = alpha; // A (smooth edges)
        }
    }
    
    let mut image = Image::new(
        Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    // Use linear filtering for smoother edges
    image.sampler = ImageSampler::linear();
    
    image
}


// ============================================================================
// WORLD SETUP
// ============================================================================

/// Initial world setup - generates the entire game world
pub fn setup_world(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<WorldSeed>,
    size: Res<WorldSize>,
) {
    // Camera setup is now handled by CameraPlugin
    
    // Initialize spatial index for fast province lookups
    let mut spatial_index = ProvincesSpatialIndex::default();
    
    // UI setup is now handled by UIPlugin
    
    // Initialize Perlin noise with seed
    let perlin = Perlin::new(seed.0);
    let mut rng = StdRng::seed_from_u64(seed.0 as u64);
    
    // Define map dimensions based on selected world size
    let (provinces_per_row, provinces_per_col) = size.dimensions();
    let provinces_per_row = provinces_per_row as u32;
    let provinces_per_col = provinces_per_col as u32;
    
    // Calculate actual map bounds based on hex grid coordinates
    // FLAT-TOP hexagon map bounds for honeycomb pattern
    let map_x_min = -(provinces_per_row as f32 / 2.0) * HEX_SIZE_PIXELS * 1.5; // Column spacing = 3/2 * radius
    let map_x_max = (provinces_per_row as f32 / 2.0) * HEX_SIZE_PIXELS * 1.5;
    let map_y_min = -(provinces_per_col as f32 / 2.0) * HEX_SIZE_PIXELS * SQRT3; // Row spacing = sqrt(3) * radius
    let map_y_max = (provinces_per_col as f32 / 2.0) * HEX_SIZE_PIXELS * SQRT3;
    
    println!("Map bounds: X({:.0} to {:.0}), Y({:.0} to {:.0})", 
             map_x_min, map_x_max, map_y_min, map_y_max);
    
    // Tectonic plate system for realistic continent distribution
    let num_plates = TECTONIC_PLATES_BASE + (seed.0 % TECTONIC_PLATES_VARIATION) as usize;
    let mut plate_centers = Vec::new();
    let mut continent_centers = Vec::new();
    
    // Place tectonic plates randomly across the ENTIRE map
    for _i in 0..num_plates {
        let px = rng.gen_range(map_x_min * 0.95..map_x_max * 0.95);
        let py = rng.gen_range(map_y_min * 0.95..map_y_max * 0.95);
        plate_centers.push((px, py));
        
        // 90% chance this plate has a continent on it for 25% land coverage
        if rng.gen_range(0.0..1.0) < 0.9 {
            // Continent offset from plate center (for variety)
            let offset_x = rng.gen_range(-200.0..200.0);
            let offset_y = rng.gen_range(-150.0..150.0);
            continent_centers.push((px + offset_x, py + offset_y));
        }
    }
    
    // Add island chains at plate boundaries (convergent zones)
    for _ in 0..ISLAND_CHAIN_COUNT {
        if plate_centers.len() >= 2 {
            let idx1 = rng.gen_range(0..plate_centers.len());
            let idx2 = rng.gen_range(0..plate_centers.len());
            if idx1 != idx2 {
                let (p1x, p1y) = plate_centers[idx1];
                let (p2x, p2y) = plate_centers[idx2];
                // Place small island chains along plate boundaries
                let mix = rng.gen_range(0.3..0.7);
                let island_x = p1x * (1.0 - mix) + p2x * mix;
                let island_y = p1y * (1.0 - mix) + p2y * mix;
                continent_centers.push((island_x, island_y));
            }
        }
    }
    
    // Add archipelagos between major continents
    for _ in 0..ARCHIPELAGO_COUNT {
        // Place archipelagos in open ocean areas
        let arch_x = rng.gen_range(map_x_min * 0.8..map_x_max * 0.8);
        let arch_y = rng.gen_range(map_y_min * 0.8..map_y_max * 0.8);
        
        // Create a cluster of small islands
        for _ in 0..rng.gen_range(3..7) {
            let offset_x = rng.gen_range(-300.0..300.0);
            let offset_y = rng.gen_range(-300.0..300.0);
            continent_centers.push((arch_x + offset_x, arch_y + offset_y));
        }
    }
    
    println!("Generated {} tectonic plates with {} landmasses", 
             num_plates, continent_centers.len());
    
    // Create a single hexagon texture to be shared by ALL sprites (massive performance boost!)
    let hexagon_texture = create_hexagon_texture(HEX_SIZE_PIXELS);
    let hexagon_handle = images.add(hexagon_texture);
    
    // Generate provinces with terrain using the dimensions defined above
    let hex_size = HEX_SIZE_PIXELS;
    
    let mut land_provinces = Vec::new();
    let mut all_provinces = Vec::new();
    let mut ocean_positions = Vec::new();
    let mut land_positions = Vec::new();
    
    // First pass: generate terrain and collect positions
    for row in 0..provinces_per_col {
        for col in 0..provinces_per_row {
            let province_id = row * provinces_per_row + col;
            
            // Calculate FLAT-TOP hexagon position for HONEYCOMB pattern
            let (x, y) = calculate_hex_position(col, row, hex_size, provinces_per_row, provinces_per_col);
            
            // Generate elevation and terrain with climate
            let map_width = provinces_per_row as f32 * HEX_SIZE_PIXELS * 1.5;
            let map_height = provinces_per_col as f32 * HEX_SIZE_PIXELS * SQRT3;
            let elevation = generate_elevation_with_edges(x, y, &perlin, &continent_centers, map_width, map_height);
            let terrain = classify_terrain_with_climate(elevation, y, map_height);
            let _terrain_color = get_terrain_color_gradient(terrain, elevation);
            
            // Track land and ocean positions for depth calculation
            if terrain != TerrainType::Ocean {
                land_provinces.push((province_id, Vec2::new(x, y)));
                land_positions.push(Vec2::new(x, y));
            } else {
                ocean_positions.push((province_id, Vec2::new(x, y)));
            }
            
            // Create province data with deterministic population based on ID
            let base_pop = if terrain == TerrainType::Ocean { 
                0.0 
            } else {
                // Deterministic population based on province ID and terrain
                let pop_seed = (province_id as u32).wrapping_mul(2654435761); // Golden ratio hash
                let pop_factor = (pop_seed % 1000) as f32 / 1000.0; // 0.0 to 1.0
                let terrain_multiplier = get_terrain_population_multiplier(terrain);
                PROVINCE_MIN_POPULATION + pop_factor * PROVINCE_MAX_ADDITIONAL_POPULATION * terrain_multiplier
            };
            
            let province = Province {
                id: province_id,
                position: Vec2::new(x, y),
                nation_id: None,  // Will assign nations later
                population: base_pop,
                terrain,
                elevation,
                agriculture: 0.0,  // Will be calculated later based on rivers
                fresh_water_distance: f32::MAX,  // Will be calculated after rivers
            };
            
            all_provinces.push(province.clone());
        }
    }
    
    // Second pass: calculate ocean depths more efficiently
    // Build spatial grid for land positions for O(1) lookups
    let grid_size = hex_size * OCEAN_DEPTH_GRID_SIZE_MULTIPLIER;
    let mut land_grid: HashMap<(i32, i32), Vec<Vec2>> = HashMap::new();
    
    for land_pos in land_positions.iter() {
        let grid_x = (land_pos.x / grid_size).floor() as i32;
        let grid_y = (land_pos.y / grid_size).floor() as i32;
        land_grid.entry((grid_x, grid_y))
            .or_insert_with(Vec::new)
            .push(*land_pos);
    }
    
    // Now calculate ocean depths with spatial lookup
    for (ocean_id, ocean_pos) in ocean_positions.iter() {
        if let Some(province) = all_provinces.iter_mut().find(|p| p.id == *ocean_id) {
            // Check nearby grid cells only (9-cell neighborhood)
            let grid_x = (ocean_pos.x / grid_size).floor() as i32;
            let grid_y = (ocean_pos.y / grid_size).floor() as i32;
            
            let mut min_dist_to_land = f32::MAX;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if let Some(land_tiles) = land_grid.get(&(grid_x + dx, grid_y + dy)) {
                        for land_pos in land_tiles {
                            let dist = ocean_pos.distance(*land_pos);
                            min_dist_to_land = min_dist_to_land.min(dist);
                        }
                    }
                }
            }
            
            // If no land found nearby, it's deep ocean
            if min_dist_to_land == f32::MAX {
                province.elevation = 0.02;  // Deep ocean
            } else {
                // Assign depth based on distance
                let hex_distance = min_dist_to_land / hex_size;
                if hex_distance <= 1.8 {
                    province.elevation = 0.12;  // Shallow water
                } else if hex_distance <= 5.0 {
                    province.elevation = 0.07;  // Medium depth
                } else {
                    province.elevation = 0.02;  // Deep ocean
                }
            }
        }
    }
    
    // Generate rivers from mountains to ocean
    println!("Generating rivers...");
    let mut river_tiles = Vec::new();
    let mut delta_tiles = Vec::new();  // Track where rivers meet ocean
    let mut mountain_provinces = Vec::new();
    
    // Find all mountain provinces that could be river sources
    for province in all_provinces.iter() {
        if province.terrain == TerrainType::Mountains && province.elevation >= RIVER_MIN_ELEVATION {
            mountain_provinces.push((province.id, province.position));
        }
    }
    
    // Randomly select some mountains to be river sources
    let num_rivers = RIVER_COUNT.min(mountain_provinces.len());
    let mut selected_sources: Vec<(u32, Vec2)> = Vec::new();
    
    for _ in 0..num_rivers {
        if !mountain_provinces.is_empty() {
            let idx = rng.gen_range(0..mountain_provinces.len());
            let source = mountain_provinces.remove(idx);
            selected_sources.push(source);
        }
    }
    
    // Trace rivers from each source to the ocean
    for (_source_id, source_pos) in selected_sources {
        let mut current_pos = source_pos;
        let mut river_path = Vec::new();
        let mut visited = std::collections::HashSet::new();
        visited.insert((current_pos.x as i32, current_pos.y as i32));
        
        // Follow downhill gradient until we reach ocean
        let mut steps = 0;
        const MAX_RIVER_LENGTH: usize = 100;
        
        while steps < MAX_RIVER_LENGTH {
            steps += 1;
            
            // Find the lowest neighboring province
            let mut lowest_neighbor: Option<(Vec2, f32, u32)> = None;
            let search_radius = hex_size * 1.8; // Look at immediate neighbors
            
            for province in all_provinces.iter() {
                let dist = province.position.distance(current_pos);
                if dist > 0.1 && dist <= search_radius {
                    let grid_pos = (province.position.x as i32, province.position.y as i32);
                    
                    // Skip if we've already visited this tile
                    if visited.contains(&grid_pos) {
                        continue;
                    }
                    
                    // If we hit ocean, we're done - mark as delta
                    if province.terrain == TerrainType::Ocean {
                        // The last river tile becomes a delta
                        if !river_path.is_empty() {
                            delta_tiles.push(*river_path.last().unwrap());
                        }
                        // Also mark ocean tiles adjacent to the river mouth
                        delta_tiles.push(province.id);
                        steps = MAX_RIVER_LENGTH; // Exit outer loop
                        break;
                    }
                    
                    // Otherwise, find the lowest elevation neighbor
                    if lowest_neighbor.is_none() || province.elevation < lowest_neighbor.as_ref().unwrap().1 {
                        lowest_neighbor = Some((province.position, province.elevation, province.id));
                    }
                }
            }
            
            // If we found a lower neighbor, continue the river
            if let Some((next_pos, _elev, next_id)) = lowest_neighbor {
                river_path.push(next_id);
                current_pos = next_pos;
                visited.insert((next_pos.x as i32, next_pos.y as i32));
            } else {
                // No lower neighbor found, end the river
                break;
            }
        }
        
        // Add this river's path to our collection
        river_tiles.extend(river_path);
    }
    
    // Convert river tiles to River terrain type
    for river_id in river_tiles.iter() {
        if let Some(province) = all_provinces.iter_mut().find(|p| p.id == *river_id) {
            // Only convert non-ocean tiles to rivers
            if province.terrain != TerrainType::Ocean {
                province.terrain = TerrainType::River;
                // Rivers increase population in surrounding areas
                province.population *= 1.5;
            }
        }
    }
    
    // Create river deltas at river mouths
    for delta_id in delta_tiles.iter() {
        if let Some(delta_province) = all_provinces.iter_mut().find(|p| p.id == *delta_id) {
            // Convert river mouths and adjacent tiles to fertile deltas
            if delta_province.terrain != TerrainType::Ocean {
                delta_province.terrain = TerrainType::Delta;
                // Deltas are extremely fertile - huge population boost
                delta_province.population *= 2.0;
            }
            
            // Also convert nearby land tiles to delta (spreading fertility)
            let delta_pos = delta_province.position;
            let delta_radius = hex_size * 2.5; // Affect 2-hex radius
            
            for nearby_province in all_provinces.iter_mut() {
                if nearby_province.position.distance(delta_pos) <= delta_radius {
                    // Convert beach/plains near river mouths to delta
                    if matches!(nearby_province.terrain, TerrainType::Beach | TerrainType::Plains) {
                        nearby_province.terrain = TerrainType::Delta;
                        nearby_province.population *= 1.5;
                    }
                }
            }
        }
    }
    
    println!("Generated {} river tiles and {} delta tiles", river_tiles.len(), delta_tiles.len());
    
    // ============================================================================
    // CALCULATE FRESH WATER DISTANCE AND AGRICULTURE
    // ============================================================================
    
    println!("Calculating fresh water distance and agriculture zones...");
    
    // First, collect all fresh water sources (rivers and deltas)
    let mut fresh_water_positions = Vec::new();
    for province in all_provinces.iter() {
        if matches!(province.terrain, TerrainType::River | TerrainType::Delta) {
            fresh_water_positions.push(province.position);
        }
    }
    
    // Calculate distance to nearest fresh water for each province
    for province in all_provinces.iter_mut() {
        if province.terrain != TerrainType::Ocean {
            // Find minimum distance to any fresh water source
            let mut min_distance = f32::MAX;
            for water_pos in fresh_water_positions.iter() {
                let distance = province.position.distance(*water_pos) / hex_size; // Convert to hex units
                min_distance = min_distance.min(distance);
            }
            province.fresh_water_distance = min_distance;
            
            // Calculate agriculture based on terrain and water proximity
            province.agriculture = match province.terrain {
                TerrainType::Delta => 3.0,  // Deltas are extremely fertile
                TerrainType::River => 2.5,  // Rivers have excellent agriculture
                TerrainType::Plains | TerrainType::Forest => {
                    // Agriculture decreases with distance from water
                    if min_distance <= 1.0 {
                        2.0  // Adjacent to water - very fertile
                    } else if min_distance <= 2.0 {
                        1.5  // 1 hex from water - good farming
                    } else if min_distance <= 3.0 {
                        1.0  // 2 hexes from water - moderate farming
                    } else {
                        0.5  // Far from water - poor farming
                    }
                },
                TerrainType::Beach => {
                    if min_distance <= 1.0 { 1.0 } else { 0.5 }
                },
                TerrainType::Desert => {
                    // Desert farming only possible near water (oases)
                    if min_distance <= 1.0 { 1.0 } else { 0.1 }
                },
                TerrainType::Jungle => {
                    if min_distance <= 2.0 { 1.2 } else { 0.6 }
                },
                TerrainType::Hills => {
                    if min_distance <= 2.0 { 0.8 } else { 0.4 }
                },
                _ => 0.1,  // Mountains, Ice, Tundra - minimal agriculture
            };
        }
    }
    
    let high_agriculture_count = all_provinces.iter()
        .filter(|p| p.agriculture >= 1.5)
        .count();
    println!("Set up {} provinces with high agriculture (>= 1.5)", high_agriculture_count);
    
    // ============================================================================
    // MINERAL RESOURCE GENERATION
    // ============================================================================
    
    // Generate all mineral resources using centralized function in minerals.rs
    let province_resources = generate_world_minerals(seed.0, &all_provinces);
    
    // Now spawn all provinces with correct depths
    for province in all_provinces.iter() {
        let row = province.id / provinces_per_row;
        let col = province.id % provinces_per_row;
        
        // Recalculate position (MUST match first pass exactly!)
        let (x, y) = calculate_hex_position(col, row, hex_size, provinces_per_row, provinces_per_col);
        
        // Get the color based on nation ownership or terrain
        let province_color = if let Some(nation_id) = province.nation_id {
            // Use nation color with slight terrain tinting
            let hue = nation_id as f32 / 8.0;
            let nation_color = Color::hsl(hue * 360.0, 0.7, 0.5);
            // Blend with terrain for variation
            let terrain_color = get_terrain_color_gradient(province.terrain, province.elevation);
            Color::srgb(
                nation_color.to_srgba().red * 0.8 + terrain_color.to_srgba().red * 0.2,
                nation_color.to_srgba().green * 0.8 + terrain_color.to_srgba().green * 0.2,
                nation_color.to_srgba().blue * 0.8 + terrain_color.to_srgba().blue * 0.2,
            )
        } else {
            // Ocean or unowned - use terrain color
            get_terrain_color_gradient(province.terrain, province.elevation)
        };
        
        // Get the mineral resources for this province
        let resources = province_resources.get(&province.id)
            .cloned()
            .unwrap_or_default();
        
        // Spawn province entity with SPRITE (much faster than Mesh2d!)
        // Sprites batch automatically when using the same texture
        let entity = commands.spawn((
            province.clone(),
            resources,  // Add mineral resources
            ProvinceInfrastructure::default(),  // Start with no infrastructure
            Sprite {
                image: hexagon_handle.clone(),  // Share the SAME texture handle for batching!
                color: province_color,  // Tint with nation or terrain color
                // FLAT-TOP: Width = 2 * radius, Height = sqrt(3) * radius
                custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * SQRT3)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            Name::new(format!("Province {}", province.id)),
        )).id();
        
        // Add to spatial index for O(1) lookups
        spatial_index.insert(entity, Vec2::new(x, y), province.id);
    }
    
    // Place nations on land using flood fill from random capitals
    if !land_provinces.is_empty() {
        let nation_count = NATION_COUNT.min(land_provinces.len());
        let mut nations = Vec::new();
        let mut nation_capitals = Vec::new();
        
        // Create nations with distinct colors
        for i in 0..nation_count {
            let hue = i as f32 / nation_count as f32;
            let nation = Nation {
                id: i as u32,
                name: format!("Nation {}", i),
                color: Color::hsl(hue * 360.0, 0.7, 0.5),
            };
            nations.push(nation.clone());
            commands.spawn(nation);
            
            // Pick a random capital for this nation
            let capital_idx = rng.gen_range(0..land_provinces.len());
            let (capital_id, capital_pos) = land_provinces[capital_idx];
            nation_capitals.push((i as u32, capital_id, capital_pos));
        }
        
        // Simple distance-based assignment (flood fill would be better but this works)
        // Assign each land province to the nearest nation capital
        for province in all_provinces.iter_mut() {
            if province.terrain != TerrainType::Ocean {
                let mut min_distance = f32::MAX;
                let mut closest_nation = 0;
                
                for &(nation_id, _capital_id, capital_pos) in nation_capitals.iter() {
                    let distance = province.position.distance(capital_pos);
                    if distance < min_distance {
                        min_distance = distance;
                        closest_nation = nation_id;
                    }
                }
                
                province.nation_id = Some(closest_nation);
            }
        }
    }
    
    // Game time is already initialized by main()
    
    // Initialize selected province resource
    commands.insert_resource(SelectedProvinceInfo::default());
    
    // Insert spatial index as a resource for O(1) province lookups
    commands.insert_resource(spatial_index);
    
    // Initialize cloud system using the clouds module
    let map_width = provinces_per_row as f32 * HEX_SIZE_PIXELS * 1.5;
    let map_height = provinces_per_col as f32 * HEX_SIZE_PIXELS * SQRT3;
    spawn_clouds(&mut commands, &mut images, seed.0, map_width, map_height);
    
    println!("Generated world with {} provinces, {} land tiles", 
             provinces_per_row * provinces_per_col, land_provinces.len());
}