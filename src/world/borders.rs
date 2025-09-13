//! Province Selection and Border Rendering Module
//! 
//! This module handles both province selection (mouse picking) and visual feedback
//! (border rendering). It provides a single golden border that highlights the 
//! currently selected province, using just ONE entity that moves to the selected 
//! province position.
//! 
//! Performance impact:
//! - Old system: 900,000 border entities (one per province)
//! - New system: 1 border entity
//! - Result: 899,999 fewer entities to process every frame!

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MeshMaterial2d;
use bevy::window::PrimaryWindow;

use crate::constants::{HEX_SIZE_PIXELS, SQRT_3};
use crate::resources::{SelectedProvinceInfo, ProvincesSpatialIndex};
use super::mesh::ProvinceStorage;

/// Plugin that manages selection border rendering
pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;
        
        app
            // Resources
            .init_resource::<SelectionBorder>()
            .init_resource::<SelectedProvinceInfo>()
            
            // Systems
            .add_systems(OnEnter(GameState::InGame), setup_selection_border)
            .add_systems(Update, (
                handle_tile_selection,
                update_selection_border,
            ).run_if(in_state(GameState::InGame)));
    }
}

/// Resource storing the single selection border entity
#[derive(Resource, Default)]
pub struct SelectionBorder {
    entity: Option<Entity>,
}

/// Create the hexagon border mesh geometry
fn create_hexagon_border_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    // Pre-calculated flat-top hexagon vertices (starts at 0 degrees)
    // This creates a perfect flat-top hexagon border
    const HEX_OFFSETS: [(f32, f32); 7] = [
        (1.0, 0.0),           // 0° - Right
        (0.5, 0.866025404),   // 60° - Top-Right
        (-0.5, 0.866025404),  // 120° - Top-Left
        (-1.0, 0.0),          // 180° - Left
        (-0.5, -0.866025404), // 240° - Bottom-Left
        (0.5, -0.866025404),  // 300° - Bottom-Right
        (1.0, 0.0),           // 360° - Right (close the loop)
    ];
    
    // Scale vertices by hex size
    let vertices: Vec<Vec3> = HEX_OFFSETS
        .iter()
        .map(|(x, y)| Vec3::new(x * HEX_SIZE_PIXELS, y * HEX_SIZE_PIXELS, 0.0))
        .collect();
    
    // Set vertex positions
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    
    // Line meshes need vertex colors
    let colors = vec![[1.0, 0.84, 0.0, 1.0]; 7]; // Golden color
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    
    mesh
}

/// Setup the single selection border entity
fn setup_selection_border(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut selection_border: ResMut<SelectionBorder>,
) {
    println!("Setting up selection border system...");
    
    // Create the hexagon mesh
    let mesh_handle = meshes.add(create_hexagon_border_mesh());
    
    // Create golden material for selection
    let golden_material = materials.add(ColorMaterial::from(Color::srgba(1.0, 0.84, 0.0, 0.9)));
    
    // Spawn the single selection border entity (initially hidden)
    let entity = commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(golden_material),
        Transform::from_xyz(0.0, 0.0, 100.0), // Very high Z to render above everything
        Visibility::Hidden, // Start hidden until something is selected
        ViewVisibility::default(),
        InheritedVisibility::default()
    )).id();
    
    selection_border.entity = Some(entity);
    
    println!("Selection border ready (1 entity instead of 900,000!)");
}

/// Update selection border position and visibility
fn update_selection_border(
    mut commands: Commands,
    selection_border: Res<SelectionBorder>,
    selected_info: Res<SelectedProvinceInfo>,
    province_storage: Res<ProvinceStorage>,
) {
    // Skip if selection hasn't changed
    if !selected_info.is_changed() {
        return;
    }
    
    // Get the selection border entity
    let Some(border_entity) = selection_border.entity else { return; };
    
    // If something is selected, show the border at that position
    if let Some(province_id) = selected_info.province_id {
        // Use HashMap for O(1) lookup instead of O(n) linear search
        if let Some(&idx) = province_storage.province_by_id.get(&province_id) {
            let province = &province_storage.provinces[idx];
            println!("Showing selection border for province {} at ({:.0}, {:.0})", 
                     province_id, province.position.x, province.position.y);
            // Move border to selected province and make visible
            commands.entity(border_entity)
                .insert(Transform::from_xyz(province.position.x, province.position.y, 100.0))
                .insert(Visibility::Inherited);
        }
    } else {
        // Nothing selected - hide the border
        commands.entity(border_entity)
            .insert(Visibility::Hidden);
    }
}

/// Handle mouse clicks for tile selection using hexagonal grid math
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
    let search_radius = hex_size * 1.5; // Search within 1.5 hex radius for better coverage
    
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
        if abs_y <= hex_size * SQRT_3 / 2.0 && 
           abs_x <= hex_size &&
           (abs_y / SQRT_3 + abs_x / 2.0 <= hex_size) {
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
        
        // Get province data for debug output - O(1) HashMap lookup
        if let Some(&idx) = province_storage.province_by_id.get(&province_id) {
            let province = &province_storage.provinces[idx];
            println!("Selected province {} at ({:.0}, {:.0}), terrain: {:?}", 
                     province_id, province.position.x, province.position.y, province.terrain);
        }
    }
}