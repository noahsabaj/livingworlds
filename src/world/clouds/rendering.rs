//! Cloud rendering and animation system for Living Worlds
//!
//! Provides procedural cloud generation using our centralized Perlin noise
//! module with layered sprites for realistic atmospheric effects.

use super::types::{CloudLayer, CloudSystem};
use crate::constants::*;
use crate::math::{fast_sin, smoothstep, PerlinNoise};
use crate::resources::{WeatherState, WeatherSystem};
use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use rayon::prelude::*;
use std::f32::consts::{PI, TAU};

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
    Cumulus,   // Puffy, cotton-like clusters
    Stratus,   // Horizontal layers
    Cirrus,    // High, wispy streaks
    Scattered, // Random individual clouds
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
                let angle = rng.gen_range(0.0..TAU);
                let distance = rng.gen_range(0.0..cluster_radius) * rng.r#gen::<f32>().sqrt();
                let (sin_angle, cos_angle) = angle.sin_cos();
                let offset = Vec2::new(cos_angle * distance, sin_angle * distance);
                positions.push(center + offset);
            }
        }
        CloudFormationType::Stratus => {
            // Horizontal band formation - IMPROVED: Add vertical sine wave variation
            let band_width = spread * 2.0;
            let band_height = spread * 0.3;
            let wave_amplitude = spread * 0.2; // Sine wave height
            let wave_frequency = 3.0; // Number of waves across the band

            for i in 0..count {
                let t = i as f32 / count as f32;
                let x = center.x + (t - 0.5) * band_width;

                // Add sine wave variation for more natural looking stratus
                let wave_offset = fast_sin(t * wave_frequency * TAU) * wave_amplitude;
                let random_offset = rng.gen_range(-band_height * 0.5..band_height * 0.5);
                let y = center.y + wave_offset + random_offset;

                positions.push(Vec2::new(x, y));
            }
        }
        CloudFormationType::Cirrus => {
            // Wispy diagonal streaks
            let streak_length = spread * 1.5;
            let streak_angle = rng.gen_range(0.0..PI / 3.0);
            let (sin_streak, cos_streak) = streak_angle.sin_cos();
            for i in 0..count {
                let t = i as f32 / count as f32;
                let base_x = center.x + (t - 0.5) * streak_length * cos_streak;
                let base_y = center.y + (t - 0.5) * streak_length * sin_streak;
                // Add some waviness
                let wave = fast_sin(t * PI * 2.0) * spread * 0.1;
                positions.push(Vec2::new(
                    base_x + wave * sin_streak,
                    base_y - wave * cos_streak,
                ));
            }
        }
        CloudFormationType::Scattered => {
            // Random placement (original behavior)
            for _ in 0..count {
                let x = center.x + rng.gen_range(-spread..spread);
                let y = center.y + rng.gen_range(-spread..spread);
                positions.push(Vec2::new(x, y));
            }
        }
    }

    positions
}

/// Enhanced cloud texture parameters for more variety
pub struct CloudTextureParams {
    pub size: u32,
    pub seed: u32,
    pub octaves: usize,
    pub coverage: f32,
    pub turbulence: f32,    // 0.0 = smooth, 1.0 = very turbulent
    pub stretch: Vec2,      // Elongate clouds in x/y directions
    pub edge_hardness: f32, // 0.0 = soft edges, 1.0 = hard edges
}

impl Default for CloudTextureParams {
    fn default() -> Self {
        Self {
            size: 256,
            seed: 0xDEADBEEF, // Placeholder - should be overridden with proper seed
            octaves: 4,
            coverage: 0.5,
            turbulence: 0.0,
            stretch: Vec2::ONE,
            edge_hardness: 0.0,
        }
    }
}

