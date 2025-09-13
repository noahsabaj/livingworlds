//! Map overlay rendering system for Living Worlds
//! 
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! It acts as the central coordinator for how provinces are visually
//! represented based on the current viewing mode.

use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use crate::constants::MS_PER_SECOND;
use crate::resources::ResourceOverlay;
// TerrainType import moved to CachedOverlayColors
// Color calculations moved to CachedOverlayColors::get_or_calculate
use super::mesh::ProvinceStorage;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Bytes per megabyte for memory calculations
const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

/// System that updates province colors in the mega-mesh based on active overlay mode
/// Uses lazy-loaded colors with LRU caching to reduce memory usage
pub fn update_province_colors(
    overlay: Res<ResourceOverlay>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
    province_storage: Res<ProvinceStorage>,
    mesh_handle: Res<super::mesh::WorldMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    let start = std::time::Instant::now();
    trace!("Starting color update for overlay: {} at {:.3}s",
           overlay.display_name(),
           time.elapsed_secs());

    // System already only runs when overlay changes due to run_if condition

    // Get the mesh from the handle with proper error handling
    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        warn!("Failed to get world mesh for overlay update");
        return;
    };

    let mesh_lookup_time = start.elapsed();

    // Get or calculate colors for the requested overlay (lazy loading)
    let colors = cached_colors.get_or_calculate(*overlay, &province_storage);

    let selection_time = start.elapsed() - mesh_lookup_time;

    // Calculate buffer size for logging
    let buffer_size_mb = (colors.len() * std::mem::size_of::<[f32; 4]>()) as f32 / BYTES_PER_MB;

    // Safe clone operation - Vec::clone() internally uses optimized memcpy for primitive types
    let clone_start = std::time::Instant::now();
    let cloned_colors = colors.clone();
    let clone_time = clone_start.elapsed();

    let insert_start = std::time::Instant::now();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, cloned_colors);
    let insert_time = insert_start.elapsed();

    let total_time = start.elapsed();

    // Log memory usage for monitoring
    let total_memory = cached_colors.memory_usage_mb();

    debug!("Color update complete: Total={:.1}ms (buffer={:.1}MB, total_mem={:.1}MB, clone={:.1}ms, GPU upload={:.1}ms)",
           total_time.as_secs_f32() * MS_PER_SECOND,
           buffer_size_mb,
           total_memory,
           clone_time.as_secs_f32() * MS_PER_SECOND,
           insert_time.as_secs_f32() * MS_PER_SECOND);
}

/// Plugin that manages map overlay rendering
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;

        app
            .init_resource::<ResourceOverlay>()
            .init_resource::<crate::resources::CachedOverlayColors>()
            // Initialize with default overlay only - others loaded on-demand
            .add_systems(OnExit(GameState::LoadingWorld), initialize_overlay_colors)
            .add_systems(Update, (
                handle_overlay_input,
                update_province_colors.run_if(resource_changed::<ResourceOverlay>),
            ).run_if(in_state(GameState::InGame)));
    }
}

/// Handle overlay mode cycling input (M key)
pub fn handle_overlay_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay_res: ResMut<ResourceOverlay>,
    time: Res<Time>,
) {
    // M key to cycle resource overlay modes
    if keyboard.just_pressed(KeyCode::KeyM) {
        let start = std::time::Instant::now();
        overlay_res.cycle();
        debug!("Overlay switched to: {} (input lag: {:.1}ms)", 
               overlay_res.display_name(),
               start.elapsed().as_secs_f32() * MS_PER_SECOND);
    }
}

// Helper function moved to CachedOverlayColors implementation

/// Initialize the overlay system with the default terrain overlay
/// Only calculates the initial overlay, others are loaded on-demand
pub fn initialize_overlay_colors(
    province_storage: Res<ProvinceStorage>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
) {
    info!("Initializing overlay system for {} provinces", province_storage.provinces.len());
    let start = std::time::Instant::now();

    // Only pre-calculate the default terrain overlay
    // Other overlays will be calculated on-demand when first accessed
    let default_overlay = ResourceOverlay::None;
    cached_colors.get_or_calculate(default_overlay, &province_storage);

    let elapsed = start.elapsed();
    let memory_mb = cached_colors.memory_usage_mb();

    info!("Overlay system initialized in {:.2}s (using {:.1}MB)",
          elapsed.as_secs_f32(),
          memory_mb);
}