//! Core load game operations
//!
//! This module handles the actual loading of game state, separated from UI and I/O.

use super::{LoadCompleteEvent, LoadGameEvent};
use super::{PendingLoadData, SaveGameList};
use crate::loading_screen::{set_loading_progress, start_save_loading, LoadingState};
use crate::resources::{ProvincesSpatialIndex, WorldName, WorldSeed};
use crate::states::{GameState, RequestStateTransition};
use crate::world::{build_world_mesh, CloudBuilder, ProvinceStorage, WorldMeshHandle};
use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::sprite::MeshMaterial2d;
use rand::{rngs::StdRng, SeedableRng};
use std::collections::HashMap;
use std::fs;

/// Handle load game requests with decompression and deserialization
pub fn handle_load_game(
    mut load_events: EventReader<LoadGameEvent>,
    mut complete_events: EventWriter<LoadCompleteEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in load_events.read() {
        info!("Loading game from: {:?}", event.save_path);

        match fs::read(&event.save_path) {
            Ok(compressed_data) => {
                // Decompress
                match super::decompress_data(&compressed_data) {
                    Ok(decompressed) => {
                        // Deserialize
                        let data_str = String::from_utf8_lossy(&decompressed);
                        match super::deserialize_save_data(&data_str) {
                            Ok(save_data) => {
                                if save_data.version > super::super::types::SAVE_VERSION {
                                    error!(
                                        "Save file version {} is newer than game version {}",
                                        save_data.version,
                                        super::super::types::SAVE_VERSION
                                    );
                                    complete_events.write(LoadCompleteEvent {
                                        success: false,
                                        message: format!("Save file version {} incompatible with game version {}",
                                            save_data.version, super::super::types::SAVE_VERSION),
                                    });
                                    continue;
                                }

                                info!("Successfully loaded save from {}", save_data.timestamp);
                                info!(
                                    "Game time: {} days, World size: {:?}",
                                    save_data.game_time.current_date, save_data.world_size
                                );

                                // Initialize loading screen
                                let mut loading_state = LoadingState::default();
                                let file_size = std::fs::metadata(&event.save_path)
                                    .map(|m| super::format_file_size(m.len()))
                                    .unwrap_or_else(|_| "Unknown".to_string());
                                start_save_loading(
                                    &mut loading_state,
                                    event
                                        .save_path
                                        .file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("Save")
                                        .to_string(),
                                    save_data.game_time.current_date,
                                    file_size,
                                );
                                commands.insert_resource(loading_state);

                                // Store data for loading
                                commands.insert_resource(PendingLoadData(save_data));

                                // Transition to loading state
                                next_state.set(GameState::LoadingWorld);

                                complete_events.write(LoadCompleteEvent {
                                    success: true,
                                    message: format!("Game loaded from {:?}", event.save_path),
                                });
                            }
                            Err(e) => {
                                complete_events.write(LoadCompleteEvent {
                                    success: false,
                                    message: e,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        complete_events.write(LoadCompleteEvent {
                            success: false,
                            message: e,
                        });
                    }
                }
            }
            Err(e) => {
                complete_events.write(LoadCompleteEvent {
                    success: false,
                    message: format!("Failed to read save file: {}", e),
                });
            }
        }
    }
}

/// Check if we have pending save data to load instead of generating a new world
pub fn check_for_pending_load(
    mut commands: Commands,
    pending_load: Option<Res<PendingLoadData>>,
    mut state_events: EventWriter<RequestStateTransition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut loading_state: ResMut<LoadingState>,
) {
    if let Some(load_data) = pending_load {
        info!("Restoring game state from save...");
        set_loading_progress(&mut loading_state, 0.2, "Restoring game state...");

        // Insert all the loaded resources
        commands.insert_resource(WorldSeed(load_data.0.world_seed));
        commands.insert_resource(WorldName(load_data.0.world_name.clone()));
        commands.insert_resource(load_data.0.world_size);
        commands.insert_resource(load_data.0.map_dimensions);
        commands.insert_resource(load_data.0.game_time.clone());
        commands.insert_resource(load_data.0.world_tension.clone());
        commands.insert_resource(load_data.0.map_mode);
        set_loading_progress(&mut loading_state, 0.4, "Resources restored...");

        // Rebuild world mesh
        info!(
            "Rebuilding world mesh from {} provinces...",
            load_data.0.provinces.len()
        );
        set_loading_progress(&mut loading_state, 0.5, "Rebuilding world mesh...");
        let mesh_handle = build_world_mesh(&load_data.0.provinces, &mut meshes, load_data.0.world_seed);
        set_loading_progress(&mut loading_state, 0.8, "Creating game entities...");

        commands.spawn((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("World Mega-Mesh"),
        ));

        // Store the mesh handle
        commands.insert_resource(WorldMeshHandle(mesh_handle.clone()));

        // Create province storage
        let mut province_by_id = HashMap::new();
        for (idx, province) in load_data.0.provinces.iter().enumerate() {
            province_by_id.insert(province.id, idx);
        }

        commands.insert_resource(ProvinceStorage {
            provinces: load_data.0.provinces.clone(),
            province_by_id,
        });

        // Create spatial index
        let mut spatial_index = ProvincesSpatialIndex::default();
        for province in &load_data.0.provinces {
            spatial_index.insert(province.position, province.id.value());
        }
        commands.insert_resource(spatial_index);

        // Generate cloud system
        let mut rng = StdRng::seed_from_u64(load_data.0.world_seed as u64);
        let cloud_system = CloudBuilder::new(&mut rng, &load_data.0.map_dimensions).build();
        commands.insert_resource(cloud_system);

        // Remove pending load data
        commands.remove_resource::<PendingLoadData>();
        set_loading_progress(&mut loading_state, 1.0, "Load complete!");

        // Transition to game
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::InGame,
        });
    }
}

/// Load the most recent save
pub fn load_latest_save(world: &mut World) {
    // Scan for saves
    {
        let mut save_list = world.resource_mut::<SaveGameList>();
        super::scan_save_files_internal(&mut save_list);
    }

    let save_list = world.resource::<SaveGameList>();
    if let Some(latest) = save_list.saves.first() {
        world.send_event(LoadGameEvent {
            save_path: latest.path.clone(),
        });
    }
}
