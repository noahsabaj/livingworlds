//! Save and Load functionality for Living Worlds
//! 
//! This module handles saving and loading game state with:
//! - Compressed saves using zstd
//! - Save versioning for compatibility
//! - Auto-save functionality
//! - Save browser UI
//! - Full state deserialization

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use zstd::stream::{encode_all, decode_all};
use crate::menus::SpawnSaveBrowserEvent;

use crate::mesh::ProvinceStorage;
use crate::resources::{WorldSeed, WorldSize, MapDimensions, GameTime, WorldTension, MineralStorage, ResourceOverlay};
use crate::states::{GameState, RequestStateTransition};
use crate::ui_toolbox::buttons::{ButtonBuilder, ButtonStyle, ButtonSize};
use crate::loading_screen::{LoadingState, start_save_loading, set_loading_progress};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Directory where save files are stored
const SAVE_DIRECTORY: &str = "saves";

/// Save file extension (compressed RON)
const SAVE_EXTENSION: &str = "lws"; // Living Worlds Save

/// Current save version for compatibility checking
const SAVE_VERSION: u32 = 1;

/// Auto-save interval in seconds
const AUTO_SAVE_INTERVAL: f32 = 300.0; // 5 minutes

// ============================================================================
// RESOURCES
// ============================================================================

/// Tracks available save files
#[derive(Resource, Default)]
pub struct SaveGameList {
    pub saves: Vec<SaveGameInfo>,
}

/// Information about a save file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SaveGameInfo {
    pub name: String,
    pub path: PathBuf,
    pub date_created: DateTime<Local>,
    pub world_size: String,
    pub game_time: f32,
    pub version: u32,
    pub compressed_size: u64,
}

/// Complete game state for serialization
#[derive(Serialize, Deserialize)]
pub struct SaveGameData {
    pub version: u32,
    pub timestamp: DateTime<Local>,
    pub world_seed: u32,
    pub world_size: WorldSize,
    pub map_dimensions: MapDimensions,
    pub game_time: GameTime,
    pub world_tension: WorldTension,
    pub mineral_storage: MineralStorage,
    pub resource_overlay: ResourceOverlay,
    pub provinces: Vec<crate::components::Province>,
}

/// Auto-save timer resource
#[derive(Resource)]
pub struct AutoSaveTimer {
    pub timer: Timer,
    pub enabled: bool,
}

/// Save browser UI state
#[derive(Resource, Default)]
pub struct SaveBrowserState {
    pub is_open: bool,
    pub selected_save: Option<usize>,
}

/// Marker for save browser UI root
#[derive(Component)]
pub struct SaveBrowserRoot;

/// Marker for save slot buttons
#[derive(Component)]
pub struct SaveSlotButton {
    pub index: usize,
    pub save_info: SaveGameInfo,
}

/// Marker for delete save buttons
#[derive(Component)]
pub struct DeleteSaveButton {
    pub save_path: PathBuf,
    pub save_name: String,
}

/// Marker for the delete confirmation dialog
#[derive(Component)]
pub struct DeleteConfirmationDialog;

/// Delete confirmation button
#[derive(Component)]
pub struct ConfirmDeleteButton {
    pub save_path: PathBuf,
}

/// Cancel delete button
#[derive(Component)]
pub struct CancelDeleteButton;

/// Marker for the loading screen UI
#[derive(Component)]
pub struct LoadingScreenRoot;

/// Marker for loading text that shows progress
#[derive(Component)]
pub struct LoadingProgressText;

// ============================================================================
// EVENTS
// ============================================================================

/// Event to trigger saving the game
#[derive(Event)]
pub struct SaveGameEvent {
    pub slot_name: String,
}

/// Event to trigger deleting a save file
#[derive(Event)]
pub struct DeleteSaveEvent {
    pub save_path: PathBuf,
    pub save_name: String,
}

/// Event to trigger loading a game
#[derive(Event)]
pub struct LoadGameEvent {
    pub save_path: PathBuf,
}

/// Event sent when save completes
#[derive(Event)]
pub struct SaveCompleteEvent {
    pub success: bool,
    pub message: String,
}

