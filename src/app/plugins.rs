//! Game Plugins Aggregator

use bevy::prelude::*;
use bevy_plugin_builder::define_plugin;

// Import all game plugins
use crate::{
    camera::CameraPlugin,
    content_creation::ContentCreationPlugin,
    diagnostics::DiagnosticsPlugin,
    loading::LoadingScreenPlugin,
    menus::MenusPlugin,
    modding::ModdingPlugin,
    nations::{DramaEnginePlugin, NationPlugin},
    parallel::ParallelPlugin,
    performance::PerformanceMonitoringPlugin,
    relationships::RelationshipsPlugin,
    save_load::SaveLoadPlugin,
    settings::SettingsUIPlugin,
    simulation::SimulationPlugin,
    states::StatesPlugin,
    ui::UIPlugin,
    world::{NoiseComputePlugin, ProvinceEventsPlugin, WorldPlugin},
};

define_plugin!(GamePlugins {
    plugins: [
        // ========================================================================
        // === CORE SYSTEMS (MUST BE FIRST) ===
        // ========================================================================
        // These plugins provide fundamental functionality that other plugins depend on.
        // ORDER CRITICAL: Do not reorder these without understanding dependencies!

        // StatesPlugin: Provides GameState enum (MainMenu, InGame, Paused, etc.)
        // DEPENDENCIES: None
        // DEPENDENTS: Nearly all other plugins use GameState for lifecycle management
        // MUST BE FIRST: Required before any plugin that uses on_enter/on_exit/run_if
        StatesPlugin,

        // ParallelPlugin: Configures rayon thread pool (75% CPU utilization, ~21 threads)
        // DEPENDENCIES: None
        // DEPENDENTS: All simulation systems that use .par_iter() or parallel processing
        // PROVIDES: ParallelOperationStats resource for monitoring
        ParallelPlugin,

        // RelationshipsPlugin: Entity relationship graph for nations, provinces, laws
        // DEPENDENCIES: None
        // DEPENDENTS: NationPlugin, UIPlugin, SimulationPlugin
        // PROVIDES: Relationship component and query systems
        RelationshipsPlugin,

        // ========================================================================
        // === FOUNDATION SYSTEMS ===
        // ========================================================================
        // These plugins set up base configuration and data structures.

        // ModdingPlugin: Loads base config + active mods, hot-reload system
        // DEPENDENCIES: None (but must be early to configure other systems)
        // DEPENDENTS: All systems that read moddable configuration
        // PROVIDES: ModManager resource, configuration file watching
        // MUST BE EARLY: Config must be loaded before systems that read it
        ModdingPlugin,

        // ProvinceEventsPlugin: Event handling for province-level changes
        // DEPENDENCIES: None
        // DEPENDENTS: WorldPlugin, NationPlugin, UIPlugin
        // PROVIDES: ProvinceSelected, ProvinceHovered event types
        ProvinceEventsPlugin,

        // ========================================================================
        // === UI FOUNDATION (depend on StatesPlugin) ===
        // ========================================================================
        // User interface systems that need GameState for lifecycle.

        // MenusPlugin: Main menu, pause menu, game menus
        // DEPENDENCIES: StatesPlugin (uses GameState::MainMenu)
        // DEPENDENTS: None (self-contained UI)
        // LIFECYCLE: Spawns UI on_enter(MainMenu), despawns on_exit
        MenusPlugin,

        // LoadingScreenPlugin: Async world generation loading screen
        // DEPENDENCIES: StatesPlugin (uses GameState::Loading)
        // DEPENDENTS: WorldPlugin (shows progress during generation)
        // LIFECYCLE: Active during world generation transition
        LoadingScreenPlugin,

        // SettingsUIPlugin: Settings menu, persistence, validation
        // DEPENDENCIES: StatesPlugin (settings menu is a state)
        // DEPENDENTS: All systems that read GameSettings
        // PROVIDES: GameSettings resource (graphics, audio, controls)
        // NOTE: Persistence temporarily disabled (bevy_pkv compatibility pending)
        SettingsUIPlugin,

        // ========================================================================
        // === WORLD AND SIMULATION ===
        // ========================================================================
        // Core gameplay systems for world generation and simulation.

        // WorldPlugin: Province generation, terrain, minerals, rivers
        // DEPENDENCIES: ModdingPlugin (reads world gen settings)
        // DEPENDENTS: NationPlugin (needs provinces), UIPlugin (displays world)
        // PROVIDES: ProvinceStorage, ProvincesSpatialIndex resources
        // LIFECYCLE: Generates world on_enter(GameState::Loading)
        WorldPlugin,

        // NoiseComputePlugin: GPU-accelerated noise generation for terrain
        // DEPENDENCIES: WorldPlugin (works with world generation)
        // DEPENDENTS: WorldPlugin (provides GPU compute shaders)
        // PROVIDES: NoiseComputeSettings, GpuComputeStatus resources
        NoiseComputePlugin,

        // NationPlugin: Territory, governance, laws, history
        // DEPENDENCIES: WorldPlugin (needs provinces), RelationshipsPlugin
        // DEPENDENTS: DramaEnginePlugin, SimulationPlugin, UIPlugin
        // PROVIDES: NationRegistry, ProvinceOwnershipCache resources
        NationPlugin,

        // DramaEnginePlugin: Royal houses, characters, drama events
        // DEPENDENCIES: NationPlugin (drama happens within nations)
        // DEPENDENTS: SimulationPlugin (drama affects simulation)
        // PROVIDES: GlobalRng, CharacterRegistry resources
        DramaEnginePlugin,

        // SimulationPlugin: Game time, simulation tick, pressures, history
        // DEPENDENCIES: WorldPlugin, NationPlugin (simulates their data)
        // DEPENDENTS: ContentCreationPlugin (records simulation events)
        // PROVIDES: GameTime, VisualTime, PressureSystemTimer resources
        // ARCHITECTURE: Central simulation loop that drives game progression
        SimulationPlugin,

        // SaveLoadPlugin: Save/load system, auto-save, file browser
        // DEPENDENCIES: All gameplay plugins (saves their state)
        // DEPENDENTS: MenusPlugin (save/load UI)
        // PROVIDES: Save/load functionality for entire game state
        SaveLoadPlugin,

        // ========================================================================
        // === INTERFACE AND CONTROLS ===
        // ========================================================================
        // User input and visual feedback systems.

        // UIPlugin: HUD, panels, notifications, shortcuts, dialogs
        // DEPENDENCIES: StatesPlugin, NationPlugin (displays nation data)
        // DEPENDENTS: All systems that show UI notifications or use shortcuts
        // PROVIDES: NotificationPlugin, ShortcutRegistry, various UI panels
        // ARCHITECTURE: Orchestrates all UI subsystems (HUD, tooltips, etc.)
        UIPlugin,

        // CameraPlugin: Camera movement, bounds, edge scrolling, zoom
        // DEPENDENCIES: WorldPlugin (needs world bounds for camera limits)
        // DEPENDENTS: None (independent input system)
        // PROVIDES: CameraBounds, WindowFocusState resources
        CameraPlugin,

        // ContentCreationPlugin: Screenshot, recording, viral moments
        // DEPENDENCIES: SimulationPlugin (records simulation events)
        // DEPENDENTS: None (optional recording system)
        // PROVIDES: ContentRecorder, HighlightReel resources
        // NOTE: Viral moment detection for auto-capturing interesting events
        ContentCreationPlugin,

        // ========================================================================
        // === MONITORING AND DIAGNOSTICS ===
        // ========================================================================
        // Performance monitoring and debugging tools.

        // DiagnosticsPlugin: Error tracking, performance metrics
        // DEPENDENCIES: None (observes other systems)
        // DEPENDENTS: PerformanceMonitoringPlugin (uses diagnostic data)
        // PROVIDES: Error context, performance counters
        DiagnosticsPlugin,

        // PerformanceMonitoringPlugin: FPS counter, memory usage, parallel stats
        // DEPENDENCIES: ParallelPlugin (displays parallel operation stats)
        // DEPENDENTS: None (display-only system)
        // PROVIDES: Performance dashboard UI
        PerformanceMonitoringPlugin
    ],

    // Custom initialization for conditional plugins
    custom_init: |app: &mut bevy::app::App| {
        // Add parallel safety validation in debug builds
        #[cfg(debug_assertions)]
        {
            info!("Debug mode: Adding parallel safety validation");
            app.add_plugins(crate::safety::ParallelSafetyPlugin);
        }
    }
});