//! Event type definitions for loading system

use bevy::prelude::*;

/// Event to cancel world generation
///
/// This event is triggered when the user clicks the cancel button
/// during world generation. It causes the loading system to clean up
/// generation resources and return to the world configuration screen.
#[derive(Event)]
pub struct CancelWorldGeneration;
