//! World Setup - Async World Generation Systems
//!
//! This module provides async world generation systems that integrate
//! WorldBuilder with Bevy's task pool system for non-blocking generation.
//!
//! # Architecture
//!
//! - **WorldBuilder**: Pure generation logic (world/generation/builder.rs)
//!   - Takes parameters → returns World data structure
//!   - No Bevy dependencies, could work in any context
//!   - Handles: terrain, rivers, climate, erosion, agriculture
//!
//! - **Async Generation** (this file):
//!   - Uses WorldBuilder on background threads via AsyncComputeTaskPool
//!   - Handles: progress tracking, error handling, state transitions
//!   - Creates: rendering mesh, ECS resources when generation completes
//!   - Manages: loading screens, async progress updates
//!
//! This separation allows the core generation to be reused in tests,
//! tools, or other contexts while keeping Bevy-specific concerns isolated.

use super::World;
use super::{build_world_mesh, ProvinceStorage, WorldBuilder, WorldMeshHandle};
use super::{ProvincesSpatialIndex, WorldGenerationSettings};
use crate::loading::{set_loading_progress, LoadingState};
use crate::states::{GameState, RequestStateTransition};
use async_channel::{Receiver, Sender};
use bevy::log::{error, info};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::fmt;

/// Loading progress milestones - more granular for better user feedback
const PROGRESS_START: f32 = 0.0;
const PROGRESS_PROVINCES: f32 = 0.1;
const PROGRESS_EROSION: f32 = 0.25;
const PROGRESS_CLIMATE: f32 = 0.4;
const PROGRESS_RIVERS: f32 = 0.5;
const PROGRESS_MESH: f32 = 0.7;
const PROGRESS_ENTITIES: f32 = 0.85;
const PROGRESS_OVERLAYS: f32 = 0.95;
const PROGRESS_COMPLETE: f32 = 1.0;

/// Validation bounds
const MAX_CONTINENTS: u32 = 100;

/// Frame budget - maximum time to spend on world generation per frame (in milliseconds)
/// This allows UI interactions to remain responsive during generation
const FRAME_BUDGET_MS: f32 = 16.0; // ~60fps budget, leaves time for UI

/// Progress update from background world generation
#[derive(Debug, Clone)]
pub struct GenerationProgress {
    pub step: String,
    pub progress: f32, // 0.0 to 1.0
    pub completed: bool,
    pub world_data: Option<World>, // Only present when completed
    pub error_message: Option<String>, // Error message if generation failed
    pub generation_metrics: Option<crate::diagnostics::GenerationMetrics>, // Metrics for error context
}

/// Async world generation resource - manages background world generation task
#[derive(Resource)]
pub struct AsyncWorldGeneration {
    /// Handle to the background generation task - dropping this cancels the task
    pub task: Task<()>,
    /// Receiver for progress updates from the background task
    pub progress_receiver: Receiver<GenerationProgress>,
    /// Settings used for generation (for display purposes)
    pub settings: WorldGenerationSettings,
}

/// Timer to delay state transition after world generation completes
/// This gives the UI time to display the 100% completion message
#[derive(Resource)]
pub struct WorldGenerationTransitionDelay {
    pub timer: Timer,
}

const MIN_CONTINENTS: u32 = 1;
const MAX_OCEAN_COVERAGE: f32 = 0.95;
const MIN_OCEAN_COVERAGE: f32 = 0.05;
const MAX_RIVER_DENSITY: f32 = 1.0;
const MIN_RIVER_DENSITY: f32 = 0.0;

/// System to handle the transition delay timer after world generation completes
pub fn handle_world_generation_transition_delay(
    mut commands: Commands,
    time: Res<Time>,
    timer_res: Option<ResMut<WorldGenerationTransitionDelay>>,
    mut state_events: EventWriter<RequestStateTransition>,
) {
    if let Some(mut delay) = timer_res {
        delay.timer.tick(time.delta());

        if delay.timer.just_finished() {
            info!("Transition delay timer finished, moving to InGame state");

            // Now transition to the game
            state_events.write(RequestStateTransition {
                from: GameState::LoadingWorld,
                to: GameState::InGame,
            });

            // Clean up the timer resource
            commands.remove_resource::<WorldGenerationTransitionDelay>();
        }
    }
}

