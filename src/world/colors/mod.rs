//! Gateway module for the unified color system
//!
//! This module is the EXCLUSIVE entry/exit point for all color functionality.
//! Following the Gateway Architecture pattern, all submodules are private and
//! only the carefully selected public API is exposed through re-exports.
//!
//! # Architecture
//!
//! This mod.rs acts as a "train station" - external code cannot directly access
//! any submodules. All functionality must pass through this gateway.
//!
//! # Module Structure (ALL PRIVATE)
//!
//! - `theme` - Color constants
//! - `terrain` - Terrain color computation
//! - `biomes` - Biome color functions
//! - `minerals` - Mineral color functions
//! - `dynamic` - Time/weather effects
//! - `providers` - Color provider traits
//! - `utils` - Helper functions
//! - `world_colors` - Main API implementation

// PRIVATE MODULES - No direct access allowed
mod biomes;
mod dynamic;
mod minerals;
mod providers;
mod terrain;
mod theme;
mod utils;
mod world_colors;

// PUBLIC RE-EXPORTS - The only way to access functionality
// NO BACKWARDS COMPATIBILITY - Only clean API through WorldColors

// Main API
pub use world_colors::WorldColors;

// Types
pub use providers::{ColorProvider, Colorable};
pub use utils::{SafeColor, StoneAbundance};

// Theme constants (read-only access)
pub mod theme_colors {
    pub use super::theme::*;
}