pub fn create_cloud_texture(params: CloudTextureParams) -> Image {
    // Use our centralized noise generator - works out of the box!
    let noise = PerlinNoise::with_seed(params.seed);

    let center = params.size as f64 / 2.0;
    let radius = params.size as f64 / (2.2 - params.edge_hardness as f64 * 0.5); // Harder edges = larger radius

    // Generate pixels in PARALLEL for massive speedup!
    // Each row can be processed independently
    let pixels: Vec<u8> = (0..params.size)
        .into_par_iter() // Parallel iterator over rows
        .flat_map(|y| {
            let mut row_pixels = Vec::with_capacity((params.size * 4) as usize);

            for x in 0..params.size {
                // Apply stretch transformation
                let stretched_x = (x as f64 - center) / (params.stretch.x as f64) + center;
                let stretched_y = (y as f64 - center) / (params.stretch.y as f64) + center;

                // Normalize coordinates to [0, 1]
                let nx = stretched_x / params.size as f64;
                let ny = stretched_y / params.size as f64;

                // Use our ready-made cloud sampling with turbulence!
                // All the complexity of multi-octave noise is handled internally
                let noise_value = if params.turbulence > 0.0 {
                    // Sample clouds with billow effect for fluffy appearance
                    use crate::math::CloudPreset;
                    noise.sample_clouds(nx * 2.0, ny * 2.0, CloudPreset::Fluffy)
                } else {
                    // Regular FBM for smoother clouds
                    use crate::math::FbmSettings;
                    noise.sample_fbm(
                        nx * 2.0,
                        ny * 2.0,
                        FbmSettings {
                            octaves: params.octaves as u32,
                            frequency: 0.02, // Cloud frequency
                            persistence: 0.5,
                            lacunarity: 2.0,
                        },
                    )
                };

                // Already normalized to [0, 1] by our module!

                // Apply coverage factor (higher coverage = more clouds)
                let threshold = 1.0 - params.coverage as f64;
                let mut cloud_density = if noise_value > threshold {
                    ((noise_value - threshold) / params.coverage as f64).min(1.0)
                } else {
                    0.0
                };

                // Apply radial falloff to eliminate hard square edges
                let dist_from_center =
                    ((x as f64 - center).powi(2) + (y as f64 - center).powi(2)).sqrt();
                let edge_power = 2.0 - params.edge_hardness as f64 * 1.5; // Lower power = harder edge
                let edge_falloff =
                    (1.0 - (dist_from_center / radius).min(1.0).powf(edge_power)).max(0.0);
                cloud_density *= edge_falloff;

                // Apply smoothstep for softer cloud appearance using centralized function
                let smooth_factor = 1.0 - params.edge_hardness as f64 * 0.7;
                let smoothed = if smooth_factor > 0.0 {
                    let smooth_value = smoothstep(0.0, 1.0, cloud_density as f32) as f64;
                    smooth_value * smooth_factor + cloud_density * (1.0 - smooth_factor)
                } else {
                    cloud_density
                };

                // Add pixel to row (white cloud with varying alpha)
                row_pixels.push(255); // R
                row_pixels.push(255); // G
                row_pixels.push(255); // B
                row_pixels.push((smoothed * 255.0) as u8); // Alpha
            }

            row_pixels
        })
        .collect(); // Collect all parallel results into final pixel buffer

    Image::new(
        bevy::render::render_resource::Extent3d {
            width: params.size,
            height: params.size,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        pixels,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    )
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

    weather.time_since_change += time.delta_secs();

    if weather.time_since_change > weather.min_weather_duration {
        let change_roll = rng.r#gen::<f32>();
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
                debug!(
                    "☁️ Weather changing from {:?} to {:?}",
                    weather.current_state.description(),
                    weather.target_state.description()
                );
            }
        }
    }

    if weather.transition_progress < 1.0 {
        weather.transition_progress =
            (weather.transition_progress + time.delta_secs() * 0.1).min(1.0);

        // Interpolate cloud coverage
        let (current_min, current_max) = weather.current_state.coverage_range();
        let (target_min, target_max) = weather.target_state.coverage_range();

        let current_coverage = (current_min + current_max) / 2.0;
        let target_coverage = (target_min + target_max) / 2.0;

        weather.cloud_coverage =
            current_coverage + (target_coverage - current_coverage) * weather.transition_progress;

        // Complete transition
        if weather.transition_progress >= 1.0 {
            weather.current_state = weather.target_state;
        }
    }

    for (cloud_sprite, mut sprite, mut transform) in &mut clouds {
        // Fade clouds in/out based on weather
        let target_alpha = cloud_sprite.base_alpha * weather.cloud_coverage;
        let current_alpha = sprite.color.alpha();
        let new_alpha = current_alpha + (target_alpha - current_alpha) * time.delta_secs();
        sprite.color.set_alpha(new_alpha);

        // Hide clouds completely when very transparent
        if new_alpha < 0.01 {
            transform.translation.z = OFF_SCREEN_Z; // Move off-screen
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
    let coverage_change = (weather.cloud_coverage - *last_coverage).abs();
    if coverage_change < 0.1 {
        return; // No significant change
    }
    *last_coverage = weather.cloud_coverage;

    let target_count = (36.0 * weather.cloud_coverage) as usize;
    let current_count = clouds.iter().count();

    if current_count < target_count {
        // Need to spawn more clouds
        let to_spawn = target_count - current_count;
        info!(
            "🌤️ Spawning {} new clouds for {:?} weather",
            to_spawn, weather.current_state
        );

        if let Ok((_, camera_transform)) = camera.single() {
            let camera_pos = camera_transform.translation().truncate();

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
                    super::types::CloudEntity, // Mark as cloud entity
                ));
            }
        }
    } else if current_count > target_count && target_count > 0 {
        // Need to remove clouds
        let to_remove = current_count - target_count;

        // Remove excess clouds
        for (i, entity) in clouds.iter().enumerate() {
            if i >= to_remove {
                break;
            }
            commands.entity(entity).despawn();
        }
    }
}

