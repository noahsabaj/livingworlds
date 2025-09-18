//! Steam integration plugin - ADVANCED CUSTOM INITIALIZATION!
//!
//! This plugin demonstrates COMPLEX custom init automation with early returns!
//! 65 lines with intricate Steam initialization → 45 lines declarative + custom logic!

use bevy::prelude::*;
use bevy_steamworks::*;
use std::sync::Arc;
use bevy_plugin_builder::define_plugin;

use super::{
    achievements, callbacks, leaderboards, rich_presence, statistics,
    types::{AchievementUnlockedEvent, SteamClient, WorkshopItemDownloadedEvent},
};

// Your Steam App ID (replace with actual ID from Valve)
const STEAM_APP_ID: u32 = 480; // Using Spacewar ID for testing - REPLACE WITH YOUR ACTUAL APP ID

/// Steam integration plugin using ADVANCED CUSTOM INITIALIZATION AUTOMATION!
///
/// **AUTOMATION ACHIEVEMENT**: 65 lines complex init → 45 lines declarative + custom!
define_plugin!(SteamPlugin {
    events: [AchievementUnlockedEvent, WorkshopItemDownloadedEvent],

    startup: [
        (callbacks::setup_steam_callbacks, statistics::initialize_stats)
    ],

    update: [
        (callbacks::poll_steam_callbacks,
         rich_presence::update_rich_presence,
         achievements::handle_achievement_triggers)
    ],

    on_exit: {
        bevy::app::AppExit::Success => [callbacks::cleanup_steam]
    },

    custom_init: |app| {
        // Complex Steam initialization with early return handling
        let (client, single) = match Client::init_app(STEAM_APP_ID) {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to initialize Steam: {:?}", e);
                warn!("Running in offline mode - Steam features disabled");
                return; // Early return preserves the automation pattern!
            }
        };

        // Store Steam resources
        app.insert_resource(SteamClient(Arc::new(client.clone())));
        app.insert_resource(single);

        // Log successful initialization
        info!("Steam integration initialized successfully!");
        info!("Steam App ID: {}", STEAM_APP_ID);

        if let Ok(user) = client.user() {
            let steam_id = user.steam_id();
            let name = client.friends().get_friend_name(steam_id);
            info!("Logged in as: {} ({})", name, steam_id.raw());
        }
    }
});