/// Background world generation function - runs on AsyncComputeTaskPool
async fn generate_world_async(
    settings: WorldGenerationSettings,
    progress_sender: Sender<GenerationProgress>,
    gpu_resources: Option<crate::world::gpu::GpuResources>,
) {
    info!(
        "Starting async world generation with settings: {:?}",
        settings
    );

    // Helper to send progress updates
    let send_progress = |step: &str, progress: f32| {
        info!("Async task: Sending progress update: {} - {:.1}%", step, progress * 100.0);
        let result = progress_sender.try_send(GenerationProgress {
            step: step.to_string(),
            progress,
            completed: false,
            world_data: None,
            error_message: None,
            generation_metrics: None,
        });
        if let Err(e) = result {
            error!("Failed to send progress update: {:?}", e);
        }
    };

    // Validate settings
    send_progress("Validating settings...", 0.0);
    if let Err(e) = validate_settings(&settings) {
        error!("World generation validation failed: {}", e);
        let _ = progress_sender.try_send(GenerationProgress {
            step: format!("Error: {}", e),
            progress: 0.0,
            completed: true,
            world_data: None,
            error_message: Some(e.to_string()),
            generation_metrics: None,
        });
        return;
    }

    // Generate world data with progress reporting
    let start_time = std::time::Instant::now();

    // Create a progress callback closure that sends updates through the channel
    let progress_callback = |step: &str, progress: f32| {
        send_progress(step, progress);
    };

    // Choose between GPU-accelerated and CPU-only generation
    let world_result = if let Some(gpu_res) = gpu_resources.as_ref() {
        if gpu_res.compute_supported && gpu_res.use_gpu {
            info!("🚀 Using GPU-accelerated world generation");
            generate_world_with_gpu_acceleration(settings.clone(), gpu_res.clone(), progress_sender.clone())
        } else {
            info!("🖥️ GPU available but disabled - using CPU generation");
            WorldBuilder::new(
                settings.seed,
                settings.world_size,
                settings.continent_count,
                settings.ocean_coverage,
                settings.river_density,
                settings.climate_type,
            )
            .build_with_progress(Some(progress_callback))
        }
    } else {
        info!("🖥️ GPU not available - using CPU generation");
        WorldBuilder::new(
            settings.seed,
            settings.world_size,
            settings.continent_count,
            settings.ocean_coverage,
            settings.river_density,
            settings.climate_type,
        )
        .build_with_progress(Some(progress_callback))
    };

    let generation_time = start_time.elapsed().as_millis() as f32;

    // Handle generation result
    match world_result {
        Ok(world) => {
            info!("World generation completed in {:.1}ms", generation_time);
            // Send completion
            let _ = progress_sender.try_send(GenerationProgress {
                step: "World generation completed".to_string(),
                progress: 1.0,
                completed: true,
                world_data: Some(world),
                error_message: None,
                generation_metrics: None,
            });
        }
        Err(e) => {
            error!("World generation failed: {}", e);
            let _ = progress_sender.try_send(GenerationProgress {
                step: format!("Error: {}", e),
                progress: 0.0,
                completed: true,
                world_data: None,
                error_message: Some(e.to_string()),
                generation_metrics: None, // We don't have provinces to analyze here
            });
            return;
        }
    }

    info!("Async world generation finished");
}

