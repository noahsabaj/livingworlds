//! Living Worlds - Main entry point
//! 
//! An observer civilization simulation built with Bevy

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Import from our library
use living_worlds::prelude::*;
use living_worlds::{
    build_app, WorldSeed, WorldSize, ShowFps, GameTime,
    HEX_SIZE_PIXELS, PROVINCES_PER_ROW, PROVINCES_PER_COL,
};
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::collections::HashMap;
use noise::{NoiseFn, Perlin};

/// Living Worlds - A procedural civilization simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed for world generation (defaults to current timestamp)
    #[arg(short, long)]
    seed: Option<u32>,

    /// World size (small=1000, medium=2000, large=5000)
    #[arg(short, long, default_value = "medium")]
    world_size: String,

    /// Run in test mode
    #[arg(long)]
    test: bool,
}

fn main() {
    let mut args = Args::parse();
    
    // Use system time as seed if not provided
    let seed = args.seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    });
    args.seed = Some(seed);
    
    println!("Living Worlds - Starting with seed: {}", seed);
    println!("World size: {}", args.world_size);
    
    // Platform integration can be added here later
    
    // Build the Bevy app using our library function
    let mut app = build_app();
    
    // Add game-specific resources and configuration
    app.insert_resource(WorldSeed(args.seed.unwrap()))
        .insert_resource(WorldSize::from_str(&args.world_size))
        .insert_resource(ShowFps(true))  // FPS display always on for now
        .insert_resource(GameTime::default())
        // Add our game systems
        .add_systems(Startup, setup_world)
        .add_systems(Update, (
            handle_input,
            handle_tile_selection,
            draw_hexagon_borders,
            update_provinces,
            simulate_time,
            update_tile_info_ui,
        ))
        .run();
}

// Resources are now defined in lib.rs

// TerrainType and ClimateZone are now imported from terrain module

#[derive(Component, Clone)]
struct Province {
    id: u32,
    position: Vec2,
    nation_id: Option<u32>,  // None for ocean provinces
    population: f32,
    terrain: TerrainType,
    elevation: f32,
}

/// Marker for selected province
#[derive(Component)]
struct SelectedProvince;

/// Marker for ghost provinces (duplicates for wrapping)
#[derive(Component)]
struct GhostProvince {
    original_col: u32,  // Original column this is a ghost of
}

/// Resource tracking the currently selected province
#[derive(Resource, Default)]
struct SelectedProvinceInfo {
    entity: Option<Entity>,
    province_id: Option<u32>,
}

/// Spatial index for O(1) province lookups instead of O(n)
#[derive(Resource)]
struct ProvincesSpatialIndex {
    // Grid cell size should be about hexagon size for optimal performance
    cell_size: f32,
    // HashMap: grid_coord -> list of (entity, position, province_id)
    grid: HashMap<(i32, i32), Vec<(Entity, Vec2, u32)>>,
}

impl Default for ProvincesSpatialIndex {
    fn default() -> Self {
        Self {
            cell_size: HEX_SIZE_PIXELS * 2.0, // 2x hex size for good coverage
            grid: HashMap::new(),
        }
    }
}

impl ProvincesSpatialIndex {
    /// Insert a province into the spatial index
    fn insert(&mut self, entity: Entity, position: Vec2, province_id: u32) {
        let grid_x = (position.x / self.cell_size).floor() as i32;
        let grid_y = (position.y / self.cell_size).floor() as i32;
        
        self.grid
            .entry((grid_x, grid_y))
            .or_insert_with(Vec::new)
            .push((entity, position, province_id));
    }
    
    /// Query provinces near a world position
    fn query_near(&self, world_pos: Vec2, search_radius: f32) -> Vec<(Entity, Vec2, u32)> {
        let mut results = Vec::new();
        
        // Calculate grid cells to check based on search radius
        let min_x = ((world_pos.x - search_radius) / self.cell_size).floor() as i32;
        let max_x = ((world_pos.x + search_radius) / self.cell_size).floor() as i32;
        let min_y = ((world_pos.y - search_radius) / self.cell_size).floor() as i32;
        let max_y = ((world_pos.y + search_radius) / self.cell_size).floor() as i32;
        
        // Check all relevant grid cells
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(provinces) = self.grid.get(&(x, y)) {
                    for &(entity, pos, id) in provinces {
                        let dist = world_pos.distance(pos);
                        if dist <= search_radius {
                            results.push((entity, pos, id));
                        }
                    }
                }
            }
        }
        
        results
    }
}

