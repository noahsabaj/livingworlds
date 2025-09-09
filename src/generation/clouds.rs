//! Cloud generation for atmospheric effects

use bevy::prelude::*;
use rand::{Rng, rngs::StdRng};
use super::types::{CloudSystem, CloudData, CloudLayer, MapDimensions};

/// Generate cloud positions and parameters
pub fn generate(rng: &mut StdRng, dimensions: &MapDimensions) -> CloudSystem {
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