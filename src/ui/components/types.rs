//! Shared types used across UI components

use bevy::prelude::*;

/// Orientation for UI elements like separators and layouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Marker trait for custom component markers
/// This allows users to add their own marker components to builders
pub trait ComponentMarker: Component {}

// Implement ComponentMarker for any Component automatically
impl<T: Component> ComponentMarker for T {}
