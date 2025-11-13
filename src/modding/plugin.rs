//! Modding plugin implementation - COMPLEX INITIALIZATION AUTOMATION!
//!
//! This module demonstrates ADVANCED automation with custom initialization logic!
//! 65 lines with complex setup → ~45 lines declarative with custom_init pattern!

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

use super::handlers::{
    handle_mod_toggle_events, handle_refresh_workshop_data_events,
    handle_workshop_subscribe_events, handle_workshop_unsubscribe_events,
    sync_workshop_installations,
};
use super::loader::ConfigReloadEvent;
use super::manager::ModManager;

// The modding plugin using ADVANCED custom initialization automation!
///
// **AUTOMATION ACHIEVEMENT**: 65 lines with complex setup → ~45 lines declarative!
define_plugin!(ModdingPlugin {
    messages: [
        ConfigReloadEvent,
        super::handlers::ModEnabledEvent,
        super::handlers::ModDisabledEvent,
        super::handlers::WorkshopSubscribeEvent,
        super::handlers::WorkshopUnsubscribeEvent,
        super::handlers::RefreshWorkshopDataEvent
    ],

    plugins: [super::ui::ModBrowserUIPlugin],

    startup: [super::loader::setup_config_watching],

    update: [
        // Config and event handling systems (chained for proper order)
        (
            super::loader::check_config_changes,
            super::loader::handle_config_reload,
            handle_mod_toggle_events,
            handle_workshop_subscribe_events,
            handle_workshop_unsubscribe_events,
            handle_refresh_workshop_data_events,
            sync_workshop_installations
        )
            .chain()
    ],

    custom_init: |app: &mut App| {
        // Custom mod manager initialization with logging
        let mut mod_manager = ModManager::new();
        mod_manager.initialize();

        // Log loaded mods for debugging
        info!("=== Modding System Initialized ===");
        info!("Available mods: {}", mod_manager.available_mods.len());
        for loaded_mod in &mod_manager.available_mods {
            info!(
                "  - {} v{} by {}",
                loaded_mod.manifest.name, loaded_mod.manifest.version, loaded_mod.manifest.author
            );
        }

        // Insert the initialized mod manager
        app.insert_resource(mod_manager);
    }
});
