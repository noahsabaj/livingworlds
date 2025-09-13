//! World Entity Marker Components for Living Worlds
//!
//! This module contains marker components for world entities like terrain,
//! clouds, borders, etc. These replace the generic GameWorld marker with
//! specific semantic markers for better organization and clarity.

use bevy::prelude::*;

// ============================================================================
// WORLD ENTITY MARKERS
// ============================================================================

/// Marker component for the main terrain/world mesh entity
/// 
/// This component marks the mega-mesh that represents the entire game world
/// terrain with all provinces rendered as a single mesh.
#[derive(Component, Default)]
pub struct TerrainEntity;

/// Marker component for cloud entities
/// 
/// This component marks cloud sprite entities that float above the world.
/// Clouds are animated and drift across the map.
#[derive(Component, Default)]
pub struct CloudEntity;

/// Marker component for border entities
/// 
/// This component marks entities that render province borders or
/// selection highlights around provinces.
#[derive(Component, Default)]
pub struct BorderEntity;

/// Legacy marker being phased out - use specific markers above instead
/// 
/// This was the generic marker for all game world entities but is being
/// replaced with more specific markers for better code organization.
#[deprecated(note = "Use TerrainEntity, CloudEntity, or BorderEntity instead")]
#[derive(Component, Default)]
pub struct GameWorld;