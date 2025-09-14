//! Settings persistence using bevy_pkv
//!
//! This module handles saving and loading game settings to/from disk
//! using the bevy_pkv cross-platform key-value store.

use super::types::GameSettings;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

/// Load settings from disk on startup
pub fn load_settings(mut commands: Commands, pkv: Res<PkvStore>) {
    println!("Loading settings from disk...");

    // Try to load saved settings, fall back to defaults if not found
    let settings = match pkv.get::<GameSettings>("game_settings") {
        Ok(loaded_settings) => {
            println!("Settings loaded successfully from disk");
            loaded_settings
        }
        Err(e) => {
            println!("No saved settings found ({}), using defaults", e);
            GameSettings::default()
        }
    };

    // Insert the loaded (or default) settings as a resource
    commands.insert_resource(settings);
}

/// Save settings to disk
pub fn save_settings(settings: &GameSettings, pkv: &mut PkvStore) {
    println!("Saving settings to disk...");

    match pkv.set("game_settings", settings) {
        Ok(_) => println!("Settings saved successfully"),
        Err(e) => eprintln!("Failed to save settings: {}", e),
    }
}