/// Event sent when load completes
#[derive(Event)]
pub struct LoadCompleteEvent {
    pub success: bool,
    pub message: String,
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<SaveGameList>()
            .init_resource::<SaveBrowserState>()
            .insert_resource(AutoSaveTimer {
                timer: Timer::from_seconds(AUTO_SAVE_INTERVAL, TimerMode::Repeating),
                enabled: true,
            })
            
            // Events
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<SaveCompleteEvent>()
            .add_event::<LoadCompleteEvent>()
            .add_event::<DeleteSaveEvent>()
            
            // Systems
            .add_systems(Update, (
                handle_save_game,
                handle_load_game,
                handle_save_load_shortcuts.run_if(in_state(GameState::InGame)),
                handle_auto_save.run_if(in_state(GameState::InGame)),
                update_save_browser,
                handle_save_browser_interactions,
                handle_spawn_save_browser_event,
                handle_delete_button_click,
                handle_delete_confirmation,
            ))
            .add_systems(OnEnter(GameState::LoadingWorld), check_for_pending_load)
            .add_systems(OnExit(GameState::MainMenu), close_save_browser)
            .add_systems(OnExit(GameState::Paused), close_save_browser)
            .add_systems(Startup, ensure_save_directory);
        
        // Register types for reflection
        app.register_type::<crate::components::Province>()
            .register_type::<crate::components::ProvinceResources>()
            .register_type::<crate::components::MineralType>()
            .register_type::<crate::terrain::TerrainType>()
            .register_type::<WorldSeed>()
            .register_type::<WorldSize>()
            .register_type::<MapDimensions>()
            .register_type::<GameTime>()
            .register_type::<WorldTension>()
            .register_type::<MineralStorage>()
            .register_type::<ResourceOverlay>()
            .register_type::<ProvinceStorage>();
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Ensure the save directory exists
fn ensure_save_directory() {
    if let Err(e) = fs::create_dir_all(SAVE_DIRECTORY) {
        eprintln!("Failed to create save directory: {}", e);
    }
}

/// Handle keyboard shortcuts for save/load (F5 = quick save, F9 = quick load)
fn handle_save_load_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_events: EventWriter<SaveGameEvent>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut save_list: ResMut<SaveGameList>,
) {
    // F5 for quick save
    if keyboard.just_pressed(KeyCode::F5) {
        println!("F5 pressed - Quick saving...");
        save_events.write(SaveGameEvent {
            slot_name: "quicksave".to_string(),
        });
    }
    
    // F9 for quick load
    if keyboard.just_pressed(KeyCode::F9) {
        println!("F9 pressed - Quick loading...");
        // Scan for saves directly
        scan_save_files_internal(&mut save_list);
        
        // Load the most recent save
        if let Some(latest) = save_list.saves.first() {
            load_events.write(LoadGameEvent {
                save_path: latest.path.clone(),
            });
        } else {
            println!("No save files found to load");
        }
    }
}

