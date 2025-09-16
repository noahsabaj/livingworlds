//! Event handlers for the modding system
//!
//! This module contains all the business logic for handling mod-related events,
//! Steam Workshop integration, and synchronization with the mod manager.

use bevy::prelude::*;

use super::manager::ModManager;
use super::types::ModSource;

/// Events for mod enable/disable operations
#[derive(Event)]
pub struct ModEnabledEvent {
    pub mod_id: String,
}

#[derive(Event)]
pub struct ModDisabledEvent {
    pub mod_id: String,
}

/// Events for Steam Workshop integration
#[derive(Event)]
pub struct WorkshopSubscribeEvent {
    pub workshop_id: u64,
}

#[derive(Event)]
pub struct WorkshopUnsubscribeEvent {
    pub workshop_id: u64,
}

#[derive(Event)]
pub struct RefreshWorkshopDataEvent;

// Import Steam Workshop integration (conditionally)
#[cfg(feature = "steam")]
use crate::steam::{SteamClient, subscribe_to_workshop_item, get_subscribed_items, get_workshop_item_install_info};

// Placeholder Steam types when steam feature is disabled
#[cfg(not(feature = "steam"))]
#[derive(Resource)]
pub struct SteamClient;

/// System to handle mod enable/disable events
pub fn handle_mod_toggle_events(
    mut mod_manager: ResMut<ModManager>,
    mut enable_events: EventReader<ModEnabledEvent>,
    mut disable_events: EventReader<ModDisabledEvent>,
) {
    for event in enable_events.read() {
        mod_manager.enable_mod(&event.mod_id);
        info!("Mod enabled: {}", event.mod_id);
    }

    for event in disable_events.read() {
        mod_manager.disable_mod(&event.mod_id);
        info!("Mod disabled: {}", event.mod_id);
    }
}

/// Handle Steam Workshop subscribe events
pub fn handle_workshop_subscribe_events(
    mut events: EventReader<WorkshopSubscribeEvent>,
    #[cfg(feature = "steam")]
    steam: Option<Res<SteamClient>>,
    #[cfg(not(feature = "steam"))]
    _steam: Option<Res<SteamClient>>,
) {
    #[cfg(feature = "steam")]
    {
        let Some(steam) = steam else {
            if !events.is_empty() {
                warn!("Cannot subscribe to workshop items - Steam not available");
                events.clear();
            }
            return;
        };

        for event in events.read() {
            info!("Subscribing to workshop item: {}", event.workshop_id);

            subscribe_to_workshop_item(&steam, event.workshop_id);

            info!("Successfully subscribed to workshop item: {}", event.workshop_id);
        }
    }

    #[cfg(not(feature = "steam"))]
    {
        if !events.is_empty() {
            warn!("Steam Workshop integration not available - compile with steam feature");
            events.clear();
        }
    }
}

/// Handle Steam Workshop unsubscribe events
pub fn handle_workshop_unsubscribe_events(
    mut events: EventReader<WorkshopUnsubscribeEvent>,
    #[cfg(feature = "steam")]
    steam: Option<Res<SteamClient>>,
    #[cfg(not(feature = "steam"))]
    _steam: Option<Res<SteamClient>>,
) {
    #[cfg(feature = "steam")]
    {
        let Some(_steam) = steam else {
            if !events.is_empty() {
                warn!("Cannot unsubscribe from workshop items - Steam not available");
                events.clear();
            }
            return;
        };

        for event in events.read() {
            info!("Unsubscribing from workshop item: {}", event.workshop_id);

            // Note: bevy_steamworks doesn't expose unsubscribe directly
            // This would be implemented via Steam's UGC API
            warn!("Unsubscribe functionality requires additional Steam UGC API integration");

            info!("Unsubscribe requested for workshop item: {}", event.workshop_id);
        }
    }

    #[cfg(not(feature = "steam"))]
    {
        if !events.is_empty() {
            warn!("Steam Workshop integration not available - compile with steam feature");
            events.clear();
        }
    }
}

/// Handle workshop data refresh events
pub fn handle_refresh_workshop_data_events(
    mut events: EventReader<RefreshWorkshopDataEvent>,
    #[cfg(feature = "steam")]
    steam: Option<Res<SteamClient>>,
    #[cfg(not(feature = "steam"))]
    _steam: Option<Res<SteamClient>>,
    mut mod_manager: ResMut<ModManager>,
) {
    #[cfg(feature = "steam")]
    {
        let Some(_steam) = steam else {
            if !events.is_empty() {
                warn!("Cannot refresh workshop data - Steam not available");
                events.clear();
            }
            return;
        };

        for _ in events.read() {
            info!("Refreshing workshop data...");

            // Trigger mod discovery to pick up newly downloaded workshop items
            mod_manager.refresh_workshop_mods();

            info!("Workshop data refresh completed");
        }
    }

    #[cfg(not(feature = "steam"))]
    {
        if !events.is_empty() {
            warn!("Steam Workshop integration not available - compile with steam feature");
            events.clear();
        }
    }
}

/// Synchronize Steam Workshop installations with local mod manager
pub fn sync_workshop_installations(
    #[cfg(feature = "steam")]
    steam: Option<Res<SteamClient>>,
    #[cfg(not(feature = "steam"))]
    _steam: Option<Res<SteamClient>>,
    mut mod_manager: ResMut<ModManager>,
    time: Res<Time>,
) {
    #[cfg(feature = "steam")]
    {
        let Some(steam) = steam else {
            return; // No warning - this system runs every frame
        };

        // Only sync every few seconds to avoid spam
        static mut LAST_SYNC: f64 = 0.0;
        unsafe {
            if time.elapsed_secs_f64() - LAST_SYNC < 5.0 {
                return;
            }
            LAST_SYNC = time.elapsed_secs_f64();
        }

        // Get subscribed items from Steam
        let subscribed_items = get_subscribed_items(&steam);

        for workshop_id in subscribed_items {
            // Check if the item is installed and get its path
            if let Some(install_path) = get_workshop_item_install_info(&steam, workshop_id) {
                // Check if we already have this workshop mod loaded
                let found = mod_manager.available_mods.iter().any(|m| {
                    matches!(&m.source, ModSource::Workshop(id) if *id == workshop_id)
                });

                if !found {
                    info!("New workshop item detected: {} at {}", workshop_id, install_path);
                    mod_manager.add_workshop_mod(workshop_id, install_path);
                }
            }
        }
    }

    #[cfg(not(feature = "steam"))]
    {
        // Steam not available - just refresh local workshop mods directory
        static mut LAST_SYNC: f64 = 0.0;
        unsafe {
            if time.elapsed_secs_f64() - LAST_SYNC < 30.0 {
                return;
            }
            LAST_SYNC = time.elapsed_secs_f64();
        }

        // Refresh workshop mods from local directory
        mod_manager.refresh_workshop_mods();
    }
}