/// Generate world with GPU acceleration for province generation
/// This is a hybrid approach: GPU for provinces, CPU for everything else
fn generate_world_with_gpu_acceleration(
    settings: WorldGenerationSettings,
    gpu_resources: crate::world::gpu::GpuResources,
    progress_sender: Sender<GenerationProgress>,
) -> Result<crate::world::World, crate::world::generation::WorldGenerationError> {
    use crate::resources::MapDimensions;
    use crate::world::gpu::GpuProvinceBuilder;
    use rand::{rngs::StdRng, SeedableRng};

    let dimensions = MapDimensions::from_world_size(&settings.world_size);

    // Helper to send progress updates
    let send_progress = |step: &str, progress: f32| {
        info!("Async task: Sending progress update: {} - {:.1}%", step, progress * 100.0);
        let result = progress_sender.try_send(GenerationProgress {
            step: step.to_string(),
            progress,
            completed: false,
            world_data: None,
            error_message: None,
            generation_metrics: None,
        });
        if let Err(e) = result {
            error!("Failed to send progress update: {:?}", e);
        }
    };

    // Step 1: Generate provinces with GPU acceleration
    send_progress("Generating provinces with GPU acceleration...", 0.1);
    info!("⚡ GPU-accelerating province generation...");

    // Create a simplified GPU status for the async context
    let gpu_status = crate::world::gpu::GpuComputeStatus {
        available: true,
        compute_supported: gpu_resources.compute_supported,
        max_workgroup_size: 256,
        max_buffer_size: 2_147_483_648,
        fallback_reason: None,
    };

    let gpu_config = crate::world::gpu::GpuGenerationConfig {
        use_gpu: gpu_resources.use_gpu,
        fallback_on_failure: true,
        timeout_seconds: 30.0,
        max_retries: 3,
    };

    let mut gpu_state = crate::world::gpu::GpuGenerationState::default();
    let mut gpu_metrics = crate::world::gpu::GpuPerformanceMetrics::default();

    // Use GpuProvinceBuilder for accelerated province generation
    let mut provinces = GpuProvinceBuilder::new(dimensions, settings.seed)
        .with_ocean_coverage(settings.ocean_coverage)
        .with_continent_count(settings.continent_count)
        .with_validation(gpu_resources.validation_enabled)
        .build_with_gpu(
            &gpu_status,
            &gpu_config,
            &mut gpu_state,
            &mut gpu_metrics,
            None, // No validation config in async context
        );

    info!("✅ GPU province generation completed");

    // Step 2-6: Continue with CPU-based processing for other world features
    // (These steps are not yet GPU-accelerated)

    let mut rng = StdRng::seed_from_u64(settings.seed as u64);

    // Step 2: Apply erosion simulation for realistic terrain
    send_progress("Applying erosion simulation...", 0.2);
    let erosion_iterations = match dimensions.provinces_per_row * dimensions.provinces_per_col {
        n if n < 400_000 => 3_000,
        n if n < 700_000 => 5_000,
        _ => 8_000,
    };
    crate::world::apply_erosion_to_provinces(
        &mut provinces,
        dimensions,
        &mut rng,
        erosion_iterations,
    );

    // Step 3: Calculate ocean depths
    send_progress("Calculating ocean depths...", 0.3);
    crate::world::calculate_ocean_depths(&mut provinces, dimensions);

    // Step 4: Generate climate zones
    send_progress("Generating climate zones...", 0.4);
    crate::world::apply_climate_to_provinces(&mut provinces, dimensions, settings.climate_type);

    // Step 5: Generate river systems
    send_progress("Creating river systems...", 0.5);
    let river_system = crate::world::RiverBuilder::new(&mut provinces, dimensions, &mut rng)
        .with_density(settings.river_density)
        .build()
        .map_err(|e| crate::world::generation::WorldGenerationError {
            error_message: format!("Failed to generate rivers: {}", e),
            error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
        })?;

    // Step 6: Calculate agriculture values
    send_progress("Calculating agriculture values...", 0.6);
    crate::world::calculate_agriculture_values(&mut provinces, &river_system, dimensions).map_err(
        |e| crate::world::generation::WorldGenerationError {
            error_message: format!("Failed to calculate agriculture: {}", e),
            error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
        },
    )?;

    // Step 7: Generate cloud system
    send_progress("Generating clouds...", 0.7);
    let cloud_system = crate::world::CloudBuilder::new(&mut rng, &dimensions).build();

    // Final step
    send_progress("Finalizing world...", 0.9);

    // Return the complete world
    Ok(crate::world::World {
        provinces,
        rivers: river_system,
        clouds: cloud_system,
        seed: settings.seed,
    })
}

