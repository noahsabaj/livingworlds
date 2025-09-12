//! Game state management for Living Worlds
//! 
//! This module defines all possible game states and manages transitions between them.
//! The state system controls which systems run and which UI is displayed.
//!
//! ## State Flow
//! ```text
//! Loading → MainMenu → WorldConfiguration → WorldGeneration → LoadingWorld → InGame ⇄ Paused
//!             ↑                   ↑                              ↑            ↓
//!             └───────────────────┴──────────────────────────────┴────────────┘
//! ```
//!
//! ## Transition Rules
//! - Loading: Initial state, auto-transitions to MainMenu after assets load
//! - MainMenu: Hub for new game, load game, or settings
//! - WorldConfiguration: Player configures world generation parameters
//! - WorldGeneration: Actual world generation processing
//! - LoadingWorld: Unified loading screen for both world generation and save loading
//! - InGame: Active gameplay, can pause/unpause
//! - Paused: Game paused, can resume or return to menu

use bevy::prelude::*;

// Constants for test transitions (if needed for debugging)
#[cfg(feature = "debug-states")]
const TEST_TRANSITION_DELAY: f32 = 0.5;
#[cfg(feature = "debug-states")]
const MENU_TRANSITION_DELAY: f32 = 1.0;

/// Main game states that control the overall flow of the application
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Initial loading/splash screen state
    #[default]
    Loading,
    
    /// Main menu - entry point after loading
    MainMenu,
    
    /// World configuration - player sets generation parameters
    WorldConfiguration,
    
    /// World generation - creating the world
    WorldGeneration,
    
    /// Unified loading screen (for both world generation and save loading)
    LoadingWorld,
    
    /// Active gameplay - world simulation running
    InGame,
    
    /// Game is paused (ESC menu)
    Paused,
}

/// Sub-states for menu navigation (only active during MainMenu state)
/// Note: For Paused state, we'll use a separate system
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::MainMenu)]
pub enum MenuState {
    #[default]
    /// No menu active
    None,
    
    /// Main menu root
    Main,
    
    /// Settings menu (graphics, audio, etc.)
    Settings,
    
    /// Load game browser
    LoadBrowser,
    
    /// Save game dialog
    SaveDialog,
    
    /// Statistics/Observatory view
    Statistics,
    
    /// Help/Controls overlay
    Help,
    
    /// Credits screen
    Credits,
    
    /// Exit confirmation dialog
    ExitConfirmation,
}

/// Settings menu tabs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SettingsTab {
    #[default]
    Graphics,
    Audio,
    Interface,
    Performance,
    Controls,
}

/// Resource for tracking the current settings tab
#[derive(Resource, Default)]
pub struct CurrentSettingsTab(pub SettingsTab);

/// Event for requesting state transitions with validation
#[derive(Event)]
pub struct RequestStateTransition {
    pub to: GameState,
    pub from: GameState,
}

/// Event triggered when world generation should begin
#[derive(Event, Default)]
pub struct StartWorldGeneration;

/// Event for menu navigation
#[derive(Event)]
pub enum MenuEvent {
    Open(MenuState),
    Close,
    Back,
}

/// Resource to track if there's a saved world to continue
#[derive(Resource, Default)]
pub struct SavedWorldExists(pub bool);

/// Resource to track if world generation is in progress
#[derive(Resource, Default)]
pub struct WorldGenerationInProgress(pub bool);

/// Resource to signal that we need to generate a world in LoadingWorld state
#[derive(Resource, Default)]
pub struct PendingWorldGeneration {
    pub pending: bool,
    pub delay_timer: f32,  // Delay before starting generation to allow loading screen to render
}

