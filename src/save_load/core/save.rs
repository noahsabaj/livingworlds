//! Core save game operations
//!
//! This module handles the actual saving of game state, separated from UI and I/O.

use super::{SaveCompleteEvent, SaveGameEvent};
use super::{SaveGameData, SAVE_DIRECTORY, SAVE_EXTENSION, SAVE_VERSION};
use crate::resources::{
    GameTime, MapDimensions, MapMode, WorldName, WorldSeed, WorldSize, WorldTension,
};
use crate::world::ProvinceStorage;
use crate::nations::{Nation, laws::NationLaws};
use bevy::prelude::*;
use std::collections::HashMap;
use bevy::tasks::IoTaskPool;
use chrono::Local;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;

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
    map_mode: Option<Res<MapMode>>,
    province_storage: Option<Res<ProvinceStorage>>,
    nations_query: Query<(&Nation, &NationLaws)>,
) {
    for event in save_events.read() {
        info!("Saving game to slot: {}", event.slot_name);

        // Gather all game state into SaveGameData (optimized province handling)
        let save_data = SaveGameData {
            version: SAVE_VERSION,
            timestamp: Local::now(),
            world_name: world_name
                .as_ref()
                .map(|n| n.0.clone())
                .unwrap_or_else(|| "Unnamed World".to_string()),
            world_seed: world_seed.as_ref().map(|s| s.0).unwrap_or(0),
            world_size: world_size.as_deref().copied().unwrap_or(WorldSize::Medium),
            map_dimensions: map_dims.as_deref().copied().unwrap_or_default(),
            game_time: game_time.as_deref().cloned().unwrap_or_default(),
            world_tension: world_tension.as_deref().cloned().unwrap_or_default(),
            map_mode: map_mode.as_deref().copied().unwrap_or_default(),
            provinces: province_storage
                .as_ref()
                .map(|s| {
                    // Parallelize province copying for better performance with large worlds
                    if s.provinces.len() > 100000 {
                        s.provinces
                            .par_chunks(50000) // Process in 50k chunks
                            .map(|chunk| chunk.to_vec())
                            .flatten()
                            .collect()
                    } else {
                        s.provinces.clone()
                    }
                })
                .unwrap_or_default(),
            // Collect nation laws data
            nation_laws: nations_query
                .iter()
                .map(|(nation, laws)| (nation.id, laws.clone()))
                .collect(),
        };

        // Serialize and compress
        match super::serialize_save_data(&save_data) {
            Ok(serialized) => {
                match super::compress_data(serialized.as_bytes()) {
                    Ok(compressed) => {
                        // Generate filename with timestamp
                        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                        let filename = format!(
                            "{}/{}_{}.{}",
                            SAVE_DIRECTORY, event.slot_name, timestamp, SAVE_EXTENSION
                        );

                        let filename_clone = filename.clone();
                        let compressed_size = compressed.len() as u64;

                        // Write to file asynchronously
                        IoTaskPool::get()
                            .spawn(async move {
                                match File::create(&filename_clone) {
                                    Ok(mut file) => {
                                        if let Err(e) = file.write_all(&compressed) {
                                            error!("Failed to write save file: {}", e);
                                            false
                                        } else {
                                            info!(
                                                "Game saved successfully to: {} ({}KB compressed)",
                                                filename_clone,
                                                compressed_size / 1024
                                            );
                                            true
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to create save file: {}", e);
                                        false
                                    }
                                }
                            })
                            .detach();

                        complete_events.write(SaveCompleteEvent {
                            success: true,
                            message: format!(
                                "Game saved to {} ({}KB)",
                                filename,
                                compressed_size / 1024
                            ),
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