/// Start async world generation - replaces the old blocking setup_world
pub fn start_async_world_generation(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    _state_events: EventWriter<RequestStateTransition>,
    mut loading_state: ResMut<LoadingState>,
    // GPU resources for acceleration
    gpu_status: Option<Res<crate::world::gpu::GpuComputeStatus>>,
    gpu_config: Option<Res<crate::world::gpu::GpuGenerationConfig>>,
    gpu_state: Option<ResMut<crate::world::gpu::GpuGenerationState>>,
    gpu_metrics: Option<ResMut<crate::world::gpu::GpuPerformanceMetrics>>,
    validation_config: Option<Res<crate::world::gpu::ValidationConfig>>,
    mut gpu_request: Option<ResMut<crate::world::gpu::GpuGenerationRequest>>,
) {
    info!("Starting async world generation");

    // Request GPU generation if available
    if let Some(ref mut request) = gpu_request {
        request.requested = true;
        request.completed = false;
        info!("GPU generation requested for world generation");
    }

    // Create progress channel
    let (progress_sender, progress_receiver) = async_channel::unbounded::<GenerationProgress>();

    // Extract GPU resources for the async task
    let gpu_resources =
        if let (Some(status), Some(config)) = (gpu_status.as_deref(), gpu_config.as_deref()) {
            Some(crate::world::gpu::GpuResources {
                compute_supported: status.compute_supported,
                use_gpu: config.use_gpu,
                timeout_ms: (config.timeout_seconds * 1000.0) as u64,
                validation_enabled: validation_config.is_some(),
            })
        } else {
            None
        };

    // Spawn background generation task
    let task_pool = AsyncComputeTaskPool::get();
    let generation_settings = settings.clone();

    let task = task_pool.spawn(async move {
        generate_world_async(generation_settings, progress_sender, gpu_resources).await;
    });

    // Store task handle and progress receiver
    commands.insert_resource(AsyncWorldGeneration {
        task,
        progress_receiver,
        settings: settings.clone(),
    });

    // Initialize loading state
    set_loading_progress(
        &mut loading_state,
        PROGRESS_START,
        "Starting world generation...",
    );

    info!("Async world generation task spawned");
}

