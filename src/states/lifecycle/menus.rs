//! Menu State Lifecycle Management
//!
//! This module handles the enter/exit lifecycle for menu-related states,
//! including MainMenu and Paused states with entity cleanup and camera management.

use crate::nations::{NationLabel, NationLabelShadow};
use crate::states::definitions::*;
use bevy::prelude::*;

/// System that runs when entering the MainMenu state
pub fn enter_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    game_world_query: Query<
        Entity,
        Or<(
            With<crate::world::TerrainEntity>,
            With<crate::world::CloudEntity>,
            With<crate::world::BorderEntity>,
            With<WorldMeshEntity>,
        )>,
    >,
    nation_labels_query: Query<Entity, Or<(With<NationLabel>, With<NationLabelShadow>)>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    #[cfg(feature = "debug-states")]
    debug!("Entering MainMenu state");

    // Clean up any game world entities if returning from game
    for entity in &game_world_query {
        commands.entity(entity).despawn();
    }
    if !game_world_query.is_empty() {
        #[cfg(feature = "debug-states")]
        debug!(
            "Cleaned up {} game world entities",
            game_world_query.iter().count()
        );
    }

    // Clean up nation labels (Text2d entities that aren't part of the world mesh)
    for entity in &nation_labels_query {
        commands.entity(entity).despawn();
    }
    if !nation_labels_query.is_empty() {
        #[cfg(feature = "debug-states")]
        debug!(
            "Cleaned up {} nation label entities",
            nation_labels_query.iter().count()
        );
    }

    // Reset camera position to origin when returning to main menu
    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
        #[cfg(feature = "debug-states")]
        debug!("Reset camera position to origin");
    }

    menu_state.set(MenuState::Main);
    // Main menu UI is spawned by menus.rs module
}

/// Cleanup when exiting the MainMenu state
pub fn exit_main_menu(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    debug!("Exiting MainMenu state");
    // Main menu UI cleanup handled by menus.rs module
}

/// System that runs when entering the Paused state
pub fn enter_paused(_commands: Commands, _time: Res<Time>) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        debug!(
            "[ENTER] Entering Paused state at time: {:.2}s",
            _time.elapsed_secs()
        );
        debug!(
            "[ENTER COMPLETE] Paused state entered in {:.1}ms",
            start.elapsed().as_secs_f32() * 1000.0
        );
    }
    // Pause menu UI is spawned by menus.rs module
}

/// Cleanup when exiting the Paused state
pub fn exit_paused(_commands: Commands) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        debug!("[EXIT] Exiting Paused state");
        debug!(
            "[EXIT COMPLETE] Paused state exited in {:.1}ms",
            start.elapsed().as_secs_f32() * 1000.0
        );
    }
    // Pause menu UI cleanup handled by menus.rs module
}
