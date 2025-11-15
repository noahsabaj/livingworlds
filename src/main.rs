//! Living Worlds - Main entry point
//!
//! This is the entry point for the Living Worlds game, a menu-driven
//! civilization observer simulator. This file serves as a thin orchestration
//! layer that delegates all complex logic to specialized modules through
//! controlled gateway interfaces.
//!
//! # Architecture
//!
//! This main.rs follows Living Worlds' gateway architecture principles:
//! - Infrastructure management: `infrastructure::` module
//! - Command-line processing: `cli::` module
//! - Development mode setup: `states::` module
//! - Application building: `app::` module (via lib.rs gateway)
//!
//! All implementation details are delegated to these specialized modules,
//! keeping main.rs focused solely on orchestration and error handling.

use bevy::log::info;

// Import from our library through controlled gateways
use living_worlds::{build_app_with_config, cli::{self, Parser}, infrastructure, states};

/// Main entry point with proper error handling
///
/// Orchestrates the application startup through gateway modules:
/// 1. Parse command-line arguments through CLI gateway
/// 2. Initialize system infrastructure (logging, thread pools)
/// 3. Build application configuration from CLI inputs
/// 4. Create Bevy application with all Living Worlds systems
/// 5. Optionally setup development mode for quick-start workflows
/// 6. Launch the game
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments through CLI gateway
    let args = cli::Args::parse();

    // Initialize infrastructure through gateway modules
    infrastructure::LoggingConfig::initialize(args.debug);
    infrastructure::ThreadPoolManager::initialize(args.threads)?;

    // Build application configuration through CLI gateway
    let config = cli::build_app_config(&args);

    // Build Bevy application through app module (via lib.rs gateway)
    let mut app = build_app_with_config(config)
        .map_err(|e| format!("Failed to initialize application: {e}"))?;

    // Setup development mode through states gateway if requested
    if args.dev_quick_start {
        states::setup_development_world(&mut app, &args);
    } else {
        // Normal menu-driven mode - the UI will handle world configuration
        info!("Starting in normal mode - main menu will handle world configuration");
    }

    info!("Launching Living Worlds...");
    app.run();

    Ok(())
}
