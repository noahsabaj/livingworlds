//! World generation gateway module
//!
//! This module contains all builders and generators that create world data.
//! Following the builder pattern, each generator produces data structures
//! defined in the data module.
//!
//! This is a GATEWAY file - it only controls access, no implementation here.

use bevy::prelude::*;

// PRIVATE MODULES - Implementation details hidden from outside

mod builder;     // Main world builder implementation
mod provinces;   // Province generation
mod rivers;      // River system generation
mod agriculture; // Agriculture calculation
mod clouds;      // Cloud system generation
mod erosion;     // Erosion simulation
mod climate;     // Climate zone application
mod utils;       // Utility functions

// PUBLIC INTERFACE - The only way to generate worlds

// Re-export the WorldBuilder from builder.rs
pub use builder::WorldBuilder;


/// Plugin that registers world generation systems
pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        // Generation is typically a one-time process at startup
        // No systems to register, but we could add generation commands here
        app.add_systems(Startup, log_generation_ready);
    }
}

fn log_generation_ready() {
    info!("World generation module ready");
}


/// The generation module provides all world generation functionality.
///
/// # Architecture
///
/// This module follows the builder pattern. The main `WorldBuilder` orchestrates
/// various specialized builders (provinces, rivers, climate, etc.) to create
/// a complete `World` data structure.
///
/// # Gateway Pattern
///
/// This mod.rs file is a pure gateway - it contains NO implementation logic.
/// All implementation is in the private submodules, with only `WorldBuilder`
/// exposed as the public API.
///
/// # Usage
///
/// ```rust
/// use crate::world::generation::WorldBuilder;
/// use crate::resources::WorldSize;
///
/// let world = WorldBuilder::new(
///     42,                    // seed
///     WorldSize::Medium,     // size
///     7,                     // continents
///     0.6,                   // ocean coverage
///     1.0,                   // river density
/// ).build();
/// ```
///
/// # Generation Pipeline
///
/// 1. **Province Generation**: Creates hexagonal tiles with Perlin noise elevation
/// 2. **Erosion Simulation**: Applies realistic erosion patterns
/// 3. **Ocean Depths**: Calculates depth values for ocean tiles
/// 4. **Climate Zones**: Assigns climate based on latitude and elevation
/// 5. **River Systems**: Generates rivers using flow accumulation
/// 6. **Agriculture**: Calculates food production capacity
/// 7. **Cloud System**: Creates atmospheric cloud coverage