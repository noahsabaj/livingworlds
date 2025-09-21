//! Steam callback handling
//!
//! This module handles Steam API callbacks and communication with the Steam client.
//! Callbacks are used for asynchronous Steam operations like achievements,
//! workshop downloads, and leaderboard submissions.

use bevy::prelude::*;
use bevy_steamworks::*;

use super::types::SteamClient;

/// Set up Steam callbacks for the application
pub fn setup_steam_callbacks(steam: Res<SteamClient>) {
    debug!("Setting up Steam callbacks...");
    // Callbacks are handled automatically by bevy_steamworks
    // This system serves as a placeholder for any future custom callback setup
}

/// Poll Steam for callbacks - this must be called regularly
pub fn poll_steam_callbacks(single: Res<SingleClient>) {
    // Poll Steam for callbacks
    single.run_callbacks();
}

/// Handle Steam shutdown and cleanup
pub fn cleanup_steam(steam: Res<SteamClient>, stats: Res<super::types::SteamStats>) {
    debug!("Cleaning up Steam integration...");

    // Final stats update
    super::statistics::update_steam_stats(&steam, &stats);

    // Steam cleanup is handled automatically by the library
}
