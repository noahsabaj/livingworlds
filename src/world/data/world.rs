//! World data structures - the WHAT of the game world
//! 
//! This module contains the actual data representations of the world,
//! as opposed to the builders/generators that create them.

use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::Province;
use crate::resources::MapDimensions;


/// Complete generated world data, ready for rendering and simulation
/// 
/// This is the final product of world generation - a complete world with
/// all its provinces, rivers, clouds, and spatial indices ready for use.
#[derive(Debug, Clone)]
pub struct World {
    /// All provinces in the world
    pub provinces: Vec<Province>,
    
    /// River system with flow data
    pub rivers: RiverSystem,
    
    /// Spatial index for O(1) province lookups
    pub spatial_index: HashMap<(i32, i32), u32>,
    
    /// Map dimensions and bounds
    pub map_dimensions: MapDimensions,
    
    /// Atmospheric cloud system
    pub clouds: CloudSystem,
}

impl World {
    /// Get a province by its ID
    pub fn get_province(&self, id: u32) -> Option<&Province> {
        self.provinces.iter().find(|p| p.id.value() == id)
    }
    
    /// Get a mutable province by its ID
    pub fn get_province_mut(&mut self, id: u32) -> Option<&mut Province> {
        self.provinces.iter_mut().find(|p| p.id.value() == id)
    }
    
    /// Get total number of provinces
    pub fn province_count(&self) -> usize {
        self.provinces.len()
    }
    
    /// Get number of land provinces (non-ocean)
    pub fn land_count(&self) -> usize {
        use crate::world::terrain::TerrainType;
        self.provinces.iter()
            .filter(|p| p.terrain != TerrainType::Ocean)
            .count()
    }
    
    /// Get number of ocean provinces
    pub fn ocean_count(&self) -> usize {
        use crate::world::terrain::TerrainType;
        self.provinces.iter()
            .filter(|p| p.terrain == TerrainType::Ocean)
            .count()
    }
}


/// River system with flow accumulation tracking
/// 
/// Represents the complete river network in the world, including
/// which provinces contain rivers and how much water flows through them.
#[derive(Debug, Clone, Default)]
pub struct RiverSystem {
    /// Province IDs that contain river tiles
    pub river_tiles: Vec<u32>,
    
    /// Province IDs where rivers meet the ocean (deltas)
    pub delta_tiles: Vec<u32>,
    
    /// Flow accumulation - how much water flows through each tile
    pub flow_accumulation: HashMap<u32, f32>,
}

impl RiverSystem {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if a province contains a river
    pub fn is_river(&self, province_id: u32) -> bool {
        self.river_tiles.contains(&province_id)
    }
    
    /// Check if a province is a river delta
    pub fn is_delta(&self, province_id: u32) -> bool {
        self.delta_tiles.contains(&province_id)
    }
    
    pub fn get_flow(&self, province_id: u32) -> f32 {
        self.flow_accumulation.get(&province_id).copied().unwrap_or(0.0)
    }
    
    pub fn add_river(&mut self, province_id: u32, flow: f32) {
        self.river_tiles.push(province_id);
        self.flow_accumulation.insert(province_id, flow);
    }
    
    pub fn add_delta(&mut self, province_id: u32, flow: f32) {
        self.delta_tiles.push(province_id);
        self.flow_accumulation.insert(province_id, flow);
    }
}


/// Cloud system with all cloud instances
/// 
/// Contains all the clouds in the world, organized for efficient
/// rendering and animation.
#[derive(Debug, Clone, Resource, Default)]
pub struct CloudSystem {
    /// All clouds in the system
    pub clouds: Vec<CloudData>,
}

impl CloudSystem {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_cloud(&mut self, cloud: CloudData) {
        self.clouds.push(cloud);
    }
    
    /// Get clouds by layer
    pub fn get_layer(&self, layer: CloudLayer) -> Vec<&CloudData> {
        self.clouds.iter()
            .filter(|c| c.layer == layer)
            .collect()
    }
    
    /// Get total number of clouds
    pub fn cloud_count(&self) -> usize {
        self.clouds.len()
    }
}

/// Individual cloud instance data
/// 
/// Represents a single cloud with its position, appearance, and movement.
#[derive(Debug, Clone)]
pub struct CloudData {
    /// World position of the cloud
    pub position: Vec2,
    
    /// Which atmospheric layer this cloud is in
    pub layer: CloudLayer,
    
    /// Size/scale of the cloud
    pub size: f32,
    
    /// Transparency (0.0 = invisible, 1.0 = opaque)
    pub alpha: f32,
    
    /// Movement velocity
    pub velocity: Vec2,
    
    /// Index into the texture pool for variety
    pub texture_index: usize,
}

impl CloudData {
    pub fn new(position: Vec2, layer: CloudLayer) -> Self {
        Self {
            position,
            layer,
            size: 1.0,
            alpha: 0.5,
            velocity: Vec2::ZERO,
            texture_index: 0,
        }
    }
    
    /// Builder-style method to set size
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    /// Builder-style method to set alpha
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }
    
    /// Builder-style method to set velocity
    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }
    
    /// Builder-style method to set texture index
    pub fn with_texture(mut self, index: usize) -> Self {
        self.texture_index = index;
        self
    }
}

/// Cloud layer types for atmospheric stratification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloudLayer {
    /// High altitude clouds (cirrus-like)
    High = 0,
    
    /// Medium altitude clouds (cumulus-like)
    Medium = 1,
    
    /// Low altitude clouds (stratus-like)
    Low = 2,
}

impl CloudLayer {
    pub fn altitude(&self) -> f32 {
        match self {
            CloudLayer::High => 10000.0,
            CloudLayer::Medium => 5000.0,
            CloudLayer::Low => 2000.0,
        }
    }
    
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            CloudLayer::High => 1.5,
            CloudLayer::Medium => 1.0,
            CloudLayer::Low => 0.7,
        }
    }
    
    pub fn default_alpha(&self) -> f32 {
        match self {
            CloudLayer::High => 0.3,
            CloudLayer::Medium => 0.5,
            CloudLayer::Low => 0.7,
        }
    }
}