//! Steam integration shared types and events
//!
//! This module contains all the shared data structures, events, and enums
//! used across the Steam integration system.

use bevy::prelude::*;
use bevy_steamworks::*;
use std::sync::Arc;

/// Wrapper for the Steam client
#[derive(Resource, Clone)]
pub struct SteamClient(pub Arc<Client>);

/// Steam statistics for Living Worlds
#[derive(Resource, Default)]
pub struct SteamStats {
    pub total_playtime_minutes: f32,
    pub worlds_generated: u32,
    pub provinces_explored: u32,
    pub years_simulated: u32,
    pub nations_witnessed: u32,
    pub wars_observed: u32,
    pub peak_world_population: u64,
}

/// Event triggered when an achievement is unlocked
#[derive(Event)]
pub struct AchievementUnlockedEvent {
    pub achievement_id: String,
    pub name: String,
}

/// Event triggered when a workshop item is downloaded
#[derive(Event)]
pub struct WorkshopItemDownloadedEvent {
    pub item_id: PublishedFileId,
    pub title: String,
}


/// Workshop item types for Living Worlds
#[derive(Debug, Clone, Copy)]
pub enum WorkshopItemType {
    WorldPreset, // Custom world generation settings
    ColorScheme, // Custom terrain/nation colors
    BalanceMod,  // Modified simulation parameters
}