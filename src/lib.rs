//! Living Worlds - Core Game Library
//! 
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// === Module Declarations ===
// Core systems
pub mod components;
pub mod constants;
pub mod resources;
pub mod states;

// World generation and representation
pub mod generation;  // Tools and builders for world creation
pub mod math;        // Single source of truth for spatial math and noise
pub mod world;       // World representation and rendering
pub mod setup;       // World initialization

// Gameplay systems
pub mod minerals;
pub mod province_events;
pub mod simulation;

// UI and menus
pub mod loading_screen;
pub mod menus;
pub mod settings;
pub mod ui;

// Utility systems
pub mod camera;
pub mod colors;
pub mod modding;
pub mod name_generator;
pub mod save_load;

// Steam integration (only when feature is enabled)
#[cfg(feature = "steam")]
pub mod steam;

// === Configuration Constants ===
/// Default window width in pixels
pub const DEFAULT_WINDOW_WIDTH: f32 = 1920.0;

/// Default window height in pixels
pub const DEFAULT_WINDOW_HEIGHT: f32 = 1080.0;

/// Interval between FPS display updates in seconds
pub const FPS_DISPLAY_INTERVAL_SECS: f32 = 1.0;

/// Milliseconds per second for frame time calculation
pub const MS_PER_SECOND: f32 = 1000.0;

// === Prelude Module ===
/// Re-export only the most essential types that are used across many modules.
/// For other types, prefer explicit imports from their respective modules.
/// 
/// # Guidelines for prelude inclusion:
/// - Core component types used in most game systems
/// - Fundamental enums that define the game world
/// - State types needed by UI and game logic
/// 
/// # Explicit imports required for:
/// - Resources (WorldSeed, GameTime, etc.) - use `resources::Type`
/// - Constants - use `constants::CONSTANT_NAME`
/// - Specific systems - import from their modules
pub mod prelude {
    // Core components (used in almost every game system)
    pub use crate::components::{
        Province, ProvinceId,
    };
    
    // Core game states (needed by UI and systems)
    pub use crate::states::{
        GameState, MenuState,
    };
    
    // Fundamental world types (define the game world)
    pub use crate::world::terrain::{
        TerrainType, ClimateZone,
    };
}

// === Imports ===
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};
use bevy::audio::AudioPlugin;
use bevy_pkv::PkvStore;

// Import all plugins consistently
use crate::{
    camera::CameraPlugin,
    loading_screen::LoadingScreenPlugin,
    menus::MenusPlugin,
    modding::ModdingPlugin,
    province_events::ProvinceEventsPlugin,
    save_load::SaveLoadPlugin,
    settings::SettingsPlugin,
    simulation::SimulationPlugin,
    states::StatesPlugin,
    ui::UIPlugin,
    // World module plugins
    world::{
        BorderPlugin,
        CloudPlugin,
        OverlayPlugin,
        TerrainPlugin,
        WorldConfigPlugin,
    },
};

// === Error Types ===
/// Errors that can occur during app building
#[derive(Debug, thiserror::Error)]
pub enum AppBuildError {
    #[error("Failed to initialize storage: {0}")]
    StorageInit(String),
    
    #[error("Failed to initialize plugin: {0}")]
    PluginInit(String),
}

// === Configuration Types ===
/// Configuration for the application window
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
    pub resizable: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            title: "Living Worlds".to_string(),
            resizable: true,
        }
    }
}

/// Configuration for diagnostics display
#[derive(Debug, Clone, Resource)]
pub struct DiagnosticsConfig {
    pub show_fps: bool,
    pub fps_interval: f32,
    pub use_console: bool,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            show_fps: cfg!(debug_assertions), // Only show in debug builds by default
            fps_interval: FPS_DISPLAY_INTERVAL_SECS,
            use_console: false, // Use UI display by default, not console
        }
    }
}

/// Complete application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window: WindowConfig,
    pub diagnostics: DiagnosticsConfig,
    pub enable_audio: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            diagnostics: DiagnosticsConfig::default(),
            enable_audio: false, // Permanently disabled - prevents ALSA underrun errors on Linux
        }
    }
}

