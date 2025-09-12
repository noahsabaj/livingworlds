//! Shared types for world generation system

use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::Province;

/// Complete generated world data, ready for rendering
#[derive(Debug, Clone)]
pub struct GeneratedWorld {
    pub provinces: Vec<Province>,
    pub rivers: RiverSystem,
    pub spatial_index: HashMap<(i32, i32), u32>,
    pub map_dimensions: MapDimensions,
    pub clouds: CloudSystem,
}

/// River system with flow accumulation tracking
#[derive(Debug, Clone)]
pub struct RiverSystem {
    pub river_tiles: Vec<u32>,        // Province IDs that are rivers
    pub delta_tiles: Vec<u32>,        // Province IDs where rivers meet ocean
    pub flow_accumulation: HashMap<u32, f32>, // How much water flows through each tile
}

/// Cloud system with generated cloud positions and parameters
#[derive(Debug, Clone, Resource)]
pub struct CloudSystem {
    pub clouds: Vec<CloudData>,
}

/// Individual cloud data for spawning
#[derive(Debug, Clone)]
pub struct CloudData {
    pub position: Vec2,
    pub layer: CloudLayer,
    pub size: f32,
    pub alpha: f32,
    pub velocity: Vec2,
    pub texture_index: usize,  // Index into texture pool
}

/// Cloud layer types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CloudLayer {
    High = 0,
    Medium = 1,
    Low = 2,
}

/// Map dimension information
#[derive(Debug, Clone, Copy, Resource)]
pub struct MapDimensions {
    pub provinces_per_row: u32,
    pub provinces_per_col: u32,
    pub hex_size: f32,
    pub bounds: MapBounds,
}

#[derive(Debug, Clone, Copy)]
pub struct MapBounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl MapDimensions {
    /// Create map dimensions from world size
    pub fn from_world_size(size: &crate::resources::WorldSize) -> Self {
        use crate::constants::*;
        
        let (provinces_per_row, provinces_per_col) = match size {
            crate::resources::WorldSize::Small => (600, 500),   // 300k provinces
            crate::resources::WorldSize::Medium => (800, 750),  // 600k provinces  
            crate::resources::WorldSize::Large => (1000, 900),  // 900k provinces
        };
        
        let hex_size = HEX_SIZE_PIXELS;
        let map_width = provinces_per_row as f32 * hex_size * 1.732;
        let map_height = provinces_per_col as f32 * hex_size * 1.5;
        
        Self {
            provinces_per_row,
            provinces_per_col,
            hex_size,
            bounds: MapBounds {
                x_min: -map_width / 2.0,
                x_max: map_width / 2.0,
                y_min: -map_height / 2.0,
                y_max: map_height / 2.0,
            },
        }
    }
}