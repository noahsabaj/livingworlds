//! Steam integration gateway module
//!
//! This module provides complete Steam integration for Living Worlds through
//! a clean gateway architecture. Each Steam feature is self-contained in its
//! own submodule while sharing common Steam client infrastructure.
//!
//! # Architecture
//!
//! The Steam system follows the gateway pattern where this mod.rs file acts
//! as the sole entry point, controlling access to Steam implementations:
//!
//! - **types**: Shared resources, events, and enums used across Steam features
//! - **plugin**: Main SteamPlugin and core Steam initialization
//! - **achievements/**: Achievement system with constants, triggers, and display
//! - **rich_presence**: Real-time status updates visible to Steam friends
//! - **workshop**: User-generated content integration
//! - **statistics**: Steam stats collection and reporting
//! - **leaderboards**: Competitive score submission
//! - **callbacks**: Steam API callback handling
//!
//! Each Steam feature submodule contains focused functionality that can be
//! developed, tested, and maintained independently.

// PRIVATE MODULES - Steam implementation details

mod callbacks;
mod leaderboards;
mod plugin;
mod rich_presence;
mod statistics;
mod types;
mod workshop;

// Achievement system is large enough for its own subdirectory
mod achievements;  // PRIVATE MODULE - Gateway architecture compliance

// SELECTIVE PUBLIC EXPORTS - Controlled Steam API

// Export main integration point
pub use plugin::SteamPlugin;

// Export shared types for external use
pub use types::{
    AchievementUnlockedEvent, SteamClient, SteamStats, WorkshopItemDownloadedEvent,
    WorkshopItemType,
};

// Export key functionality that other systems need
pub use leaderboards::{
    submit_observation_time, submit_peace_record, submit_population_record, submit_world_count,
};
pub use statistics::{increment_stat, set_stat, update_steam_stats};
pub use workshop::{get_subscribed_items, subscribe_to_workshop_item};

// Re-export achievements functionality for controlled access
pub use achievements::{
    get_achievement_display_name, handle_achievement_triggers, unlock_achievement,
};

// PURE GATEWAY - No Implementation Logic
//
// Note: All actual implementations are in their respective files:
// - Core plugin logic is in plugin.rs
// - Achievement system is in achievements/ subdirectory
// - Rich presence logic is in rich_presence.rs
// - Workshop logic is in workshop.rs
// - Statistics logic is in statistics.rs
// - Leaderboards logic is in leaderboards.rs
// - Callback handling is in callbacks.rs
// - Shared types are in types.rs
//
// This gateway file contains ZERO implementation logic - only controlled exports.