/// Handle save game requests with compression and versioning
fn handle_save_game(
    mut save_events: EventReader<SaveGameEvent>,
    mut complete_events: EventWriter<SaveCompleteEvent>,
    world_seed: Option<Res<WorldSeed>>,
    world_size: Option<Res<WorldSize>>,
    map_dims: Option<Res<MapDimensions>>,
    game_time: Option<Res<GameTime>>,
    world_tension: Option<Res<WorldTension>>,
    mineral_storage: Option<Res<MineralStorage>>,
    resource_overlay: Option<Res<ResourceOverlay>>,
    province_storage: Option<Res<ProvinceStorage>>,
) {
    for event in save_events.read() {
        println!("Saving game to slot: {}", event.slot_name);
        
        // Gather all game state into SaveGameData
        let save_data = SaveGameData {
            version: SAVE_VERSION,
            timestamp: Local::now(),
            world_seed: world_seed.as_ref().map(|s| s.0).unwrap_or(0),
            world_size: world_size.as_deref().copied().unwrap_or(WorldSize::Medium),
            map_dimensions: map_dims.as_deref().copied().unwrap_or_default(),
            game_time: game_time.as_deref().cloned().unwrap_or_default(),
            world_tension: world_tension.as_deref().cloned().unwrap_or_default(),
            mineral_storage: mineral_storage.as_deref().cloned().unwrap_or_default(),
            resource_overlay: resource_overlay.as_deref().copied().unwrap_or_default(),
            provinces: province_storage.as_ref()
                .map(|s| s.provinces.clone())
                .unwrap_or_default(),
        };
        
        // Serialize to RON
        match ron::to_string(&save_data) {
            Ok(serialized) => {
                // Compress with zstd
                match encode_all(serialized.as_bytes(), 3) {
                    Ok(compressed) => {
                        // Generate filename with timestamp
                        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                        let filename = format!("{}/{}_{}.{}", 
                            SAVE_DIRECTORY, event.slot_name, timestamp, SAVE_EXTENSION);
                        
                        // Write compressed data asynchronously
                        let filename_clone = filename.clone();
                        let compressed_size = compressed.len() as u64;
                        
                        IoTaskPool::get()
                            .spawn(async move {
                                match File::create(&filename_clone) {
                                    Ok(mut file) => {
                                        if let Err(e) = file.write_all(&compressed) {
                                            eprintln!("Failed to write save file: {}", e);
                                            false
                                        } else {
                                            println!("Game saved successfully to: {} ({}KB compressed)", 
                                                filename_clone, compressed_size / 1024);
                                            true
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to create save file: {}", e);
                                        false
                                    }
                                }
                            })
                            .detach();
                        
                        complete_events.write(SaveCompleteEvent {
                            success: true,
                            message: format!("Game saved to {} ({}KB)", filename, compressed_size / 1024),
                        });
                    }
                    Err(e) => {
                        eprintln!("Failed to compress save data: {}", e);
                        complete_events.write(SaveCompleteEvent {
                            success: false,
                            message: format!("Failed to compress save data: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize save data: {:?}", e);
                complete_events.write(SaveCompleteEvent {
                    success: false,
                    message: format!("Failed to serialize game state: {:?}", e),
                });
            }
        }
    }
}

/// Pending load data to be applied when LoadingWorld state is entered
#[derive(Resource)]
pub struct PendingLoadData(pub SaveGameData);

/// Handle load game requests with decompression and deserialization
fn handle_load_game(
    mut load_events: EventReader<LoadGameEvent>,
    mut complete_events: EventWriter<LoadCompleteEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in load_events.read() {
        println!("Loading game from: {:?}", event.save_path);
        
        // Read the compressed save file
        match fs::read(&event.save_path) {
            Ok(compressed_data) => {
                // Decompress
                match decode_all(&compressed_data[..]) {
                    Ok(decompressed) => {
                        // Deserialize from RON
                        match ron::from_str::<SaveGameData>(&String::from_utf8_lossy(&decompressed)) {
                            Ok(save_data) => {
                                // Check version compatibility
                                if save_data.version > SAVE_VERSION {
                                    eprintln!("Save file version {} is newer than game version {}", 
                                        save_data.version, SAVE_VERSION);
                                    complete_events.write(LoadCompleteEvent {
                                        success: false,
                                        message: format!("Save file version {} incompatible with game version {}",
                                            save_data.version, SAVE_VERSION),
                                    });
                                    continue;
                                }
                                
                                println!("Successfully loaded save from {}", save_data.timestamp);
                                println!("Game time: {} days, World size: {:?}", 
                                    save_data.game_time.current_date, save_data.world_size);
                                
                                // Initialize loading screen for save loading
                                let mut loading_state = LoadingState::default();
                                let file_size = std::fs::metadata(&event.save_path)
                                    .map(|m| format_file_size(m.len()))
                                    .unwrap_or_else(|_| "Unknown".to_string());
                                start_save_loading(
                                    &mut loading_state,
                                    event.save_path.file_name()
                                        .and_then(|n| n.to_str())
                                        .unwrap_or("Save")
                                        .to_string(),
                                    save_data.game_time.current_date,
                                    file_size,
                                );
                                commands.insert_resource(loading_state);
                                
                                // Store the data to be applied when LoadingWorld state is entered
                                commands.insert_resource(PendingLoadData(save_data));
                                
                                // Transition to LoadingWorld state
                                next_state.set(GameState::LoadingWorld);
                                
                                complete_events.write(LoadCompleteEvent {
                                    success: true,
                                    message: format!("Game loaded from {:?}", event.save_path),
                                });
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize save data: {:?}", e);
                                complete_events.write(LoadCompleteEvent {
                                    success: false,
                                    message: format!("Failed to parse save file: {:?}", e),
                                });
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decompress save file: {}", e);
                        complete_events.write(LoadCompleteEvent {
                            success: false,
                            message: format!("Failed to decompress save file: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read save file: {}", e);
                complete_events.write(LoadCompleteEvent {
                    success: false,
                    message: format!("Failed to read save file: {}", e),
                });
            }
        }
    }
}

/// Check if we have pending save data to load instead of generating a new world
fn check_for_pending_load(
    mut commands: Commands,
    pending_load: Option<Res<PendingLoadData>>,
    mut state_events: EventWriter<RequestStateTransition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut loading_state: ResMut<LoadingState>,
) {
    if let Some(load_data) = pending_load {
        println!("Restoring game state from save...");
        set_loading_progress(&mut loading_state, 0.2, "Restoring game state...");
        
        // Insert all the loaded resources
        commands.insert_resource(WorldSeed(load_data.0.world_seed));
        commands.insert_resource(load_data.0.world_size);
        commands.insert_resource(load_data.0.map_dimensions);
        commands.insert_resource(load_data.0.game_time.clone());
        commands.insert_resource(load_data.0.world_tension.clone());
        commands.insert_resource(load_data.0.mineral_storage.clone());
        commands.insert_resource(load_data.0.resource_overlay);
        set_loading_progress(&mut loading_state, 0.4, "Resources restored...");
        
        // Build the world mesh from loaded provinces
        use crate::mesh::{build_world_mesh, WorldMeshHandle};
        use bevy::render::mesh::Mesh2d;
        use bevy::sprite::MeshMaterial2d;
        
        println!("Rebuilding world mesh from {} provinces...", load_data.0.provinces.len());
        set_loading_progress(&mut loading_state, 0.5, "Rebuilding world mesh...");
        let mesh_handle = build_world_mesh(&load_data.0.provinces, &mut meshes);
        set_loading_progress(&mut loading_state, 0.8, "Creating game entities...");
        
        // Spawn the world mesh entity (just like in setup_world)
        commands.spawn((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Name::new("World Mega-Mesh"),
        ));
        
        // Store the mesh handle as a resource for overlay system
        commands.insert_resource(WorldMeshHandle(mesh_handle.clone()));
        
        // Create province storage with the loaded provinces and mesh handle
        let mut province_by_id = std::collections::HashMap::new();
        for (idx, province) in load_data.0.provinces.iter().enumerate() {
            province_by_id.insert(province.id.value(), idx);
        }
        
        commands.insert_resource(ProvinceStorage {
            provinces: load_data.0.provinces.clone(),
            province_by_id,
            mesh_handle,
        });
        
        // Create the spatial index for fast province lookups
        use crate::resources::ProvincesSpatialIndex;
        let mut spatial_index = ProvincesSpatialIndex::default();
        for province in &load_data.0.provinces {
            spatial_index.insert_position_only(province.position, province.id.value());
        }
        commands.insert_resource(spatial_index);
        
        // Generate cloud system based on world seed (clouds are procedural, not saved)
        use rand::{SeedableRng, rngs::StdRng};
        use crate::generation::types::{MapDimensions as GenMapDimensions, MapBounds};
        use crate::constants::HEX_SIZE_PIXELS;
        
        let mut rng = StdRng::seed_from_u64(load_data.0.world_seed as u64);
        let gen_dimensions = GenMapDimensions {
            provinces_per_row: load_data.0.map_dimensions.provinces_per_row,
            provinces_per_col: load_data.0.map_dimensions.provinces_per_col,
            hex_size: HEX_SIZE_PIXELS,
            bounds: MapBounds {
                x_min: -load_data.0.map_dimensions.width_pixels / 2.0,
                x_max: load_data.0.map_dimensions.width_pixels / 2.0,
                y_min: -load_data.0.map_dimensions.height_pixels / 2.0,
                y_max: load_data.0.map_dimensions.height_pixels / 2.0,
            },
        };
        
        let cloud_system = crate::generation::clouds::generate(&mut rng, &gen_dimensions);
        commands.insert_resource(cloud_system);
        
        // Remove the pending load data
        commands.remove_resource::<PendingLoadData>();
        set_loading_progress(&mut loading_state, 1.0, "Load complete!");
        
        // Transition directly to InGame since we've restored the state
        state_events.write(RequestStateTransition {
            from: GameState::LoadingWorld,
            to: GameState::InGame,
        });
    }
    // If no pending load data, the normal world generation will occur
}

/// Handle auto-save timer
fn handle_auto_save(
    time: Res<Time>,
    mut timer: ResMut<AutoSaveTimer>,
    mut save_events: EventWriter<SaveGameEvent>,
) {
    if !timer.enabled {
        return;
    }
    
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        println!("Auto-saving game...");
        save_events.write(SaveGameEvent {
            slot_name: "autosave".to_string(),
        });
    }
}

/// System to handle the SpawnSaveBrowserEvent
fn handle_spawn_save_browser_event(
    mut events: EventReader<SpawnSaveBrowserEvent>,
    commands: Commands,
    save_list: ResMut<SaveGameList>,
    browser_state: ResMut<SaveBrowserState>,
) {
    for _ in events.read() {
        // Call the spawn function directly - pass the system parameters as-is
        spawn_save_browser(commands, save_list, browser_state);
        return; // Only spawn once per frame
    }
}

/// Spawn the save browser UI
pub fn spawn_save_browser(
    mut commands: Commands,
    mut save_list: ResMut<SaveGameList>,
    mut browser_state: ResMut<SaveBrowserState>,
) {
    // Mark browser as open
    browser_state.is_open = true;
    
    // Scan for saves
    scan_save_files_internal(&mut save_list);
    
    // Create UI root
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        SaveBrowserRoot,
    )).with_children(|parent| {
        // Browser panel
        parent.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(600.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            BorderColor(Color::srgb(0.4, 0.4, 0.45)),
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Load Game"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Save list container with scrolling
            panel.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::scroll_y(),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
            )).with_children(|list| {
                // Add save slots
                for (index, save_info) in save_list.saves.iter().enumerate() {
                    list.spawn((
                        Button,
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(15.0)),
                            margin: UiRect::bottom(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                        SaveSlotButton {
                            index,
                            save_info: save_info.clone(),
                        },
                    )).with_children(|slot| {
                        // Create a row container for save info and delete button
                        slot.spawn(Node {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Start,
                            ..default()
                        }).with_children(|row| {
                            // Left side: Save info
                            row.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                flex_grow: 1.0,
                                ..default()
                            }).with_children(|info| {
                                // Save name
                                info.spawn((
                                    Text::new(&save_info.name),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                                
                                // Save details
                                info.spawn((
                                    Text::new(format!("Date: {} | Size: {} | Game Time: {:.0} days",
                                        save_info.date_created.format("%Y-%m-%d %H:%M"),
                                        format_file_size(save_info.compressed_size),
                                        save_info.game_time
                                    )),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                    Node {
                                        margin: UiRect::top(Val::Px(5.0)),
                                        ..default()
                                    },
                                ));
                            });
                            
                            // Right side: Delete button
                            ButtonBuilder::new("Delete")
                                .style(ButtonStyle::Danger)
                                .size(ButtonSize::Small)
                                .with_marker(DeleteSaveButton {
                                    save_path: save_info.path.clone(),
                                    save_name: save_info.name.clone(),
                                })
                                .build(row);
                        });
                    });
                }
            });
            
            // Button row
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            }).with_children(|buttons| {
                // Load button
                ButtonBuilder::new("Load Selected")
                    .style(ButtonStyle::Primary)
                    .size(ButtonSize::Large)
                    .with_marker(LoadSelectedButton)
                    .build(buttons);
                
                // Cancel button
                ButtonBuilder::new("Cancel")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Large)
                    .with_marker(CancelBrowserButton)
                    .build(buttons);
            });
        });
    });
}

/// Marker for load button in browser
#[derive(Component)]
struct LoadSelectedButton;

/// Marker for cancel button in browser
#[derive(Component)]
struct CancelBrowserButton;

/// Handle save browser button interactions
fn handle_save_browser_interactions(
    mut interactions: Query<
        (&Interaction, Option<&SaveSlotButton>, Option<&LoadSelectedButton>, Option<&CancelBrowserButton>),
        Changed<Interaction>
    >,
    mut browser_state: ResMut<SaveBrowserState>,
    save_list: Res<SaveGameList>,
    mut load_events: EventWriter<LoadGameEvent>,
    mut commands: Commands,
    browser_query: Query<Entity, With<SaveBrowserRoot>>,
) {
    for (interaction, save_slot, load_btn, cancel_btn) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Some(slot) = save_slot {
                // Select this save
                browser_state.selected_save = Some(slot.index);
                println!("Selected save: {}", slot.save_info.name);
            } else if load_btn.is_some() {
                // Load the selected save
                if let Some(index) = browser_state.selected_save {
                    if let Some(save_info) = save_list.saves.get(index) {
                        load_events.write(LoadGameEvent {
                            save_path: save_info.path.clone(),
                        });
                        
                        // Close browser
                        close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
                    }
                }
            } else if cancel_btn.is_some() {
                // Close browser
                close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
            }
        }
    }
}

