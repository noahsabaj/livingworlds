//! Steam integration plugin
//!
//! This module contains the main SteamPlugin that integrates all Steam
//! functionality with the Bevy application.

use bevy::prelude::*;
use bevy_steamworks::*;
use std::sync::Arc;

use super::{
    achievements, callbacks, leaderboards, rich_presence, statistics,
    types::{AchievementUnlockedEvent, SteamClient, WorkshopItemDownloadedEvent},
};

// Your Steam App ID (replace with actual ID from Valve)
const STEAM_APP_ID: u32 = 480; // Using Spacewar ID for testing - REPLACE WITH YOUR ACTUAL APP ID

/// Steam integration plugin for Living Worlds
pub struct SteamPlugin;

impl Plugin for SteamPlugin {
    fn build(&self, app: &mut App) {
        // Initialize Steamworks - MUST be before RenderPlugin
        let (client, single) = match Client::init_app(STEAM_APP_ID) {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to initialize Steam: {:?}", e);
                warn!("Running in offline mode - Steam features disabled");
                return;
            }
        };

        // Store the client as a resource
        app.insert_resource(SteamClient(Arc::new(client.clone())));
        app.insert_resource(single);

        // Steam systems
        app.add_systems(
            Startup,
            (callbacks::setup_steam_callbacks, statistics::initialize_stats),
        )
        .add_systems(
            Update,
            (
                callbacks::poll_steam_callbacks,
                rich_presence::update_rich_presence,
                achievements::handle_achievement_triggers,
            ),
        )
        .add_systems(OnExit(bevy::app::AppExit::Success), callbacks::cleanup_steam);

        // Steam events
        app.add_event::<AchievementUnlockedEvent>()
            .add_event::<WorkshopItemDownloadedEvent>();

        info!("Steam integration initialized successfully!");
        info!("Steam App ID: {}", STEAM_APP_ID);

        if let Ok(user) = client.user() {
            let steam_id = user.steam_id();
            let name = client.friends().get_friend_name(steam_id);
            info!("Logged in as: {} ({})", name, steam_id.raw());
        }
    }
}