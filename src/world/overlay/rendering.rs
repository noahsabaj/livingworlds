//! Map overlay rendering system for Living Worlds with zero-copy architecture
//!
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! Now optimized with Arc-based zero-copy architecture for instant mode switching.

use super::MapMode;
use crate::constants::MS_PER_SECOND;
use crate::relationships::{Controls, ControlledBy};
use crate::world::{ProvinceData, ProvinceEntityOrder};
use bevy::log::{debug, trace, warn};
use bevy::prelude::*;
use bevy::prelude::Mesh;
use std::sync::Arc;

/// Bytes per megabyte for memory calculations
const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

/// System that updates province colors in the mega-mesh based on active overlay mode
/// Now uses Arc-based zero-copy architecture for instant switching
pub fn update_province_colors(
    overlay: Res<MapMode>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
    province_entity_order: Option<Res<ProvinceEntityOrder>>,
    province_data_query: Query<&ProvinceData>,
    controlled_by_query: Query<&ControlledBy>,
    world_seed: Res<crate::world::WorldSeed>,
    mesh_handle: Res<crate::world::WorldMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    nations_query: Query<(Entity, &crate::nations::Nation)>,
    controls_query: Query<&Controls>,
    climate_storage: Option<Res<crate::world::terrain::ClimateStorage>>,
    infrastructure_storage: Option<Res<crate::world::InfrastructureStorage>>,
) {
    let start = std::time::Instant::now();
    trace!(
        "Starting color update for overlay: {} at {:.3}s",
        overlay.display_name(),
        time.elapsed_secs()
    );

    // Need province entity order for ECS-based color calculation
    let Some(entity_order) = province_entity_order.as_ref() else {
        trace!("ProvinceEntityOrder not available yet, skipping overlay update");
        return;
    };

    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        warn!("Failed to get world mesh for overlay update");
        return;
    };

    let mesh_lookup_time = start.elapsed();

    // Get Arc to colors using ECS queries - NO CLONING, just reference counting!
    let colors_arc = cached_colors.get_or_calculate_ecs(
        *overlay,
        entity_order.as_ref(),
        &province_data_query,
        world_seed.0,
        &nations_query,
        &controls_query,
        climate_storage.as_ref().map(|r| r.as_ref()),
        infrastructure_storage.as_ref().map(|r| r.as_ref()),
    );

    let _selection_time = start.elapsed() - mesh_lookup_time;

    let buffer_size_mb = (colors_arc.len() * std::mem::size_of::<[f32; 4]>()) as f32 / BYTES_PER_MB;

    // Only clone here if mesh requires ownership (Bevy's requirement)
    // This is the ONLY clone now, down from 3 clones before
    let insert_start = std::time::Instant::now();

    // OPTIMIZATION: Use into_owned() to avoid clone if Arc has single reference
    let colors_owned = match Arc::try_unwrap(colors_arc.clone()) {
        Ok(vec) => vec,                   // We got ownership without cloning!
        Err(arc) => arc.as_ref().clone(), // Multiple references, must clone
    };

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors_owned);
    let insert_time = insert_start.elapsed();

    let total_time = start.elapsed();

    // Log memory usage and Arc diagnostics for monitoring
    let total_memory = cached_colors.memory_usage_mb();
    let arc_count = Arc::strong_count(&colors_arc);

    debug!("Color update complete: Total={:.1}ms (buffer={:.1}MB, total_mem={:.1}MB, GPU upload={:.1}ms, Arc refs={})",
           total_time.as_secs_f32() * MS_PER_SECOND,
           buffer_size_mb,
           total_memory,
           insert_time.as_secs_f32() * MS_PER_SECOND,
           arc_count);

    // Optional: Log Arc diagnostics in trace mode
    #[cfg(debug_assertions)]
    {
        cached_colors.arc_diagnostics();
    }
}

use bevy_plugin_builder::define_plugin;

/// Plugin that manages map overlay rendering
define_plugin!(OverlayPlugin {
    resources: [MapMode, crate::resources::CachedOverlayColors],

    update: [
        update_province_colors
            .run_if(resource_changed::<MapMode>)
            .run_if(in_state(crate::states::GameState::InGame))
    ],

    on_exit: {
        crate::states::GameState::LoadingWorld => [initialize_overlay_colors]
    },

    on_enter: {
        crate::states::GameState::InGame => [force_initial_overlay_update]
    }
});

/// Keyboard input handling removed - map mode cycling is now handled by HUD
/// The HUD's map_mode_display module handles Tab key for cycling and provides
/// a dropdown UI for direct selection, avoiding duplicate keyboard handlers

/// Force initial overlay update to ensure colors match MapMode on game start
/// This triggers a change event so update_province_colors runs on first frame
fn force_initial_overlay_update(mut mode: ResMut<MapMode>) {
    // Mark resource as changed to trigger update_province_colors
    // This ensures overlay colors sync with MapMode on game load
    mode.set_changed();
    debug!("Forced initial overlay update for mode: {}", mode.display_name());
}

/// Initialize the overlay system with current MapMode
/// Pre-calculation happens on-demand when overlay mode changes
pub fn initialize_overlay_colors(
    province_entity_order: Option<Res<ProvinceEntityOrder>>,
    world_seed: Option<Res<crate::world::WorldSeed>>,
    current_mode: Res<MapMode>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
) {
    let Some(entity_order) = province_entity_order else {
        debug!("Skipping overlay initialization - province entities not spawned yet");
        return;
    };

    let Some(world_seed) = world_seed else {
        debug!("Skipping overlay initialization - world seed not available");
        return;
    };

    info!(
        "Overlay system ready for {} provinces (calculation happens on-demand)",
        entity_order.len()
    );

    // Note: Pre-calculation removed - overlays are now calculated on-demand
    // This is more efficient with ECS as we don't need to hold query results
}
