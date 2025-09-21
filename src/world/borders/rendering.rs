//! Province Selection and Border Rendering Module
//!
//! This module handles both province selection (mouse picking) and visual feedback
//! (border rendering). It provides a single golden border that highlights the
//! currently selected province, using just ONE entity that moves to the selected
//! province position.
//!
//! In the mega-mesh architecture, provinces are data stored in ProvinceStorage,
//! not individual entities. This dramatically improves performance by reducing

use bevy::log::error;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::MeshMaterial2d;
use bevy::window::PrimaryWindow;

use crate::math::{Hexagon, HEX_SIZE as HEX_SIZE_PIXELS};
use crate::resources::{ProvincesSpatialIndex, SelectedProvinceInfo};
use crate::world::ProvinceId;
use crate::world::ProvinceStorage;

/// Z-index for border rendering (above all provinces and terrain)
const BORDER_Z_INDEX: f32 = 100.0;

/// Search radius multiplier for hexagon hit detection
const HEXAGON_SEARCH_RADIUS_MULTIPLIER: f32 = 1.5;

/// Golden color for selection border (RGBA)
const GOLDEN_COLOR: [f32; 4] = [1.0, 0.84, 0.0, 1.0];

/// Golden color with transparency for material
const GOLDEN_COLOR_ALPHA: f32 = 0.9;

use crate::states::GameState;
/// Plugin that manages selection border rendering using BORDER AUTOMATION!
///
/// **AUTOMATION ACHIEVEMENT**: 18 lines manual → 12 lines declarative!
use bevy_plugin_builder::define_plugin;

define_plugin!(BorderPlugin {
    resources: [SelectionBorder, SelectedProvinceInfo],

    update: [
        (handle_tile_selection, update_selection_border)
            .run_if(in_state(GameState::InGame))
    ],

    on_enter: {
        GameState::InGame => [setup_selection_border]
    }
});

/// Resource storing the single selection border entity
#[derive(Resource, Default)]
pub struct SelectionBorder {
    entity: Option<Entity>,
}

/// Create the hexagon border mesh geometry using the geometry module
fn create_hexagon_border_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Use the geometry module's Hexagon to get proper vertices
    let hexagon = Hexagon::with_size(Vec2::ZERO, HEX_SIZE_PIXELS);
    let corners = hexagon.corners();

    // Convert corners to Vec3 and add the first corner again to close the loop
    let mut vertices = Vec::with_capacity(corners.len() + 1);
    for corner in corners.iter() {
        vertices.push(Vec3::new(corner.x, corner.y, 0.0));
    }
    // Add first vertex again to close the hexagon
    vertices.push(Vec3::new(corners[0].x, corners[0].y, 0.0));

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());

    // Line meshes need vertex colors - use our constant
    let colors = vec![GOLDEN_COLOR; vertices.len()];
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
    debug!("Setting up selection border system");

    let mesh_handle = meshes.add(create_hexagon_border_mesh());

    // Create golden material for selection using constants
    let golden_material = materials.add(ColorMaterial::from(Color::srgba(
        GOLDEN_COLOR[0],
        GOLDEN_COLOR[1],
        GOLDEN_COLOR[2],
        GOLDEN_COLOR_ALPHA,
    )));

    let entity = commands
        .spawn((
            Mesh2d(mesh_handle),
            MeshMaterial2d(golden_material),
            Transform::from_xyz(0.0, 0.0, BORDER_Z_INDEX),
            Visibility::Hidden, // Start hidden until something is selected
            ViewVisibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    selection_border.entity = Some(entity);

    trace!("Selection border ready (1 entity for all provinces)");
}

/// Update selection border position and visibility
/// Uses component mutation for better performance
fn update_selection_border(
    selection_border: Res<SelectionBorder>,
    selected_info: Res<SelectedProvinceInfo>,
    province_storage: Res<ProvinceStorage>,
    mut transforms: Query<&mut Transform>,
    mut visibilities: Query<&mut Visibility>,
) {
    // Skip if selection hasn't changed
    if !selected_info.is_changed() {
        return;
    }

    let Some(border_entity) = selection_border.entity else {
        warn!("Selection border entity not found");
        return;
    };

    // If something is selected, show the border at that position
    if let Some(province_id) = selected_info.province_id {
        // Use HashMap for O(1) lookup instead of O(n) linear search
        if let Some(&idx) = province_storage
            .province_by_id
            .get(&ProvinceId::new(province_id))
        {
            // Bounds check to prevent panic on invalid index
            if let Some(province) = province_storage.provinces.get(idx) {
                trace!(
                    "Showing selection border for province {} at ({:.0}, {:.0})",
                    province_id,
                    province.position.x,
                    province.position.y
                );

                // Mutate existing transform instead of creating new one
                if let Ok(mut transform) = transforms.get_mut(border_entity) {
                    transform.translation.x = province.position.x;
                    transform.translation.y = province.position.y;
                    transform.translation.z = BORDER_Z_INDEX;
                }

                // Show the border by mutating visibility
                if let Ok(mut visibility) = visibilities.get_mut(border_entity) {
                    *visibility = Visibility::Inherited;
                }
            } else {
                // Handle invalid index gracefully with error reporting
                error!(
                    "Invalid province index {} for province ID {} in border rendering. Data structures may be out of sync.",
                    idx, province_id
                );
                // Hide border when data is corrupted
                if let Ok(mut visibility) = visibilities.get_mut(border_entity) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    } else {
        // Nothing selected - hide the border by mutating visibility
        if let Ok(mut visibility) = visibilities.get_mut(border_entity) {
            *visibility = Visibility::Hidden;
        }
    }
}

/// Handle mouse clicks for tile selection using hexagonal grid math
/// Private function as it's only used internally by this module
fn handle_tile_selection(
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

    let Ok(window) = windows.single() else {
        warn!("Failed to get primary window for tile selection");
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        trace!("No cursor position available");
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        warn!("Failed to get camera for tile selection");
        return;
    };

    // Convert screen position to world position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        warn!("Failed to convert viewport to world position");
        return;
    };
    let world_pos = ray.origin.truncate();

    // Clear previous selection
    selected_info.province_id = None;

    let hex_size = HEX_SIZE_PIXELS;

    // Use fast direct lookup instead of expensive radius search
    if let Some((province_id, _actual_pos)) =
        spatial_index.pick_province_at_position(world_pos, hex_size)
    {
        // Province found - update selection
        selected_info.province_id = Some(province_id.value());

        if let Some(&idx) = province_storage.province_by_id.get(&province_id) {
            // Bounds check to prevent panic on invalid index
            if let Some(province) = province_storage.provinces.get(idx) {
                trace!(
                    "Selected province {} at ({:.0}, {:.0}), terrain: {:?}",
                    province_id,
                    province.position.x,
                    province.position.y,
                    province.terrain
                );
            } else {
                // Handle invalid index gracefully with error reporting
                error!(
                    "Invalid province index {} for province ID {} during selection. Data structures may be out of sync.",
                    idx, province_id
                );
            }
        }
    }
}
