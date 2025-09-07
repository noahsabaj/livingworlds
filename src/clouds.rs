//! Cloud rendering and animation system for Living Worlds
//! 
//! Provides procedural cloud generation using Perlin noise with layered
//! sprites for realistic atmospheric effects.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;

/// Cloud layer depth for parallax effect
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CloudLayer {
    High,    // Furthest, slowest, most transparent
    Medium,  // Middle layer
    Low,     // Closest, fastest, most opaque
}

/// Component for cloud sprites with movement properties
#[derive(Component)]
pub struct CloudSprite {
    pub layer: CloudLayer,
    pub velocity: Vec2,
    pub base_alpha: f32,
}

/// Global cloud system settings
#[derive(Resource)]
pub struct CloudSettings {
    pub global_coverage: f32,  // 0.0 = clear sky, 1.0 = overcast
    pub wind_direction: Vec2,
    pub base_wind_speed: f32,
}

impl Default for CloudSettings {
    fn default() -> Self {
        Self {
            global_coverage: 0.3,  // Light cloud coverage
            wind_direction: Vec2::new(1.0, 0.2).normalize(),
            base_wind_speed: 5.0,  // pixels per second
        }
    }
}

/// Create a procedural cloud texture using Perlin noise with edge falloff
pub fn create_cloud_texture(size: u32, seed: u32, octaves: usize, coverage: f32) -> Image {
    let perlin = Perlin::new(seed);
    let mut pixels = vec![0u8; (size * size * 4) as usize];
    
    // Calculate center and radius for edge falloff
    let center = size as f64 / 2.0;
    let radius = size as f64 / 2.2; // Slightly smaller for soft edges
    
    for y in 0..size {
        for x in 0..size {
            // Normalize coordinates to [0, 1]
            let nx = x as f64 / size as f64;
            let ny = y as f64 / size as f64;
            
            // Generate multi-octave noise for cloud detail
            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 2.0;  // Base frequency
            let mut max_amplitude = 0.0;
            
            for _ in 0..octaves {
                noise_value += perlin.get([nx * frequency, ny * frequency]) * amplitude;
                max_amplitude += amplitude;
                amplitude *= 0.5;
                frequency *= 2.0;
            }
            
            // Normalize and apply coverage threshold
            noise_value = (noise_value / max_amplitude + 1.0) / 2.0;  // Map to [0, 1]
            
            // Apply coverage factor (higher coverage = more clouds)
            let threshold = 1.0 - coverage as f64;
            let mut cloud_density = if noise_value > threshold {
                ((noise_value - threshold) / coverage as f64).min(1.0)
            } else {
                0.0
            };
            
            // Apply radial falloff to eliminate hard square edges
            let dist_from_center = ((x as f64 - center).powi(2) + 
                                   (y as f64 - center).powi(2)).sqrt();
            let edge_falloff = (1.0 - (dist_from_center / radius).min(1.0).powi(2)).max(0.0);
            cloud_density *= edge_falloff;
            
            // Apply smoothstep for softer cloud appearance
            let smoothed = cloud_density * cloud_density * (3.0 - 2.0 * cloud_density);
            
            // Set pixel (white cloud with varying alpha)
            let idx = ((y * size + x) * 4) as usize;
            pixels[idx] = 255;     // R
            pixels[idx + 1] = 255; // G  
            pixels[idx + 2] = 255; // B
            pixels[idx + 3] = (smoothed * 255.0) as u8; // Alpha
        }
    }
    
    Image::new(
        bevy::render::render_resource::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        pixels,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
}

/// Spawn cloud sprites across the map
pub fn spawn_clouds(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    seed: u32,
    map_width: f32,
    map_height: f32,
) {
    // Generate cloud textures with different sizes and coverage for variety
    let cloud_textures = vec![
        (create_cloud_texture(256, seed, 4, 0.4), CloudLayer::High),
        (create_cloud_texture(512, seed + 1, 5, 0.5), CloudLayer::Medium),
        (create_cloud_texture(384, seed + 2, 6, 0.6), CloudLayer::Low),
    ];
    
    // Add cloud textures to assets and spawn cloud sprites
    for (cloud_texture, layer) in cloud_textures {
        let cloud_handle = images.add(cloud_texture);
        
        // Spawn multiple cloud sprites per layer for coverage
        let (num_clouds, z_order, base_alpha, speed_mult) = match layer {
            CloudLayer::High => (8, 100.0, 0.2, 0.3),
            CloudLayer::Medium => (12, 80.0, 0.35, 0.6),
            CloudLayer::Low => (16, 60.0, 0.5, 1.0),
        };
        
        let mut rng = StdRng::seed_from_u64((seed as u64) * 1000 + layer as u64);
        
        for _ in 0..num_clouds {
            let x = rng.gen_range(-map_width..map_width);
            let y = rng.gen_range(-map_height..map_height);
            let scale = rng.gen_range(3.0..6.0);  // Vary cloud sizes
            
            commands.spawn((
                Sprite {
                    image: cloud_handle.clone(),
                    color: Color::srgba(1.0, 1.0, 1.0, base_alpha),
                    custom_size: Some(Vec2::new(256.0 * scale, 256.0 * scale)),
                    ..default()
                },
                Transform::from_xyz(x, y, z_order),
                CloudSprite {
                    layer,
                    velocity: Vec2::new(speed_mult * 5.0, speed_mult * 1.0),
                    base_alpha,
                },
                Name::new(format!("Cloud {:?}", layer)),
            ));
        }
    }
}

/// Animate clouds with wind movement
pub fn animate_clouds(
    mut clouds: Query<(&CloudSprite, &mut Transform)>,
    _cloud_settings: Res<CloudSettings>,
    time: Res<Time>,
) {
    // Use constants from parent module (will be passed as parameters later)
    const MAP_WIDTH: f32 = 300.0 * 50.0 * 1.5;
    const MAP_HEIGHT: f32 = 200.0 * 50.0 * 1.732050808;
    
    for (cloud, mut transform) in &mut clouds {
        // Move cloud based on its layer's speed and wind direction
        let speed_mult = match cloud.layer {
            CloudLayer::High => 0.3,
            CloudLayer::Medium => 0.6,
            CloudLayer::Low => 1.0,
        };
        
        let movement = cloud.velocity * speed_mult * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // Wrap clouds around the map for continuous coverage
        // When a cloud exits one side, it reappears on the opposite side
        if transform.translation.x > MAP_WIDTH * 1.5 {
            transform.translation.x = -MAP_WIDTH * 1.5;
        } else if transform.translation.x < -MAP_WIDTH * 1.5 {
            transform.translation.x = MAP_WIDTH * 1.5;
        }
        
        if transform.translation.y > MAP_HEIGHT * 1.5 {
            transform.translation.y = -MAP_HEIGHT * 1.5;
        } else if transform.translation.y < -MAP_HEIGHT * 1.5 {
            transform.translation.y = MAP_HEIGHT * 1.5;
        }
    }
}

/// Bevy plugin for the cloud system
pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CloudSettings>()
            .add_systems(Update, animate_clouds);
    }
}