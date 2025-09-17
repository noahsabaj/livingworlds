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

use super::{build_world_mesh, ProvinceStorage, WorldBuilder, WorldMeshHandle};
use super::World;
use super::{ProvincesSpatialIndex, WorldGenerationSettings};
use crate::loading_screen::{set_loading_progress, LoadingState};
use crate::states::{GameState, RequestStateTransition};
use bevy::log::{debug, error, info};
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::fmt;
use async_channel::{Receiver, Sender};

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

const MIN_CONTINENTS: u32 = 1;
const MAX_OCEAN_COVERAGE: f32 = 0.95;
const MIN_OCEAN_COVERAGE: f32 = 0.05;
const MAX_RIVER_DENSITY: f32 = 1.0;
const MIN_RIVER_DENSITY: f32 = 0.0;

/// Background world generation function - runs on AsyncComputeTaskPool
async fn generate_world_async(
    settings: WorldGenerationSettings,
    progress_sender: Sender<GenerationProgress>,
) {
    info!("Starting async world generation with settings: {:?}", settings);

    // Helper to send progress updates
    let send_progress = |step: &str, progress: f32| {
        let _ = progress_sender.try_send(GenerationProgress {
            step: step.to_string(),
            progress,
            completed: false,
            world_data: None,
        });
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
        });
        return;
    }

    // Generate world data with progress reporting
    send_progress("Generating terrain...", 0.1);

    let start_time = std::time::Instant::now();
    let world_result = WorldBuilder::new(
        settings.seed,
        settings.world_size,
        settings.continent_count,
        settings.ocean_coverage,
        settings.river_density,
    ).build();

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
            });
        }
        Err(e) => {
            error!("World generation failed: {}", e);
            let _ = progress_sender.try_send(GenerationProgress {
                step: format!("Error: {}", e),
                progress: 0.0,
                completed: true,
                world_data: None,
            });
            return;
        }
    }

    info!("Async world generation finished");
}

/// Start async world generation - replaces the old blocking setup_world
pub fn start_async_world_generation(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    _state_events: EventWriter<RequestStateTransition>,
    mut loading_state: ResMut<LoadingState>,
) {
    info!("Starting async world generation");

    // Create progress channel
    let (progress_sender, progress_receiver) = async_channel::unbounded::<GenerationProgress>();

    // Spawn background generation task
    let task_pool = AsyncComputeTaskPool::get();
    let generation_settings = settings.clone();

    let task = task_pool.spawn(async move {
        generate_world_async(generation_settings, progress_sender).await;
    });

    // Store task handle and progress receiver
    commands.insert_resource(AsyncWorldGeneration {
        task,
        progress_receiver,
        settings: settings.clone(),
    });

    // Initialize loading state
    set_loading_progress(&mut loading_state, PROGRESS_START, "Starting world generation...");

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
    let Some(mut generation) = async_generation else {
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

                // Create map dimensions resource (required by camera system)
                let map_dimensions = crate::resources::MapDimensions::from_world_size(&generation.settings.world_size);
                commands.insert_resource(map_dimensions.clone());

                // Create spatial index
                let spatial_index = ProvincesSpatialIndex::build(&province_storage.provinces, &map_dimensions);
                commands.insert_resource(spatial_index);

                // Spawn nations with ruling houses
                let nation_settings = crate::nations::NationGenerationSettings::default();
                let (nations, houses) = crate::nations::spawn_nations(
                    &mut province_storage.provinces,
                    &nation_settings,
                    world.seed,
                );

                // Build ownership cache from province data
                let mut ownership_cache = crate::nations::ProvinceOwnershipCache::default();
                ownership_cache.rebuild(&province_storage.provinces);
                commands.insert_resource(ownership_cache);

                // Spawn nation entities
                for nation in nations {
                    commands.spawn(crate::nations::NationBundle {
                        nation,
                        transform: Transform::default(),
                        visibility: Visibility::default(),
                    });
                }

                // Spawn house entities
                for house in houses {
                    commands.spawn(house);
                }

                // Insert the updated province storage (with nation ownership)
                commands.insert_resource(province_storage);

                // Update loading state to completion
                set_loading_progress(&mut loading_state, PROGRESS_COMPLETE, "World ready!");

                // Transition to game
                state_events.write(RequestStateTransition {
                    from: GameState::LoadingWorld,
                    to: GameState::InGame,
                });

                // Clean up async generation resource
                commands.remove_resource::<AsyncWorldGeneration>();

                info!("Async world generation fully completed");
                return;
            } else {
                // Generation failed
                error!("Async world generation failed");
                set_loading_progress(&mut loading_state, 0.0, "Generation failed");

                // Clean up
                commands.remove_resource::<AsyncWorldGeneration>();

                // Transition back to main menu
                state_events.write(RequestStateTransition {
                    from: GameState::LoadingWorld,
                    to: GameState::MainMenu,
                });
                return;
            }
        } else {
            // Progress update
            set_loading_progress(&mut loading_state, progress.progress, &progress.step);
            debug!("World generation progress: {:.1}% - {}", progress.progress * 100.0, progress.step);
        }
    }

    // Check if task is still running (if we can't poll it, it might be done)
    if generation.task.is_finished() {
        warn!("Async world generation task finished without sending completion message");

        // Clean up
        commands.remove_resource::<AsyncWorldGeneration>();

        // Transition back to main menu
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::MainMenu,
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


