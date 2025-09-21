//! Development Mode State Management for Living Worlds
//!
//! This module handles development-specific state initialization and world
//! setup for quick-start development workflows. It provides utilities for
//! bypassing the normal menu flow during development.

use bevy::log::{info, warn};
use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::Args;
use crate::resources::{MapDimensions, WorldSeed, WorldSize};

/// Setup development world for quick-start mode
///
/// This function configures the application with development-specific
/// world parameters when using the `--dev-quick-start` flag. It bypasses
/// the normal menu system and immediately prepares world generation.
///
/// # Arguments
/// * `app` - Mutable reference to the Bevy application
/// * `args` - Parsed command-line arguments containing development parameters
///
/// # Functionality
/// - Generates or uses provided seed for deterministic world generation
/// - Sets world size (defaults to Medium if not specified)
/// - Inserts necessary resources for world generation to begin
/// - Logs development mode configuration for debugging
///
/// # Development Resources Inserted
/// - `WorldSeed` - Seed for procedural generation
/// - `WorldSize` - Size configuration for the world
/// - `MapDimensions` - Calculated dimensions based on world size
///
/// # Usage
/// ```rust
/// use living_worlds::states::setup_development_world;
/// use living_worlds::cli::Args;
///
/// let args = Args::parse();
/// if args.dev_quick_start {
///     setup_development_world(&mut app, &args);
/// }
/// ```
pub fn setup_development_world(app: &mut App, args: &Args) {
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
