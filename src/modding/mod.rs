//! Modding system for Living Worlds
//! 
//! This module provides comprehensive mod support including:
//! - Configuration externalization
//! - Mod discovery and loading
//! - Hot-reload support
//! - Steam Workshop integration

pub mod types;
pub mod manager;
pub mod loader;
pub mod ui;

// #[cfg(test)]
// mod test; // TODO: Add test module when needed

use bevy::prelude::*;
use manager::ModManager;
use loader::ConfigReloadEvent;

/// The main modding plugin
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
            info!("  - {} v{} by {}", 
                loaded_mod.manifest.name, 
                loaded_mod.manifest.version,
                loaded_mod.manifest.author
            );
        }
        
        app
            // Resources
            .insert_resource(mod_manager)
            
            // Events
            .add_event::<ConfigReloadEvent>()
            .add_event::<ModEnabledEvent>()
            .add_event::<ModDisabledEvent>()
            
            // Add the UI plugin
            .add_plugins(ui::ModBrowserUIPlugin)
            
            // Systems
            .add_systems(Startup, loader::setup_config_watching)
            .add_systems(Update, (
                loader::check_config_changes,
                loader::handle_config_reload,
                handle_mod_toggle_events,
            ).chain());
    }
}

/// Event fired when a mod is enabled
#[derive(Event)]
pub struct ModEnabledEvent {
    pub mod_id: String,
}

/// Event fired when a mod is disabled
#[derive(Event)]
pub struct ModDisabledEvent {
    pub mod_id: String,
}

/// System to handle mod enable/disable events
fn handle_mod_toggle_events(
    mut mod_manager: ResMut<ModManager>,
    mut enable_events: EventReader<ModEnabledEvent>,
    mut disable_events: EventReader<ModDisabledEvent>,
) {
    for event in enable_events.read() {
        mod_manager.enable_mod(&event.mod_id);
        info!("Mod enabled: {}", event.mod_id);
    }
    
    for event in disable_events.read() {
        mod_manager.disable_mod(&event.mod_id);
        info!("Mod disabled: {}", event.mod_id);
    }
}

/// Helper function to get the current game configuration
pub fn get_game_config(mod_manager: &ModManager) -> &types::GameConfig {
    mod_manager.get_config()
}

/// Create an example mod for testing
pub fn create_example_mod() {
    use std::fs;
    use std::path::Path;
    
    let example_mod_path = Path::new("mods/example_balance");
    
    // Create directory structure
    fs::create_dir_all(example_mod_path.join("config")).ok();
    
    // Create manifest
    let manifest = r#"ModManifest(
    id: "example_balance",
    name: "Example Balance Mod",
    version: "1.0.0",
    author: "Living Worlds Team",
    description: "An example mod that tweaks game balance for faster gameplay",
    dependencies: [],
    compatible_game_version: "*",
    load_order: 100,
)"#;
    
    fs::write(example_mod_path.join("manifest.ron"), manifest).ok();
    
    // Create balance override
    let balance = r#"BalanceConfig(
    world: WorldConfig(
        hex_size_pixels: 60.0,  // Larger hexes
        provinces_per_row: 300,
        provinces_per_col: 200,
        edge_buffer: 200.0,
        ocean_depth_shallow: 0.12,
        ocean_depth_medium: 0.07,
        ocean_depth_deep: 0.02,
    ),
    camera: CameraConfig(
        zoom_speed: 0.15,  // Faster zoom
        min_zoom: 0.2,     // Can zoom in more
        max_zoom: 8.0,     // Can zoom out more
        pan_speed_base: 750.0,  // Faster panning
        speed_multiplier: 4.0,  // Higher speed boost
        edge_pan_threshold: 10.0,
        edge_pan_speed_base: 1000.0,
        map_padding_factor: 1.25,
    ),
    simulation: SimulationBalanceConfig(
        starting_year: 500,  // Earlier start
        days_per_year: 365.0,
        default_speed: 2.0,  // Faster default
        max_speed: 20.0,     // Much faster max speed
        min_population: 2000.0,  // Higher starting pop
        max_additional_population: 98000.0,  // Higher max pop
    ),
    // ... other config sections remain default
    ui: UIConfig(
        tile_info_text_size: 18.0,
        padding_percent: 1.0,
        margin_percent: 2.0,
    ),
    clouds: CloudConfig(
        min_scale: 3.0,
        max_scale: 6.0,
        layer_count: 3,
        base_speed: 10.0,
    ),
    generation: GenerationBalanceConfig(
        nation_count: 12,  // More nations
        tectonic_plates_base: 4,
        tectonic_plates_variation: 3,
        island_chain_count: 0,
        archipelago_count: 2,
        continent_size_multiplier: 1.5,
        continent_massive_base: 8000.0,
        continent_massive_variation: 3000.0,
        continent_medium_base: 5000.0,
        continent_medium_variation: 2000.0,
        continent_archipelago_base: 2000.0,
        continent_archipelago_variation: 800.0,
        continent_tiny_base: 800.0,
        continent_tiny_variation: 400.0,
        continent_falloff_base: 0.8,
        continent_falloff_variation: 0.3,
        river_count: 300,  // More rivers
        river_min_elevation: 0.4,  // Rivers at lower elevations
    ),
    spatial: SpatialConfig(
        index_cell_size_multiplier: 2.0,
        ocean_depth_grid_size_multiplier: 3.0,
    ),
    hexagon: HexagonConfig(
        aa_width: 1.5,
        texture_alpha_opaque: 255,
        texture_alpha_transparent: 0,
    ),
)"#;
    
    fs::write(example_mod_path.join("config/balance.ron"), balance).ok();
    
    info!("Created example mod at: {:?}", example_mod_path);
}