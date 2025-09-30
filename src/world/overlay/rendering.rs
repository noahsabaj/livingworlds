//! Map overlay rendering system for Living Worlds with zero-copy architecture
//!
//! This module handles all visual overlay modes for the map including
//! political boundaries, terrain, mineral resources, and infrastructure.
//! Now optimized with Arc-based zero-copy architecture for instant mode switching.

use super::MapMode;
use crate::constants::MS_PER_SECOND;
use crate::world::ProvinceStorage;
use bevy::log::{debug, trace, warn};
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use std::sync::Arc;

/// Bytes per megabyte for memory calculations
const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

/// System that updates province colors in the mega-mesh based on active overlay mode
/// Now uses Arc-based zero-copy architecture for instant switching
pub fn update_province_colors(
    overlay: Res<MapMode>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
    province_storage: Res<ProvinceStorage>,
    world_seed: Res<crate::world::WorldSeed>,
    mesh_handle: Res<crate::world::WorldMeshHandle>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    nation_colors: Option<Res<crate::nations::NationColorRegistry>>,
    climate_storage: Option<Res<crate::world::terrain::ClimateStorage>>,
    infrastructure_storage: Option<Res<crate::world::InfrastructureStorage>>,
) {
    let start = std::time::Instant::now();
    trace!(
        "Starting color update for overlay: {} at {:.3}s",
        overlay.display_name(),
        time.elapsed_secs()
    );

    // System already only runs when overlay changes due to run_if condition

    let Some(mesh) = meshes.get_mut(&mesh_handle.0) else {
        warn!("Failed to get world mesh for overlay update");
        return;
    };

    let mesh_lookup_time = start.elapsed();

    // Get Arc to colors - NO CLONING, just reference counting!
    let colors_arc = cached_colors.get_or_calculate_with_nations(
        *overlay,
        &province_storage,
        world_seed.0,
        nation_colors.as_ref().map(|r| r.as_ref()),
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

/// Plugin that manages map overlay rendering
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        use crate::states::GameState;

        app.init_resource::<MapMode>()
            .init_resource::<crate::resources::CachedOverlayColors>()
            // Initialize with default overlay and pre-calculate common modes
            .add_systems(OnExit(GameState::LoadingWorld), initialize_overlay_colors)
            // Force initial update to ensure sync between MapMode and overlay colors
            .add_systems(OnEnter(GameState::InGame), force_initial_overlay_update)
            .add_systems(
                Update,
                update_province_colors
                    .run_if(resource_changed::<MapMode>)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

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

/// Initialize the overlay system with current MapMode and pre-calculate common modes
/// If ProvinceStorage doesn't exist (cancelled generation), skip initialization
pub fn initialize_overlay_colors(
    province_storage: Option<Res<ProvinceStorage>>,
    world_seed: Option<Res<crate::world::WorldSeed>>,
    current_mode: Res<MapMode>,
    mut cached_colors: ResMut<crate::resources::CachedOverlayColors>,
) {
    let Some(province_storage) = province_storage else {
        debug!("Skipping overlay initialization - world generation was cancelled");
        return;
    };

    let Some(world_seed) = world_seed else {
        debug!("Skipping overlay initialization - world seed not available");
        return;
    };

    info!(
        "Initializing overlay system for {} provinces",
        province_storage.provinces.len()
    );
    let start = std::time::Instant::now();

    // Don't pre-calculate Political mode here - it needs nation colors which aren't available yet
    // Political will be calculated on-demand by update_province_colors with proper nation data
    // This prevents caching gray fallback colors before nations are loaded
    if *current_mode != MapMode::Political {
        info!("Pre-calculating initial overlay mode: {}", current_mode.display_name());
        cached_colors.get_or_calculate(*current_mode, &province_storage, world_seed.0);
    } else {
        info!("Skipping Political mode pre-calculation - will be calculated with nation colors");
    }

    // Pre-calculate common overlays for instant switching
    cached_colors.pre_calculate_common_modes(&province_storage, world_seed.0);

    let elapsed = start.elapsed();
    let memory_mb = cached_colors.memory_usage_mb();

    info!(
        "Overlay system initialized in {:.2}s (using {:.1}MB, {} modes pre-calculated)",
        elapsed.as_secs_f32(),
        memory_mb,
        cached_colors.cache.len() + 1 // +1 for current
    );
}
