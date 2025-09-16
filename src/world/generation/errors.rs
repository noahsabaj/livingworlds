//! World generation error types
//!
//! This module contains error types specific to world generation failures.

use bevy::prelude::*;
use std::fmt;

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

impl fmt::Display for WorldGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::error::Error for WorldGenerationError {}
