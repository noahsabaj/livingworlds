//! Game state management for Living Worlds
//! 
//! This module defines all possible game states and manages transitions between them.
//! The state system controls which systems run and which UI is displayed.

use bevy::prelude::*;

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
    
    /// Loading a saved world
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
            
            // Add events
            .add_event::<RequestStateTransition>()
            .add_event::<MenuEvent>()
            
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
            
            .add_systems(OnEnter(GameState::LoadingWorld), (
                enter_loading_world,
                crate::setup::setup_world,  // Generate the world when loading
            ))
            .add_systems(OnExit(GameState::LoadingWorld), exit_loading_world)
            
            .add_systems(OnEnter(GameState::InGame), enter_in_game)
            .add_systems(OnExit(GameState::InGame), exit_in_game)
            
            .add_systems(OnEnter(GameState::Paused), enter_paused)
            .add_systems(OnExit(GameState::Paused), exit_paused)
            
            // Debug state logging
            .add_systems(Update, log_state_changes);
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
                let start_time = std::time::Instant::now();
                println!("[TRANSITION START] State transition: {:?} → {:?}", event.from, event.to);
                next_state.set(event.to);
                println!("[TRANSITION QUEUED] Transition queued in {:.1}ms", start_time.elapsed().as_secs_f32() * 1000.0);
            } else {
                println!("WARNING: Invalid transition request: current state {:?} doesn't match expected {:?}", 
                      **current_state, event.from);
            }
        } else {
            println!("WARNING: Illegal state transition from {:?} to {:?}", event.from, event.to);
        }
    }
}

/// Checks if a state transition is valid
fn is_valid_transition(from: GameState, to: GameState, world_gen: &WorldGenerationInProgress) -> bool {
    use GameState::*;
    
    match (from, to) {
        // Loading can only go to MainMenu
        (Loading, MainMenu) => true,
        
        // MainMenu can go to WorldConfiguration, LoadingWorld, or back to Loading (for reset)
        (MainMenu, WorldConfiguration) => true,
        (MainMenu, LoadingWorld) => true,
        (MainMenu, Loading) => true,
        
        // WorldConfiguration can go to WorldGeneration or back to MainMenu
        (WorldConfiguration, WorldGeneration) => true,
        (WorldConfiguration, MainMenu) => true,
        
        // WorldGeneration can go to LoadingWorld or back to WorldConfiguration
        (WorldGeneration, LoadingWorld) => !world_gen.0, // Only if generation complete
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

fn enter_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Entering Loading state");
    // Will spawn loading screen UI here
    
    // Auto-transition to MainMenu after brief loading
    // In a real game, this would happen after actual loading completes
    next_state.set(GameState::MainMenu);
}

fn exit_loading(mut commands: Commands) {
    println!("Exiting Loading state");
    // Cleanup loading screen
}

fn enter_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    game_world_query: Query<Entity, With<crate::components::GameWorld>>,
) {
    println!("Entering MainMenu state");
    
    // Clean up any game world entities if returning from game
    let mut cleaned_count = 0;
    for entity in &game_world_query {
        commands.entity(entity).despawn_recursive();
        cleaned_count += 1;
    }
    if cleaned_count > 0 {
        println!("Cleaned up {} game world entities", cleaned_count);
    }
    
    menu_state.set(MenuState::Main);
    // Will spawn main menu UI here
}

fn exit_main_menu(mut commands: Commands) {
    println!("Exiting MainMenu state");
    // Cleanup main menu UI
}

fn enter_world_configuration(
    mut commands: Commands,
) {
    println!("Entering WorldConfiguration state");
    // The world_config module will handle spawning the configuration UI
}

fn exit_world_configuration(mut commands: Commands) {
    println!("Exiting WorldConfiguration state");
    // The world_config module will handle cleanup
}

fn enter_world_generation(
    mut commands: Commands,
    mut world_gen: ResMut<WorldGenerationInProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Entering WorldGeneration state");
    world_gen.0 = false; // Not in progress yet
    // Will spawn world generation progress UI here
    
    // Transition to LoadingWorld to actually generate the world
    // The WorldGenerationSettings from WorldConfiguration will be used
    next_state.set(GameState::LoadingWorld);
}

fn exit_world_generation(mut commands: Commands) {
    println!("Exiting WorldGeneration state");
    // Cleanup world generation UI
}

fn enter_loading_world(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Entering LoadingWorld state");
    // World generation is triggered by adding setup_world to OnEnter schedule
    // After world is set up, transition to InGame
    // Note: This happens after setup_world completes in the same frame
    next_state.set(GameState::InGame);
}

fn exit_loading_world(mut commands: Commands) {
    println!("Exiting LoadingWorld state");
    // Cleanup loading UI
}

fn enter_in_game(mut commands: Commands) {
    let start = std::time::Instant::now();
    println!("[ENTER] Entering InGame state");
    // Resume simulation
    println!("[ENTER COMPLETE] InGame state entered in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
}

fn exit_in_game(mut commands: Commands) {
    let start = std::time::Instant::now();
    println!("[EXIT] Exiting InGame state");
    // Pause simulation
    println!("[EXIT COMPLETE] InGame state exited in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
}

fn enter_paused(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    time: Res<Time>,
) {
    let start = std::time::Instant::now();
    println!("[ENTER] Entering Paused state at time: {:.2}s", time.elapsed_secs());
    menu_state.set(MenuState::Main); // Show pause menu
    // Will spawn pause menu UI here
    println!("[ENTER COMPLETE] Paused state entered in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
}

fn exit_paused(mut commands: Commands) {
    let start = std::time::Instant::now();
    println!("[EXIT] Exiting Paused state");
    // Cleanup pause menu UI
    println!("[EXIT COMPLETE] Paused state exited in {:.1}ms", start.elapsed().as_secs_f32() * 1000.0);
}

// ============================================================================
// DEBUG HELPERS
// ============================================================================

/// Logs state changes for debugging
fn log_state_changes(
    state: Res<State<GameState>>,
    menu_state: Option<Res<State<MenuState>>>,
    mut prev_state: Local<Option<GameState>>,
) {
    let current = **state;
    
    if let Some(prev) = *prev_state {
        if prev != current {
            println!("DEBUG: State changed: {:?} -> {:?}", prev, current);
            if let Some(menu) = menu_state {
                println!("  Menu state: {:?}", **menu);
            }
        }
    }
    
    *prev_state = Some(current);
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