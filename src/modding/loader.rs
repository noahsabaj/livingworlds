//! Configuration loader with hot-reload support
//! 
//! This module handles loading configuration files and watching for changes.

use bevy::prelude::*;
use notify::{Watcher, RecursiveMode, Event as NotifyEvent};
use crossbeam_channel::{unbounded, Receiver};
use std::path::{Path, PathBuf};
use super::types::*;
use super::manager::ModManager;

/// Event sent when configuration files change
#[derive(Event)]
pub struct ConfigReloadEvent {
    pub path: PathBuf,
    pub mod_id: Option<String>,
}

/// Resource for hot-reload file watching
#[derive(Resource)]
pub struct ConfigWatcher {
    watcher: Option<notify::RecommendedWatcher>,
    receiver: Receiver<Result<NotifyEvent, notify::Error>>,
    watching_paths: Vec<PathBuf>,
}

impl ConfigWatcher {
    /// Create a new config watcher
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        
        let watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }).ok();
        
        Self {
            watcher,
            receiver: rx,
            watching_paths: Vec::new(),
        }
    }
    
    /// Start watching a directory for changes
    pub fn watch_directory(&mut self, path: &Path) {
        if let Some(ref mut watcher) = self.watcher {
            if watcher.watch(path, RecursiveMode::Recursive).is_ok() {
                self.watching_paths.push(path.to_path_buf());
                info!("Watching directory for config changes: {:?}", path);
            }
        }
    }
    
    /// Check for file change events
    pub fn check_events(&mut self) -> Vec<ConfigReloadEvent> {
        let mut events = Vec::new();
        
        // Process all pending file events
        while let Ok(Ok(event)) = self.receiver.try_recv() {
            for path in event.paths {
                if path.extension().and_then(|s| s.to_str()) == Some("ron") {
                    info!("Config file changed: {:?}", path);
                    
                    // Determine which mod this belongs to
                    let mod_id = self.get_mod_id_from_path(&path);
                    
                    events.push(ConfigReloadEvent {
                        path: path.clone(),
                        mod_id,
                    });
                }
            }
        }
        
        events
    }
    
    /// Determine which mod a file belongs to based on its path
    fn get_mod_id_from_path(&self, path: &Path) -> Option<String> {
        let path_str = path.to_string_lossy();
        
        if path_str.contains("/mods/") {
            // Extract mod ID from path like "mods/example_mod/config/balance.ron"
            if let Some(start) = path_str.find("/mods/") {
                let after_mods = &path_str[start + 6..];
                if let Some(end) = after_mods.find('/') {
                    return Some(after_mods[..end].to_string());
                }
            }
        }
        
        None
    }
}

/// System to handle configuration hot-reloading
pub fn handle_config_reload(
    mut reload_events: EventReader<ConfigReloadEvent>,
    mut mod_manager: ResMut<ModManager>,
) {
    for event in reload_events.read() {
        info!("Reloading configuration from: {:?}", event.path);
        
        if let Some(ref mod_id) = event.mod_id {
            // Reload specific mod configuration
            if let Some(loaded_mod) = mod_manager.available_mods.iter_mut()
                .find(|m| m.manifest.id == *mod_id) 
            {
                // Re-load the mod's configuration overrides
                let config_dir = loaded_mod.path.join("config");
                
                // Check which config file changed
                if event.path.ends_with("balance.ron") {
                    if let Ok(contents) = std::fs::read_to_string(&event.path) {
                        if let Ok(balance) = ron::from_str::<BalanceConfig>(&contents) {
                            loaded_mod.config_overrides.balance = Some(balance);
                            info!("Reloaded balance config for mod: {}", mod_id);
                        }
                    }
                }
                // Handle other config files...
            }
        } else {
            // Reload base configuration
            mod_manager.initialize();
        }
        
        // Re-apply all active mods to update merged config
        mod_manager.apply_active_mods();
    }
}

/// System to check for file changes
pub fn check_config_changes(
    mut watcher: ResMut<ConfigWatcher>,
    mut reload_events: EventWriter<ConfigReloadEvent>,
) {
    let events = watcher.check_events();
    for event in events {
        reload_events.write(event);
    }
}

/// Initialize config watching for hot-reload
pub fn setup_config_watching(
    mut commands: Commands,
    mod_manager: Res<ModManager>,
) {
    let mut watcher = ConfigWatcher::new();
    
    // Watch base config directory
    watcher.watch_directory(&mod_manager.mod_paths.base_config);
    
    // Watch local mods directory
    watcher.watch_directory(&mod_manager.mod_paths.local_mods);
    
    // Watch workshop mods directory if it exists
    if mod_manager.mod_paths.workshop_mods.exists() {
        watcher.watch_directory(&mod_manager.mod_paths.workshop_mods);
    }
    
    commands.insert_resource(watcher);
    info!("Configuration hot-reload system initialized");
}

/// Helper functions for loading specific config types
pub mod loaders {
    use super::*;
    use std::fs;
    
    /// Load a RON file into a specific type
    pub fn load_ron_file<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let parsed = ron::from_str::<T>(&contents)?;
        Ok(parsed)
    }
    
    /// Load balance configuration
    pub fn load_balance_config(path: &Path) -> Result<BalanceConfig, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
    
    /// Load colors configuration
    pub fn load_colors_config(path: &Path) -> Result<ColorsConfig, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
    
    /// Load generation configuration
    pub fn load_generation_config(path: &Path) -> Result<GenerationConfig, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
    
    /// Load simulation configuration
    pub fn load_simulation_config(path: &Path) -> Result<SimulationConfig, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
    
    /// Load audio configuration
    pub fn load_audio_config(path: &Path) -> Result<AudioConfig, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
    
    /// Load mod manifest
    pub fn load_mod_manifest(path: &Path) -> Result<ModManifest, Box<dyn std::error::Error>> {
        load_ron_file(path)
    }
}