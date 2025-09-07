//! Living Worlds - Main entry point
//! 
//! An observer civilization simulation built with Bevy

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Import from our library
use living_worlds::prelude::*;
use living_worlds::{
    build_app, WorldSeed, WorldSize, GameTime,
    resources::{WorldTension, ResourceOverlay},
};
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

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
        .insert_resource(GameTime::default())
        .insert_resource(ResourceOverlay::default())
        // Add our game systems
        .add_systems(Startup, setup_world)
        .add_systems(Update, (
            handle_input,
            handle_tile_selection,
            draw_hexagon_borders,
            update_provinces,
            simulate_time,
            calculate_world_tension,
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
    selected_info: Res<SelectedProvinceInfo>,
    provinces: Query<&Province>,
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
    }
    // Note: UI panel creation has been moved to ui.rs module to avoid duplication
}

/// Simulate time passing and nations expanding
fn simulate_time(
    mut game_time: ResMut<GameTime>,
    time: Res<Time>,
    mut provinces: Query<&mut Province>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_res: ResMut<ResourceOverlay>,
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
    
    // M key to cycle resource overlay modes
    if keyboard.just_pressed(KeyCode::KeyM) {
        overlay_res.cycle();
        println!("Resource Overlay: {}", overlay_res.display_name());
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

/// Calculate world tension based on simulated nation conflicts
/// This is a mock implementation until nations are fully implemented
fn calculate_world_tension(
    mut tension: ResMut<WorldTension>,
    game_time: Res<GameTime>,
    provinces: Query<&Province>,
) {
    // Skip if paused
    if game_time.paused {
        return;
    }
    
    // Mock simulation of world events based on game time
    // This creates a dynamic tension pattern that evolves over time
    let years = game_time.current_date / 365.0;
    
    // Base tension from time progression (civilizations naturally develop conflicts)
    let time_factor = (years / 1000.0).min(0.3); // Max 30% from time
    
    // Cyclic war patterns (wars tend to come in waves)
    let war_cycle = ((years / 50.0).sin() + 1.0) / 2.0; // 50-year war cycles
    let minor_conflicts = ((years / 7.0).sin() + 1.0) / 4.0; // 7-year minor conflicts
    
    // Random events (simplified - would be event-driven in real implementation)
    let crisis_chance = if years as i32 % 100 < 5 { 0.2 } else { 0.0 }; // Major crisis every ~100 years
    
    // Count land provinces (proxy for number of nations)
    let _land_provinces = provinces.iter()
        .filter(|p| p.terrain != TerrainType::Ocean)
        .count() as f32;
    
    // Simulate percentage of nations at war
    // This would be calculated from actual nation states in the full implementation
    let base_war_percentage = (time_factor + war_cycle * 0.3 + minor_conflicts * 0.1 + crisis_chance).min(1.0);
    
    // Apply the exponential curve for tension calculation
    let calculated_tension = WorldTension::calculate_from_war_percentage(base_war_percentage);
    
    // Update the contributing factors (mock values)
    tension.war_factor = base_war_percentage;
    tension.power_imbalance = war_cycle * 0.2; // Some nations becoming too powerful
    tension.economic_stress = minor_conflicts * 0.15; // Economic disruption from conflicts
    tension.instability_factor = crisis_chance; // Major events causing instability
    
    // Set the target tension (physics system will smoothly interpolate to it)
    tension.target = calculated_tension;
    
    // Debug output occasionally
    if game_time.current_date as i32 % 1000 == 0 {
        println!("World Tension: {:.1}% (Target: {:.1}%, Wars: {:.1}%)",
            tension.current * 100.0,
            tension.target * 100.0,
            base_war_percentage * 100.0
        );
    }
}

// animate_clouds is now handled by CloudPlugin

// Camera control is now handled by CameraPlugin