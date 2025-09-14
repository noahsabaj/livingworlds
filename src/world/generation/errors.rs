//! World generation error types
//!
//! This module contains error types specific to world generation failures.

use bevy::prelude::*;

/// Stores error information when world generation fails
#[derive(Resource, Clone, Debug)]
pub struct WorldGenerationError {
    pub error_message: String,
    pub error_type: WorldGenerationErrorType,
}

/// Types of world generation errors
#[derive(Clone, Debug, PartialEq)]
pub enum WorldGenerationErrorType {
    InvalidSettings,
    GenerationFailed,
    MeshBuildingFailed,
    EmptyWorld,
    ResourceError,
}