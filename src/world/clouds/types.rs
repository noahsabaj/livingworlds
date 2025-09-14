//! Cloud data structures and types
//!
//! This module contains all cloud-related data types including
//! CloudSystem, CloudData, and CloudLayer.

use bevy::prelude::{Component, Resource, Vec2};

/// Marker component for cloud entities
///
/// This component marks cloud sprite entities that float above the world.
/// Clouds are animated and drift across the map.
#[derive(Component, Default)]
pub struct CloudEntity;

/// System managing all clouds in the world
#[derive(Debug, Clone, Default, Resource)]
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
        self.clouds.iter().filter(|c| c.layer == layer).collect()
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
    /// Get all layer variants
    pub fn all() -> &'static [CloudLayer] {
        &[CloudLayer::High, CloudLayer::Medium, CloudLayer::Low]
    }

    /// Get the altitude multiplier for this layer
    pub fn altitude_factor(&self) -> f32 {
        match self {
            CloudLayer::High => 1.0,
            CloudLayer::Medium => 0.7,
            CloudLayer::Low => 0.4,
        }
    }

    /// Get the default alpha for this layer
    pub fn default_alpha(&self) -> f32 {
        match self {
            CloudLayer::High => 0.3,
            CloudLayer::Medium => 0.5,
            CloudLayer::Low => 0.7,
        }
    }
}