use bevy_plugin_builder::define_plugin;

/// Bevy plugin for the cloud system
define_plugin!(CloudPlugin {
    resources: [WeatherSystem, CloudSystem],

    update: [
        (
            update_weather_system,
            animate_clouds,
            dynamic_cloud_spawn_system,
        ).chain().run_if(in_state(crate::states::GameState::InGame))
    ],

    on_enter: {
        crate::states::GameState::InGame => [spawn_clouds_from_data]
    }
});

/// Spawn cloud entities from generated cloud data
fn spawn_clouds_from_data(
    mut commands: Commands,
    mut cloud_system: ResMut<CloudSystem>,
    mut images: ResMut<Assets<Image>>,
    mut clouds_spawned: Local<bool>,
) {
    // Only spawn if we haven't spawned yet
    if !*clouds_spawned {
        // If CloudSystem is empty, generate procedural clouds
        if cloud_system.clouds.is_empty() {
            use super::types::{CloudData, CloudLayer};
            use rand::{Rng, SeedableRng};

            let mut rng = rand::rngs::StdRng::seed_from_u64(12345); // Fixed seed for consistency

            // Generate clouds for each layer
            for &layer in CloudLayer::all() {
                let cloud_count = match layer {
                    CloudLayer::High => 15,
                    CloudLayer::Medium => 25,
                    CloudLayer::Low => 35,
                };

                for _ in 0..cloud_count {
                    let position = Vec2::new(
                        rng.gen_range(-2000.0..2000.0),
                        rng.gen_range(-1500.0..1500.0),
                    );

                    let cloud = CloudData::new(position, layer)
                        .with_size(rng.gen_range(200.0..600.0))
                        .with_alpha(layer.default_alpha())
                        .with_velocity(Vec2::new(
                            rng.gen_range(-20.0..20.0),
                            rng.gen_range(-5.0..5.0),
                        ))
                        .with_texture(rng.gen_range(0..5));

                    cloud_system.add_cloud(cloud);
                }
            }

            info!("Generated {} procedural clouds", cloud_system.clouds.len());
        }

        info!(
            "Spawning {} clouds from cloud system...",
            cloud_system.clouds.len()
        );

        // Create texture pools (5 textures per layer, 3 layers)
        let mut texture_pools: Vec<Vec<Handle<Image>>> = Vec::new();
        let textures_per_layer = 5;

        for layer_idx in 0..3 {
            let mut layer_textures = Vec::new();

            for texture_idx in 0..textures_per_layer {
                let unique_seed = (layer_idx * 1000 + texture_idx) as u32;

                // Generate texture with variety
                let texture = create_cloud_texture(CloudTextureParams {
                    size: (256 + layer_idx * 64) as u32,
                    seed: unique_seed,
                    octaves: 4 + layer_idx as usize,
                    coverage: 0.4 + layer_idx as f32 * 0.1,
                    turbulence: 0.3 + (texture_idx as f32) * 0.1,
                    stretch: Vec2::new(1.0 + (texture_idx as f32) * 0.2, 1.0),
                    edge_hardness: 0.2 + (texture_idx as f32) * 0.1,
                });

                layer_textures.push(images.add(texture));
            }

            texture_pools.push(layer_textures);
        }

        for cloud_data in &cloud_system.clouds {
            let layer_idx = cloud_data.layer as usize;
            let texture_handle = texture_pools[layer_idx]
                [cloud_data.texture_index % textures_per_layer as usize]
                .clone();

            commands.spawn((
                Sprite {
                    image: texture_handle,
                    color: Color::srgba(1.0, 1.0, 1.0, cloud_data.alpha),
                    custom_size: Some(Vec2::splat(cloud_data.size)),
                    ..default()
                },
                Transform::from_xyz(
                    cloud_data.position.x,
                    cloud_data.position.y,
                    match cloud_data.layer {
                        CloudLayer::High => 100.0,
                        CloudLayer::Medium => 80.0,
                        CloudLayer::Low => 60.0,
                    },
                ),
                CloudSprite {
                    layer: cloud_data.layer,
                    velocity: cloud_data.velocity,
                    base_alpha: cloud_data.alpha,
                },
                Name::new(format!("Cloud_{:?}", cloud_data.layer)),
                crate::world::CloudEntity, // Mark as cloud entity
            ));
        }

        // Mark clouds as spawned to prevent re-spawning
        *clouds_spawned = true;

        info!("Cloud entities spawned successfully");
    }
}

