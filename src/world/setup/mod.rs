//! World Setup - Async World Generation Systems
//!
//! This module provides async world generation systems that integrate
//! WorldBuilder with Bevy's task pool system for non-blocking generation.
//!
//! # Architecture
//!
//! - **WorldBuilder**: Pure generation logic (world/generation/builder.rs)
//!   - Takes parameters -> returns World data structure
//!   - No Bevy dependencies, could work in any context
//!   - Handles: terrain, rivers, climate, erosion, agriculture
//!
//! - **Async Generation** (this module):
//!   - Uses WorldBuilder on background threads via AsyncComputeTaskPool
//!   - Handles: progress tracking, error handling, state transitions
//!   - Creates: rendering mesh, ECS resources when generation completes
//!   - Manages: loading screens, async progress updates
//!
//! # Module Structure
//!
//! - `progress` - Progress tracking constants and GenerationProgress struct
//! - `validation` - Settings validation and WorldSetupError
//! - `types` - AsyncWorldGeneration and related resource types
//! - `async_gen` - Background async generation function
//! - `gpu` - GPU-accelerated world generation
//! - `systems` - Bevy ECS systems for generation lifecycle
//!
//! This separation allows the core generation to be reused in tests,
//! tools, or other contexts while keeping Bevy-specific concerns isolated.

pub mod async_gen;
pub mod gpu;
pub mod progress;
pub mod systems;
pub mod types;
pub mod validation;

// Re-export commonly used items
pub use progress::{
    GenerationProgress,
    PROGRESS_START, PROGRESS_PROVINCES, PROGRESS_EROSION, PROGRESS_CLIMATE,
    PROGRESS_RIVERS, PROGRESS_MESH, PROGRESS_ENTITIES, PROGRESS_OVERLAYS, PROGRESS_COMPLETE,
    FRAME_BUDGET_MS,
};
pub use types::{AsyncWorldGeneration, WorldGenerationTransitionDelay, PendingNeighborSetup, PendingProvinceSpawn};
pub use validation::{WorldSetupError, validate_settings, count_cultures};
pub use systems::{
    handle_world_generation_transition_delay,
    start_async_world_generation,
    poll_async_world_generation,
    setup_province_neighbors,
    spawn_province_entities,
};
pub use gpu::generate_world_with_gpu_acceleration;
pub use async_gen::generate_world_async;
