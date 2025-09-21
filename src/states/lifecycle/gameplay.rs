//! Gameplay State Lifecycle Management
//!
//! This module handles the enter/exit lifecycle for gameplay-related states,
//! including InGame state with world mesh spawning and simulation management.

use crate::states::definitions::*;
use bevy::prelude::*;

/// System that runs when entering the InGame state
pub fn enter_in_game(
    mut commands: Commands,
    mesh_handle: Res<crate::world::WorldMeshHandle>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_world_meshes: Query<Entity, With<WorldMeshEntity>>,
) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        debug!("[ENTER] Entering InGame state");
        debug!(
            "[ENTER COMPLETE] InGame state entered in {:.1}ms",
            start.elapsed().as_secs_f32() * 1000.0
        );
    }

    // Only spawn world mesh if it doesn't already exist
    if existing_world_meshes.is_empty() {
        info!("Spawning world mesh entity for rendering");
        let material = materials.add(ColorMaterial::from(Color::WHITE));

        commands.spawn((
            Mesh2d(mesh_handle.0.clone()),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("World Mesh"),
            WorldMeshEntity, // Marker component for cleanup
        ));

        info!("World mesh entity spawned successfully");
    } else {
        debug!("World mesh entity already exists, skipping spawn");
    }
}

/// Cleanup when exiting the InGame state
pub fn exit_in_game(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        debug!("[EXIT] Exiting InGame state");
        debug!(
            "[EXIT COMPLETE] InGame state exited in {:.1}ms",
            start.elapsed().as_secs_f32() * 1000.0
        );
    }

    // Don't clean up game world here - it should persist when pausing
    // World cleanup happens in enter_main_menu when returning to menu

    // Pause simulation
}
