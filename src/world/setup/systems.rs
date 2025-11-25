//! Bevy systems for async world generation
//!
//! Contains systems that integrate async world generation with Bevy's ECS.

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;

use super::async_gen::generate_world_async;
use super::progress::{GenerationProgress, PROGRESS_START, PROGRESS_COMPLETE};
use super::types::{AsyncWorldGeneration, WorldGenerationTransitionDelay, PendingNeighborSetup, PendingProvinceSpawn};
use crate::world::provinces::ProvinceNeighbors;
use super::validation::count_cultures;
use super::super::{
    build_world_mesh, ProvinceStorage, ProvincesSpatialIndex, WorldGenerationSettings, WorldMeshHandle,
    provinces_to_bundles, set_neighbor_entities, ProvinceEntityOrder,
};
use crate::relationships::ControlledBy;
use crate::loading::{set_loading_progress, LoadingState};
use crate::states::{GameState, RequestStateTransition};

/// Exclusive system to spawn province entities using World::spawn_batch
///
/// This uses exclusive World access to efficiently spawn 3M entities and get entity IDs.
/// Also handles neighbor setup and ownership assignment (ControlledBy relationships).
pub fn spawn_province_entities(world: &mut World) {
    // Check if there are pending province spawns
    let Some(pending) = world.remove_resource::<PendingProvinceSpawn>() else {
        return;
    };

    info!("Spawning {} province entities with spawn_batch...", pending.bundles.len());
    let spawn_start = std::time::Instant::now();

    // Use World::spawn_batch which returns entity IDs
    let province_entity_ids: Vec<Entity> = world.spawn_batch(pending.bundles).collect();

    info!(
        "Spawned {} province entities in {:.1}ms",
        province_entity_ids.len(),
        spawn_start.elapsed().as_secs_f32() * 1000.0
    );

    // Store entity order for mesh vertex alignment and lookups
    let entity_order = ProvinceEntityOrder::new(province_entity_ids.clone());
    world.insert_resource(entity_order);
    info!("Province entity order stored");

    // Set up neighbor entities while we have the entity IDs
    info!("Setting up neighbor entities for {} provinces...", province_entity_ids.len());
    let neighbor_start = std::time::Instant::now();

    for (idx, province) in pending.provinces.iter().enumerate() {
        if idx >= province_entity_ids.len() {
            continue;
        }

        let entity = province_entity_ids[idx];

        // Convert neighbor indices to entities
        let neighbor_entities: [Option<Entity>; 6] = province
            .neighbor_indices
            .map(|opt_idx| opt_idx.and_then(|i| province_entity_ids.get(i).copied()));

        // Update the ProvinceNeighbors component
        if let Some(mut neighbors) = world.get_mut::<ProvinceNeighbors>(entity) {
            neighbors.neighbors = neighbor_entities;
        }
    }

    info!(
        "Neighbor entities set up in {:.1}ms",
        neighbor_start.elapsed().as_secs_f32() * 1000.0
    );

    // Set up province ownership using ControlledBy relationships
    info!("Setting up province ownership for {} nations...", pending.ownership.len());
    let ownership_start = std::time::Instant::now();
    let mut owned_count = 0;

    for (nation_entity, province_indices) in &pending.ownership {
        for &province_idx in province_indices {
            if province_idx < province_entity_ids.len() {
                let province_entity = province_entity_ids[province_idx];
                // Insert ControlledBy relationship - Bevy auto-updates Controls on nation!
                world.entity_mut(province_entity).insert(ControlledBy(*nation_entity));
                owned_count += 1;
            }
        }
    }

    info!(
        "Province ownership set: {}/{} provinces now have owners via ControlledBy in {:.1}ms",
        owned_count,
        province_entity_ids.len(),
        ownership_start.elapsed().as_secs_f32() * 1000.0
    );
}

