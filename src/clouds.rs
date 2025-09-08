//! Cloud rendering and animation system for Living Worlds
//! 
//! Provides procedural cloud generation using Perlin noise with layered
//! sprites for realistic atmospheric effects.

use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;
use crate::constants::*;
use crate::resources::{WeatherSystem, WeatherState};

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

// CloudSettings removed - replaced by WeatherSystem

/// Types of cloud formations for realistic sky patterns
#[derive(Debug, Clone, Copy)]
pub enum CloudFormationType {
    Cumulus,    // Puffy, cotton-like clusters
    Stratus,    // Horizontal layers
    Cirrus,     // High, wispy streaks
    Scattered,  // Random individual clouds
}

/// Generate cloud positions based on formation type
pub fn generate_cloud_formation(
    center: Vec2,
    count: usize,
    formation_type: CloudFormationType,
    rng: &mut StdRng,
    spread: f32,
) -> Vec<Vec2> {
    let mut positions = Vec::new();
    
    match formation_type {
        CloudFormationType::Cumulus => {
            // Clustered formation - clouds group together
            let cluster_radius = spread * 0.5;
            for _ in 0..count {
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(0.0..cluster_radius) * rng.gen::<f32>().sqrt();
                let offset = Vec2::new(
                    angle.cos() * distance,
                    angle.sin() * distance,
                );
                positions.push(center + offset);
            }
        },
        CloudFormationType::Stratus => {
            // Horizontal band formation - IMPROVED: Add vertical sine wave variation
            let band_width = spread * 2.0;
            let band_height = spread * 0.3;
            let wave_amplitude = spread * 0.2;  // Sine wave height
            let wave_frequency = 3.0;           // Number of waves across the band
            
            for i in 0..count {
                let t = i as f32 / count as f32;
                let x = center.x + (t - 0.5) * band_width;
                
                // Add sine wave variation for more natural looking stratus
                let wave_offset = (t * wave_frequency * std::f32::consts::TAU).sin() * wave_amplitude;
                let random_offset = rng.gen_range(-band_height * 0.5..band_height * 0.5);
                let y = center.y + wave_offset + random_offset;
                
                positions.push(Vec2::new(x, y));
            }
        },
        CloudFormationType::Cirrus => {
            // Wispy diagonal streaks
            let streak_length = spread * 1.5;
            let streak_angle = rng.gen_range(0.0..std::f32::consts::PI / 3.0);
            for i in 0..count {
                let t = i as f32 / count as f32;
                let base_x = center.x + (t - 0.5) * streak_length * streak_angle.cos();
                let base_y = center.y + (t - 0.5) * streak_length * streak_angle.sin();
                // Add some waviness
                let wave = (t * std::f32::consts::PI * 2.0).sin() * spread * 0.1;
                positions.push(Vec2::new(
                    base_x + wave * streak_angle.sin(),
                    base_y - wave * streak_angle.cos(),
                ));
            }
        },
        CloudFormationType::Scattered => {
            // Random placement (original behavior)
            for _ in 0..count {
                let x = center.x + rng.gen_range(-spread..spread);
                let y = center.y + rng.gen_range(-spread..spread);
                positions.push(Vec2::new(x, y));
            }
        },
    }
    
    positions
}

/// Enhanced cloud texture parameters for more variety
pub struct CloudTextureParams {
    pub size: u32,
    pub seed: u32,
    pub octaves: usize,
    pub coverage: f32,
    pub turbulence: f32,      // 0.0 = smooth, 1.0 = very turbulent
    pub stretch: Vec2,         // Elongate clouds in x/y directions
    pub edge_hardness: f32,   // 0.0 = soft edges, 1.0 = hard edges
}

impl Default for CloudTextureParams {
    fn default() -> Self {
        Self {
            size: 256,
            seed: 0,
            octaves: 4,
            coverage: 0.5,
            turbulence: 0.0,
            stretch: Vec2::ONE,
            edge_hardness: 0.0,
        }
    }
}

