//! Core data types for save/load system
//!
//! This module contains all the data structures used for saving and loading game state.

use crate::resources::{GameTime, MapDimensions, MapMode, WorldSize, WorldTension};
use crate::nations::laws::{NationLaws, LawRegistry};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// Directory where save files are stored
pub const SAVE_DIRECTORY: &str = "saves";

/// Save file extension (compressed RON)
pub const SAVE_EXTENSION: &str = "lws"; // Living Worlds Save

/// Current save version for compatibility checking
pub const SAVE_VERSION: u32 = 1;

/// Auto-save interval in seconds
pub const AUTO_SAVE_INTERVAL: f32 = 300.0; // 5 minutes

/// Information about a save file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveGameInfo {
    pub name: String,
    pub path: PathBuf,
    pub date_created: DateTime<Local>,
    pub world_name: String,
    pub world_seed: u32,
    pub world_size: String,
    pub game_time: f32,
    pub version: u32,
    pub compressed_size: u64,
}

/// Complete game state for serialization
#[derive(Serialize, Deserialize)]
pub struct SaveGameData {
    pub version: u32,
    pub timestamp: DateTime<Local>,
    pub world_name: String,
    pub world_seed: u32,
    pub world_size: WorldSize,
    pub map_dimensions: MapDimensions,
    pub game_time: GameTime,
    pub world_tension: WorldTension,
    pub map_mode: MapMode,
    pub provinces: Vec<crate::world::Province>,
    /// Nation laws data - entity IDs will be remapped on load
    pub nation_laws: HashMap<crate::nations::NationId, NationLaws>,
}