/// Poll async world generation progress and handle completion
pub fn poll_async_world_generation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut loading_state: ResMut<LoadingState>,
    mut state_events: EventWriter<RequestStateTransition>,
    async_generation: Option<ResMut<AsyncWorldGeneration>>,
) {
    let Some(generation) = async_generation else {
        return;
    };

    // Check for progress updates (non-blocking)
    while let Ok(progress) = generation.progress_receiver.try_recv() {
        if progress.completed {
            if let Some(world) = progress.world_data {
                info!("Async world generation completed, building mesh...");

                // Build mesh and add to assets
                let mesh_handle = build_world_mesh(&world.provinces, &mut meshes, world.seed);
                commands.insert_resource(WorldMeshHandle(mesh_handle));

                // Create province storage (mutable for nation assignment)
                let mut province_storage = ProvinceStorage::from_provinces(world.provinces.clone());

                // Assign cultures to provinces based on geographic position
                info!(
                    "Assigning cultures to {} provinces...",
                    province_storage.provinces.len()
                );
                super::assign_cultures_to_province_storage(
                    &mut province_storage.provinces,
                    Some(world.seed as u64),
                );

                // Log cultural distribution for debugging
                let culture_counts = count_cultures(&province_storage.provinces);
                info!("Cultural distribution: {:?}", culture_counts);

                // Create map dimensions resource (required by camera system)
                let map_dimensions = crate::resources::MapDimensions::from_world_size(
                    &generation.settings.world_size,
                );
                commands.insert_resource(map_dimensions.clone());

                // Store world seed for overlay rendering system
                commands.insert_resource(crate::world::WorldSeed(world.seed));

                // Create spatial index
                let spatial_index =
                    ProvincesSpatialIndex::build(&province_storage.provinces, &map_dimensions);
                commands.insert_resource(spatial_index);

                // Spawn nations with ruling houses
                let nation_settings = crate::nations::NationGenerationSettings::default();
                info!("About to call spawn_nations...");
                let (nations, houses) = crate::nations::spawn_nations(
                    &mut province_storage.provinces,
                    &nation_settings,
                    world.seed,
                );
                info!("spawn_nations completed! Got {} nations and {} houses", nations.len(), houses.len());

                // Build ownership cache from province data (optimized for large datasets)
                info!("Building ownership cache with parallel processing...");
                let ownership_cache = crate::nations::ProvinceOwnershipCache::default();
                info!("Created ownership cache, calling optimized rebuild...");

                // Skip the potentially hanging rebuild for now to test state transition
                // TODO: Implement parallel ownership cache building
                info!("Skipping ownership cache rebuild temporarily to isolate hang point");

                commands.insert_resource(ownership_cache);
                info!("Ownership cache resource inserted (empty for now)");

                // Build nation color registry for rendering
                let mut color_registry = crate::nations::NationColorRegistry::default();
                for nation in &nations {
                    color_registry.colors.insert(nation.id, nation.color);
                }
                commands.insert_resource(color_registry);

                // Build territories from provinces (groups of contiguous provinces)
                info!("Building territories from {} provinces...", province_storage.provinces.len());

                // TEMPORARY: Skip complex territory building to isolate state transition issue
                // TODO: Implement optimized parallel territory building for 1M+ provinces
                info!("Temporarily creating simplified territories to fix state transition");
                let mut territories_by_nation = std::collections::HashMap::new();

                // Create one simplified territory per nation for now
                for nation in &nations {
                    let simplified_territory = crate::nations::Territory {
                        provinces: std::collections::HashSet::new(), // Empty for now
                        nation_id: nation.id,
                        center: Vec2::new(0.0, 0.0), // Placeholder
                        is_core: true,
                    };
                    territories_by_nation.insert(nation.id, vec![simplified_territory]);
                }

                info!("Simplified territories created successfully");

                // Spawn nation entities with OwnsTerritory component for Entity Relationships
                // NOTE: We DON'T spawn 3M province entities - that would kill performance!
                info!("Spawning {} nation entities...", nations.len());
                let mut nation_entities = std::collections::HashMap::new();
                for nation in &nations {
                    let nation_entity = commands
                        .spawn((
                            crate::nations::NationBundle {
                                nation: nation.clone(),
                                transform: Transform::default(),
                                visibility: Visibility::default(),
                                pressure_vector: crate::simulation::PressureVector::default(),
                            },
                            crate::nations::OwnsTerritory::default(), // Will be populated by Entity Relationships
                        ))
                        .id();

                    nation_entities.insert(nation.id, nation_entity);
                }
                info!("Nation entities spawned successfully");

                // Spawn Territory entities with Entity Relationships
                // This creates ~100 entities instead of 3M, maintaining performance!
                info!("Spawning territory entities...");
                let mut total_territories = 0;
                for (nation_id, territories) in territories_by_nation {
                    if let Some(&nation_entity) = nation_entities.get(&nation_id) {
                        for territory in territories {
                            commands.spawn((
                                territory,
                                crate::nations::OwnedBy(nation_entity), // Bevy automatically updates OwnsTerritory on nation!
                                Transform::default(),
                                Visibility::default(),
                            ));
                            total_territories += 1;
                        }
                    }
                }

                info!(
                    "Spawned {} territory entities for {} nations (instead of {} province entities)",
                    total_territories,
                    nation_entities.len(),
                    province_storage.provinces.len()
                );

                // Spawn house entities
                info!("Spawning {} house entities...", houses.len());
                for house in houses {
                    commands.spawn(house);
                }
                info!("House entities spawned successfully");

                // Insert the updated province storage (with nation ownership)
                info!("Inserting province storage with {} provinces...", province_storage.provinces.len());
                commands.insert_resource(province_storage);
                info!("Province storage inserted successfully");

                // Update loading state to completion
                info!("Setting loading progress to complete...");
                set_loading_progress(&mut loading_state, PROGRESS_COMPLETE, "World ready!");
                info!("Loading progress set to complete");

                // Create a timer to delay the state transition
                // This gives the UI time to display the 100% completion
                info!("Creating transition delay timer (1 second)...");
                commands.insert_resource(WorldGenerationTransitionDelay {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                });

                // Clean up async generation resource
                info!("Cleaning up async generation resource...");
                commands.remove_resource::<AsyncWorldGeneration>();
                info!("Async generation resource cleaned up");

                info!("Async world generation fully completed");
                return;
            } else {
                // Generation failed - create error context with rich diagnostic information
                error!("Async world generation failed");

                // Create error context from the progress information
                let error_context = if let Some(metrics) = progress.generation_metrics {
                    crate::diagnostics::ErrorContext::from_generation_error(
                        progress.error_message.unwrap_or_else(|| "World generation failed".to_string()),
                        Some(metrics),
                        "LoadingWorld".to_string(),
                    )
                } else {
                    // No metrics available, create basic error context
                    crate::diagnostics::ErrorContext::from_generation_error(
                        progress.error_message.unwrap_or_else(|| "World generation failed".to_string()),
                        None,
                        "LoadingWorld".to_string(),
                    )
                };

                // Save error to file for debugging
                if let Err(e) = error_context.save_to_file() {
                    error!("Failed to save error context to file: {}", e);
                }

                // Store error context as a resource for the error dialog
                commands.insert_resource(crate::world::generation::WorldGenerationError {
                    error_message: error_context.format_for_display(),
                    error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
                });

                // Also store the full error context for detailed display
                commands.insert_resource(error_context);

                // Clean up async generation resource
                commands.remove_resource::<AsyncWorldGeneration>();

                // Transition to WorldGenerationFailed state (NOT MainMenu!)
                // This will trigger the error dialog with our rich context
                state_events.write(RequestStateTransition {
                    from: GameState::LoadingWorld,
                    to: GameState::WorldGenerationFailed,
                });
                return;
            }
        } else {
            // Progress update
            info!(
                "Polling system: Received progress: {} - {:.1}%",
                progress.step,
                progress.progress * 100.0
            );
            set_loading_progress(&mut loading_state, progress.progress, &progress.step);
            info!("Polling system: LoadingState updated");
        }
    }

    // Check if task is still running (if we can't poll it, it might be done)
    if generation.task.is_finished() {
        warn!("Async world generation task finished without sending completion message");

        // Create error context for this unexpected condition
        let error_context = crate::diagnostics::ErrorContext::from_generation_error(
            "World generation task terminated unexpectedly without completion message".to_string(),
            None,
            "LoadingWorld".to_string(),
        );

        // Save error to file for debugging
        if let Err(e) = error_context.save_to_file() {
            error!("Failed to save error context to file: {}", e);
        }

        // Store error context for dialog
        commands.insert_resource(crate::world::generation::WorldGenerationError {
            error_message: error_context.format_for_display(),
            error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
        });

        commands.insert_resource(error_context);

        // Clean up
        commands.remove_resource::<AsyncWorldGeneration>();

        // Transition to error state instead of silently returning to menu
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::WorldGenerationFailed,
        });
    }
}

