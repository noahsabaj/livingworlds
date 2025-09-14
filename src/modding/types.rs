//! Type definitions for the modding system
//!
//! This module contains all the data structures used for configuration
//! and mod management. These types mirror the RON configuration files.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Metadata for a mod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<ModDependency>,
    pub compatible_game_version: String,
    pub load_order: i32,
}

/// Dependency specification for a mod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDependency {
    pub mod_id: String,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub optional: bool,
}

/// Represents a loaded mod
#[derive(Debug, Clone)]
pub struct LoadedMod {
    pub manifest: ModManifest,
    pub path: PathBuf,
    pub config_overrides: ModConfigOverrides,
    pub source: ModSource,
    pub enabled: bool,
}

/// Where a mod came from
#[derive(Debug, Clone)]
pub enum ModSource {
    Local(PathBuf),
    Workshop(u64), // Steam Workshop ID
}

/// The complete game configuration, merging base + all active mods
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub balance: BalanceConfig,
    pub colors: ColorsConfig,
    pub generation: GenerationConfig,
    pub simulation: SimulationConfig,
    pub audio: AudioConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            balance: BalanceConfig::default(),
            colors: ColorsConfig::default(),
            generation: GenerationConfig::default(),
            simulation: SimulationConfig::default(),
            audio: AudioConfig::default(),
        }
    }
}