/// Update save browser visuals
fn update_save_browser(
    browser_state: Res<SaveBrowserState>,
    mut save_slots: Query<(&SaveSlotButton, &mut BackgroundColor)>,
) {
    if !browser_state.is_open {
        return;
    }
    
    for (slot, mut bg_color) in &mut save_slots {
        if Some(slot.index) == browser_state.selected_save {
            *bg_color = BackgroundColor(Color::srgb(0.25, 0.25, 0.3));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.17));
        }
    }
}

/// Close the save browser
fn close_save_browser(
    mut commands: Commands,
    browser_query: Query<Entity, With<SaveBrowserRoot>>,
    mut browser_state: ResMut<SaveBrowserState>,
) {
    close_save_browser_internal(&mut commands, &browser_query, &mut browser_state);
}

fn close_save_browser_internal(
    commands: &mut Commands,
    browser_query: &Query<Entity, With<SaveBrowserRoot>>,
    browser_state: &mut SaveBrowserState,
) {
    for entity in browser_query {
        commands.entity(entity).despawn_recursive();
    }
    browser_state.is_open = false;
    browser_state.selected_save = None;
}

/// Scan the save directory and populate the save game list
pub fn scan_save_files(mut save_list: ResMut<SaveGameList>) {
    scan_save_files_internal(&mut save_list);
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Create a quick save
pub fn quick_save(world: &mut World) {
    world.send_event(SaveGameEvent {
        slot_name: "quicksave".to_string(),
    });
}

/// Load the most recent save
pub fn load_latest_save(world: &mut World) {
    // First scan for saves
    {
        let mut save_list = world.resource_mut::<SaveGameList>();
        scan_save_files_internal(&mut save_list);
    }
    
    let save_list = world.resource::<SaveGameList>();
    if let Some(latest) = save_list.saves.first() {
        world.send_event(LoadGameEvent {
            save_path: latest.path.clone(),
        });
    }
}

/// Extract minimal metadata from a save file efficiently
/// Only reads the first 8KB of the compressed file to avoid performance issues
fn extract_save_metadata(path: &Path) -> Option<(String, f32, u32)> {
    use std::io::{BufReader, Read as IoRead};
    
    // Read only the first 8KB of the compressed file (should contain metadata)
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut limited_reader = (&mut reader).take(8192); // 8KB should be enough for metadata
    let mut compressed_chunk = Vec::new();
    limited_reader.read_to_end(&mut compressed_chunk).ok()?;
    
    // Try to decompress what we have
    // Note: This might fail if we cut off mid-stream, so we'll fallback to full read if needed
    let decompressed = match decode_all(&compressed_chunk[..]) {
        Ok(data) => data,
        Err(_) => {
            // Fallback: read the full file if partial decompression fails
            let mut file = File::open(path).ok()?;
            let mut compressed_data = Vec::new();
            file.read_to_end(&mut compressed_data).ok()?;
            
            // But only decompress the first 64KB of content to keep it fast
            let mut decoder = zstd::stream::Decoder::new(&compressed_data[..]).ok()?;
            let mut partial_decompressed = Vec::new();
            let _ = (&mut decoder).take(65536).read_to_end(&mut partial_decompressed);
            partial_decompressed
        }
    };
    
    // Convert only what we need to string (first 16KB should have all metadata)
    let check_len = decompressed.len().min(16384);
    let data_str = String::from_utf8_lossy(&decompressed[..check_len]);
    
    // Extract world_size (look for the enum variant)
    let world_size = if data_str.contains("world_size: Small") {
        "Small".to_string()
    } else if data_str.contains("world_size: Medium") {
        "Medium".to_string()
    } else if data_str.contains("world_size: Large") {
        "Large".to_string()
    } else {
        "Unknown".to_string()
    };
    
    // Extract game_time (look for current_date field)
    let game_time = if let Some(idx) = data_str.find("current_date:") {
        let substr = &data_str[idx + 13..]; // Skip "current_date:"
        if let Some(end_idx) = substr.find(',') {
            substr[..end_idx].trim().parse::<f32>().unwrap_or(0.0)
        } else if let Some(end_idx) = substr.find('\n') {
            substr[..end_idx].trim().parse::<f32>().unwrap_or(0.0)
        } else {
            0.0
        }
    } else {
        0.0
    };
    
    // Extract version
    let version = if let Some(idx) = data_str.find("version:") {
        let substr = &data_str[idx + 8..]; // Skip "version:"
        if let Some(end_idx) = substr.find(',') {
            substr[..end_idx].trim().parse::<u32>().unwrap_or(1)
        } else if let Some(end_idx) = substr.find('\n') {
            substr[..end_idx].trim().parse::<u32>().unwrap_or(1)
        } else {
            1
        }
    } else {
        1
    };
    
    Some((world_size, game_time, version))
}

/// Internal function for scanning save files without ECS wrapper
pub fn scan_save_files_internal(save_list: &mut SaveGameList) {
    save_list.saves.clear();
    
    let save_dir = Path::new(SAVE_DIRECTORY);
    if !save_dir.exists() {
        return;
    }
    
    if let Ok(entries) = fs::read_dir(save_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if extension == SAVE_EXTENSION {
                            if let Some(file_name) = entry.file_name().to_str() {
                                // Extract metadata from filename if possible (has timestamps)
                                // Format: save_YYYYMMDD_HHMMSS_YYYYMMDD_HHMMSS.lws
                                let name = file_name.trim_end_matches(&format!(".{}", SAVE_EXTENSION));
                                
                                // Try to parse date from filename first (much faster!)
                                let date_created = if name.starts_with("save_") {
                                    // Parse the second timestamp (actual save time)
                                    let parts: Vec<&str> = name.split('_').collect();
                                    if parts.len() >= 4 {
                                        let date_str = format!("{} {}", parts[3], parts[4]);
                                        chrono::NaiveDateTime::parse_from_str(&date_str, "%Y%m%d %H%M%S")
                                            .ok()
                                            .and_then(|naive| {
                                                use chrono::{Local, TimeZone};
                                                Local.from_local_datetime(&naive).single()
                                            })
                                            .unwrap_or_else(|| {
                                                // Fallback to file modification time
                                                metadata.modified()
                                                    .ok()
                                                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                                    .and_then(|d| {
                                                        use chrono::{Local, TimeZone};
                                                        Local.timestamp_opt(d.as_secs() as i64, 0).single()
                                                    })
                                                    .unwrap_or_else(chrono::Local::now)
                                            })
                                    } else {
                                        chrono::Local::now()
                                    }
                                } else {
                                    chrono::Local::now()
                                };
                                
                                // Extract actual metadata from the save file
                                let (world_size, game_time, version) = 
                                    extract_save_metadata(&entry.path())
                                        .unwrap_or_else(|| ("Unknown".to_string(), 0.0, 1));
                                
                                let save_info = SaveGameInfo {
                                    name: name.to_string(),
                                    path: entry.path(),
                                    date_created,
                                    world_size,
                                    game_time,
                                    version,
                                    compressed_size: metadata.len(),
                                };
                                save_list.saves.push(save_info);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort by date, newest first
    save_list.saves.sort_by(|a, b| b.date_created.cmp(&a.date_created));
}

/// Format file size in human-readable format
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// ============================================================================
// DELETE FUNCTIONALITY
// ============================================================================

/// Handle delete button clicks - show confirmation dialog
fn handle_delete_button_click(
    mut interactions: Query<
        (&Interaction, &DeleteSaveButton),
        (Changed<Interaction>, With<Button>)
    >,
    mut commands: Commands,
    existing_dialog: Query<Entity, With<DeleteConfirmationDialog>>,
) {
    for (interaction, delete_button) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Close any existing dialog first
            for entity in &existing_dialog {
                commands.entity(entity).despawn_recursive();
            }
            
            // Spawn confirmation dialog
            spawn_delete_confirmation_dialog(
                &mut commands,
                delete_button.save_path.clone(),
                delete_button.save_name.clone(),
            );
        }
    }
}

/// Spawn a delete confirmation dialog
fn spawn_delete_confirmation_dialog(
    commands: &mut Commands,
    save_path: PathBuf,
    save_name: String,
) {
    // Create dialog manually with custom markers
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Dark overlay
        ZIndex(400), // High z-index for dialog (separate component)
        DeleteConfirmationDialog,
    )).with_children(|parent| {
        // Dialog panel
        parent.spawn((
            Node {
                width: Val::Px(450.0),
                padding: UiRect::all(Val::Px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            BorderColor(Color::srgb(0.8, 0.2, 0.2)), // Red border for danger
        )).with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Delete Save File?"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.3, 0.3)), // Red text
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Body text
            panel.spawn((
                Text::new(format!(
                    "Are you sure you want to delete \"{}\"?\n\nThis action cannot be undone.",
                    save_name
                )),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));
            
            // Button row
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            }).with_children(|buttons| {
                // Cancel button
                ButtonBuilder::new("Cancel")
                    .style(ButtonStyle::Secondary)
                    .size(ButtonSize::Medium)
                    .with_marker(CancelDeleteButton)
                    .build(buttons);
                    
                // Delete button
                ButtonBuilder::new("Delete")
                    .style(ButtonStyle::Danger)
                    .size(ButtonSize::Medium)
                    .with_marker(ConfirmDeleteButton { save_path })
                    .build(buttons);
            });
        });
    });
}