/// Marker for the tile info UI panel
#[derive(Component)]
struct TileInfoPanel;

/// Marker for the tile info text
#[derive(Component)]
struct TileInfoText;

/// Component marking a nation
#[derive(Component, Clone)]
struct Nation {
    id: u32,
    name: String,
    color: Color,
}

// GameTime is now defined in lib.rs

/// Generate elevation using advanced noise techniques for main game logic
/// This version includes map edge handling specific to our game
fn generate_elevation_with_edges(x: f32, y: f32, perlin: &Perlin, continent_centers: &[(f32, f32)]) -> f32 {
    // Calculate map bounds dynamically based on grid dimensions
    let hex_size = HEX_SIZE_PIXELS;
    let provinces_per_row = PROVINCES_PER_ROW;
    let provinces_per_col = PROVINCES_PER_COL;
    // FLAT-TOP HONEYCOMB: Column spacing is 1.5 * radius, row spacing is sqrt(3) * radius
    let map_bound_x = (provinces_per_row as f32 * hex_size * 1.5) / 2.0;
    let map_bound_y = (provinces_per_col as f32 * hex_size * SQRT3) / 2.0;
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
    
    // Ridge noise for mountain chains (inverted absolute value)
    let ridge_scale = 0.003;
    let ridge = 1.0 - (perlin.get([wx as f64 * ridge_scale, wy as f64 * ridge_scale]) as f32 * 2.0).abs();
    let ridge_contribution = ridge * 0.3;
    
    // Combine noise layers
    let mut elevation = (base + detail + fine + ridge_contribution) / 2.0 + 0.5;
    
    // Multiple continent masks with fractal distortion for natural coastlines  
    let mut continent_influence: f32 = 0.0;
    
    for (idx, &(cx, cy)) in continent_centers.iter().enumerate() {
        let dist = ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
        
        // Add multi-scale noise distortion for realistic, fractal coastlines
        let distortion_scale = 0.001;
        // Large scale features (continents/peninsulas)
        let distortion1 = perlin.get([
            (x + cx * 0.1) as f64 * distortion_scale * 0.5, 
            (y + cy * 0.1) as f64 * distortion_scale * 0.5
        ]) as f32 * 400.0;
        // Medium scale features (bays/capes)
        let distortion2 = perlin.get([
            x as f64 * distortion_scale * 2.0, 
            y as f64 * distortion_scale * 2.0
        ]) as f32 * 200.0;
        // Fine detail (rough coastline)
        let distortion3 = perlin.get([
            x as f64 * distortion_scale * 8.0, 
            y as f64 * distortion_scale * 8.0
        ]) as f32 * 50.0;
        
        // Apply fractal distortion
        let distorted_dist = dist + distortion1 + distortion2 * 0.5 + distortion3 * 0.25;
        
        // Vary continent sizes dramatically based on index
        let continent_seed = (idx as u32).wrapping_mul(2654435761) % 1000;
        let size_factor = continent_seed as f32 / 1000.0;
        
        let base_radius = if idx >= 12 {
            // Tiny islands
            300.0 + size_factor * 200.0  // 300-500 (doubled)
        } else if idx >= 7 {
            // Archipelagos and island chains
            600.0 + size_factor * 400.0  // 600-1000 (doubled)
        } else if idx >= 4 {
            // Medium continents (Australia-sized)
            1200.0 + size_factor * 600.0  // 1200-1800 (doubled)
        } else {
            // Massive continents (Eurasia-sized)
            2000.0 + size_factor * 1000.0  // 2000-3000 (doubled)
        };
        
        // Smooth falloff with varying sharpness for different edge types
        let falloff = 1.0 - (distorted_dist / base_radius).clamp(0.0, 1.0);
        let shaped_falloff = falloff.powf(1.2 + size_factor * 0.8);
        
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
    
    // For ocean tiles, set a base ocean elevation
    // We'll calculate proper depth in a second pass after we know all land positions
    if elevation < 0.01 {
        elevation = 0.05; // Temporary ocean value
    }
    
    elevation
}

// All terrain functions are now imported from terrain module


// FpsText component is now in ui module

// Cloud system components and resources are now in clouds module

/// Create a hexagon texture for sprite rendering with antialiasing
/// BUG #8 FIX: This is intentionally called only once to create a shared texture for ALL sprites
/// This enables sprite batching for massive performance gains
fn create_hexagon_texture(size: f32) -> Image {
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
            let aa_width = 1.5; // Width of antialiasing in pixels
            let alpha = if distance_from_edge <= -aa_width {
                255 // Fully inside
            } else if distance_from_edge >= aa_width {
                0 // Fully outside
            } else {
                // Smooth transition zone
                let t = (aa_width - distance_from_edge) / (aa_width * 2.0);
                (t * 255.0) as u8
            };
            
            let idx = ((y * texture_width + x) * 4) as usize;
            pixels[idx] = 255;     // R (white, will be tinted)
            pixels[idx + 1] = 255; // G
            pixels[idx + 2] = 255; // B
            pixels[idx + 3] = alpha; // A (smooth edges)
        }
    }
    
    let mut image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        pixels,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    
    // Use linear filtering for smoother edges
    image.sampler = bevy::image::ImageSampler::linear();
    
    image
}