/// Create a procedural cloud texture with enhanced variety
pub fn create_cloud_texture(params: CloudTextureParams) -> Image {
    let perlin = Perlin::new(params.seed);
    let mut pixels = vec![0u8; (params.size * params.size * 4) as usize];
    
    // Calculate center and radius for edge falloff
    let center = params.size as f64 / 2.0;
    let radius = params.size as f64 / (2.2 - params.edge_hardness as f64 * 0.5); // Harder edges = larger radius
    
    for y in 0..params.size {
        for x in 0..params.size {
            // Apply stretch transformation
            let stretched_x = (x as f64 - center) / (params.stretch.x as f64) + center;
            let stretched_y = (y as f64 - center) / (params.stretch.y as f64) + center;
            
            // Normalize coordinates to [0, 1]
            let nx = stretched_x / params.size as f64;
            let ny = stretched_y / params.size as f64;
            
            // Generate multi-octave noise for cloud detail
            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 2.0;  // Base frequency
            let mut max_amplitude = 0.0;
            
            // Add turbulence by using absolute value of noise
            for i in 0..params.octaves {
                let sample = perlin.get([nx * frequency, ny * frequency]);
                
                // Apply turbulence
                let turbulent_sample = if params.turbulence > 0.0 && i > 0 {
                    sample.abs() * params.turbulence as f64 + sample * (1.0 - params.turbulence as f64)
                } else {
                    sample
                };
                
                noise_value += turbulent_sample * amplitude;
                max_amplitude += amplitude;
                amplitude *= 0.5;
                frequency *= 2.0;
            }
            
            // Normalize and apply coverage threshold
            noise_value = (noise_value / max_amplitude + 1.0) / 2.0;  // Map to [0, 1]
            
            // Apply coverage factor (higher coverage = more clouds)
            let threshold = 1.0 - params.coverage as f64;
            let mut cloud_density = if noise_value > threshold {
                ((noise_value - threshold) / params.coverage as f64).min(1.0)
            } else {
                0.0
            };
            
            // Apply radial falloff to eliminate hard square edges
            let dist_from_center = ((x as f64 - center).powi(2) + 
                                   (y as f64 - center).powi(2)).sqrt();
            let edge_power = 2.0 - params.edge_hardness as f64 * 1.5; // Lower power = harder edge
            let edge_falloff = (1.0 - (dist_from_center / radius).min(1.0).powf(edge_power)).max(0.0);
            cloud_density *= edge_falloff;
            
            // Apply smoothstep for softer cloud appearance (less smoothing for harder edges)
            let smooth_factor = 1.0 - params.edge_hardness as f64 * 0.7;
            let smoothed = if smooth_factor > 0.0 {
                cloud_density * cloud_density * (3.0 - 2.0 * cloud_density) * smooth_factor 
                + cloud_density * (1.0 - smooth_factor)
            } else {
                cloud_density
            };
            
            // Set pixel (white cloud with varying alpha)
            let idx = ((y * params.size + x) * 4) as usize;
            pixels[idx] = 255;     // R
            pixels[idx + 1] = 255; // G  
            pixels[idx + 2] = 255; // B
            pixels[idx + 3] = (smoothed * 255.0) as u8; // Alpha
        }
    }
    
    Image::new(
        bevy::render::render_resource::Extent3d {
            width: params.size,
            height: params.size,
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
    // Define layer configurations - IMPROVED: More clouds (20/30/40 instead of 8/12/16)
    let layer_configs = vec![
        (CloudLayer::High, 20, 100.0, 0.2, 0.3, 256, 4, 0.4),      // (layer, count, z, alpha, speed, size, octaves, coverage)
        (CloudLayer::Medium, 30, 80.0, 0.35, 0.6, 384, 5, 0.5),
        (CloudLayer::Low, 40, 60.0, 0.5, 1.0, 512, 6, 0.6),
    ];
    
    let mut cloud_id = 0u32;
    
    // Generate unique cloud textures for each sprite with formations
    for (layer, num_clouds, z_order, base_alpha, speed_mult, texture_size, octaves, coverage) in layer_configs {
        let mut rng = StdRng::seed_from_u64((seed as u64) * 1000 + layer as u64);
        
        // Choose formation type based on layer - IMPROVED: More variety for low clouds
        let formation_type = match layer {
            CloudLayer::High => CloudFormationType::Cirrus,     // High clouds are wispy
            CloudLayer::Medium => {
                // Mix of cumulus and scattered for medium clouds
                if rng.gen_bool(0.7) {
                    CloudFormationType::Cumulus  // 70% cumulus
                } else {
                    CloudFormationType::Scattered // 30% scattered
                }
            },
            CloudLayer::Low => {
                // IMPROVED: More variety in low cloud formations
                let roll = rng.gen::<f32>();
                if roll < 0.3 {
                    CloudFormationType::Stratus    // 30% horizontal bands
                } else if roll < 0.6 {
                    CloudFormationType::Cumulus    // 30% puffy clusters
                } else if roll < 0.8 {
                    CloudFormationType::Scattered  // 20% scattered
                } else {
                    CloudFormationType::Cirrus     // 20% wispy (low cirrus)
                }
            },
        };
        
        // Generate multiple formations across the map - IMPROVED: More formations (6/8/10)
        let formations_per_layer = match layer {
            CloudLayer::High => 6,     // More high formations for better coverage
            CloudLayer::Medium => 8,   // More medium formations
            CloudLayer::Low => 10,     // Many low formations for dense coverage
        };
        
        let clouds_per_formation = num_clouds / formations_per_layer;
        let remaining_clouds = num_clouds % formations_per_layer;
        
        // IMPROVED: Grid-based distribution instead of random
        let grid_cols = (formations_per_layer as f32).sqrt().ceil() as usize;
        let grid_rows = (formations_per_layer + grid_cols - 1) / grid_cols;
        
        for formation_idx in 0..formations_per_layer {
            // Calculate grid position for even distribution
            let grid_x = formation_idx % grid_cols;
            let grid_y = formation_idx / grid_cols;
            
            // Convert grid position to world coordinates with some randomness
            // Constrain to 95% of map dimensions to match terrain spawn border
            let base_x = (grid_x as f32 / grid_cols as f32 - 0.5) * map_width * 0.95;
            let base_y = (grid_y as f32 / grid_rows as f32 - 0.5) * map_height * 0.95;
            
            // Add random offset to prevent perfect grid appearance
            let formation_center = Vec2::new(
                base_x + rng.gen_range(-map_width * 0.1..map_width * 0.1),
                base_y + rng.gen_range(-map_height * 0.1..map_height * 0.1),
            );
            
            // IMPROVED: Smaller formation spreads (15/12/10% instead of 40/30/25%)
            let formation_spread = match layer {
                CloudLayer::High => map_width * 0.15,    // 15% spread
                CloudLayer::Medium => map_width * 0.12,  // 12% spread
                CloudLayer::Low => map_width * 0.10,     // 10% spread for denser clusters
            };
            
            // Add extra clouds to last formation
            let clouds_in_formation = if formation_idx == formations_per_layer - 1 {
                clouds_per_formation + remaining_clouds
            } else {
                clouds_per_formation
            };
            
            // Generate positions for this formation
            let positions = generate_cloud_formation(
                formation_center,
                clouds_in_formation,
                formation_type,
                &mut rng,
                formation_spread,
            );
            
            for (i, position) in positions.into_iter().enumerate() {
                // Generate unique seed for each cloud sprite
                let unique_seed = seed.wrapping_mul(31)
                    .wrapping_add(cloud_id * 137)
                    .wrapping_add((layer as u32) * 7919)
                    .wrapping_add((formation_idx * 100 + i) as u32);
                cloud_id += 1;
                
                // Vary parameters for unique cloud appearance
                let coverage_variation = rng.gen_range(-0.1..0.1);
                let actual_coverage = ((coverage + coverage_variation) as f32).clamp(0.2, 0.8);
                
                // Enhanced parameters for variety
                let turbulence = match layer {
                    CloudLayer::High => rng.gen_range(0.0..0.2),    // Smooth high clouds
                    CloudLayer::Medium => rng.gen_range(0.1..0.4),  // Some turbulence
                    CloudLayer::Low => rng.gen_range(0.2..0.6),     // More turbulent low clouds
                };
                
                let stretch = match formation_type {
                    CloudFormationType::Stratus => Vec2::new(rng.gen_range(1.5..2.5), rng.gen_range(0.5..0.8)),
                    CloudFormationType::Cirrus => Vec2::new(rng.gen_range(2.0..3.0), rng.gen_range(0.3..0.6)),
                    _ => Vec2::new(rng.gen_range(0.8..1.2), rng.gen_range(0.8..1.2)),
                };
                
                let edge_hardness = rng.gen_range(0.0..0.3);
                
                // Generate unique texture with enhanced variety
                let cloud_texture = create_cloud_texture(CloudTextureParams {
                    size: texture_size,
                    seed: unique_seed,
                    octaves,
                    coverage: actual_coverage,
                    turbulence,
                    stretch,
                    edge_hardness,
                });
                let cloud_handle = images.add(cloud_texture);
                
                // Scale variation
                let scale = rng.gen_range(CLOUD_MIN_SCALE..CLOUD_MAX_SCALE);
                
                // Formation-based velocity (clouds in formation move together)
                let base_velocity = Vec2::new(
                    speed_mult * 5.0,
                    speed_mult * 0.5 * (formation_idx as f32 - 1.5) * 0.2, // Slight layer offset
                );
                let velocity_variation = rng.gen_range(0.9..1.1);
                let velocity = base_velocity * velocity_variation;
                
                commands.spawn((
                    Sprite {
                        image: cloud_handle,
                        color: Color::srgba(1.0, 1.0, 1.0, base_alpha),
                        custom_size: Some(Vec2::new(texture_size as f32 * scale, texture_size as f32 * scale)),
                        ..default()
                    },
                    Transform::from_xyz(position.x, position.y, z_order),
                    CloudSprite {
                        layer,
                        velocity,
                        base_alpha,
                    },
                    Name::new(format!("Cloud_{:?}_F{}_C{}", layer, formation_idx, i)),
                ));
            }
        }
    }
}

/// Animate clouds with wind movement
pub fn animate_clouds(
    mut clouds: Query<(&CloudSprite, &mut Transform)>,
    weather: Res<WeatherSystem>,
    time: Res<Time>,
) {
    // Use global constants for map dimensions
    const MAP_WIDTH: f32 = MAP_WIDTH_PIXELS;
    const MAP_HEIGHT: f32 = MAP_HEIGHT_PIXELS;
    
    for (cloud, mut transform) in &mut clouds {
        // Move cloud based on its layer's speed and weather wind
        let speed_mult = match cloud.layer {
            CloudLayer::High => 0.3,
            CloudLayer::Medium => 0.6,
            CloudLayer::Low => 1.0,
        };
        
        // Combine cloud's individual velocity with global wind
        let effective_velocity = cloud.velocity + weather.wind_speed * speed_mult;
        let movement = effective_velocity * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // Wrap clouds around the map for continuous coverage
        // When a cloud exits one side, it reappears on the opposite side
        // Use tighter boundaries (55% from center) to prevent void rendering
        let wrap_x = MAP_WIDTH * 0.55;
        let wrap_y = MAP_HEIGHT * 0.55;
        
        if transform.translation.x > wrap_x {
            transform.translation.x = -wrap_x;
        } else if transform.translation.x < -wrap_x {
            transform.translation.x = wrap_x;
        }
        
        if transform.translation.y > wrap_y {
            transform.translation.y = -wrap_y;
        } else if transform.translation.y < -wrap_y {
            transform.translation.y = wrap_y;
        }
    }
}

/// System to update weather and manage cloud visibility
pub fn update_weather_system(
    mut weather: ResMut<WeatherSystem>,
    time: Res<Time>,
    mut clouds: Query<(&CloudSprite, &mut Sprite, &mut Transform)>,
    mut rng: Local<Option<StdRng>>,
) {
    // Initialize RNG on first run
    if rng.is_none() {
        *rng = Some(StdRng::from_entropy());
    }
    let rng = rng.as_mut().unwrap();
    
    // Update time since last change
    weather.time_since_change += time.delta_secs();
    
    // Check if we should change weather
    if weather.time_since_change > weather.min_weather_duration {
        let change_roll = rng.gen::<f32>();
        if change_roll < weather.weather_change_chance {
            // Pick a new weather state
            let states = [
                WeatherState::Clear,
                WeatherState::Fair,
                WeatherState::Partly,
                WeatherState::Cloudy,
                WeatherState::Overcast,
            ];
            let new_state = states[rng.gen_range(0..states.len())];
            
            if new_state != weather.current_state {
                weather.target_state = new_state;
                weather.transition_progress = 0.0;
                weather.time_since_change = 0.0;
                println!("☁️ Weather changing from {:?} to {:?}", 
                    weather.current_state.description(), 
                    weather.target_state.description()
                );
            }
        }
    }
    
    // Update transition
    if weather.transition_progress < 1.0 {
        weather.transition_progress = (weather.transition_progress + time.delta_secs() * 0.1).min(1.0);
        
        // Interpolate cloud coverage
        let (current_min, current_max) = weather.current_state.coverage_range();
        let (target_min, target_max) = weather.target_state.coverage_range();
        
        let current_coverage = (current_min + current_max) / 2.0;
        let target_coverage = (target_min + target_max) / 2.0;
        
        weather.cloud_coverage = current_coverage + (target_coverage - current_coverage) * weather.transition_progress;
        
        // Complete transition
        if weather.transition_progress >= 1.0 {
            weather.current_state = weather.target_state;
        }
    }
    
    // Update cloud visibility based on weather
    for (cloud_sprite, mut sprite, mut transform) in &mut clouds {
        // Fade clouds in/out based on weather
        let target_alpha = cloud_sprite.base_alpha * weather.cloud_coverage;
        let current_alpha = sprite.color.alpha();
        let new_alpha = current_alpha + (target_alpha - current_alpha) * time.delta_secs();
        sprite.color.set_alpha(new_alpha);
        
        // Hide clouds completely when very transparent
        if new_alpha < 0.01 {
            transform.translation.z = -1000.0; // Move off-screen
        } else {
            // Restore proper z-order
            let z_order = match cloud_sprite.layer {
                CloudLayer::High => 100.0,
                CloudLayer::Medium => 80.0,
                CloudLayer::Low => 60.0,
            };
            if transform.translation.z < 0.0 {
                transform.translation.z = z_order;
            }
        }
    }
}

/// Dynamic cloud spawning system - only spawn clouds when needed
pub fn dynamic_cloud_spawn_system(
    mut commands: Commands,
    weather: Res<WeatherSystem>,
    clouds: Query<Entity, With<CloudSprite>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut images: ResMut<Assets<Image>>,
    mut last_coverage: Local<f32>,
) {
    // Check if coverage changed significantly
    let coverage_change = (weather.cloud_coverage - *last_coverage).abs();
    if coverage_change < 0.1 {
        return; // No significant change
    }
    *last_coverage = weather.cloud_coverage;
    
    // Calculate target cloud count based on weather
    let target_count = (36.0 * weather.cloud_coverage) as usize;
    let current_count = clouds.iter().count();
    
    if current_count < target_count {
        // Need to spawn more clouds
        let to_spawn = target_count - current_count;
        println!("🌤️ Spawning {} new clouds for {:?} weather", to_spawn, weather.current_state);
        
        // Get camera bounds for spawning
        if let Ok((_, camera_transform)) = camera.single() {
            let camera_pos = camera_transform.translation().truncate();
            
            // Spawn new clouds near camera
            for i in 0..to_spawn {
                let seed = (weather.time_since_change * 1000.0) as u32 + i as u32;
                let texture = create_cloud_texture(CloudTextureParams {
                    size: 256,
                    seed,
                    octaves: 4,
                    coverage: weather.cloud_coverage,
                    ..default()
                });
                let handle = images.add(texture);
                
                let mut rng = StdRng::seed_from_u64(seed as u64);
                let x = camera_pos.x + rng.gen_range(-2000.0..2000.0);
                let y = camera_pos.y + rng.gen_range(-1500.0..1500.0);
                
                commands.spawn((
                    Sprite {
                        image: handle,
                        color: Color::srgba(1.0, 1.0, 1.0, 0.0), // Start invisible
                        custom_size: Some(Vec2::splat(1000.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, y, 70.0),
                    CloudSprite {
                        layer: CloudLayer::Medium,
                        velocity: Vec2::new(5.0, 0.5),
                        base_alpha: 0.4,
                    },
                    Name::new("DynamicCloud"),
                ));
            }
        }
    } else if current_count > target_count && target_count > 0 {
        // Need to remove clouds
        let to_remove = current_count - target_count;
        println!("🌤️ Removing {} clouds for {:?} weather", to_remove, weather.current_state);
        
        // Remove excess clouds
        for (i, entity) in clouds.iter().enumerate() {
            if i >= to_remove {
                break;
            }
            commands.entity(entity).despawn();
        }
    }
}

/// Bevy plugin for the cloud system
pub struct CloudPlugin;

impl Plugin for CloudPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WeatherSystem>()
            .add_systems(Update, (
                update_weather_system,
                animate_clouds,
                dynamic_cloud_spawn_system,
            ).chain());
    }
}