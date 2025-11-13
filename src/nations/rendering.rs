//! Nation rendering and visual representation
//!
//! This module handles displaying nations on the map by coloring provinces
//! according to their controlling nation.

use bevy::prelude::*;
use bevy::sprite::Text2d;  // Moved from bevy::text in Bevy 0.17
use std::collections::HashMap;

use super::types::{Nation, NationId};
use crate::resources::MapMode;
use crate::ui::ShortcutRegistry;
use crate::world::ProvinceStorage;

/// Get text color that contrasts well with the nation color
/// Uses perceived luminance calculation to ensure readability
fn get_contrasting_text_color(nation_color: Color) -> Color {
    // Calculate perceived luminance using ITU-R BT.709 coefficients
    let srgba = nation_color.to_srgba();
    let luminance = 0.299 * srgba.red + 0.587 * srgba.green + 0.114 * srgba.blue;

    // Use black text for bright nations, white for dark nations
    if luminance > 0.5 {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

/// Check if a label placed at position would overlap with other nations' territories
/// Samples multiple points across the label bounding box to detect collisions
fn check_label_collision(
    label_text: &str,
    font_size: f32,
    position: Vec2,
    owner_nation_id: NationId,
    province_storage: &ProvinceStorage,
    spatial_index: &crate::world::ProvincesSpatialIndex,
) -> bool {
    // Estimate text dimensions (rough: 0.6 * font_size per character width)
    let char_width = font_size * 0.6;
    let text_width = label_text.len() as f32 * char_width;
    let text_height = font_size * 1.2;

    // Sample 9 points across label bounding box for comprehensive collision detection
    let sample_offsets = [
        Vec2::new(-text_width / 2.0, text_height / 2.0),   // Top-left
        Vec2::new(0.0, text_height / 2.0),                  // Top-center
        Vec2::new(text_width / 2.0, text_height / 2.0),    // Top-right
        Vec2::new(-text_width / 2.0, 0.0),                  // Middle-left
        Vec2::new(0.0, 0.0),                                 // Center
        Vec2::new(text_width / 2.0, 0.0),                   // Middle-right
        Vec2::new(-text_width / 2.0, -text_height / 2.0),  // Bottom-left
        Vec2::new(0.0, -text_height / 2.0),                 // Bottom-center
        Vec2::new(text_width / 2.0, -text_height / 2.0),   // Bottom-right
    ];

    // Check each sample point for collision with other nations
    for offset in sample_offsets {
        let sample_point = position + offset;

        // Find province at this position using spatial index
        // Note: hex_size is typically 50.0, matching the standard hex size in the game
        if let Some(province_id) = spatial_index.get_at_position(sample_point, 50.0) {
            if let Some(province) = province_storage
                .provinces
                .get(province_id.value() as usize)
            {
                // Collision detected if this province belongs to a different nation
                if let Some(owner) = province.owner {
                    if owner != owner_nation_id {
                        return true; // COLLISION!
                    }
                }
            }
        }
    }

    false // No collision detected
}

/// Find optimal placement for label avoiding collisions with other nations
/// Tries multiple positions and returns the first non-colliding one
fn find_non_colliding_position(
    label_text: &str,
    font_size: f32,
    centroid: Vec2,
    territory_bounds: (Vec2, Vec2),
    owner_nation_id: NationId,
    province_storage: &ProvinceStorage,
    spatial_index: &crate::world::ProvincesSpatialIndex,
) -> (Vec2, f32) {
    // Returns (position, opacity)
    let (min_bounds, max_bounds) = territory_bounds;
    let territory_width = max_bounds.x - min_bounds.x;
    let territory_height = max_bounds.y - min_bounds.y;

    // Try multiple placement positions in order of preference
    let candidates = vec![
        (centroid, 1.0), // Original centroid, full opacity
        (
            centroid + Vec2::new(0.0, territory_height * 0.2),
            1.0,
        ), // North
        (
            centroid + Vec2::new(0.0, -territory_height * 0.2),
            1.0,
        ), // South
        (
            centroid + Vec2::new(territory_width * 0.2, 0.0),
            1.0,
        ), // East
        (
            centroid + Vec2::new(-territory_width * 0.2, 0.0),
            1.0,
        ), // West
    ];

    // Return first non-colliding position
    for (pos, opacity) in candidates {
        if !check_label_collision(
            label_text,
            font_size,
            pos,
            owner_nation_id,
            province_storage,
            spatial_index,
        ) {
            return (pos, opacity);
        }
    }

    // All positions collide - use centroid with reduced opacity as visual cue
    (centroid, 0.7)
}

/// Spawn nation labels when entering Political mode
/// This wrapper ensures labels only spawn when MapMode changes TO Political
pub fn spawn_nation_labels_on_mode_enter(
    commands: Commands,
    nations: Query<&Nation>,
    province_storage: Res<ProvinceStorage>,
    spatial_index: Res<crate::world::ProvincesSpatialIndex>,
    ownership_cache: Res<super::types::ProvinceOwnershipCache>,
    territory_cache: ResMut<super::territory_analysis::TerritoryMetricsCache>,
    existing_labels: Query<Entity, With<NationLabel>>,
    current_mode: Res<MapMode>,
) {
    // Only spawn if we're now in Political mode
    if *current_mode == MapMode::Political {
        debug!("Spawning nation labels for Political mode");
        spawn_nation_labels(
            commands,
            nations,
            province_storage,
            spatial_index,
            ownership_cache,
            territory_cache,
            existing_labels,
        );
    }
}

/// Cleanup nation labels when switching away from Political mode
/// Despawns both main label entities and shadow entities to prevent memory leaks
pub fn cleanup_labels_on_mode_exit(
    mut commands: Commands,
    current_mode: Res<MapMode>,
    label_query: Query<Entity, With<NationLabel>>,
    shadow_query: Query<Entity, With<NationLabelShadow>>,
) {
    // Only cleanup if NOT in Political mode
    if *current_mode != MapMode::Political {
        // Despawn main labels
        for entity in &label_query {
            commands.entity(entity).despawn();
        }
        // Despawn shadows to prevent memory leak
        for entity in &shadow_query {
            commands.entity(entity).despawn();
        }
        debug!("Cleaned up {} labels and {} shadows", label_query.iter().count(), shadow_query.iter().count());
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
        .get(&crate::ui::ShortcutId::ToggleBorders)
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
    spatial_index: Res<crate::world::ProvincesSpatialIndex>,
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

        // Get contrasting text color for readability
        let text_color = get_contrasting_text_color(nation.color);

        // For disconnected empires, optionally create multiple labels
        if !metrics.is_contiguous && metrics.clusters.len() > 1 {
            // Only label major clusters (>20% of total territory)
            for (idx, cluster) in metrics.clusters.iter().enumerate() {
                if cluster.relative_size >= 0.2 {
                    let cluster_font_size = base_font_size * cluster.relative_size;
                    let label_text = if idx == 0 {
                        nation.name.clone()
                    } else {
                        // Secondary clusters get abbreviated names
                        format!("{} (Colony)", &nation.name[..nation.name.len().min(10)])
                    };
                    let base_opacity = if idx == 0 { 1.0 } else { 0.7 };

                    // Find non-colliding position for this cluster label
                    let (label_position, collision_opacity) = find_non_colliding_position(
                        &label_text,
                        cluster_font_size,
                        cluster.centroid,
                        cluster.bounds,
                        nation.id,
                        &province_storage,
                        &spatial_index,
                    );
                    let final_opacity = base_opacity * collision_opacity;

                    // Spawn drop shadow for better visibility
                    commands.spawn((
                        Text2d::new(label_text.clone()),
                        TextFont {
                            font_size: cluster_font_size,
                            ..default()
                        },
                        TextColor(Color::srgba(0.0, 0.0, 0.0, final_opacity * 0.8)), // Black shadow
                        Transform::from_translation(Vec3::new(
                            label_position.x + 2.0,
                            label_position.y - 2.0,
                            149.0, // Just behind main text
                        )),
                        NationLabelShadow, // Marker for cleanup
                    ));

                    // Spawn main text label
                    commands.spawn((
                        Text2d::new(label_text),
                        TextFont {
                            font_size: cluster_font_size,
                            ..default()
                        },
                        TextColor(text_color.with_alpha(final_opacity)),
                        Transform::from_translation(Vec3::new(
                            label_position.x,
                            label_position.y,
                            150.0, // Above borders (z=100)
                        )),
                        NationLabel {
                            nation_id: nation.id,
                            base_font_size: cluster_font_size,
                            territory_bounds: cluster.bounds,
                            centroid: label_position, // Use collision-adjusted position
                            province_count: cluster.province_ids.len(),
                            is_primary: idx == 0,
                        },
                    ));
                }
            }
        } else {
            // Single contiguous territory - find non-colliding position
            let (label_position, collision_opacity) = find_non_colliding_position(
                &nation.name,
                base_font_size,
                metrics.centroid,
                metrics.bounds,
                nation.id,
                &province_storage,
                &spatial_index,
            );

            // Spawn drop shadow first
            commands.spawn((
                Text2d::new(&nation.name),
                TextFont {
                    font_size: base_font_size,
                    ..default()
                },
                TextColor(Color::srgba(0.0, 0.0, 0.0, collision_opacity * 0.8)), // Black shadow
                Transform::from_translation(Vec3::new(
                    label_position.x + 2.0,
                    label_position.y - 2.0,
                    149.0, // Just behind main text
                )),
                NationLabelShadow, // Marker for cleanup
            ));

            // Spawn main text label
            commands.spawn((
                Text2d::new(&nation.name),
                TextFont {
                    font_size: base_font_size,
                    ..default()
                },
                TextColor(text_color.with_alpha(collision_opacity)),
                Transform::from_translation(Vec3::new(
                    label_position.x,
                    label_position.y,
                    150.0, // Above borders (z=100)
                )),
                NationLabel {
                    nation_id: nation.id,
                    base_font_size,
                    territory_bounds: metrics.bounds,
                    centroid: label_position, // Use collision-adjusted position
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
    let Ok((_, camera_transform)) = camera_query.single() else {
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
        font.font_size = scaled_size.clamp(16.0, 320.0); // Increased from (8.0, 200.0)

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
    let Ok((_, camera_transform)) = camera_query.single() else {
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

/// Marker component for nation label shadow entities
/// Used for cleanup to prevent memory leaks
#[derive(Component)]
pub struct NationLabelShadow;
