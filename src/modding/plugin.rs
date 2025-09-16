//! Main modding plugin implementation
//!
//! This module contains the ModdingPlugin that orchestrates all modding functionality
//! and integrates it with the Bevy app.

use bevy::prelude::*;

use super::handlers::{
    handle_mod_toggle_events, handle_workshop_subscribe_events,
    handle_workshop_unsubscribe_events, handle_refresh_workshop_data_events,
    sync_workshop_installations,
};
use super::loader::ConfigReloadEvent;
use super::manager::ModManager;

/// The main modding plugin that orchestrates all modding functionality
pub struct ModdingPlugin;

impl Plugin for ModdingPlugin {
    fn build(&self, app: &mut App) {
        // Create and initialize mod manager
        let mut mod_manager = ModManager::new();
        mod_manager.initialize();

        // Log loaded mods
        info!("=== Modding System Initialized ===");
        info!("Available mods: {}", mod_manager.available_mods.len());
        for loaded_mod in &mod_manager.available_mods {
            info!(
                "  - {} v{} by {}",
                loaded_mod.manifest.name, loaded_mod.manifest.version, loaded_mod.manifest.author
            );
        }

        app
            // Resources
            .insert_resource(mod_manager)
            // Events - defined in handlers module
            .add_event::<ConfigReloadEvent>()
            .add_event::<super::handlers::ModEnabledEvent>()
            .add_event::<super::handlers::ModDisabledEvent>()
            .add_event::<super::handlers::WorkshopSubscribeEvent>()
            .add_event::<super::handlers::WorkshopUnsubscribeEvent>()
            .add_event::<super::handlers::RefreshWorkshopDataEvent>()
            // UI plugin
            .add_plugins(super::ui::ModBrowserUIPlugin)
            // Systems
            .add_systems(Startup, super::loader::setup_config_watching)
            .add_systems(
                Update,
                (
                    // Config management
                    super::loader::check_config_changes,
                    super::loader::handle_config_reload,
                    // Event handlers
                    handle_mod_toggle_events,
                    handle_workshop_subscribe_events,
                    handle_workshop_unsubscribe_events,
                    handle_refresh_workshop_data_events,
                    sync_workshop_installations,
                )
                    .chain(),
            );
    }
}