//! World generation orchestrator
//!
//! This module orchestrates world generation using builders from feature modules.
//! It contains only the WorldBuilder that coordinates feature builders.
//!
//! This is a PURE ORCHESTRATOR - feature-specific generation lives in feature modules.

use bevy::prelude::*;

// PRIVATE MODULES - Only orchestration logic

mod builder;     // Main world builder orchestrator
mod errors;      // Error types for generation failures
mod utils;       // Shared utilities

// PUBLIC INTERFACE - The only way to generate worlds

// Re-export the WorldBuilder from builder.rs
pub use builder::WorldBuilder;

// Re-export error types for generation failures
pub use errors::{WorldGenerationError, WorldGenerationErrorType};


/// Plugin that registers world generation systems
pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        // Generation is typically a one-time process at startup
        // No systems to register, but we could add generation commands here
        app.add_systems(Startup, log_generation_ready);
    }
}

fn log_generation_ready() {
    info!("World generation module ready");
}


