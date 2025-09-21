//! Loading state resource management

use super::operations::{LoadingDetails, LoadingOperation};
use bevy::prelude::*;

/// Tracks what's being loaded and the current progress
///
/// This resource is the central state manager for all loading operations.
/// It contains the current operation type, progress percentage, status text,
/// and operation-specific details.
#[derive(Resource, Default)]
pub struct LoadingState {
    /// The type of loading operation currently in progress
    pub operation: LoadingOperation,

    /// Progress from 0.0 to 1.0 (0% to 100%)
    pub progress: f32,

    /// Current step description shown to the user
    pub current_step: String,

    /// Operation-specific details (seed, save name, etc.)
    pub details: LoadingDetails,
}