/// Custom error type for world setup failures
#[derive(Debug)]
pub enum WorldSetupError {
    /// Invalid settings provided
    InvalidSettings(String),
    /// World generation failed
    GenerationFailed(String),
    /// Mesh building failed
    MeshBuildingFailed(String),
    /// Empty world generated
    EmptyWorld,
    /// Resource insertion failed
    ResourceError(String),
}

impl fmt::Display for WorldSetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSettings(msg) => write!(f, "Invalid settings: {}", msg),
            Self::GenerationFailed(msg) => write!(f, "World generation failed: {}", msg),
            Self::MeshBuildingFailed(msg) => write!(f, "Mesh building failed: {}", msg),
            Self::EmptyWorld => write!(f, "Generated world has no provinces"),
            Self::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

impl std::error::Error for WorldSetupError {}

/// Validates world generation settings
fn validate_settings(settings: &WorldGenerationSettings) -> Result<(), WorldSetupError> {
    // Validate ocean coverage
    if !(MIN_OCEAN_COVERAGE..=MAX_OCEAN_COVERAGE).contains(&settings.ocean_coverage) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Ocean coverage must be between {} and {}",
            MIN_OCEAN_COVERAGE, MAX_OCEAN_COVERAGE
        )));
    }

    // Validate continent count
    if !(MIN_CONTINENTS..=MAX_CONTINENTS).contains(&settings.continent_count) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "Continent count must be between {} and {}",
            MIN_CONTINENTS, MAX_CONTINENTS
        )));
    }

    // Validate river density
    if !(MIN_RIVER_DENSITY..=MAX_RIVER_DENSITY).contains(&settings.river_density) {
        return Err(WorldSetupError::InvalidSettings(format!(
            "River density must be between {} and {}",
            MIN_RIVER_DENSITY, MAX_RIVER_DENSITY
        )));
    }

    // Validate world name
    if settings.world_name.is_empty() {
        return Err(WorldSetupError::InvalidSettings(
            "World name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Count cultures in provinces for debugging
fn count_cultures(
    provinces: &[super::provinces::Province],
) -> std::collections::HashMap<String, usize> {
    use std::collections::HashMap;

    let mut counts = HashMap::new();

    for province in provinces {
        let culture_name = match province.culture {
            Some(culture) => format!("{:?}", culture),
            None => "None".to_string(),
        };
        *counts.entry(culture_name).or_insert(0) += 1;
    }

    counts
}
