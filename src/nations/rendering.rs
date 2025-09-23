//! Nation rendering and visual representation
//!
//! This module handles displaying nations on the map by coloring provinces
//! according to their controlling nation.

use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::text::Text2d;
use std::collections::HashMap;

use super::types::{Nation, NationId};
use crate::resources::MapMode;
use crate::world::{ProvinceStorage, WorldMeshHandle};

/// System to update province colors based on nation ownership
/// Only runs when map mode changes to Political
pub fn update_nation_colors(
    nations: Query<&Nation>,
    province_storage: Res<ProvinceStorage>,
    current_map_mode: Res<MapMode>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_handle: Res<WorldMeshHandle>,
) {
    // Only update if we're in political mode
    if *current_map_mode != MapMode::Political {
        return;
    }

    // Only update when map mode changes to Political
    if !current_map_mode.is_changed() {
        return;
    }

    // Build nation color lookup
    let mut nation_colors: HashMap<NationId, Color> = HashMap::new();
    for nation in nations.iter() {
        nation_colors.insert(nation.id, nation.color);
    }

    // Get the world mesh
    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        return;
    };

    // Update vertex colors based on nation ownership
    if let Some(colors_attribute) = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x4(colors) = colors_attribute {
            // For each province, use its owner field directly - O(1) per province!
            for (province_idx, province) in province_storage.provinces.iter().enumerate() {
                let color = if let Some(nation_id) = province.owner {
                    nation_colors
                        .get(&nation_id)
                        .copied()
                        .unwrap_or(Color::srgb(0.2, 0.2, 0.2))
                } else {
                    // Unowned provinces are gray
                    Color::srgb(0.2, 0.2, 0.2)
                };

                // Each hexagon has 7 vertices (center + 6 corners)
                let vertex_start = province_idx * 7;
                for vertex_idx in vertex_start..vertex_start + 7 {
                    if vertex_idx < colors.len() {
                        colors[vertex_idx] = color.to_linear().to_f32_array();
                    }
                }
            }
        }
    }
}

/// System to render nation borders (thicker lines between different nations)
/// Always renders in Terrain mode, or when B key is pressed in other modes
pub fn render_nation_borders(
    mut gizmos: Gizmos,
    province_storage: Res<ProvinceStorage>,
    camera: Query<(&Camera, &Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    current_map_mode: Res<crate::world::MapMode>,
) {
    // Render borders in Terrain mode OR when B key is pressed in other modes
    let should_render_borders = *current_map_mode == crate::world::MapMode::Terrain
        || keyboard.pressed(KeyCode::KeyB);

    if !should_render_borders {
        return;
    }

    // Only render borders when zoomed in enough
    let Ok((_camera, camera_transform)) = camera.single() else {
        return;
    };

    let camera_pos = camera_transform.translation;
    let zoom_level = camera_pos.z.abs();

    // Don't render borders when too far zoomed out
    if zoom_level > 2000.0 {
        return;
    }

    // Draw borders between nations
    for province in &province_storage.provinces {
        let Some(owner) = province.owner else {
            continue;
        };

        // Check each neighbor
        for (i, neighbor_id_opt) in province.neighbors.iter().enumerate() {
            if let Some(neighbor_id) = neighbor_id_opt {
                let neighbor = &province_storage.provinces[neighbor_id.value() as usize];
                let neighbor_owner = neighbor.owner;

                // Draw border if different owner or no owner
                if neighbor_owner.map_or(true, |n| n != owner) {
                    // Calculate edge positions
                    let angle1 = (i as f32 * 60.0).to_radians();
                    let angle2 = ((i + 1) as f32 * 60.0).to_radians();

                    let offset1 = Vec2::new(angle1.cos(), angle1.sin()) * 50.0;
                    let offset2 = Vec2::new(angle2.cos(), angle2.sin()) * 50.0;

                    let pos1 = Vec3::new(
                        province.position.x + offset1.x,
                        province.position.y + offset1.y,
                        1.0,
                    );
                    let pos2 = Vec3::new(
                        province.position.x + offset2.x,
                        province.position.y + offset2.y,
                        1.0,
                    );

                    gizmos.line(pos1, pos2, Color::BLACK);
                }
            }
        }
    }
}

/// System to display nation names on the map (optimized)
/// Only spawns labels once and keeps them persistent
pub fn render_nation_labels(
    mut commands: Commands,
    nations: Query<&Nation>,
    province_storage: Res<ProvinceStorage>,
    existing_labels: Query<Entity, With<NationLabel>>,
    camera: Query<(&Camera, &Transform)>,
) {
    // Check if labels already exist - only spawn once for performance
    if !existing_labels.is_empty() {
        return;
    }

    // Check zoom level - only show labels when reasonably zoomed in
    let Ok((_, camera_transform)) = camera.single() else {
        return;
    };

    let zoom_level = camera_transform.translation.z.abs();

    // Show labels at most zoom levels (more generous than before)
    if zoom_level > 8000.0 {
        return;
    }

    // Create persistent labels for each nation at their capital
    for nation in nations.iter() {
        // Find capital province using the province_by_id index
        let Some(&capital_idx) = province_storage.province_by_id.get(&crate::world::ProvinceId::new(nation.capital_province)) else {
            continue;
        };

        let capital = &province_storage.provinces[capital_idx];

        // Spawn persistent text label at capital position
        commands.spawn((
            Text2d::new(&nation.name),
            TextFont {
                font_size: 28.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(
                capital.position.x,
                capital.position.y,
                10.0, // Above provinces
            )),
            NationLabel,
        ));
    }

    /*
    // Check zoom level
    let Ok((_, camera_transform)) = camera.get_single() else {
        return;
    };

    let zoom_level = camera_transform.translation.z.abs();

    // Only show labels when zoomed in enough
    if zoom_level > 3000.0 {
        return;
    }

    // Create label for each nation at its capital
    for nation in nations.iter() {
        let capital_idx = nation.capital_province as usize;
        if capital_idx >= province_storage.provinces.len() {
            continue;
        }

        let capital = &province_storage.provinces[capital_idx];

        // Spawn text label at capital position
        commands.spawn((
            Text2d::new(&nation.name),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(
                capital.position.x,
                capital.position.y,
                10.0, // Above provinces
            )),
            NationLabel,
        ));
    }
    */
}

/// Marker component for nation label entities
#[derive(Component)]
pub struct NationLabel;