/// Builds the core Bevy application with all Living Worlds plugins.
/// 
/// This sets up the engine, window, and all game systems but doesn't
/// include game-specific resources or startup systems.
/// 
/// # Plugin Initialization Order
/// 
/// The plugins are initialized in a specific order to ensure proper dependencies:
/// 1. **Steam** (if enabled) - Must be before rendering plugins
/// 2. **States** - Core state management, required by most other plugins
/// 3. **Modding** - Loads mod configurations early
/// 4. **Province Events** - Event system for province changes
/// 5. **Menus** - UI menus (depends on States)
/// 6. **World Config** - World generation configuration UI
/// 7. **Loading Screen** - Unified loading UI
/// 8. **Settings** - Game settings management
/// 9. **Simulation & Game Systems** - Core gameplay plugins
/// 10. **UI & Camera** - User interface and camera controls
/// 11. **Borders** - GPU-instanced border rendering (visual layer)
/// 
/// # Errors
/// 
/// Returns `AppBuildError` if:
/// - Storage initialization fails
/// - Critical plugin initialization fails
pub fn build_app() -> Result<App, AppBuildError> {
    build_app_with_config(AppConfig::default())
}

/// Builds the app with custom configuration
pub fn build_app_with_config(config: AppConfig) -> Result<App, AppBuildError> {
    let mut app = App::new();
    
    // Add Steam plugin FIRST (must be before DefaultPlugins/RenderPlugin)
    #[cfg(feature = "steam")]
    {
        info!("Initializing Steam integration");
        app.add_plugins(crate::steam::SteamPlugin);
    }
    
    // Configure Bevy's default plugins with our settings
    let mut default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: config.window.title.clone(),
                resolution: (config.window.width, config.window.height).into(),
                resizable: config.window.resizable,
                ..default()
            }),
            ..default()
        });
    
    // Conditionally disable audio based on configuration
    if !config.enable_audio {
        info!("Audio disabled in configuration");
        default_plugins = default_plugins.disable::<AudioPlugin>();
    }
    
    app.add_plugins(default_plugins);
    
    // Add diagnostics if enabled
    if config.diagnostics.show_fps {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(config.diagnostics.clone())
            .add_systems(Update, display_fps.run_if(resource_exists::<DiagnosticsConfig>));
    }
    
    // Initialize storage - PkvStore::new returns PkvStore directly, not Result
    app.insert_resource(PkvStore::new("LivingWorlds", "LivingWorlds"));
    
    // Add all Living Worlds game plugins in dependency order
    info!("Initializing game plugins");
    
    // Core systems (required by others)
    app.add_plugins(StatesPlugin)  // State management (required by menus, world_config, etc.)
        .add_plugins(ModdingPlugin)  // Mod system (loads configs early)
        .add_plugins(ProvinceEventsPlugin); // Province change events
    
    // UI Systems (depend on States)
    app.add_plugins(MenusPlugin)   // Menu UI system
        .add_plugins(WorldConfigPlugin) // World configuration UI
        .add_plugins(LoadingScreenPlugin) // Unified loading screen
        .add_plugins(SettingsPlugin); // Settings menu system
    
    // World and simulation systems
    app.add_plugins(CloudPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(OverlayPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(SaveLoadPlugin);
    
    // User interface and controls
    app.add_plugins(UIPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BorderPlugin);  // GPU-instanced border rendering
    
    Ok(app)
}

/// Display FPS information based on configuration
fn display_fps(
    diagnostics: Res<DiagnosticsStore>,
    config: Res<DiagnosticsConfig>,
    mut last_print: Local<f32>,
    time: Res<Time>,
) {
    // Check if enough time has passed since last update
    if time.elapsed_secs() - *last_print < config.fps_interval {
        return;
    }
    
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let frame_time_ms = MS_PER_SECOND / value as f32;
            
            if config.use_console {
                // Console output (only in debug builds or when explicitly enabled)
                info!("FPS: {:.1} | Frame Time: {:.2}ms", value, frame_time_ms);
            } else {
                // In production, this would update a UI element instead
                // This is a placeholder for future UI-based FPS display
                #[cfg(debug_assertions)]
                trace!("FPS: {:.1} | Frame Time: {:.2}ms", value, frame_time_ms);
            }
            
            *last_print = time.elapsed_secs();
        }
    }
}