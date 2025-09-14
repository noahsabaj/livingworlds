//! Cloud generation for atmospheric effects

use bevy::prelude::Vec2;
use rand::{Rng, rngs::StdRng};
use crate::resources::MapDimensions;
use super::types::{CloudSystem, CloudData, CloudLayer};

/// Builder for generating cloud systems following the builder pattern
/// 
/// This builder encapsulates cloud generation with configurable density and layers.
pub struct CloudBuilder<'a> {
    rng: &'a mut StdRng,
    dimensions: &'a MapDimensions,
    cloud_density: f32,
    high_layer_count: usize,
    medium_layer_count: usize,
    low_layer_count: usize,
}

impl<'a> CloudBuilder<'a> {
    pub fn new(rng: &'a mut StdRng, dimensions: &'a MapDimensions) -> Self {
        Self {
            rng,
            dimensions,
            cloud_density: 1.0,
            high_layer_count: 20,
            medium_layer_count: 30,
            low_layer_count: 40,
        }
    }
    
    /// Set overall cloud density multiplier
    pub fn with_density(mut self, density: f32) -> Self {
        self.cloud_density = density.max(0.0);
        self
    }
    
    /// Set number of high altitude clouds
    pub fn with_high_clouds(mut self, count: usize) -> Self {
        self.high_layer_count = count;
        self
    }
    
    /// Set number of medium altitude clouds
    pub fn with_medium_clouds(mut self, count: usize) -> Self {
        self.medium_layer_count = count;
        self
    }
    
    /// Set number of low altitude clouds
    pub fn with_low_clouds(mut self, count: usize) -> Self {
        self.low_layer_count = count;
        self
    }
    
    pub fn build(self) -> CloudSystem {
        // Delegate to the existing internal implementation
        generate_clouds_internal(self.rng, self.dimensions)
    }
}


// Internal implementation moved to CloudBuilder
fn generate_clouds_internal(rng: &mut StdRng, dimensions: &MapDimensions) -> CloudSystem {
    let map_width = dimensions.bounds.x_max - dimensions.bounds.x_min;
    let map_height = dimensions.bounds.y_max - dimensions.bounds.y_min;
    
    // Define layer configurations
    let layer_configs = vec![
        (CloudLayer::High, 20, 0.2, 0.3),      // (layer, count, alpha, speed)
        (CloudLayer::Medium, 30, 0.35, 0.6),
        (CloudLayer::Low, 40, 0.5, 1.0),
    ];
    
    let mut all_clouds = Vec::new();
    
    for (layer, num_clouds, base_alpha, speed_mult) in layer_configs {
        // Generate clouds in grid pattern with randomness
        let grid_cols = (num_clouds as f32).sqrt().ceil() as usize;
        let grid_rows = (num_clouds + grid_cols - 1) / grid_cols;
        
        for i in 0..num_clouds {
            let grid_x = i % grid_cols;
            let grid_y = i / grid_cols;
            
            // Convert grid position to world coordinates with randomness
            let base_x = (grid_x as f32 / grid_cols as f32 - 0.5) * map_width * 0.95;
            let base_y = (grid_y as f32 / grid_rows as f32 - 0.5) * map_height * 0.95;
            
            // Add random offset
            let offset_x = (rng.gen::<f32>() - 0.5) * map_width * 0.1;
            let offset_y = (rng.gen::<f32>() - 0.5) * map_height * 0.1;
            
            let position = Vec2::new(base_x + offset_x, base_y + offset_y);
            
            // Random size variation
            let size = 100.0 + rng.gen::<f32>() * 200.0;
            
            // Wind velocity based on layer
            let wind_speed = speed_mult * (0.5 + rng.gen::<f32>() * 0.5);
            let velocity = Vec2::new(wind_speed, 0.0);
            
            // Alpha variation
            let alpha = base_alpha * (0.7 + rng.gen::<f32>() * 0.3);
            
            all_clouds.push(CloudData {
                position,
                layer,
                size,
                alpha,
                velocity,
                texture_index: rng.gen::<usize>() % 5, // 5 textures per layer
            });
        }
    }
    
    CloudSystem { clouds: all_clouds }
}