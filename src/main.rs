//! Living Worlds - Main entry point
//! 
//! An observer civilization simulation built with Bevy

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Import from our library
use living_worlds::prelude::*;
use living_worlds::{
    build_app, WorldSeed, WorldSize,
    resources::{ResourceOverlay, MapDimensions},
    setup::ProvinceStorage,
};
use std::sync::Once;
use clap::Parser;
use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Living Worlds - A procedural civilization simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed for world generation (defaults to current timestamp)
    #[arg(short, long)]
    seed: Option<u32>,

    /// World size (small, medium, large) - random if not specified
    #[arg(short, long)]
    world_size: Option<String>,

    /// Run in test mode
    #[arg(long)]
    test: bool,
}

fn main() {
    unsafe {
        STARTUP_TIME = Some(std::time::Instant::now());
    }
    
    let mut args = Args::parse();
    
    // Configure rayon thread pool for optimal performance
    // Use 75% of cores to leave room for rendering and OS
    let num_threads = std::thread::available_parallelism()
        .map(|n| ((n.get() * 3) / 4).max(2))  // Use 75% of cores, minimum 2
        .unwrap_or(4);
    
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to initialize rayon thread pool");
    
    println!("Initialized with {} parallel threads (of {} cores total)", 
             num_threads, 
             std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4));
    
    // Use system time as seed if not provided
    let seed = args.seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    });
    args.seed = Some(seed);
    
    // Randomly select world size if not provided
    let world_size = args.world_size.unwrap_or_else(|| {
        let mut rng = thread_rng();
        let sizes = ["small", "medium", "large"];
        sizes.choose(&mut rng).unwrap().to_string()
    });
    
    println!("Living Worlds - Starting with seed: {}", seed);
    println!("World size: {}", world_size);
    
    // Platform integration can be added here later
    
    // Build the Bevy app using our library function
    let mut app = build_app();
    
    // Add game-specific resources and configuration
    let world_size_enum = WorldSize::from_str(&world_size);
    let map_dimensions = MapDimensions::from_world_size(&world_size_enum);
    
    app.insert_resource(WorldSeed(args.seed.unwrap()))
        .insert_resource(world_size_enum)
        .insert_resource(map_dimensions)
        .insert_resource(ResourceOverlay::default())
        // Add our game systems
        .add_systems(Startup, setup_world)
        .add_systems(Update, (
            track_first_frame,
            handle_input,
            handle_overlay_input,
            handle_tile_selection,
            // update_provinces,  // Currently does nothing, commented out for performance
            update_tile_info_ui.run_if(resource_changed::<SelectedProvinceInfo>),
        ))
        .run();
}

// Resources and Components are now defined in lib.rs

// All terrain functions are now imported from terrain module
// All setup functions are now imported from setup module

static FIRST_FRAME: Once = Once::new();
static mut STARTUP_TIME: Option<std::time::Instant> = None;

/// Track when the first frame renders
fn track_first_frame() {
    FIRST_FRAME.call_once(|| {
        unsafe {
            if let Some(start) = STARTUP_TIME {
                println!("First frame rendered after {:.2}s from startup", start.elapsed().as_secs_f32());
            }
        }
    });
}

// Border rendering is now handled by the BorderPlugin in borders.rs
// Using GPU-instanced rendering for 135,000 borders in a single draw call

/// Handle keyboard input
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    // ESC to exit
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    
    // B key toggles borders - handled by BorderPlugin in borders.rs
}

/// Update province colors based on nation control
fn update_provinces(
    _provinces: Query<&Province>,
    _nations: Query<&Nation>,
) {
    // For now just keep the colors - later we'll change nation control
}

// Hexagon border rendering has been moved to borders.rs using GPU instancing
// This eliminates the performance bottleneck of drawing 135,000 Gizmos every frame

/// Handle mouse clicks for tile selection
pub fn handle_tile_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    province_storage: Res<ProvinceStorage>,
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
    
    for (_entity, pos, province_id) in nearby_provinces {
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
                closest_province = Some(province_id);
            }
        }
    }
    
    // Select the closest province if found
    if let Some(province_id) = closest_province {
        selected_info.entity = None;  // No entity in mega-mesh architecture
        selected_info.province_id = Some(province_id);
        
        // Get province data for debug output
        if let Some(province) = province_storage.provinces.get(&province_id) {
            println!("Selected province {} at ({:.0}, {:.0}), terrain: {:?}", 
                     province_id, province.position.x, province.position.y, province.terrain);
        }
    }
}

/// Update UI panel showing selected tile info
fn update_tile_info_ui(
    selected_info: Res<SelectedProvinceInfo>,
    province_storage: Res<ProvinceStorage>,
    mut text_query: Query<&mut Text, With<TileInfoText>>,
) {
    // Update text if we have a UI panel
    if let Ok(mut text) = text_query.single_mut() {
        if let Some(province_id) = selected_info.province_id {
            if let Some(province) = province_storage.provinces.get(&province_id) {
                text.0 = format!(
                    "Province #{}\nTerrain: {:?}\nElevation: {:.2}\nPopulation: {:.0}\nAgriculture: {:.1}\nWater Distance: {:.1} hex\nPosition: ({:.0}, {:.0})",
                    province.id,
                    province.terrain,
                    province.elevation,
                    province.population,
                    province.agriculture,
                    province.fresh_water_distance,
                    province.position.x,
                    province.position.y,
                );
            }
        } else {
            text.0 = "Click a tile to see info".to_string();
        }
    }
    // Note: UI panel creation has been moved to ui.rs module to avoid duplication
}

/// Handle overlay mode cycling input
fn handle_overlay_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_res: ResMut<ResourceOverlay>,
) {
    // M key to cycle resource overlay modes
    if keyboard.just_pressed(KeyCode::KeyM) {
        overlay_res.cycle();
        println!("Resource Overlay: {}", overlay_res.display_name());
    }
}


// animate_clouds is now handled by CloudPlugin

// Camera control is now handled by CameraPlugin