/// Mod-specific configuration overrides
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModConfigOverrides {
    pub balance: Option<BalanceConfig>,
    pub colors: Option<ColorsConfig>,
    pub generation: Option<GenerationConfig>,
    pub simulation: Option<SimulationConfig>,
    pub audio: Option<AudioConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceConfig {
    pub world: WorldConfig,
    pub camera: CameraConfig,
    pub ui: UIConfig,
    pub simulation: SimulationBalanceConfig,
    pub clouds: CloudConfig,
    pub generation: GenerationBalanceConfig,
    pub spatial: SpatialConfig,
    pub hexagon: HexagonConfig,
}

impl Default for BalanceConfig {
    fn default() -> Self {
        Self {
            world: WorldConfig::default(),
            camera: CameraConfig::default(),
            ui: UIConfig::default(),
            simulation: SimulationBalanceConfig::default(),
            clouds: CloudConfig::default(),
            generation: GenerationBalanceConfig::default(),
            spatial: SpatialConfig::default(),
            hexagon: HexagonConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub hex_size_pixels: f32,
    pub provinces_per_row: u32,
    pub provinces_per_col: u32,
    pub edge_buffer: f32,
    pub ocean_depth_shallow: f32,
    pub ocean_depth_medium: f32,
    pub ocean_depth_deep: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            hex_size_pixels: 50.0,
            provinces_per_row: 300,
            provinces_per_col: 200,
            edge_buffer: 200.0,
            ocean_depth_shallow: 0.12,
            ocean_depth_medium: 0.07,
            ocean_depth_deep: 0.02,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub pan_speed_base: f32,
    pub speed_multiplier: f32,
    pub edge_pan_threshold: f32,
    pub edge_pan_speed_base: f32,
    pub map_padding_factor: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            zoom_speed: 0.1,
            min_zoom: 0.3,
            max_zoom: 6.0,
            pan_speed_base: 500.0,
            speed_multiplier: 3.0,
            edge_pan_threshold: 10.0,
            edge_pan_speed_base: 800.0,
            map_padding_factor: 1.25,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub tile_info_text_size: f32,
    pub padding_percent: f32,
    pub margin_percent: f32,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            tile_info_text_size: 18.0,
            padding_percent: 1.0,
            margin_percent: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationBalanceConfig {
    pub starting_year: u64,
    pub days_per_year: f32,
    pub default_speed: f32,
    pub max_speed: f32,
    pub min_population: f32,
    pub max_additional_population: f32,
}

impl Default for SimulationBalanceConfig {
    fn default() -> Self {
        Self {
            starting_year: 1000,
            days_per_year: 365.0,
            default_speed: 1.0,
            max_speed: 10.0,
            min_population: 1000.0,
            max_additional_population: 49000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    pub min_scale: f32,
    pub max_scale: f32,
    pub layer_count: usize,
    pub base_speed: f32,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            min_scale: 3.0,
            max_scale: 6.0,
            layer_count: 3,
            base_speed: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationBalanceConfig {
    pub nation_count: usize,
    pub tectonic_plates_base: usize,
    pub tectonic_plates_variation: u32,
    pub island_chain_count: usize,
    pub archipelago_count: usize,
    pub continent_size_multiplier: f32,
    pub continent_massive_base: f32,
    pub continent_massive_variation: f32,
    pub continent_medium_base: f32,
    pub continent_medium_variation: f32,
    pub continent_archipelago_base: f32,
    pub continent_archipelago_variation: f32,
    pub continent_tiny_base: f32,
    pub continent_tiny_variation: f32,
    pub continent_falloff_base: f32,
    pub continent_falloff_variation: f32,
    pub river_count: usize,
    pub river_min_elevation: f32,
}

impl Default for GenerationBalanceConfig {
    fn default() -> Self {
        Self {
            nation_count: 8,
            tectonic_plates_base: 4,
            tectonic_plates_variation: 3,
            island_chain_count: 0,
            archipelago_count: 2,
            continent_size_multiplier: 1.5,
            continent_massive_base: 8000.0,
            continent_massive_variation: 3000.0,
            continent_medium_base: 5000.0,
            continent_medium_variation: 2000.0,
            continent_archipelago_base: 2000.0,
            continent_archipelago_variation: 800.0,
            continent_tiny_base: 800.0,
            continent_tiny_variation: 400.0,
            continent_falloff_base: 0.8,
            continent_falloff_variation: 0.3,
            river_count: 200,
            river_min_elevation: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialConfig {
    pub index_cell_size_multiplier: f32,
    pub ocean_depth_grid_size_multiplier: f32,
}

impl Default for SpatialConfig {
    fn default() -> Self {
        Self {
            index_cell_size_multiplier: 2.0,
            ocean_depth_grid_size_multiplier: 3.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexagonConfig {
    pub aa_width: f32,
    pub texture_alpha_opaque: u8,
    pub texture_alpha_transparent: u8,
}

impl Default for HexagonConfig {
    fn default() -> Self {
        Self {
            aa_width: 1.5,
            texture_alpha_opaque: 255,
            texture_alpha_transparent: 0,
        }
    }
}

// COLOR CONFIGURATION (simplified for brevity)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorsConfig {
    pub terrain: HashMap<String, Color>,
    pub minerals: HashMap<String, Color>,
    pub ui: HashMap<String, Color>,
}

impl Default for ColorsConfig {
    fn default() -> Self {
        Self {
            terrain: HashMap::new(),
            minerals: HashMap::new(),
            ui: HashMap::new(),
        }
    }
}

// GENERATION CONFIGURATION (simplified for brevity)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub continent_generation: HashMap<String, f32>,
    pub elevation: HashMap<String, f32>,
    pub rivers: HashMap<String, f32>,
    pub climate: HashMap<String, f32>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            continent_generation: HashMap::new(),
            elevation: HashMap::new(),
            rivers: HashMap::new(),
            climate: HashMap::new(),
        }
    }
}

// SIMULATION CONFIGURATION (simplified for brevity)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub time: HashMap<String, f32>,
    pub population: HashMap<String, f32>,
    pub tension: HashMap<String, f32>,
    pub economics: HashMap<String, f32>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            time: HashMap::new(),
            population: HashMap::new(),
            tension: HashMap::new(),
            economics: HashMap::new(),
        }
    }
}

// AUDIO CONFIGURATION (simplified for brevity)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub effects: HashMap<String, f32>,
    pub ambient: HashMap<String, f32>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            effects: HashMap::new(),
            ambient: HashMap::new(),
        }
    }
}