/// Handle delete confirmation dialog buttons
fn handle_delete_confirmation(
    mut interactions: Query<
        (&Interaction, AnyOf<(&ConfirmDeleteButton, &CancelDeleteButton)>),
        Changed<Interaction>
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<DeleteConfirmationDialog>>,
    mut save_list: ResMut<SaveGameList>,
    browser_root: Query<Entity, With<SaveBrowserRoot>>,
    mut spawn_browser_events: EventWriter<crate::menus::SpawnSaveBrowserEvent>,
) {
    for (interaction, (confirm_button, cancel_button)) in &mut interactions {
        if *interaction == Interaction::Pressed {
            // Close the dialog
            if let Ok(dialog_entity) = dialog_query.get_single() {
                commands.entity(dialog_entity).despawn_recursive();
            }
            
            if let Some(confirm) = confirm_button {
                // Delete the save file
                if let Err(e) = fs::remove_file(&confirm.save_path) {
                    eprintln!("Failed to delete save file: {}", e);
                } else {
                    println!("Deleted save file: {:?}", confirm.save_path);
                    
                    // Refresh the save list
                    scan_save_files_internal(&mut save_list);
                    
                    // Refresh the save browser UI by closing and reopening it
                    if let Ok(browser_entity) = browser_root.get_single() {
                        commands.entity(browser_entity).despawn_recursive();
                        
                        // Trigger respawn of the browser with updated list
                        // This will be handled by the existing spawn_save_browser system
                        spawn_browser_events.send(crate::menus::SpawnSaveBrowserEvent);
                    }
                }
            } else if cancel_button.is_some() {
                println!("Delete cancelled");
            }
        }
    }
}

