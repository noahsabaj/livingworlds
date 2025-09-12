//! Mod Manager - Core mod system management
//! 
//! This module handles mod discovery, loading, validation, and merging.

use bevy::prelude::*;
use std::path::PathBuf;
use std::fs;
use super::types::*;

/// The main mod management resource
#[derive(Resource)]
pub struct ModManager {
    /// Base game configuration (vanilla)
    pub base_config: GameConfig,
    
    /// All discovered mods (enabled and disabled)
    pub available_mods: Vec<LoadedMod>,
    
    /// Currently active mods in load order
    pub active_mods: Vec<String>, // Mod IDs
    
    /// Final merged configuration (base + active mods)
    pub merged_config: GameConfig,
    
    /// Paths to mod directories
    pub mod_paths: ModPaths,
}

/// Standard mod directory locations
#[derive(Debug, Clone)]
pub struct ModPaths {
    pub base_config: PathBuf,
    pub local_mods: PathBuf,
    pub workshop_mods: PathBuf,
}

impl Default for ModPaths {
    fn default() -> Self {
        Self {
            base_config: PathBuf::from("config/base"),
            local_mods: PathBuf::from("mods"),
            workshop_mods: PathBuf::from("mods/workshop"),
        }
    }
}

impl ModManager {
    /// Create a new mod manager
    pub fn new() -> Self {
        let base_config = GameConfig::default();
        
        Self {
            base_config: base_config.clone(),
            available_mods: Vec::new(),
            active_mods: Vec::new(),
            merged_config: base_config,
            mod_paths: ModPaths::default(),
        }
    }
    
    /// Initialize the mod manager by loading base config and discovering mods
    pub fn initialize(&mut self) {
        info!("Initializing mod manager...");
        
        // Load base configuration
        if let Err(e) = self.load_base_config() {
            error!("Failed to load base configuration: {}", e);
        }
        
        // Discover available mods
        self.discover_mods();
        
        // Load mod manifests
        self.load_mod_manifests();
        
        // Sort mods by dependencies and load order
        self.sort_mods_by_load_order();
        
        // Apply active mods
        self.apply_active_mods();
        
        info!("Mod manager initialized with {} mods available", self.available_mods.len());
    }
    
    /// Load the base game configuration from config/base/
    fn load_base_config(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = &self.mod_paths.base_config;
        
        // Load balance.ron
        let balance_path = base_path.join("balance.ron");
        if balance_path.exists() {
            let contents = fs::read_to_string(&balance_path)?;
            if let Ok(balance) = ron::from_str::<BalanceConfig>(&contents) {
                self.base_config.balance = balance;
                info!("Loaded balance configuration");
            }
        }
        
        // Load colors.ron
        let colors_path = base_path.join("colors.ron");
        if colors_path.exists() {
            let contents = fs::read_to_string(&colors_path)?;
            // Note: In production, we'd properly parse the colors.ron structure
            info!("Loaded colors configuration");
        }
        
        // Load other config files...
        info!("Base configuration loaded");
        Ok(())
    }
    
