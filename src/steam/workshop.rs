//! Steam Workshop integration
//!
//! This module handles Steam Workshop functionality, allowing players to
//! subscribe to and manage user-generated content for Living Worlds.

use bevy::prelude::*;
use bevy_steamworks::*;

use super::types::{SteamClient, WorkshopItemType};

/// Subscribe to a workshop item using u64 ID
pub fn subscribe_to_workshop_item(steam: &SteamClient, workshop_id: u64) {
    let client = &steam.0;
    let ugc = client.ugc();

    let item_id = PublishedFileId::new(workshop_id);
    ugc.subscribe_item(item_id);
    info!("Subscribed to workshop item: {}", workshop_id);
}

/// Get list of subscribed workshop items as u64 IDs
pub fn get_subscribed_items(steam: &SteamClient) -> Vec<u64> {
    let client = &steam.0;
    let ugc = client.ugc();

    ugc.subscribed_items().into_iter().map(|id| id.0).collect()
}

/// Check if a workshop item is downloaded and ready to use
pub fn is_workshop_item_installed(steam: &SteamClient, workshop_id: u64) -> bool {
    let client = &steam.0;
    let ugc = client.ugc();

    let item_id = PublishedFileId::new(workshop_id);
    // Check if the item is downloaded and installed
    if let Ok(state) = ugc.item_state(item_id) {
        state.contains(ItemState::INSTALLED)
    } else {
        false
    }
}

/// Get the installation path for a workshop item using u64 ID
pub fn get_workshop_item_install_info(steam: &SteamClient, workshop_id: u64) -> Option<String> {
    let client = &steam.0;
    let ugc = client.ugc();

    let item_id = PublishedFileId::new(workshop_id);
    ugc.item_install_info(item_id)
        .ok()
        .map(|(path, _timestamp)| path)
}
