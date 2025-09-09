// ============================================================================
// WORLD GENERATION MODULE
// ============================================================================
// Clean, modular world generation system following SOLID principles
// All generation logic in one file with clear module boundaries

use bevy::prelude::*;
use noise::Perlin;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::{HashMap, HashSet};

use crate::components::{Province, ProvinceResources};
use crate::resources::WorldSize;
use crate::terrain::{TerrainType, classify_terrain_with_climate};
use crate::constants::*;

// ============================================================================
// PUBLIC DATA STRUCTURES
// ============================================================================

/// Complete generated world data, ready for rendering
#[derive(Debug, Clone)]
pub struct GeneratedWorld {
    pub provinces: Vec<Province>,
    pub rivers: RiverSystem,
    pub minerals: HashMap<u32, ProvinceResources>,
    pub spatial_index: HashMap<(i32, i32), u32>,
    pub map_dimensions: MapDimensions,
}

/// River system with flow accumulation tracking
#[derive(Debug, Clone)]
pub struct RiverSystem {
    pub river_tiles: Vec<u32>,        // Province IDs that are rivers
    pub delta_tiles: Vec<u32>,        // Province IDs where rivers meet ocean
    pub flow_accumulation: HashMap<u32, f32>, // How much water flows through each tile
}

/// Map dimension information
#[derive(Debug, Clone, Copy, Resource)]
pub struct MapDimensions {
    pub provinces_per_row: u32,
    pub provinces_per_col: u32,
    pub hex_size: f32,
    pub bounds: MapBounds,
}

#[derive(Debug, Clone, Copy)]
pub struct MapBounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

// ============================================================================
// MAIN WORLD GENERATOR
// ============================================================================

/// Main world generator that orchestrates all generation steps
pub struct WorldGenerator {
    seed: u32,
    size: WorldSize,
    perlin: Perlin,
    rng: StdRng,
    dimensions: MapDimensions,
}

impl WorldGenerator {
    /// Create a new world generator with the given seed and size
    pub fn new(seed: u32, size: WorldSize) -> Self {
        let perlin = Perlin::new(seed);
        let rng = StdRng::seed_from_u64(seed as u64);
        
        let (width, height) = size.dimensions();
        let hex_size = HEX_SIZE_PIXELS;
        
        // Calculate map bounds for flat-top hexagons
        let bounds = MapBounds {
            x_min: -(width as f32 / 2.0) * hex_size * 1.5,
            x_max: (width as f32 / 2.0) * hex_size * 1.5,
            y_min: -(height as f32 / 2.0) * hex_size * SQRT3,
            y_max: (height as f32 / 2.0) * hex_size * SQRT3,
        };
        
        let dimensions = MapDimensions {
            provinces_per_row: width as u32,
            provinces_per_col: height as u32,
            hex_size,
            bounds,
        };
        
        Self {
            seed,
            size,
            perlin,
            rng,
            dimensions,
        }
    }
    
    /// Generate the complete world
    pub fn generate(mut self) -> GeneratedWorld {
        let start_time = std::time::Instant::now();
        
        // Step 1: Generate tectonic plates and continent centers
        println!("Generating tectonic plates...");
        let tectonic_system = tectonics::generate(&mut self.rng, self.dimensions.bounds, self.seed);
        
        // Step 2: Generate provinces with terrain types
        println!("Generating {} provinces...", self.dimensions.provinces_per_row * self.dimensions.provinces_per_col);
        let mut provinces = provinces::generate(
            &tectonic_system,
            self.dimensions,
            &self.perlin,
            &mut self.rng,
        );
        
        // Step 3: Calculate ocean depths
        println!("Calculating ocean depths...");
        provinces::calculate_ocean_depths(&mut provinces, self.dimensions);
        
        // Step 4: Generate river system with proper flow
        println!("Generating river system...");
        let river_system = rivers::generate(&mut provinces, self.dimensions, &mut self.rng);
        
        // Step 5: Calculate agriculture based on water proximity
        println!("Calculating agriculture zones...");
        agriculture::calculate(&mut provinces, &river_system, self.dimensions);
        
        // Step 6: Generate minerals (delegate to existing system)
        println!("Generating mineral resources...");
        let minerals = crate::minerals::generate_world_minerals(self.seed, &provinces);
        
        // Step 7: Build spatial index for fast lookups
        // Use the same cell size calculation as ProvincesSpatialIndex
        use crate::constants::SPATIAL_INDEX_CELL_SIZE_MULTIPLIER;
        let spatial_cell_size = self.dimensions.hex_size * SPATIAL_INDEX_CELL_SIZE_MULTIPLIER;
        
        let mut spatial_index = HashMap::new();
        for province in &provinces {
            let grid_x = (province.position.x / spatial_cell_size).floor() as i32;
            let grid_y = (province.position.y / spatial_cell_size).floor() as i32;
            spatial_index.insert((grid_x, grid_y), province.id);
        }
        println!("Spatial index built with {} grid cells for {} provinces (cell size: {:.1})", 
                 spatial_index.len(), provinces.len(), spatial_cell_size);
        
        let total_time = start_time.elapsed().as_secs_f32();
        println!("World generation completed in {:.2}s", total_time);
        
        GeneratedWorld {
            provinces,
            rivers: river_system,
            minerals,
            spatial_index,
            map_dimensions: self.dimensions,
        }
    }
}

