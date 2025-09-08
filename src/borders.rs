//! Selection Border Rendering Module
//! 
//! This module provides a single golden border that highlights the currently
//! selected province. Instead of rendering 135,000 borders, we have just ONE
//! entity that moves to the selected province position.
//! 
//! Performance impact:
//! - Old system: 135,000 border entities
//! - New system: 1 border entity
//! - Result: 134,999 fewer entities to process every frame!

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MeshMaterial2d;

use crate::constants::HEX_SIZE_PIXELS;
use crate::resources::SelectedProvinceInfo;
use crate::setup::ProvinceStorage;

/// Plugin that manages selection border rendering
pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resource for the single border entity
            .init_resource::<SelectionBorder>()
            
            // Systems
            .add_systems(Startup, setup_selection_border.after(crate::setup::setup_world))
            .add_systems(Update, update_selection_border);
    }
}

/// Resource storing the single selection border entity
#[derive(Resource, Default)]
struct SelectionBorder {
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
        Transform::from_xyz(0.0, 0.0, 10.0), // High Z to render above everything
        Visibility::Hidden, // Start hidden until something is selected
    )).id();
    
    selection_border.entity = Some(entity);
    
    println!("Selection border ready (1 entity instead of 135,000!)");
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
        if let Some(province) = province_storage.provinces.get(&province_id) {
            // Move border to selected province and make visible
            commands.entity(border_entity)
                .insert(Transform::from_xyz(province.position.x, province.position.y, 10.0))
                .insert(Visibility::Inherited);
        }
    } else {
        // Nothing selected - hide the border
        commands.entity(border_entity)
            .insert(Visibility::Hidden);
    }
}