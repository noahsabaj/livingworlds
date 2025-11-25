//! Generation progress tracking
//!
//! Provides progress reporting structures and constants for world generation.

use super::super::World;

/// Loading progress milestones - more granular for better user feedback
pub const PROGRESS_START: f32 = 0.0;
pub const PROGRESS_PROVINCES: f32 = 0.1;
pub const PROGRESS_EROSION: f32 = 0.25;
pub const PROGRESS_CLIMATE: f32 = 0.4;
pub const PROGRESS_RIVERS: f32 = 0.5;
pub const PROGRESS_MESH: f32 = 0.7;
pub const PROGRESS_ENTITIES: f32 = 0.85;
pub const PROGRESS_OVERLAYS: f32 = 0.95;
pub const PROGRESS_COMPLETE: f32 = 1.0;

/// Frame budget - maximum time to spend on world generation per frame (in milliseconds)
/// This allows UI interactions to remain responsive during generation
pub const FRAME_BUDGET_MS: f32 = 16.0; // ~60fps budget, leaves time for UI

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
