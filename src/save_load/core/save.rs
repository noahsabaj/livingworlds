//! Core save game operations
//!
//! This module handles the actual saving of game state, separated from UI and I/O.

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use chrono::Local;
use std::fs::File;
use std::io::Write;
use crate::world::ProvinceStorage;
use crate::resources::{WorldSeed, WorldName, WorldSize, MapDimensions, GameTime, WorldTension, ResourceOverlay};
use super::{SaveGameData, SAVE_VERSION, SAVE_DIRECTORY, SAVE_EXTENSION};
use super::{SaveGameEvent, SaveCompleteEvent};

/// Handle save game requests with compression and versioning
pub fn handle_save_game(
    mut save_events: EventReader<SaveGameEvent>,
    mut complete_events: EventWriter<SaveCompleteEvent>,
    world_seed: Option<Res<WorldSeed>>,
    world_name: Option<Res<WorldName>>,
    world_size: Option<Res<WorldSize>>,
    map_dims: Option<Res<MapDimensions>>,
    game_time: Option<Res<GameTime>>,
    world_tension: Option<Res<WorldTension>>,
    resource_overlay: Option<Res<ResourceOverlay>>,
    province_storage: Option<Res<ProvinceStorage>>,
) {
    for event in save_events.read() {
        println!("Saving game to slot: {}", event.slot_name);

        // Gather all game state into SaveGameData
        let save_data = SaveGameData {
            version: SAVE_VERSION,
            timestamp: Local::now(),
            world_name: world_name.as_ref().map(|n| n.0.clone()).unwrap_or_else(|| "Unnamed World".to_string()),
            world_seed: world_seed.as_ref().map(|s| s.0).unwrap_or(0),
            world_size: world_size.as_deref().copied().unwrap_or(WorldSize::Medium),
            map_dimensions: map_dims.as_deref().copied().unwrap_or_default(),
            game_time: game_time.as_deref().cloned().unwrap_or_default(),
            world_tension: world_tension.as_deref().cloned().unwrap_or_default(),
            resource_overlay: resource_overlay.as_deref().copied().unwrap_or_default(),
            provinces: province_storage.as_ref()
                .map(|s| s.provinces.clone())
                .unwrap_or_default(),
        };

        // Serialize and compress
        match super::serialize_save_data(&save_data) {
            Ok(serialized) => {
                match super::compress_data(serialized.as_bytes()) {
                    Ok(compressed) => {
                        // Generate filename with timestamp
                        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                        let filename = format!("{}/{}_{}.{}",
                            SAVE_DIRECTORY, event.slot_name, timestamp, SAVE_EXTENSION);

                        let filename_clone = filename.clone();
                        let compressed_size = compressed.len() as u64;

                        // Write to file asynchronously
                        IoTaskPool::get()
                            .spawn(async move {
                                match File::create(&filename_clone) {
                                    Ok(mut file) => {
                                        if let Err(e) = file.write_all(&compressed) {
                                            eprintln!("Failed to write save file: {}", e);
                                            false
                                        } else {
                                            println!("Game saved successfully to: {} ({}KB compressed)",
                                                filename_clone, compressed_size / 1024);
                                            true
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create save file: {}", e);
                                        false
                                    }
                                }
                            })
                            .detach();

                        complete_events.write(SaveCompleteEvent {
                            success: true,
                            message: format!("Game saved to {} ({}KB)", filename, compressed_size / 1024),
                        });
                    }
                    Err(e) => {
                        complete_events.write(SaveCompleteEvent {
                            success: false,
                            message: e,
                        });
                    }
                }
            }
            Err(e) => {
                complete_events.write(SaveCompleteEvent {
                    success: false,
                    message: e,
                });
            }
        }
    }
}

/// Quick save functionality
pub fn quick_save(world: &mut World) {
    world.send_event(SaveGameEvent {
        slot_name: "quicksave".to_string(),
    });
}