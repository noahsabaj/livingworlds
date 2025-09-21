//! World generation orchestrator
//!
//! This module orchestrates world generation using builders from feature modules.
//! It contains only the WorldBuilder that coordinates feature builders.
//!
//! This is a PURE ORCHESTRATOR - feature-specific generation lives in feature modules.

// PRIVATE MODULES - Only orchestration logic

mod builder; // Main world builder orchestrator
mod errors; // Error types for generation failures
mod plugin;
mod utils; // Shared utilities // Generation plugin

// PUBLIC INTERFACE - The only way to generate worlds

// Re-export the WorldBuilder from builder.rs
pub use builder::WorldBuilder;

// Re-export error types for generation failures
pub use errors::{WorldGenerationError, WorldGenerationErrorType};

// Re-export shared generation utilities
pub use utils::GenerationUtils;

// Re-export the plugin from plugin.rs
pub use plugin::GenerationPlugin;