/// Plugin that manages all state-related systems
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize states
            .init_state::<GameState>()
            .add_sub_state::<MenuState>()
            
            // Add resources
            .init_resource::<CurrentSettingsTab>()
            .init_resource::<SavedWorldExists>()
            .init_resource::<WorldGenerationInProgress>()
            .init_resource::<PendingWorldGeneration>()
            
            // Add events
            .add_event::<RequestStateTransition>()
            .add_event::<MenuEvent>()
            .add_event::<StartWorldGeneration>()
            
            // State transition systems
            .add_systems(Update, handle_state_transitions)
            // Menu events only when MenuState exists (in MainMenu state)
            .add_systems(Update, handle_menu_events.run_if(in_state(GameState::MainMenu)))
            
            // State enter/exit systems
            .add_systems(OnEnter(GameState::Loading), enter_loading)
            .add_systems(OnExit(GameState::Loading), exit_loading)
            
            .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
            .add_systems(OnExit(GameState::MainMenu), exit_main_menu)
            
            .add_systems(OnEnter(GameState::WorldConfiguration), enter_world_configuration)
            .add_systems(OnExit(GameState::WorldConfiguration), exit_world_configuration)
            
            .add_systems(OnEnter(GameState::WorldGeneration), enter_world_generation)
            .add_systems(OnExit(GameState::WorldGeneration), exit_world_generation)
            
            .add_systems(OnEnter(GameState::LoadingWorld), enter_loading_world)
            .add_systems(Update, 
                check_and_trigger_world_generation
                    .run_if(in_state(GameState::LoadingWorld))
            )
            .add_systems(OnExit(GameState::LoadingWorld), exit_loading_world)
            
            .add_systems(OnEnter(GameState::InGame), enter_in_game)
            .add_systems(OnExit(GameState::InGame), exit_in_game)
            
            .add_systems(OnEnter(GameState::Paused), enter_paused)
            .add_systems(OnExit(GameState::Paused), exit_paused)
            
            // Debug state logging - only when state actually changes
            .add_systems(Update, log_state_changes.run_if(state_changed::<GameState>));
    }
}

// ============================================================================
// STATE TRANSITION HANDLERS
// ============================================================================

/// Validates and handles state transition requests
fn handle_state_transitions(
    mut events: EventReader<RequestStateTransition>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    world_gen_progress: Res<WorldGenerationInProgress>,
) {
    for event in events.read() {
        // Validate the transition is legal
        if is_valid_transition(event.from, event.to, &world_gen_progress) {
            if event.from == **current_state {
                #[cfg(feature = "debug-states")]
                {
                    let start_time = std::time::Instant::now();
                    println!("[TRANSITION START] State transition: {:?} → {:?}", event.from, event.to);
                    next_state.set(event.to);
                    println!("[TRANSITION QUEUED] Transition queued in {:.1}ms", start_time.elapsed().as_secs_f32() * 1000.0);
                }
                #[cfg(not(feature = "debug-states"))]
                next_state.set(event.to);
            } else {
                eprintln!("WARNING: Invalid transition request: current state {:?} doesn't match expected {:?}", 
                      **current_state, event.from);
            }
        } else {
            eprintln!("WARNING: Illegal state transition from {:?} to {:?}", event.from, event.to);
        }
    }
}

/// Checks if a state transition is valid
fn is_valid_transition(from: GameState, to: GameState, world_gen: &WorldGenerationInProgress) -> bool {
    use GameState::*;
    
    match (from, to) {
        // Loading can only go to MainMenu
        (Loading, MainMenu) => true,
        
        // MainMenu can go to WorldConfiguration or LoadingWorld
        (MainMenu, WorldConfiguration) => true,
        (MainMenu, LoadingWorld) => true,
        
        // WorldConfiguration can go to LoadingWorld, WorldGeneration, or back to MainMenu
        (WorldConfiguration, LoadingWorld) => true,  // Direct to loading for new world
        (WorldConfiguration, WorldGeneration) => true,  // Legacy path
        (WorldConfiguration, MainMenu) => true,
        
        // WorldGeneration can go to LoadingWorld or back to WorldConfiguration/MainMenu
        (WorldGeneration, LoadingWorld) => true,
        (WorldGeneration, WorldConfiguration) => true,
        (WorldGeneration, MainMenu) => true,
        
        // LoadingWorld can go to InGame or back to MainMenu (on error)
        (LoadingWorld, InGame) => true,
        (LoadingWorld, MainMenu) => true,
        
        // InGame can only be paused
        (InGame, Paused) => true,
        
        // Paused can resume or go to MainMenu
        (Paused, InGame) => true,
        (Paused, MainMenu) => true,
        (Paused, LoadingWorld) => true, // For loading a different save
        
        // All other transitions are invalid
        _ => false,
    }
}