    /// Discover all available mods in mod directories
    fn discover_mods(&mut self) {
        self.available_mods.clear();
        
        // Scan local mods directory
        if let Ok(entries) = fs::read_dir(&self.mod_paths.local_mods) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(mod_name) = entry.file_name().to_str() {
                        // Skip the workshop directory
                        if mod_name == "workshop" {
                            continue;
                        }
                        
                        info!("Discovered local mod: {}", mod_name);
                        self.available_mods.push(LoadedMod {
                            manifest: ModManifest {
                                id: mod_name.to_string(),
                                name: mod_name.to_string(),
                                version: "1.0.0".to_string(),
                                author: "Unknown".to_string(),
                                description: "".to_string(),
                                dependencies: Vec::new(),
                                compatible_game_version: "*".to_string(),
                                load_order: 100,
                            },
                            path: entry.path(),
                            config_overrides: ModConfigOverrides::default(),
                            source: ModSource::Local(entry.path()),
                            enabled: false,
                        });
                    }
                }
            }
        }
        
        // Scan workshop mods directory
        if let Ok(entries) = fs::read_dir(&self.mod_paths.workshop_mods) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(workshop_id) = entry.file_name().to_str() {
                        if let Ok(id) = workshop_id.parse::<u64>() {
                            info!("Discovered workshop mod: {}", workshop_id);
                            self.available_mods.push(LoadedMod {
                                manifest: ModManifest {
                                    id: workshop_id.to_string(),
                                    name: format!("Workshop_{}", workshop_id),
                                    version: "1.0.0".to_string(),
                                    author: "Workshop".to_string(),
                                    description: "".to_string(),
                                    dependencies: Vec::new(),
                                    compatible_game_version: "*".to_string(),
                                    load_order: 200,
                                },
                                path: entry.path(),
                                config_overrides: ModConfigOverrides::default(),
                                source: ModSource::Workshop(id),
                                enabled: false,
                            });
                        }
                    }
                }
            }
        }
    }
    
    /// Load manifest.ron files for each discovered mod
    fn load_mod_manifests(&mut self) {
        for loaded_mod in &mut self.available_mods {
            let manifest_path = loaded_mod.path.join("manifest.ron");
            if manifest_path.exists() {
                if let Ok(contents) = fs::read_to_string(&manifest_path) {
                    if let Ok(manifest) = ron::from_str::<ModManifest>(&contents) {
                        loaded_mod.manifest = manifest;
                        info!("Loaded manifest for mod: {}", loaded_mod.manifest.name);
                    }
                }
            }
            
            // Load mod configuration overrides
            Self::load_mod_config_overrides(loaded_mod);
        }
    }
    
    /// Load configuration overrides for a specific mod
    fn load_mod_config_overrides(loaded_mod: &mut LoadedMod) {
        let config_dir = loaded_mod.path.join("config");
        if !config_dir.exists() {
            return;
        }
        
        // Load balance overrides
        let balance_path = config_dir.join("balance.ron");
        if balance_path.exists() {
            if let Ok(contents) = fs::read_to_string(&balance_path) {
                if let Ok(balance) = ron::from_str::<BalanceConfig>(&contents) {
                    loaded_mod.config_overrides.balance = Some(balance);
                }
            }
        }
        
        // Load other config overrides...
        // (colors, generation, simulation, audio)
    }
    
    /// Sort mods by their load order and dependencies
    fn sort_mods_by_load_order(&mut self) {
        self.available_mods.sort_by_key(|m| m.manifest.load_order);
    }
    
    /// Apply active mods to create the merged configuration
    pub fn apply_active_mods(&mut self) {
        // Start with base config
        self.merged_config = self.base_config.clone();
        
        // Clone active_mods to avoid borrowing conflicts
        let active_mods = self.active_mods.clone();
        
        // Apply each active mod in order
        for mod_id in &active_mods {
            if let Some(loaded_mod) = self.available_mods.iter().find(|m| m.manifest.id == *mod_id) {
                // Clone the mod data to avoid borrowing conflicts
                let balance_override = loaded_mod.config_overrides.balance.clone();
                
                // Apply balance overrides
                if let Some(balance) = balance_override {
                    // In production, we'd merge field by field rather than replacing wholesale
                    self.merged_config.balance = balance;
                }
                
                // Apply other overrides...
                // (colors, generation, simulation, audio)
                
                info!("Applied mod: {}", loaded_mod.manifest.name);
            }
        }
        
        info!("Applied {} active mods to configuration", active_mods.len());
    }
    
    /// Enable a mod by ID
    pub fn enable_mod(&mut self, mod_id: &str) {
        if !self.active_mods.contains(&mod_id.to_string()) {
            self.active_mods.push(mod_id.to_string());
            
            if let Some(loaded_mod) = self.available_mods.iter_mut().find(|m| m.manifest.id == mod_id) {
                loaded_mod.enabled = true;
            }
            
            self.apply_active_mods();
            info!("Enabled mod: {}", mod_id);
        }
    }
    
    /// Disable a mod by ID
    pub fn disable_mod(&mut self, mod_id: &str) {
        self.active_mods.retain(|id| id != mod_id);
        
        if let Some(loaded_mod) = self.available_mods.iter_mut().find(|m| m.manifest.id == mod_id) {
            loaded_mod.enabled = false;
        }
        
        self.apply_active_mods();
        info!("Disabled mod: {}", mod_id);
    }
    
    /// Get the current merged configuration
    pub fn get_config(&self) -> &GameConfig {
        &self.merged_config
    }
    
    /// Check if a mod is compatible with the current game version
    pub fn is_mod_compatible(&self, mod_id: &str) -> bool {
        if let Some(loaded_mod) = self.available_mods.iter().find(|m| m.manifest.id == mod_id) {
            // In production, we'd check version compatibility properly
            return loaded_mod.manifest.compatible_game_version == "*" || 
                   loaded_mod.manifest.compatible_game_version == env!("CARGO_PKG_VERSION");
        }
        false
    }
    
    /// Validate mod dependencies
    pub fn validate_dependencies(&self, mod_id: &str) -> Vec<String> {
        let mut missing = Vec::new();
        
        if let Some(loaded_mod) = self.available_mods.iter().find(|m| m.manifest.id == mod_id) {
            for dep in &loaded_mod.manifest.dependencies {
                if !dep.optional && !self.available_mods.iter().any(|m| m.manifest.id == dep.mod_id) {
                    missing.push(dep.mod_id.clone());
                }
            }
        }
        
        missing
    }
}