/// Initial world setup
fn setup_world(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    seed: Res<WorldSeed>,
    _size: Res<WorldSize>,
) {
    // Camera setup is now handled by CameraPlugin
    
    // Initialize spatial index for fast province lookups
    let mut spatial_index = ProvincesSpatialIndex::default();
    
    // UI setup is now handled by UIPlugin
    
    // Initialize Perlin noise with seed
    let perlin = Perlin::new(seed.0);
    let mut rng = StdRng::seed_from_u64(seed.0 as u64);
    
    // Define map dimensions first - MASSIVE world
    let provinces_per_row = PROVINCES_PER_ROW;
    let provinces_per_col = PROVINCES_PER_COL;
    
    // Calculate actual map bounds based on hex grid coordinates
    // POINTY-TOP hexagon map bounds (correct spacing)
    let map_x_min = -(provinces_per_row as f32 / 2.0) * HEX_SIZE_PIXELS * SQRT3; // sqrt(3) horizontal
    let map_x_max = (provinces_per_row as f32 / 2.0) * HEX_SIZE_PIXELS * SQRT3;
    let map_y_min = -(provinces_per_col as f32 / 2.0) * HEX_SIZE_PIXELS * 1.5; // 3/2 vertical
    let map_y_max = (provinces_per_col as f32 / 2.0) * HEX_SIZE_PIXELS * 1.5;
    
    println!("Map bounds: X({:.0} to {:.0}), Y({:.0} to {:.0})", 
             map_x_min, map_x_max, map_y_min, map_y_max);
    
    // Tectonic plate system for realistic continent distribution
    let num_plates = 15 + (seed.0 % 10) as usize; // 15-25 plates for bigger map
    let mut plate_centers = Vec::new();
    let mut continent_centers = Vec::new();
    
    // Place tectonic plates randomly across the ENTIRE map
    for _i in 0..num_plates {
        let px = rng.gen_range(map_x_min * 0.95..map_x_max * 0.95);
        let py = rng.gen_range(map_y_min * 0.95..map_y_max * 0.95);
        plate_centers.push((px, py));
        
        // 100% chance this plate has a continent on it (was 70%)
        if rng.gen_range(0.0..1.0) < 1.0 {
            // Continent offset from plate center (for variety)
            let offset_x = rng.gen_range(-200.0..200.0);
            let offset_y = rng.gen_range(-150.0..150.0);
            continent_centers.push((px + offset_x, py + offset_y));
        }
    }
    
    // Add island chains at plate boundaries (convergent zones)
    for _ in 0..8 {
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
            // Odd columns shift UP by half the vertical spacing for tessellation
            let y_offset = if col % 2 == 1 { hex_size * SQRT3 / 2.0 } else { 0.0 };
            let x = (col as f32 - provinces_per_row as f32 / 2.0) * hex_size * 1.5;
            let y = (row as f32 - provinces_per_col as f32 / 2.0) * hex_size * SQRT3 + y_offset;
            
            // Generate elevation and terrain with climate
            let elevation = generate_elevation_with_edges(x, y, &perlin, &continent_centers);
            let map_height = provinces_per_col as f32 * HEX_SIZE_PIXELS * 1.5; // POINTY-TOP height (3/2)
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
                let terrain_multiplier = match terrain {
                    TerrainType::Plains => 1.5,  // More population in plains
                    TerrainType::Beach => 1.2,   // Coastal areas attract people
                    TerrainType::Forest => 1.0,  // Moderate population in forests
                    TerrainType::Jungle => 0.6,  // Dense jungle is harder to settle
                    TerrainType::Hills => 0.8,   // Less in hills
                    TerrainType::Mountains => 0.3, // Few in mountains
                    TerrainType::Desert => 0.4,  // Low in deserts
                    TerrainType::Tundra => 0.2,  // Very low in tundra
                    TerrainType::Ice => 0.0,     // No permanent population on ice
                    _ => 1.0,
                };
                1000.0 + pop_factor * 49000.0 * terrain_multiplier
            };
            
            let province = Province {
                id: province_id,
                position: Vec2::new(x, y),
                nation_id: None,  // Will assign nations later
                population: base_pop,
                terrain,
                elevation,
            };
            
            all_provinces.push(province.clone());
        }
    }
    
    // Second pass: calculate ocean depths more efficiently - BUG #3 FIX
    // Build spatial grid for land positions for O(1) lookups
    let grid_size = hex_size * 3.0; // Grid cells of 3 hex sizes
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
    
    // Now spawn all provinces with correct depths
    for province in all_provinces.iter() {
        let row = province.id / provinces_per_row;
        let col = province.id % provinces_per_row;
        
        // Recalculate position (MUST match first pass exactly!)
        // FLAT-TOP HONEYCOMB: Odd columns shift UP
        let y_offset = if col % 2 == 1 { hex_size * SQRT3 / 2.0 } else { 0.0 };
        let x = (col as f32 - provinces_per_row as f32 / 2.0) * hex_size * 1.5;
        let y = (row as f32 - provinces_per_col as f32 / 2.0) * hex_size * SQRT3 + y_offset;
        
        // Get the color based on nation ownership or terrain - BUG #6 FIX
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
        
        // Spawn province entity with SPRITE (much faster than Mesh2d!)
        // Sprites batch automatically when using the same texture
        let entity = commands.spawn((
            province.clone(),
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
        
        // TEMPORARILY DISABLED: Ghost provinces causing double border issues
        // TODO: Re-implement with correct pointy-top offset positioning
        /*
        // Spawn ghost provinces for edge wrapping
        let map_width = provinces_per_row as f32 * hex_size * 1.732; // POINTY-TOP width (sqrt(3))
        let wrap_threshold = 20; // Reduced from 40 - less ghosts, better performance
        
        // Left edge provinces get ghosts on the right
        if col < wrap_threshold {
            let ghost_x = x + map_width;
            commands.spawn((
                province.clone(),
                GhostProvince { original_col: col },
                Sprite {
                    image: hexagon_handle.clone(),  // Same shared texture!
                    color: terrain_color,
                    // FLAT-TOP: Width = 2 * radius, Height = sqrt(3) * radius
                custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * SQRT3)),
                    ..default()
                },
                Transform::from_xyz(ghost_x, y, 0.0),
                Name::new(format!("Ghost Province {} (Left)", province.id)),
            ));
        }
        
        // Right edge provinces get ghosts on the left
        if col >= provinces_per_row - wrap_threshold {
            let ghost_x = x - map_width;
            commands.spawn((
                province.clone(),
                GhostProvince { original_col: col },
                Sprite {
                    image: hexagon_handle.clone(),  // Same shared texture!
                    color: terrain_color,
                    // FLAT-TOP: Width = 2 * radius, Height = sqrt(3) * radius
                custom_size: Some(Vec2::new(hex_size * 2.0, hex_size * SQRT3)),
                    ..default()
                },
                Transform::from_xyz(ghost_x, y, 0.0),
                Name::new(format!("Ghost Province {} (Right)", province.id)),
            ));
        }
        */
    }
    
    // Place nations on land using flood fill from random capitals - BUG #6 FIX
    if !land_provinces.is_empty() {
        let nation_count = 8.min(land_provinces.len());
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
        
        // BUG #6 FIX: Simple distance-based assignment (flood fill would be better but this works)
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
    let map_width = provinces_per_row as f32 * hex_size * 1.5;
    let map_height = provinces_per_col as f32 * hex_size * SQRT3;
    spawn_clouds(&mut commands, &mut images, seed.0, map_width, map_height);
    
    println!("Generated world with {} provinces, {} land tiles", 
             provinces_per_row * provinces_per_col, land_provinces.len());
}


// ShowFps is now defined in lib.rs

/// Handle keyboard input
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    _show_fps: ResMut<ShowFps>,
    // TODO: Update to use SimulationState
    // mut simulation: ResMut<SimulationState>,
) {
    // ESC to exit
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

/// Update province colors based on nation control
fn update_provinces(
    _provinces: Query<&Province>,
    _nations: Query<&Nation>,
) {
    // For now just keep the colors - later we'll change nation control
}

/// Draw hexagon borders using Gizmos (with LOD support) - BUG #4 FIX: Limit draw calls
fn draw_hexagon_borders(
    mut gizmos: Gizmos,
    provinces: Query<(&Province, &Transform, Option<&SelectedProvince>, &ViewVisibility)>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection)>,
    time: Res<Time>,
) {
    // Get camera zoom level and position to determine what to draw
    let Ok((camera, camera_transform, projection)) = camera_query.single() else { return; };
    let current_scale = match projection {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => return,
    };
    
    // Don't draw borders when zoomed out too far (improves performance)
    const BORDER_HIDE_THRESHOLD: f32 = 1.5;
    if current_scale > BORDER_HIDE_THRESHOLD {
        return; // Skip all border drawing when zoomed out
    }
    
    let hex_size = HEX_SIZE_PIXELS;
    let camera_pos = camera_transform.translation().truncate();
    
    // Only draw borders for provinces near the camera (BUG #4 FIX)
    let max_draw_distance = camera.viewport.as_ref()
        .map(|vp| vp.physical_size.x.max(vp.physical_size.y) as f32)
        .unwrap_or(2000.0) * current_scale;
    
    let mut borders_drawn = 0;
    const MAX_BORDERS: usize = 500; // Limit number of borders drawn per frame
    
    for (_province, transform, selected, visibility) in provinces.iter() {
        // Skip invisible provinces
        if !visibility.get() {
            continue;
        }
        
        // Skip provinces too far from camera (BUG #4 FIX)
        let distance = camera_pos.distance(transform.translation.truncate());
        if distance > max_draw_distance {
            continue;
        }
        
        // Stop if we've drawn too many borders (BUG #4 FIX)
        if borders_drawn >= MAX_BORDERS && selected.is_none() {
            break;
        }
        
        // Calculate hexagon vertices for FLAT-TOP hexagons
        // Derive from ACTUAL transform position to ensure alignment!
        let mut vertices = Vec::new();
        for i in 0..=6 {
            // FLAT-TOP starts with flat edge at top (no offset needed)
            let angle = (i as f32 * 60.0).to_radians();
            let x = transform.translation.x + hex_size * angle.cos();
            let y = transform.translation.y + hex_size * angle.sin();
            vertices.push(Vec2::new(x, y));
        }
        
        // Choose color based on selection
        let color = if selected.is_some() {
            // Golden shimmer for selected tile - looks professional!
            let shimmer = (time.elapsed_secs() * 3.0).sin() * 0.3 + 0.7;
            Color::srgb(1.0 * shimmer, 0.8 * shimmer, 0.0)
        } else {
            // Darker but thinner borders (using lower alpha for visual thinness)
            // Fade out borders as we zoom out
            let alpha = ((BORDER_HIDE_THRESHOLD - current_scale) / BORDER_HIDE_THRESHOLD).clamp(0.0, 0.5);
            Color::srgba(0.3, 0.3, 0.3, alpha)
        };
        
        // Draw the hexagon border
        gizmos.linestrip_2d(vertices, color);
        borders_drawn += 1;
    }
}

/// Handle mouse clicks for tile selection
fn handle_tile_selection(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    provinces: Query<&Province>,
    selected: Query<Entity, With<SelectedProvince>>,
    mut selected_info: ResMut<SelectedProvinceInfo>,
    spatial_index: Res<ProvincesSpatialIndex>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    
    // Convert screen position to world position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return; };
    let world_pos = ray.origin.truncate();
    
    // Clear previous selection
    for entity in selected.iter() {
        commands.entity(entity).remove::<SelectedProvince>();
    }
    selected_info.entity = None;
    selected_info.province_id = None;
    
    // Find clicked province using spatial index (O(1) instead of O(n))
    let hex_size = HEX_SIZE_PIXELS;
    let search_radius = hex_size; // Search within one hex radius
    
    // Query spatial index for provinces near click position
    let nearby_provinces = spatial_index.query_near(world_pos, search_radius);
    
    // Find the closest province that contains the point
    let mut closest_province = None;
    let mut closest_distance = f32::MAX;
    
    for (entity, pos, province_id) in nearby_provinces {
        let dx = world_pos.x - pos.x;
        let dy = world_pos.y - pos.y;
        
        // Check if point is inside flat-top hexagon
        let abs_x = dx.abs();
        let abs_y = dy.abs();
        
        // Exact flat-top hexagon hit test for HONEYCOMB pattern
        // Check both horizontal bounds and diagonal bounds
        if abs_y <= hex_size * SQRT3 / 2.0 && 
           abs_x <= hex_size &&
           (abs_y / SQRT3 + abs_x / 2.0 <= hex_size) {
            let distance = (dx * dx + dy * dy).sqrt();
            if distance < closest_distance {
                closest_distance = distance;
                closest_province = Some((entity, province_id));
            }
        }
    }
    
    // Select the closest province if found
    if let Some((entity, province_id)) = closest_province {
        commands.entity(entity).insert(SelectedProvince);
        selected_info.entity = Some(entity);
        selected_info.province_id = Some(province_id);
        
        // Get province data for debug output
        if let Ok(province) = provinces.get(entity) {
            println!("Selected province {} at ({:.0}, {:.0}), terrain: {:?}", 
                     province_id, province.position.x, province.position.y, province.terrain);
        }
    }
}

/// Update UI panel showing selected tile info
fn update_tile_info_ui(
    mut commands: Commands,
    selected_info: Res<SelectedProvinceInfo>,
    provinces: Query<&Province>,
    ui_root: Query<Entity, With<TileInfoPanel>>,
    mut text_query: Query<&mut Text, With<TileInfoText>>,
) {
    // Update text if we have a UI panel
    if let Ok(mut text) = text_query.single_mut() {
        if let Some(entity) = selected_info.entity {
            if let Ok(province) = provinces.get(entity) {
                text.0 = format!(
                    "Province #{}\nTerrain: {:?}\nElevation: {:.2}\nPopulation: {:.0}\nPosition: ({:.0}, {:.0})",
                    province.id,
                    province.terrain,
                    province.elevation,
                    province.population,
                    province.position.x,
                    province.position.y,
                );
            }
        } else {
            text.0 = "Click a tile to see info".to_string();
        }
    } else if ui_root.is_empty() {
        // Create UI panel if it doesn't exist - BUG #7 FIX: Responsive scaling
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(2.0),   // 2% from bottom
                left: Val::Percent(2.0),     // 2% from left
                padding: UiRect::all(Val::Percent(1.0)),  // 1% padding
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            TileInfoPanel,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Click a tile to see info"),
                TextFont {
                    font_size: 18.0,  // Slightly larger for readability
                    ..default()
                },
                TextColor(Color::WHITE),
                TileInfoText,
            ));
        });
    }
}

