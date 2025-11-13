//! Settings persistence using bevy_pkv
//!
//! This module handles saving and loading game settings to/from disk
//! using the bevy_pkv cross-platform key-value store.
//!
//! TEMPORARILY DISABLED: bevy_pkv doesn't support Bevy 0.17 yet
//! This entire module is disabled until bevy_pkv 0.14+ is released

// TODO: Re-enable this module when bevy_pkv supports Bevy 0.17
/*
use super::types::GameSettings;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

/// Load settings from disk on startup
pub fn load_settings(mut commands: Commands, pkv: Res<PkvStore>) {
    info!("Loading settings from disk...");

    // Try to load saved settings, fall back to defaults if not found
    let settings = match pkv.get::<GameSettings>("game_settings") {
        Ok(loaded_settings) => {
            info!("Settings loaded successfully from disk");
            loaded_settings
        }
        Err(e) => {
            info!("No saved settings found ({}), using defaults", e);
            GameSettings::default()
        }
    };

    // Insert the loaded (or default) settings as a resource
    commands.insert_resource(settings);
}

/// Save settings to disk
pub fn save_settings(settings: &GameSettings, pkv: &mut PkvStore) {
    info!("Saving settings to disk...");

    match pkv.set("game_settings", settings) {
        Ok(_) => info!("Settings saved successfully"),
        Err(e) => error!("Failed to save settings: {}", e),
    }
}
*/
