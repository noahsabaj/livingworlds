//! Core game components with type-safe wrappers
//!
//! This module contains ECS components used throughout the game that are NOT
//! part of the world module. For Province and world-related types, see the
//! world module.
//!
//! MineralType has been moved to world::minerals module where it belongs.

use bevy::prelude::*;

/// Marker component for selected provinces
#[derive(Component)]
pub struct SelectedProvince;

/// Marker component for hoverable provinces
#[derive(Component)]
pub struct HoverableProvince;

/// Component for UI panels showing province info
#[derive(Component)]
pub struct ProvinceInfoPanel;

/// Component for UI text that displays information
#[derive(Component)]
pub struct InfoText;

/// Component for the main game speed display
#[derive(Component)]
pub struct GameSpeedText;

/// Component for the pause indicator
#[derive(Component)]
pub struct PauseIndicator;