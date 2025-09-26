//! State management for the mod browser UI
//!
//! This module contains the state resources that track the current state
//! of the mod browser, including active tabs, search queries, and workshop data.

use super::types::ModBrowserTab;
use bevy::prelude::*;
use std::collections::HashSet;

// ============================================================================
// BROWSER STATE
// ============================================================================

/// Resource tracking the current mod browser state
#[derive(Resource, Debug)]
pub struct ModBrowserState {
    /// Currently active tab
    pub current_tab: ModBrowserTab,

    /// Current search query
    pub search_query: String,

    /// Set of currently active mod IDs
    pub active_mods: HashSet<String>,

    /// Filter states
    pub show_enabled: bool,
    pub show_disabled: bool,
    pub show_local: bool,
    pub show_workshop: bool,
}

impl Default for ModBrowserState {
    fn default() -> Self {
        Self {
            current_tab: ModBrowserTab::Installed,
            search_query: String::new(),
            active_mods: HashSet::new(),
            show_enabled: true,
            show_disabled: true,
            show_local: true,
            show_workshop: true,
        }
    }
}

// ============================================================================
// WORKSHOP CACHE
// ============================================================================

/// Cache for Steam Workshop data
///
/// This cache stores workshop item metadata to avoid repeated API calls
/// and provides a smooth browsing experience.
#[derive(Resource, Default, Debug)]
pub struct WorkshopCache {
    /// Cached workshop items
    pub items: Vec<WorkshopItem>,

    /// Last refresh timestamp
    pub last_refresh: Option<f64>,
}

/// Represents a Steam Workshop item
#[derive(Debug, Clone)]
pub struct WorkshopItem {
    pub workshop_id: u64,
    pub title: String,
    pub description: String,
    pub author: String,
    pub subscriber_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
    pub preview_url: Option<String>,
    pub last_updated: f64,
    pub file_size: u64,
    pub is_subscribed: bool,
}