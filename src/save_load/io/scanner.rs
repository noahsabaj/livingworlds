//! Save file scanning and directory management
//!
//! This module handles directory operations and save file discovery.

use super::metadata::extract_save_metadata;
use super::SaveGameList;
use super::{SaveGameInfo, SAVE_DIRECTORY, SAVE_EXTENSION};
use bevy::prelude::*;
use chrono::Local;
use std::fs;
use std::path::Path;

/// Ensure the save directory exists
pub fn ensure_save_directory() {
    if let Err(e) = fs::create_dir_all(SAVE_DIRECTORY) {
        eprintln!("Failed to create save directory: {}", e);
    }
}

/// Scan the save directory and populate the save game list (ECS wrapper)
pub fn scan_save_files(mut save_list: ResMut<SaveGameList>) {
    scan_save_files_internal(&mut save_list);
}

/// Internal function for scanning save files without ECS wrapper
pub fn scan_save_files_internal(save_list: &mut SaveGameList) {
    save_list.saves.clear();

    let save_dir = Path::new(SAVE_DIRECTORY);
    if !save_dir.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(save_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if extension == SAVE_EXTENSION {
                            if let Some(file_name) = entry.file_name().to_str() {
                                let name =
                                    file_name.trim_end_matches(&format!(".{}", SAVE_EXTENSION));

                                // Try to parse date from filename
                                let date_created =
                                    parse_date_from_filename(name).unwrap_or_else(|| {
                                        metadata
                                            .modified()
                                            .ok()
                                            .and_then(|t| {
                                                t.duration_since(std::time::UNIX_EPOCH).ok()
                                            })
                                            .and_then(|d| {
                                                use chrono::{Local, TimeZone};
                                                Local.timestamp_opt(d.as_secs() as i64, 0).single()
                                            })
                                            .unwrap_or_else(Local::now)
                                    });

                                // Extract metadata from the save file
                                let (world_size, game_time, version, world_name, world_seed) =
                                    extract_save_metadata(&entry.path()).unwrap_or_else(|| {
                                        (
                                            "Unknown".to_string(),
                                            0.0,
                                            1,
                                            "Unnamed World".to_string(),
                                            0,
                                        )
                                    });

                                let save_info = SaveGameInfo {
                                    name: name.to_string(),
                                    path: entry.path(),
                                    date_created,
                                    world_name,
                                    world_seed,
                                    world_size,
                                    game_time,
                                    version,
                                    compressed_size: metadata.len(),
                                };
                                save_list.saves.push(save_info);
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort by date, newest first
    save_list
        .saves
        .sort_by(|a, b| b.date_created.cmp(&a.date_created));
}

fn parse_date_from_filename(name: &str) -> Option<chrono::DateTime<Local>> {
    if name.starts_with("save_") {
        let parts: Vec<&str> = name.split('_').collect();
        if parts.len() >= 4 {
            let date_str = format!("{} {}", parts[3], parts[4]);
            chrono::NaiveDateTime::parse_from_str(&date_str, "%Y%m%d %H%M%S")
                .ok()
                .and_then(|naive| {
                    use chrono::{Local, TimeZone};
                    Local.from_local_datetime(&naive).single()
                })
        } else {
            None
        }
    } else {
        None
    }
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
