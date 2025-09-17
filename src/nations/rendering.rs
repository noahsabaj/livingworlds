//! Nation rendering and visual representation
//!
//! This module handles displaying nations on the map by coloring provinces
//! according to their controlling nation.

use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::text::Text2d;
use std::collections::HashMap;

use crate::world::{MapMode, ProvinceStorage, WorldMeshHandle};
use super::types::{Nation, NationId};

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
                    nation_colors.get(&nation_id)
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
/// ONLY runs when explicitly enabled and zoomed in
pub fn render_nation_borders(
    mut gizmos: Gizmos,
    province_storage: Res<ProvinceStorage>,
    camera: Query<(&Camera, &Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Only render if B key is pressed (disabled by default for performance)
    if !keyboard.pressed(KeyCode::KeyB) {
        return;
    }

    // Only render borders when zoomed in enough
    let Ok((_camera, camera_transform)) = camera.get_single() else {
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

/// System to display nation names on the map
/// DISABLED FOR PERFORMANCE - uncomment when needed
pub fn render_nation_labels(
    _commands: Commands,
    _nations: Query<&Nation>,
    _province_storage: Res<ProvinceStorage>,
    _existing_labels: Query<Entity, With<NationLabel>>,
    _camera: Query<(&Camera, &Transform)>,
) {
    // DISABLED FOR PERFORMANCE
    // This system was spawning/despawning text entities every frame
    // TODO: Implement proper label caching with change detection
    return;

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