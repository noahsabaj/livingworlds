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
};
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
    let mut args = Args::parse();
    
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
            handle_input,
            handle_overlay_input,
            handle_tile_selection,
            draw_hexagon_borders,
            update_provinces,
            update_tile_info_ui,
        ))
        .run();
}

// Resources and Components are now defined in lib.rs

// All terrain functions are now imported from terrain module
// All setup functions are now imported from setup module

/// Handle keyboard input
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
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
) {
    // Get camera zoom level and position to determine what to draw
    let Ok((camera, camera_transform, projection)) = camera_query.single() else { return; };
    let current_scale = match projection {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => return,
    };
    
    // Don't draw normal borders when zoomed out too far (improves performance)
    // But check if we have a selected province that should always be visible
    const BORDER_HIDE_THRESHOLD: f32 = 1.5;
    let has_selected_province = provinces.iter().any(|(_, _, selected, _)| selected.is_some());
    
    // Skip border drawing only if zoomed out AND no province is selected
    if current_scale > BORDER_HIDE_THRESHOLD && !has_selected_province {
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
        
        // For selected provinces, always draw them regardless of distance or zoom
        if selected.is_none() {
            // Skip provinces too far from camera (BUG #4 FIX)
            let distance = camera_pos.distance(transform.translation.truncate());
            if distance > max_draw_distance {
                continue;
            }
            
            // Stop if we've drawn too many borders (BUG #4 FIX)
            if borders_drawn >= MAX_BORDERS {
                break;
            }
            
            // Skip normal borders when zoomed out
            if current_scale > BORDER_HIDE_THRESHOLD {
                continue;
            }
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
            // Static golden glow for selected tile - no distracting animation
            // Make it brighter when zoomed out for better visibility
            let brightness = if current_scale > 1.0 {
                1.0  // Full brightness when zoomed out
            } else {
                0.9  // Slightly dimmer when zoomed in
            };
            Color::srgb(1.0 * brightness, 0.84 * brightness, 0.0)  // Golden color
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
    selected_info: Res<SelectedProvinceInfo>,
    provinces: Query<&Province>,
    mut text_query: Query<&mut Text, With<TileInfoText>>,
) {
    // Update text if we have a UI panel
    if let Ok(mut text) = text_query.single_mut() {
        if let Some(entity) = selected_info.entity {
            if let Ok(province) = provinces.get(entity) {
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