/// Handles menu navigation events
fn handle_menu_events(
    mut events: EventReader<MenuEvent>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    current_menu: Res<State<MenuState>>,
) {
    for event in events.read() {
        match event {
            MenuEvent::Open(state) => {
                println!("Opening menu: {:?}", state);
                next_menu_state.set(*state);
            }
            MenuEvent::Close => {
                println!("Closing menu");
                next_menu_state.set(MenuState::None);
            }
            MenuEvent::Back => {
                // Navigate back based on current menu
                let back_to = match **current_menu {
                    MenuState::Settings => MenuState::Main,
                    MenuState::LoadBrowser => MenuState::Main,
                    MenuState::SaveDialog => MenuState::Main,
                    MenuState::Statistics => MenuState::Main,
                    MenuState::Help => MenuState::Main,
                    MenuState::Credits => MenuState::Main,
                    _ => MenuState::None,
                };
                println!("Navigating back to: {:?}", back_to);
                next_menu_state.set(back_to);
            }
        }
    }
}

// ============================================================================
// STATE ENTER/EXIT SYSTEMS
// ============================================================================

/// System that runs when entering the Loading state
fn enter_loading(
    commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    #[cfg(feature = "debug-states")]
    println!("Entering Loading state");
    
    // TODO: Spawn loading screen UI here
    // TODO: Load game assets here
    
    // For now, immediately transition to MainMenu
    // In production, this should happen after assets are loaded
    next_state.set(GameState::MainMenu);
}

/// Cleanup when exiting the Loading state
fn exit_loading(commands: Commands) {
    #[cfg(feature = "debug-states")]
    println!("Exiting Loading state");
    // TODO: Cleanup loading screen UI
}

/// System that runs when entering the MainMenu state
fn enter_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    game_world_query: Query<Entity, With<crate::components::GameWorld>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    #[cfg(feature = "debug-states")]
    println!("Entering MainMenu state");
    
    // Clean up any game world entities if returning from game
    for entity in &game_world_query {
        commands.entity(entity).despawn_recursive();
    }
    if !game_world_query.is_empty() {
        #[cfg(feature = "debug-states")]
        println!("Cleaned up {} game world entities", game_world_query.iter().count());
    }
    
    // Reset camera position to origin when returning to main menu
    for mut transform in &mut camera_query {
        transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
        #[cfg(feature = "debug-states")]
        println!("Reset camera position to origin");
    }
    
    menu_state.set(MenuState::Main);
    // Main menu UI is spawned by menus.rs module
}

/// Cleanup when exiting the MainMenu state
fn exit_main_menu(commands: Commands) {
    #[cfg(feature = "debug-states")]
    println!("Exiting MainMenu state");
    // Main menu UI cleanup handled by menus.rs module
}

/// System that runs when entering the WorldConfiguration state
fn enter_world_configuration(
    commands: Commands,
) {
    #[cfg(feature = "debug-states")]
    println!("Entering WorldConfiguration state");
    // The world_config module handles spawning the configuration UI
}

/// Cleanup when exiting the WorldConfiguration state
fn exit_world_configuration(commands: Commands) {
    #[cfg(feature = "debug-states")]
    println!("Exiting WorldConfiguration state");
    // The world_config module handles cleanup
}

/// System that runs when entering the WorldGeneration state
fn enter_world_generation(
    mut commands: Commands,
    mut world_gen: ResMut<WorldGenerationInProgress>,
) {
    #[cfg(feature = "debug-states")]
    println!("Entering WorldGeneration state");
    world_gen.0 = true; // Mark as in progress
    // Note: This state is now mostly unused - we go directly to LoadingWorld
}

/// Cleanup when exiting the WorldGeneration state
fn exit_world_generation(
    commands: Commands,
    mut world_gen: ResMut<WorldGenerationInProgress>,
) {
    #[cfg(feature = "debug-states")]
    println!("Exiting WorldGeneration state");
    world_gen.0 = false; // CRITICAL FIX: Reset the flag so generation can happen again
    // Cleanup world generation UI
}

