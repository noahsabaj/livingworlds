//! Living Worlds - Main entry point
//! 
//! A minimal orchestrator that configures and launches the game.
//! All game logic is delegated to appropriate modules.

use bevy::prelude::*;
use clap::Parser;
use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// Import from our library
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