//! Nation rendering and visual representation
//!
//! This module handles displaying nations on the map by coloring provinces
//! according to their controlling nation.

use bevy::prelude::*;
use bevy::text::Text2d;
use std::collections::HashMap;

use super::types::{Nation, NationId};
use crate::resources::MapMode;
use crate::ui::shortcuts::ShortcutRegistry;
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
    registry: Res<ShortcutRegistry>,
    current_map_mode: Res<crate::world::MapMode>,
) {
    // Get the border toggle key from the shortcuts registry (defaults to B)
    let border_key = registry
        .get(&crate::ui::shortcuts::ShortcutId::ToggleBorders)
        .map(|def| def.binding.key)
        .unwrap_or(KeyCode::KeyB);

    // Render borders in Terrain mode OR when border toggle key is pressed
    let should_render_borders = *current_map_mode == crate::world::MapMode::Terrain
        || keyboard.pressed(border_key);

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

/// System to spawn territory-spanning nation labels with dynamic sizing
pub fn spawn_nation_labels(
    mut commands: Commands,
    nations: Query<&Nation>,
    province_storage: Res<ProvinceStorage>,
    ownership_cache: Res<super::types::ProvinceOwnershipCache>,
    mut territory_cache: ResMut<super::territory_analysis::TerritoryMetricsCache>,
    existing_labels: Query<Entity, With<NationLabel>>,
) {
    // Clear existing labels when ownership changes
    if territory_cache.cache_version != ownership_cache.version {
        for entity in existing_labels.iter() {
            commands.entity(entity).despawn();
        }
    } else if !existing_labels.is_empty() {
        // Labels already exist and ownership hasn't changed
        return;
    }

    // Create labels for each nation based on territory analysis
    for nation in nations.iter() {
        let Some(metrics) = territory_cache.get_or_calculate(
            nation.id,
            &ownership_cache,
            &province_storage,
        ) else {
            continue;
        };

        // Skip nations with no territory
        if metrics.province_count == 0 {
            continue;
        }

        let base_font_size = metrics.optimal_base_font_size();

        // For disconnected empires, optionally create multiple labels
        if !metrics.is_contiguous && metrics.clusters.len() > 1 {
            // Only label major clusters (>20% of total territory)
            for (idx, cluster) in metrics.clusters.iter().enumerate() {
                if cluster.relative_size >= 0.2 {
                    let cluster_width = cluster.bounds.1.x - cluster.bounds.0.x;
                    let cluster_font_size = base_font_size * cluster.relative_size;

                    commands.spawn((
                        Text2d::new(if idx == 0 {
                            nation.name.clone()
                        } else {
                            // Secondary clusters get abbreviated names or nothing
                            format!("{} (Colony)", &nation.name[..nation.name.len().min(10)])
                        }),
                        TextFont {
                            font_size: cluster_font_size,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, if idx == 0 { 1.0 } else { 0.7 })),
                        Transform::from_translation(Vec3::new(
                            cluster.centroid.x,
                            cluster.centroid.y,
                            10.0, // Above provinces
                        )),
                        NationLabel {
                            nation_id: nation.id,
                            base_font_size: cluster_font_size,
                            territory_bounds: cluster.bounds,
                            centroid: cluster.centroid,
                            province_count: cluster.province_ids.len(),
                            is_primary: idx == 0,
                        },
                    ));
                }
            }
        } else {
            // Single contiguous territory - one label at centroid
            commands.spawn((
                Text2d::new(&nation.name),
                TextFont {
                    font_size: base_font_size,
                    ..default()
                },
                TextColor(Color::WHITE),
                Transform::from_translation(Vec3::new(
                    metrics.centroid.x,
                    metrics.centroid.y,
                    10.0,
                )),
                NationLabel {
                    nation_id: nation.id,
                    base_font_size,
                    territory_bounds: metrics.bounds,
                    centroid: metrics.centroid,
                    province_count: metrics.province_count,
                    is_primary: true,
                },
            ));
        }
    }
}

/// System to dynamically update nation label sizes based on camera zoom
pub fn update_nation_label_sizes(
    camera_query: Query<(&Camera, &Transform), Changed<Transform>>,
    mut label_query: Query<(&NationLabel, &mut TextFont, &mut TextColor)>,
) {
    let Ok((_, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let zoom_level = camera_transform.translation.z.abs();

    // Calculate zoom factor for font scaling
    // Closer zoom = larger text relative to territory
    // Further zoom = smaller text to avoid overlap
    let zoom_factor = if zoom_level < 500.0 {
        1.5 // Very close - make text larger
    } else if zoom_level < 1000.0 {
        1.2
    } else if zoom_level < 2000.0 {
        1.0
    } else if zoom_level < 4000.0 {
        0.8
    } else if zoom_level < 6000.0 {
        0.6
    } else {
        0.4 // Very far - make text smaller
    };

    for (label, mut font, mut color) in label_query.iter_mut() {
        // Scale font based on zoom and territory size
        let scaled_size = label.base_font_size * zoom_factor;
        font.font_size = scaled_size.clamp(8.0, 200.0);

        // Adjust opacity based on importance and zoom
        let importance_factor = if label.province_count > 100 {
            1.0 // Empires always visible
        } else if label.province_count > 50 {
            0.9 // Major powers
        } else if label.province_count > 20 {
            0.8 // Regional powers
        } else if label.province_count > 5 {
            0.7 // Small nations
        } else {
            0.6 // Tiny nations
        };

        // Fade out small nations at far zoom levels
        let zoom_opacity = if zoom_level > 5000.0 && label.province_count < 20 {
            0.0 // Hide small nations when very zoomed out
        } else if zoom_level > 3000.0 && label.province_count < 10 {
            0.3 // Fade tiny nations
        } else {
            1.0
        };

        let final_opacity = importance_factor * zoom_opacity * if label.is_primary { 1.0 } else { 0.7 };
        *color = TextColor(Color::srgba(1.0, 1.0, 1.0, final_opacity));
    }
}

/// System to hide/show labels based on zoom level (LOD)
pub fn update_label_visibility(
    camera_query: Query<(&Camera, &Transform), Changed<Transform>>,
    mut label_query: Query<(&NationLabel, &mut Visibility)>,
) {
    let Ok((_, camera_transform)) = camera_query.get_single() else {
        return;
    };

    let zoom_level = camera_transform.translation.z.abs();

    for (label, mut visibility) in label_query.iter_mut() {
        // Visibility thresholds based on nation size
        let should_show = if label.province_count > 100 {
            zoom_level < 10000.0 // Empires visible at all reasonable zoom levels
        } else if label.province_count > 50 {
            zoom_level < 8000.0 // Major powers
        } else if label.province_count > 20 {
            zoom_level < 6000.0 // Regional powers
        } else if label.province_count > 5 {
            zoom_level < 4000.0 // Small nations
        } else {
            zoom_level < 2000.0 // Tiny nations only when close
        };

        *visibility = if should_show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Enhanced nation label component with territory awareness
#[derive(Component)]
pub struct NationLabel {
    /// The nation this label represents
    pub nation_id: super::types::NationId,
    /// Base font size calculated from territory
    pub base_font_size: f32,
    /// Territory bounds for this nation
    pub territory_bounds: (Vec2, Vec2),
    /// Centroid of the territory
    pub centroid: Vec2,
    /// Number of provinces (for importance weighting)
    pub province_count: usize,
    /// Whether this is a main label or secondary (for disconnected territories)
    pub is_primary: bool,
}
