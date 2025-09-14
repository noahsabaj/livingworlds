//! Borders feature module gateway

use bevy::prelude::Component;

// PRIVATE MODULES
mod rendering;

/// Marker component for border entities
#[derive(Component, Default)]
pub struct BorderEntity;

// PUBLIC EXPORTS
pub use rendering::{
    SelectionBorder,
    BorderPlugin,
};