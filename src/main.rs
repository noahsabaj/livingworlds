//! Living Worlds - Main entry point
//!
//! This is the entry point for the Living Worlds game, a menu-driven
//! civilization observer simulator. The game starts with a main menu
//! where players configure world generation parameters through the UI.

use bevy::prelude::*;
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

// Import from our library - using explicit module paths
use living_worlds::{
    build_app_with_config,
    resources::{MapDimensions, WorldSeed, WorldSize},
    AppConfig, DiagnosticsConfig,
};

// Thread pool configuration constants
const THREAD_POOL_CPU_PERCENTAGE: f32 = 0.75; // Use 75% of available cores
const MIN_WORKER_THREADS: usize = 2; // Minimum threads for parallelism
const DEFAULT_WORKER_THREADS: usize = 4; // Fallback if detection fails
const MAX_WORKER_THREADS: usize = 32; // Cap to prevent resource exhaustion

/// Living Worlds - Command line arguments
///
/// The game is primarily menu-driven, but these arguments allow
/// for debugging and development workflows.
#[derive(Parser, Debug)]
#[command(
    name = "Living Worlds",
    about = "A procedural civilization observer simulator",
    version,
    author
)]
struct Args {
    /// Enable debug mode with verbose logging
    #[arg(long, help = "Enable debug output and diagnostics")]
    debug: bool,

    /// Skip main menu for development (requires --dev-seed)
    #[arg(
        long,
        help = "Skip menu and generate world immediately (development only)"
    )]
    dev_quick_start: bool,

    /// Development seed for quick start mode
    #[arg(
        long,
        requires = "dev_quick_start",
        help = "Seed for development world generation"
    )]
    dev_seed: Option<u32>,

    /// Development world size for quick start mode
    #[arg(
        long,
        requires = "dev_quick_start",
        value_parser = parse_world_size,
        help = "World size: small, medium, or large"
    )]
    dev_size: Option<WorldSize>,

    /// Override thread count (0 = auto-detect)
    #[arg(
        long,
        default_value = "0",
        help = "Number of worker threads (0 for auto)"
    )]
    threads: usize,

    /// Show FPS counter
    #[arg(long, help = "Display FPS counter")]
    show_fps: bool,
}

/// Parse and validate world size from string
fn parse_world_size(s: &str) -> Result<WorldSize, String> {
    match s.to_lowercase().as_str() {
        "small" | "s" => Ok(WorldSize::Small),
        "medium" | "m" => Ok(WorldSize::Medium),
        "large" | "l" => Ok(WorldSize::Large),
        _ => Err(format!(
            "Invalid world size '{}'. Must be: small, medium, or large",
            s
        )),
    }
}

/// Main entry point with proper error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging based on debug flag
    initialize_logging(args.debug);

    // Setup thread pool for world generation
    setup_thread_pool(args.threads)?;

    let config = build_config(&args);

    let mut app = build_app_with_config(config)
        .map_err(|e| format!("Failed to initialize application: {}", e))?;

    // Add development resources if in quick-start mode
    if args.dev_quick_start {
        setup_development_world(&mut app, &args);
    } else {
        // Normal menu-driven mode - don't set world parameters
        // The WorldConfiguration screen will handle this
        info!("Starting in normal mode - main menu will handle world configuration");
    }

    info!("Launching Living Worlds...");
    app.run();

    Ok(())
}

/// Initialize the logging system
fn initialize_logging(debug_mode: bool) {
    // Bevy will use these environment variables for its own logging
    if debug_mode {
        std::env::set_var("RUST_LOG", "debug,living_worlds=debug");
    } else {
        std::env::set_var("RUST_LOG", "info,living_worlds=info,wgpu=warn,naga=warn");
    }
}

/// Configure rayon thread pool for parallel world generation
fn setup_thread_pool(requested_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let num_threads = if requested_threads > 0 {
        // Use user-specified thread count, but apply limits
        requested_threads.min(MAX_WORKER_THREADS)
    } else {
        // Auto-detect optimal thread count
        calculate_optimal_threads()
    };

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .thread_name(|i| format!("world-gen-{}", i))
        .build_global()
        .map_err(|e| format!("Failed to initialize thread pool: {}", e))?;

    let total_cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(DEFAULT_WORKER_THREADS);

    info!(
        "Thread pool initialized: {} worker threads ({}% of {} cores)",
        num_threads,
        (num_threads * 100) / total_cores,
        total_cores
    );

    Ok(())
}

/// Calculate optimal number of worker threads
fn calculate_optimal_threads() -> usize {
    std::thread::available_parallelism()
        .map(|cores| {
            let available = cores.get();
            // Use percentage of cores, leaving some for OS and rendering
            let optimal = (available as f32 * THREAD_POOL_CPU_PERCENTAGE) as usize;
            // Apply min/max bounds
            optimal.clamp(MIN_WORKER_THREADS, MAX_WORKER_THREADS.min(available))
        })
        .unwrap_or_else(|e| {
            warn!("Failed to detect CPU cores: {}. Using default.", e);
            DEFAULT_WORKER_THREADS
        })
}

/// Build application configuration from command line arguments
fn build_config(args: &Args) -> AppConfig {
    AppConfig {
        window: Default::default(), // Use default window settings
        diagnostics: DiagnosticsConfig {
            show_fps: args.show_fps || args.debug,
            fps_interval: 1.0,
            use_console: false, // Always use UI display, not console
        },
        enable_audio: false, // Always disabled to prevent ALSA underrun errors
    }
}

/// Setup development world for quick-start mode
fn setup_development_world(app: &mut App, args: &Args) {
    // Generate or use provided seed
    let seed = args.dev_seed.unwrap_or_else(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or_else(|e| {
                warn!("System time error: {}. Using fallback seed.", e);
                // Use a deterministic fallback instead of random
                42
            })
    });

    // Use provided size or default to medium
    let world_size = args.dev_size.unwrap_or(WorldSize::Medium);
    let map_dimensions = MapDimensions::from_world_size(&world_size);

    info!(
        "Development mode: Quick-starting with seed {} and {:?} world",
        seed, world_size
    );

    // Insert resources for world generation
    app.insert_resource(WorldSeed(seed))
        .insert_resource(world_size)
        .insert_resource(map_dimensions);

    // Note: The game should skip to WorldGeneration state when these resources
    // are present at startup. This would require changes to the state system.
}
