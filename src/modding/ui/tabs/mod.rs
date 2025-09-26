//! Tab content subsystem for the mod browser
//!
//! This module provides the different tab views for browsing and managing mods.
//! Each tab has its own focused module for maintainability.

// Internal modules - all private
mod active;
mod installed;
mod workshop;

// Re-export public functions
pub use active::spawn_active_modset_tab;
pub use installed::spawn_installed_tab;
pub use workshop::spawn_workshop_tab;

use crate::modding::manager::ModManager;
use crate::modding::ui::types::ModBrowserTab;
use bevy::prelude::*;

/// Spawns the appropriate tab content based on the current tab selection
pub fn spawn_tab_content(
    parent: &mut ChildSpawnerCommands,
    tab: ModBrowserTab,
    mod_manager: &ModManager,
    search_query: &str,
) {
    match tab {
        ModBrowserTab::Installed => spawn_installed_tab(parent, mod_manager, search_query),
        ModBrowserTab::Workshop => spawn_workshop_tab(parent),
        ModBrowserTab::ActiveModset => spawn_active_modset_tab(parent, mod_manager),
    }
}