/// System that runs when entering the LoadingWorld state
fn enter_loading_world(
    commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    pending_load: Option<Res<crate::save_load::PendingLoadData>>,
) {
    #[cfg(feature = "debug-states")]
    println!("Entering LoadingWorld state");
    
    // If we have a save to load, transition to InGame after loading
    // Otherwise, wait for world generation to complete
    if pending_load.is_some() {
        // Save loading will handle the transition
    } else {
        // World generation will handle the transition
    }
}

/// Check and trigger world generation after a delay
fn check_and_trigger_world_generation(
    mut commands: Commands,
    mut pending_gen: ResMut<PendingWorldGeneration>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<crate::world_config::WorldGenerationSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut loading_state: ResMut<crate::loading_screen::LoadingState>,
) {
    if !pending_gen.pending {
        return;
    }
    
    // Wait a short delay to allow loading screen to render
    pending_gen.delay_timer -= time.delta_secs();
    if pending_gen.delay_timer > 0.0 {
        return;
    }
    
    // Clear the pending flag
    pending_gen.pending = false;
    
    println!("Starting world generation after loading screen renders");
    
    // Run world generation
    crate::setup::setup_world(
        commands,
        meshes,
        materials,
        settings,
        next_state,
        loading_state,
    );
}

/// Cleanup when exiting the LoadingWorld state
fn exit_loading_world(commands: Commands) {
    #[cfg(feature = "debug-states")]
    println!("Exiting LoadingWorld state");
    // Cleanup any loading UI
}

/// System that runs when entering the InGame state
fn enter_in_game(commands: Commands) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        println!("[ENTER] Entering InGame state");
        println!("[ENTER COMPLETE] InGame state entered in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
    }
    // Resume simulation
}

/// Cleanup when exiting the InGame state
fn exit_in_game(commands: Commands) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        println!("[EXIT] Exiting InGame state");
        println!("[EXIT COMPLETE] InGame state exited in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
    }
    
    // Don't clean up game world here - it should persist when pausing
    // World cleanup happens in enter_main_menu when returning to menu
    
    // Pause simulation
}

/// System that runs when entering the Paused state
fn enter_paused(
    commands: Commands,
    time: Res<Time>,
) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        println!("[ENTER] Entering Paused state at time: {:.2}s", time.elapsed_secs());
        println!("[ENTER COMPLETE] Paused state entered in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
    }
    // Pause menu UI is spawned by menus.rs module
}

/// Cleanup when exiting the Paused state
fn exit_paused(commands: Commands) {
    #[cfg(feature = "debug-states")]
    {
        let start = std::time::Instant::now();
        println!("[EXIT] Exiting Paused state");
        println!("[EXIT COMPLETE] Paused state exited in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
    }
    // Pause menu UI cleanup handled by menus.rs module
}

// ============================================================================
// DEBUG HELPERS
// ============================================================================

/// Logs state changes for debugging (only runs when state actually changes)
fn log_state_changes(
    state: Res<State<GameState>>,
    menu_state: Option<Res<State<MenuState>>>,
) {
    #[cfg(feature = "debug-states")]
    {
        println!("STATE CHANGED: Now in {:?}", **state);
        if let Some(menu) = menu_state {
            println!("  Menu state: {:?}", **menu);
        }
    }
    
    // In production, could log to a file or metrics system
    #[cfg(not(feature = "debug-states"))]
    println!("Game state changed to: {:?}", **state);
}

// ============================================================================
// PUBLIC API
// ============================================================================

/// Request a state transition (with validation)
pub fn request_transition(
    from: GameState,
    to: GameState,
    writer: &mut EventWriter<RequestStateTransition>,
) {
    writer.send(RequestStateTransition { from, to });
}

/// Check if we're in a menu state
pub fn is_in_menu(state: &State<GameState>) -> bool {
    matches!(**state, GameState::MainMenu | GameState::Paused)
}

/// Check if gameplay is active
pub fn is_gameplay_active(state: &State<GameState>) -> bool {
    matches!(**state, GameState::InGame | GameState::Paused)
}

/// Helper to check if world generation can proceed
pub fn can_generate_world(world_gen: &WorldGenerationInProgress) -> bool {
    !world_gen.0 // Can generate if not already in progress
}