/// Simulate time passing and nations expanding
fn simulate_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut provinces: Query<&mut Province>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut last_year: Local<u64>, // BUG #2 FIX: Thread-safe local state instead of unsafe static
) {
    // Space to pause
    if keyboard.just_pressed(KeyCode::Space) {
        game_time.paused = !game_time.paused;
        println!("Game {}", if game_time.paused { "paused" } else { "resumed" });
    }
    
    // Number keys for speed control
    if keyboard.just_pressed(KeyCode::Digit1) {
        game_time.speed = 0.5;
        println!("Speed: 0.5x");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        game_time.speed = 1.0;
        println!("Speed: 1x");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        game_time.speed = 3.0;
        println!("Speed: 3x");
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        game_time.speed = 10.0;
        println!("Speed: 10x");
    }
    
    if game_time.paused {
        return;
    }
    
    // Advance time (in days)
    game_time.current_date += time.delta().as_secs_f32() * game_time.speed;
    
    // Every 100 days, simulate population growth
    if game_time.current_date as u64 % 100 == 0 {
        let year = 1000 + (game_time.current_date / 365.0) as u64;
        
        // Only print year when it actually changes - BUG #2 FIX: Using Local instead of unsafe static
        if year != *last_year {
            println!("Year {}", year);
            *last_year = year;
        }
        
        // Population growth for land provinces
        for mut province in provinces.iter_mut() {
            if province.terrain != TerrainType::Ocean {
                province.population *= 1.001;
            }
        }
    }
}

// FPS display is now handled by UIPlugin
// animate_clouds is now handled by CloudPlugin

// Camera control is now handled by CameraPlugin