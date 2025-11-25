//! Type definitions for async world generation
//!
//! Contains resources and types used by the async generation system.

use bevy::prelude::*;
use bevy::tasks::Task;
use async_channel::Receiver;

use super::progress::GenerationProgress;
use super::super::WorldGenerationSettings;

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

/// Resource indicating province neighbor entities need to be set up
/// Used for deferred neighbor setup after spawn_batch commands are applied
#[derive(Resource)]
pub struct PendingNeighborSetup {
    /// The province data containing neighbor_indices
    pub provinces: Vec<crate::world::Province>,
}

/// Resource holding province bundles pending spawn via exclusive system
/// This allows us to use World::spawn_batch which returns entity IDs
#[derive(Resource)]
pub struct PendingProvinceSpawn {
    /// Province bundles to spawn
    pub bundles: Vec<crate::world::ProvinceBundle>,
    /// Original province data for neighbor setup
    pub provinces: Vec<crate::world::Province>,
    /// Province ownership mapping: nation_entity -> list of province indices
    pub ownership: std::collections::HashMap<bevy::prelude::Entity, Vec<usize>>,
}