// ============================================================================
// TECTONIC PLATES MODULE
// ============================================================================

mod tectonics {
    use super::*;
    
    #[derive(Debug, Clone)]
    pub struct TectonicSystem {
        pub plate_centers: Vec<(f32, f32)>,
        pub continent_centers: Vec<(f32, f32)>,
    }
    
    pub fn generate(rng: &mut StdRng, bounds: MapBounds, seed: u32) -> TectonicSystem {
        let num_plates = TECTONIC_PLATES_BASE + (seed % TECTONIC_PLATES_VARIATION) as usize;
        let mut plate_centers = Vec::new();
        let mut continent_centers = Vec::new();
        
        // Place tectonic plates randomly across the map
        for _ in 0..num_plates {
            let px = rng.gen_range(bounds.x_min * 0.95..bounds.x_max * 0.95);
            let py = rng.gen_range(bounds.y_min * 0.95..bounds.y_max * 0.95);
            plate_centers.push((px, py));
            
            // 80% chance this plate has a major continent
            if rng.gen_range(0.0..1.0) < 0.8 {
                // Continent offset from plate center for variety
                let offset_x = rng.gen_range(-800.0..800.0);
                let offset_y = rng.gen_range(-600.0..600.0);
                continent_centers.push((px + offset_x, py + offset_y));
            }
        }
        
        println!("Generated {} tectonic plates with {} landmasses", 
                 plate_centers.len(), continent_centers.len());
        
        TectonicSystem {
            plate_centers,
            continent_centers,
        }
    }
}

// ============================================================================
// PROVINCE GENERATION MODULE
// ============================================================================

mod provinces {
    use super::*;
    use rayon::prelude::*;
    
    pub fn generate(
        tectonics: &tectonics::TectonicSystem,
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
        use rayon::prelude::*;
        use std::sync::Arc;
        
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
}

// ============================================================================
// RIVER GENERATION MODULE
// ============================================================================

mod rivers {
    use super::*;
    
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
}

// ============================================================================
// AGRICULTURE MODULE
// ============================================================================

mod agriculture {
    use super::*;
    
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
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

// DELETED: Duplicate hex position function - use crate::constants::calculate_hex_position instead!

/// Get the 6 neighbors of a hexagon in odd-q offset coordinates
pub fn hex_neighbors(col: i32, row: i32) -> Vec<(i32, i32)> {
    if col % 2 == 0 {
        // Even column neighbors
        vec![
            (col, row - 1),     // North
            (col + 1, row - 1), // Northeast
            (col + 1, row),     // Southeast
            (col, row + 1),     // South
            (col - 1, row),     // Southwest
            (col - 1, row - 1), // Northwest
        ]
    } else {
        // Odd column neighbors
        vec![
            (col, row - 1),     // North
            (col + 1, row),     // Northeast
            (col + 1, row + 1), // Southeast
            (col, row + 1),     // South
            (col - 1, row + 1), // Southwest
            (col - 1, row),     // Northwest
        ]
    }
}