//! Steam statistics tracking and submission
//!
//! This module handles collecting and submitting game statistics to Steam,
//! which are used for achievements and community features.

use bevy::prelude::*;

use super::types::{SteamClient, SteamStats};

/// Initialize Steam statistics
pub fn initialize_stats(steam: Res<SteamClient>) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    // Request current stats from Steam
    user_stats.request_current_stats();

    info!("Steam statistics initialized");
}

/// Update and store statistics to Steam
pub fn update_steam_stats(steam: &SteamClient, stats: &SteamStats) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    let _ = user_stats.set_stat("total_playtime_minutes", stats.total_playtime_minutes);
    let _ = user_stats.set_stat("worlds_generated", stats.worlds_generated as f32);
    let _ = user_stats.set_stat("provinces_explored", stats.provinces_explored as f32);
    let _ = user_stats.set_stat("years_simulated", stats.years_simulated as f32);
    let _ = user_stats.set_stat("nations_witnessed", stats.nations_witnessed as f32);
    let _ = user_stats.set_stat("wars_observed", stats.wars_observed as f32);
    let _ = user_stats.set_stat("peak_world_population", stats.peak_world_population as f32);

    // Store to Steam
    let _ = user_stats.store_stats();
}

/// Increment a specific statistic by 1
pub fn increment_stat(steam: &SteamClient, stat_name: &str) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    if let Ok(current_value) = user_stats.stat(stat_name) {
        let _ = user_stats.set_stat(stat_name, current_value + 1.0);
        let _ = user_stats.store_stats();
    }
}

/// Set a specific statistic to a value
pub fn set_stat(steam: &SteamClient, stat_name: &str, value: f32) {
    let client = &steam.0;
    let user_stats = client.user_stats();

    let _ = user_stats.set_stat(stat_name, value);
    let _ = user_stats.store_stats();
}