/// Exclusive system to set up province neighbor entities after spawn_batch is applied
///
/// This runs after the command buffer is flushed, when province entities actually exist.
/// (Legacy system - kept for potential future use)
pub fn setup_province_neighbors(world: &mut World) {
    // Check if there's pending neighbor setup
    let Some(pending) = world.remove_resource::<PendingNeighborSetup>() else {
        return;
    };

    // Get the entity order
    let Some(entity_order) = world.get_resource::<ProvinceEntityOrder>() else {
        warn!("ProvinceEntityOrder not found, cannot set up neighbor entities");
        return;
    };

    let entity_ids = entity_order.entities.clone();

    info!("Setting up neighbor entities for {} provinces...", entity_ids.len());
    let start = std::time::Instant::now();

    // Update each province's neighbor entities
    for (idx, province) in pending.provinces.iter().enumerate() {
        if idx >= entity_ids.len() {
            continue;
        }

        let entity = entity_ids[idx];

        // Convert neighbor indices to entities
        let neighbor_entities: [Option<Entity>; 6] = province
            .neighbor_indices
            .map(|opt_idx| opt_idx.and_then(|i| entity_ids.get(i).copied()));

        // Update the ProvinceNeighbors component
        if let Some(mut neighbors) = world.get_mut::<ProvinceNeighbors>(entity) {
            neighbors.neighbors = neighbor_entities;
        }
    }

    info!(
        "Neighbor entities set up in {:.1}ms",
        start.elapsed().as_secs_f32() * 1000.0
    );
}

