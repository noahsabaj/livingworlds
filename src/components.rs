//! Core game components
//! 
//! This module contains all the ECS components used throughout the game.
//! Components are data attached to entities. For global singleton data,
//! see the resources module.

use bevy::prelude::*;
use crate::terrain::TerrainType;

// ============================================================================
// COMPONENTS - Data attached to entities
// ============================================================================

/// Province represents a single hexagonal tile in the world
#[derive(Component, Clone)]
pub struct Province {
    pub id: u32,
    pub position: Vec2,
    pub nation_id: Option<u32>,  // None for ocean provinces
    pub population: f32,
    pub terrain: TerrainType,
    pub elevation: f32,
}

/// Marker component for the currently selected province
#[derive(Component)]
pub struct SelectedProvince;

/// Marker for ghost provinces (duplicates for world wrapping)
/// These are visual duplicates shown at map edges for seamless scrolling
#[derive(Component)]
pub struct GhostProvince {
    pub original_col: u32,  // Original column this is a ghost of
}

/// Nation represents a political entity that controls provinces
#[derive(Component, Clone)]
pub struct Nation {
    pub id: u32,
    pub name: String,
    pub color: Color,
}

/// Marker component for the tile info UI panel
#[derive(Component)]
pub struct TileInfoPanel;

/// Marker component for the tile info text display
#[derive(Component)]
pub struct TileInfoText;