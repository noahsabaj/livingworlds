//! Core world data structures that orchestrate features
//!
//! These types don't belong to any single feature but coordinate between them.

use super::clouds::CloudSystem;
use super::provinces::Province;
use super::rivers::RiverSystem;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

/// Main world data structure containing all world state
#[derive(Resource, Debug, Clone, Reflect)]
pub struct World {
    /// All provinces in the world
    pub provinces: Vec<Province>,

    /// River system with flow data
    pub rivers: RiverSystem,

    /// Cloud system with atmospheric data
    pub clouds: CloudSystem,

    /// Climate data storage for runtime visualization
    pub climate_storage: super::terrain::ClimateStorage,

    /// Infrastructure data storage for runtime visualization
    pub infrastructure_storage: super::InfrastructureStorage,

    /// World generation seed for reproducibility
    pub seed: u32,
}

impl World {
    /// Create a new world with the given provinces
    pub fn new(provinces: Vec<Province>, seed: u32) -> Self {
        Self {
            provinces,
            rivers: RiverSystem::default(),
            clouds: CloudSystem::new(),
            climate_storage: super::terrain::ClimateStorage::new(),
            infrastructure_storage: super::InfrastructureStorage::new(),
            seed,
        }
    }

    /// Get a province by its index
    pub fn get_province(&self, index: usize) -> Option<&Province> {
        self.provinces.get(index)
    }

    /// Get a mutable province by its index
    pub fn get_province_mut(&mut self, index: usize) -> Option<&mut Province> {
        self.provinces.get_mut(index)
    }

    /// Get total number of provinces
    pub fn province_count(&self) -> usize {
        self.provinces.len()
    }
}

// ===== WORLD CONFIGURATION TYPES =====
// These resources configure world generation and runtime settings

/// Configuration for world generation - the seed determines the entire world
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct WorldSeed(pub u32);

/// The name of the current world
#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
pub struct WorldName(pub String);

impl Default for WorldName {
    fn default() -> Self {
        Self("Unnamed World".to_string())
    }
}

/// World size configuration controlling map dimensions
#[derive(Resource, Clone, Copy, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub enum WorldSize {
    Small,  // 1250x800 provinces (1,000,000 hexagons)
    Medium, // 1600x1250 provinces (2,000,000 hexagons)
    Large,  // 2000x1500 provinces (3,000,000 hexagons)
}

impl WorldSize {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "small" => WorldSize::Small,
            "large" => WorldSize::Large,
            _ => WorldSize::Medium,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            WorldSize::Small => (1250, 800),   // 1,000,000 hexagons
            WorldSize::Medium => (1600, 1250), // 2,000,000 hexagons
            WorldSize::Large => (2000, 1500),  // 3,000,000 hexagons
        }
    }
}

/// Single source of truth for map dimensions - used by generation, camera, and all systems
#[derive(Resource, Debug, Clone, Copy, Reflect, Default, Serialize, Deserialize)]
pub struct MapDimensions {
    pub provinces_per_row: u32,
    pub provinces_per_col: u32,
    pub width_pixels: f32,
    pub height_pixels: f32,
    pub hex_size: f32,
    pub bounds: MapBounds,
}

/// Map boundary information
#[derive(Debug, Clone, Copy, Reflect, Default, Serialize, Deserialize)]
pub struct MapBounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl MapDimensions {
    pub fn from_world_size(size: &WorldSize) -> Self {
        let (provinces_per_row, provinces_per_col) = size.dimensions();
        let provinces_per_row = provinces_per_row as u32;
        let provinces_per_col = provinces_per_col as u32;

        use crate::math::{HEX_SIZE, SQRT_3};
        let hex_size = HEX_SIZE;
        let width_pixels = provinces_per_row as f32 * hex_size * 1.5;
        let height_pixels = provinces_per_col as f32 * hex_size * SQRT_3;

        Self {
            provinces_per_row,
            provinces_per_col,
            width_pixels,
            height_pixels,
            hex_size,
            bounds: MapBounds {
                x_min: -width_pixels / 2.0,
                x_max: width_pixels / 2.0,
                y_min: -height_pixels / 2.0,
                y_max: height_pixels / 2.0,
            },
        }
    }
}