/// System to handle the transition delay timer after world generation completes
pub fn handle_world_generation_transition_delay(
    mut commands: Commands,
    time: Res<Time>,
    timer_res: Option<ResMut<WorldGenerationTransitionDelay>>,
    mut state_events: MessageWriter<RequestStateTransition>,
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

/// Start async world generation - replaces the old blocking setup_world
pub fn start_async_world_generation(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<WorldGenerationSettings>,
    _state_events: MessageWriter<RequestStateTransition>,
    mut loading_state: ResMut<LoadingState>,
    // GPU resources for acceleration
    gpu_status: Option<Res<crate::world::gpu::GpuComputeStatus>>,
    gpu_config: Option<Res<crate::world::gpu::GpuGenerationConfig>>,
    _gpu_state: Option<ResMut<crate::world::gpu::GpuGenerationState>>,
    _gpu_metrics: Option<ResMut<crate::world::gpu::GpuPerformanceMetrics>>,
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

/// Build and store the world mesh from generated provinces
fn build_and_store_mesh(
    world_seed: u32,
    provinces: &[crate::world::Province],
    meshes: &mut ResMut<Assets<Mesh>>,
    commands: &mut Commands,
) {
    info!("Building world mesh from {} provinces...", provinces.len());
    let mesh_handle = build_world_mesh(provinces, meshes, world_seed);
    commands.insert_resource(WorldMeshHandle(mesh_handle));
}

/// Assign cultures to provinces and log distribution
fn assign_province_cultures(
    province_storage: &mut ProvinceStorage,
    world_seed: u32,
) {
    info!("Assigning cultures to {} provinces...", province_storage.provinces.len());
    crate::world::assign_cultures_to_province_storage(
        &mut province_storage.provinces,
        Some(world_seed as u64),
    );

    let culture_counts = count_cultures(&province_storage.provinces);
    info!("Cultural distribution: {:?}", culture_counts);
}

/// Set up map-level resources (dimensions, seed, time, spatial index)
fn setup_map_resources(
    generation_settings: &WorldGenerationSettings,
    world_seed: u32,
    province_storage: &ProvinceStorage,
    commands: &mut Commands,
) -> crate::simulation::GameTime {
    let map_dimensions = crate::resources::MapDimensions::from_world_size(
        &generation_settings.world_size,
    );
    commands.insert_resource(map_dimensions.clone());
    commands.insert_resource(crate::world::WorldSeed(world_seed));

    let game_time = crate::simulation::GameTime::new(generation_settings.starting_year);
    info!("Initializing game time with starting year: {}", generation_settings.starting_year);
    commands.insert_resource(game_time.clone());

    let spatial_index = ProvincesSpatialIndex::build(&province_storage.provinces, &map_dimensions);
    commands.insert_resource(spatial_index);

    game_time
}

/// Spawn nation entities and return the entity mapping
fn spawn_nation_entities(
    nations: &[(crate::nations::NationId, crate::nations::Nation)],
    governments: &[crate::nations::GovernmentType],
    game_time: &crate::simulation::GameTime,
    commands: &mut Commands,
) -> std::collections::HashMap<crate::nations::NationId, Entity> {
    info!("Spawning {} nation entities...", nations.len());
    let mut nation_entities = std::collections::HashMap::new();

    for (i, (nation_id, nation)) in nations.iter().enumerate() {
        let government_type = governments[i];
        let nation_entity = commands
            .spawn((
                crate::nations::NationBundle {
                    nation: nation.clone(),
                    economy: crate::nations::Economy::default(),
                    transform: Transform::default(),
                    visibility: Visibility::default(),
                    pressure_vector: crate::simulation::PressureVector::default(),
                    history: crate::nations::create_initial_history(
                        &nation.name,
                        crate::nations::culture_to_display_name(nation.culture).to_string(),
                        game_time.current_year(),
                        &mut rand::thread_rng(),
                    ),
                    laws: crate::nations::NationLaws::default(),
                },
                crate::nations::OwnsTerritory::default(),
                crate::nations::Governance {
                    government_type,
                    stability: 0.75,
                    reform_pressure: 0.0,
                    tradition_strength: government_type.mechanics().reform_resistance,
                    institution_strength: 1.0,
                    last_transition: None,
                    days_in_power: 0,
                    legitimacy: 0.75,
                    legitimacy_trend: 0.0,
                    legitimacy_factors: crate::nations::LegitimacyFactors::for_government_type(government_type),
                },
                crate::nations::PoliticalPressure::default(),
                crate::nations::GovernmentHistory::new(government_type),
                *nation_id,
            ))
            .id();

        nation_entities.insert(*nation_id, nation_entity);
    }
    info!("Nation entities spawned successfully");

    nation_entities
}

/// Build ownership map and update province storage with nation entities
fn build_ownership_map(
    province_ownership: &std::collections::HashMap<crate::nations::NationId, Vec<u32>>,
    nation_entities: &std::collections::HashMap<crate::nations::NationId, Entity>,
    province_storage: &mut ProvinceStorage,
) -> std::collections::HashMap<Entity, Vec<usize>> {
    info!("Building ownership map for {} nations...", province_ownership.len());
    let mut ownership_map: std::collections::HashMap<Entity, Vec<usize>> = std::collections::HashMap::new();

    for (nation_id, province_ids) in province_ownership {
        if let Some(&nation_entity) = nation_entities.get(nation_id) {
            let indices: Vec<usize> = province_ids.iter().map(|&id| id as usize).collect();

            // Update old storage for backward compatibility
            for &province_idx in &indices {
                if province_idx < province_storage.provinces.len() {
                    province_storage.provinces[province_idx].owner_entity = Some(nation_entity);
                }
            }
            ownership_map.insert(nation_entity, indices);
        }
    }
    info!("Ownership map built with {} nation entries", ownership_map.len());

    ownership_map
}

/// Queue province spawn with ownership data
fn queue_province_spawn(
    province_bundles: Vec<crate::world::ProvinceBundle>,
    provinces: Vec<crate::world::Province>,
    ownership_map: std::collections::HashMap<Entity, Vec<usize>>,
    commands: &mut Commands,
) {
    info!("Queuing province spawn with ownership for {} provinces...", province_bundles.len());
    commands.insert_resource(PendingProvinceSpawn {
        bundles: province_bundles,
        provinces,
        ownership: ownership_map,
    });
    info!("Province spawn queued (will execute in spawn_province_entities system)");
}

/// Spawn territory entities for all nations
fn spawn_territory_entities(
    territories_by_nation: std::collections::HashMap<Entity, Vec<crate::nations::Territory>>,
    num_nations: usize,
    num_provinces: usize,
    commands: &mut Commands,
) {
    info!("Spawning territory entities...");
    let mut total_territories = 0;

    for (nation_entity, territories) in territories_by_nation {
        for territory in territories {
            commands.spawn((
                territory,
                crate::nations::OwnedBy(nation_entity),
                Transform::default(),
                Visibility::default(),
            ));
            total_territories += 1;
        }
    }

    info!(
        "Spawned {} territory entities for {} nations (instead of {} province entities)",
        total_territories,
        num_nations,
        num_provinces
    );
}

/// Spawn house entities
fn spawn_house_entities(
    houses: Vec<crate::nations::House>,
    commands: &mut Commands,
) {
    info!("Spawning {} house entities...", houses.len());
    for house in houses {
        commands.spawn(house);
    }
    info!("House entities spawned successfully");
}

/// Initialize and store coastal province cache
fn initialize_coastal_cache(
    province_storage: &ProvinceStorage,
    commands: &mut Commands,
) {
    let mut coastal_cache = crate::world::CoastalProvinceCache::default();
    coastal_cache.build(province_storage);
    commands.insert_resource(coastal_cache);
    info!("Coastal province cache initialized");
}

/// Finalize world generation completion
fn finalize_world_generation(
    commands: &mut Commands,
    loading_state: &mut ResMut<LoadingState>,
) {
    info!("Setting loading progress to complete...");
    set_loading_progress(loading_state, PROGRESS_COMPLETE, "World ready!");
    info!("Loading progress set to complete");

    info!("Creating transition delay timer (1 second)...");
    commands.insert_resource(WorldGenerationTransitionDelay {
        timer: Timer::from_seconds(1.0, TimerMode::Once),
    });

    info!("Cleaning up async generation resource...");
    commands.remove_resource::<AsyncWorldGeneration>();
    info!("Async generation resource cleaned up");

    info!("Async world generation fully completed");
}

/// Handle world generation error
fn handle_generation_error(
    progress: &GenerationProgress,
    commands: &mut Commands,
    state_events: &mut MessageWriter<RequestStateTransition>,
) {
    error!("Async world generation failed");

    let error_context = if let Some(metrics) = &progress.generation_metrics {
        crate::diagnostics::ErrorContext::from_generation_error(
            progress.error_message.clone().unwrap_or_else(|| "World generation failed".to_string()),
            Some(metrics.clone()),
            "LoadingWorld".to_string(),
        )
    } else {
        crate::diagnostics::ErrorContext::from_generation_error(
            progress.error_message.clone().unwrap_or_else(|| "World generation failed".to_string()),
            None,
            "LoadingWorld".to_string(),
        )
    };

    if let Err(e) = error_context.save_to_file() {
        error!("Failed to save error context to file: {}", e);
    }

    commands.insert_resource(crate::world::generation::WorldGenerationError {
        error_message: error_context.format_for_display(),
        error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
    });

    commands.insert_resource(error_context);
    commands.remove_resource::<AsyncWorldGeneration>();

    state_events.write(RequestStateTransition {
        from: GameState::LoadingWorld,
        to: GameState::WorldGenerationFailed,
    });
}

/// Handle unexpected task termination
fn handle_task_termination(
    commands: &mut Commands,
    state_events: &mut MessageWriter<RequestStateTransition>,
) {
    warn!("Async world generation task finished without sending completion message");

    let error_context = crate::diagnostics::ErrorContext::from_generation_error(
        "World generation task terminated unexpectedly without completion message".to_string(),
        None,
        "LoadingWorld".to_string(),
    );

    if let Err(e) = error_context.save_to_file() {
        error!("Failed to save error context to file: {}", e);
    }

    commands.insert_resource(crate::world::generation::WorldGenerationError {
        error_message: error_context.format_for_display(),
        error_type: crate::world::generation::WorldGenerationErrorType::GenerationFailed,
    });

    commands.insert_resource(error_context);
    commands.remove_resource::<AsyncWorldGeneration>();

    state_events.write(RequestStateTransition {
        from: GameState::LoadingWorld,
        to: GameState::WorldGenerationFailed,
    });
}

/// Poll async world generation progress and handle completion
pub fn poll_async_world_generation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut loading_state: ResMut<LoadingState>,
    mut state_events: MessageWriter<RequestStateTransition>,
    async_generation: Option<ResMut<AsyncWorldGeneration>>,
) {
    let Some(generation) = async_generation else {
        return;
    };

    // Check for progress updates (non-blocking)
    while let Ok(progress) = generation.progress_receiver.try_recv() {
        if progress.completed {
            if let Some(mut world) = progress.world_data {
                info!("Async world generation completed, processing...");

                // Phase 1: Build and store mesh
                build_and_store_mesh(world.seed, &world.provinces, &mut meshes, &mut commands);

                // Phase 2: Store climate data
                commands.insert_resource(world.climate_storage.clone());

                // Phase 3: Create province storage and assign cultures
                let mut province_storage = ProvinceStorage::from_provinces(world.provinces.clone());
                assign_province_cultures(&mut province_storage, world.seed);

                // Phase 4: Prepare province bundles
                info!("Preparing {} province bundles...", province_storage.provinces.len());
                let province_bundles = provinces_to_bundles(&province_storage.provinces);

                // Phase 5: Set up map resources (dimensions, seed, time, spatial index)
                let game_time = setup_map_resources(
                    &generation.settings,
                    world.seed,
                    &province_storage,
                    &mut commands,
                );

                // Phase 6: Spawn nations with ruling houses and governance
                let nation_settings = crate::nations::NationGenerationSettings::default();
                info!("About to call spawn_nations...");
                let (nations, houses, governments, province_ownership) = crate::nations::spawn_nations(
                    &nation_settings,
                    &mut province_storage.provinces,
                    world.seed,
                );
                info!("spawn_nations completed! Got {} nations, {} houses, {} governments, and {} ownership entries",
                      nations.len(), houses.len(), governments.len(), province_ownership.len());

                // Phase 7: Build territories from provinces
                info!("Building territories from {} provinces...", province_storage.provinces.len());
                let territories_by_nation = crate::nations::build_territories_from_provinces(
                    &province_storage.provinces
                );
                info!("Built {} territory groups for {} nations",
                    territories_by_nation.values().map(|v| v.len()).sum::<usize>(),
                    territories_by_nation.len()
                );

                // Phase 8: Analyze infrastructure
                info!("Analyzing infrastructure networks...");
                let infrastructure_storage = crate::world::analyze_infrastructure(
                    &province_storage.provinces,
                    &province_storage,
                );
                info!("Infrastructure analysis complete: {} provinces with infrastructure data",
                      infrastructure_storage.infrastructure.len());
                world.infrastructure_storage = infrastructure_storage.clone();
                commands.insert_resource(infrastructure_storage);

                // Phase 9: Spawn nation entities
                let nation_entities = spawn_nation_entities(
                    &nations,
                    &governments,
                    &game_time,
                    &mut commands,
                );

                // Phase 10: Build ownership map
                let ownership_map = build_ownership_map(
                    &province_ownership,
                    &nation_entities,
                    &mut province_storage,
                );

                // Phase 11: Queue province spawn
                queue_province_spawn(
                    province_bundles,
                    province_storage.provinces.clone(),
                    ownership_map,
                    &mut commands,
                );

                // Phase 12: Spawn territory entities
                spawn_territory_entities(
                    territories_by_nation,
                    nation_entities.len(),
                    province_storage.provinces.len(),
                    &mut commands,
                );

                // Phase 13: Spawn house entities
                spawn_house_entities(houses, &mut commands);

                // Phase 14: Initialize coastal cache
                initialize_coastal_cache(&province_storage, &mut commands);

                // Phase 15: Insert province storage
                info!("Inserting province storage with {} provinces...", province_storage.provinces.len());
                commands.insert_resource(province_storage);
                info!("Province storage inserted successfully");

                // Phase 16: Finalize completion
                finalize_world_generation(&mut commands, &mut loading_state);

                return;
            } else {
                handle_generation_error(&progress, &mut commands, &mut state_events);
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

    // Check if task is still running
    if generation.task.is_finished() {
        handle_task_termination(&mut commands, &mut state_events);
    }
}
