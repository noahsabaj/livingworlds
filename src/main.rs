//! Living Worlds - Main entry point
//! 
//! A minimal orchestrator that configures and launches the game.
//! All game logic is delegated to appropriate modules.

use bevy::prelude::*;
use clap::Parser;
use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// Import from our library
use living_worlds::prelude::*;
use living_worlds::{
    build_app, WorldSeed, WorldSize,
    resources::MapDimensions,
};

/// Living Worlds - A procedural civilization simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed for world generation (defaults to current timestamp)
    #[arg(short, long)]
    seed: Option<u32>,

    /// World size (small, medium, large) - random if not specified
    #[arg(short, long)]
    world_size: Option<String>,

    /// Run in test mode
    #[arg(long)]
    test: bool,
}

fn main() {
    let args = Args::parse();
    
    // Configure rayon thread pool for optimal performance
    setup_thread_pool();
    
    // Only generate default values if using command-line generation
    let (seed, world_size) = if args.seed.is_some() || args.world_size.is_some() {
        // Using command-line arguments for direct generation
        let seed = args.seed.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32
        });
        
        let world_size = args.world_size.unwrap_or_else(|| {
            use rand::SeedableRng;
            use rand::rngs::StdRng;
            let mut rng = StdRng::seed_from_u64(seed as u64);
            let sizes = ["small", "medium", "large"];
            sizes.choose(&mut rng).unwrap().to_string()
        });
        
        println!("Starting with command-line parameters: seed={}, size={}", seed, world_size);
        (seed, world_size)
    } else {
        // Menu-based generation - use defaults that will be overridden
        (0, "medium".to_string())
    };
    
    // Build and run the game
    let world_size_enum = WorldSize::from_str(&world_size);
    let map_dimensions = MapDimensions::from_world_size(&world_size_enum);
    
    build_app()
        .insert_resource(WorldSeed(seed))
        .insert_resource(world_size_enum)
        .insert_resource(map_dimensions)
        // Auto-transition disabled to allow menu interaction
        // .add_systems(Update, test_state_transitions)
        .run();
}

/// Configure rayon thread pool for parallel world generation
fn setup_thread_pool() {
    // Use 75% of cores to leave room for rendering and OS
    let num_threads = std::thread::available_parallelism()
        .map(|n| ((n.get() * 3) / 4).max(2))
        .unwrap_or(4);
    
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .expect("Failed to initialize rayon thread pool");
    
    println!("Initialized with {} parallel threads (of {} cores total)", 
             num_threads, 
             std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4));
}

/// Temporary system to test state transitions
/// This will auto-transition through states to test the flow
fn test_state_transitions(
    current_state: Res<State<GameState>>,
    mut events: EventWriter<RequestStateTransition>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    use bevy::state::state::State;
    
    *timer += time.delta_secs();
    
    // Auto-transition through states for testing
    match **current_state {
        GameState::Loading => {
            // After 0.5 seconds, go to MainMenu
            if *timer > 0.5 {
                println!("TEST: Transitioning to MainMenu");
                events.send(RequestStateTransition {
                    from: GameState::Loading,
                    to: GameState::MainMenu,
                });
                *timer = 0.0;
            }
        }
        GameState::MainMenu => {
            // After 1 second in menu, go to WorldConfiguration
            if *timer > 1.0 {
                println!("TEST: Transitioning to WorldConfiguration");
                events.send(RequestStateTransition {
                    from: GameState::MainMenu,
                    to: GameState::WorldConfiguration,
                });
                *timer = 0.0;
            }
        }
        GameState::WorldConfiguration => {
            // After 1 second in config, go to WorldGenerationLoading
            if *timer > 1.0 {
                println!("TEST: Transitioning to WorldGenerationLoading");
                events.send(RequestStateTransition {
                    from: GameState::WorldConfiguration,
                    to: GameState::WorldGenerationLoading,
                });
                *timer = 0.0;
            }
        }
        GameState::WorldGenerationLoading => {
            // After 1 second in loading, go to WorldGeneration
            if *timer > 1.0 {
                println!("TEST: Transitioning to WorldGeneration");
                events.send(RequestStateTransition {
                    from: GameState::WorldGenerationLoading,
                    to: GameState::WorldGeneration,
                });
                *timer = 0.0;
            }
        }
        GameState::WorldGeneration => {
            // After 0.5 seconds, go to LoadingWorld
            if *timer > 0.5 {
                println!("TEST: Transitioning to LoadingWorld");
                events.send(RequestStateTransition {
                    from: GameState::WorldGeneration,
                    to: GameState::LoadingWorld,
                });
                *timer = 0.0;
            }
        }
        GameState::LoadingWorld => {
            // After world loads, go to InGame (handled by world generation completion)
            // For now, just transition after 0.5 seconds
            if *timer > 0.5 {
                println!("TEST: Transitioning to InGame");
                events.send(RequestStateTransition {
                    from: GameState::LoadingWorld,
                    to: GameState::InGame,
                });
                *timer = 0.0;
            }
        }
        GameState::InGame => {
            // Stay in game, could test pause with ESC key
        }
        GameState::Paused => {
            // Would resume or go to menu
        }
    }
}