//! Living Worlds - Core Game Library
//!
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// Suppress Bevy-specific lifetime warnings that aren't actionable
// Bevy's system parameters (Res, Query, etc.) have hidden lifetimes that are managed by the framework
#![allow(elided_lifetimes_in_paths)]

// === Module Declarations ===
// Modules directly used by main.rs - must remain public
pub mod cli; // Command-line interface management
pub mod infrastructure; // System-level configuration and resource management
pub mod states; // Game state management

// Modules accessed through gateway re-exports below
mod app; // Application building and plugin management
mod camera;
mod components;
mod config; // Configuration management and settings
mod constants;
mod content_creation;
mod diagnostics; // Performance monitoring and FPS display
mod loading;
mod math; // Single source of truth for spatial math and noise
mod menus;
mod modding;
mod name_generator;
mod nations;
mod parallel; // Parallel processing infrastructure
mod performance; // Rayon performance monitoring
mod relationships; // Entity relationship system
mod resources;
mod safety;
mod save_load;
mod settings;
mod simulation;
mod ui;
mod version;
mod world; // World representation and rendering

// Steam integration (only when feature is enabled)
#[cfg(feature = "steam")]
pub mod steam;

// Test utilities (only available in test builds)
#[cfg(test)]
pub mod test_utils;

// === Configuration Constants ===
/// Default window width in pixels
pub const DEFAULT_WINDOW_WIDTH: f32 = 1920.0;

/// Default window height in pixels
pub const DEFAULT_WINDOW_HEIGHT: f32 = 1080.0;

/// Interval between FPS display updates in seconds
pub const FPS_DISPLAY_INTERVAL_SECS: f32 = 1.0;

/// Milliseconds per second for frame time calculation
pub const MS_PER_SECOND: f32 = 1000.0;

// === Prelude Module ===
/// Re-export only the most essential types that are used across many modules.
/// For other types, prefer explicit imports from their respective modules.
///
/// # Guidelines for prelude inclusion:
/// - Core component types used in most game systems
/// - Fundamental enums that define the game world
/// - State types needed by UI and game logic
///
/// # Explicit imports required for:
/// - Resources (WorldSeed, GameTime, etc.) - use `resources::Type`
/// - Constants - use `constants::CONSTANT_NAME`
/// - Specific systems - import from their modules
pub mod prelude {
    // Core components (used in almost every game system)
    pub use crate::world::{Province, ProvinceId};

    // Core game states (needed by UI and systems)
    pub use crate::states::{GameState, MenuState};

    // Fundamental world types (define the game world)
    pub use crate::world::{ClimateZone, TerrainType};
}

// === Gateway Re-exports ===
// Controlled API surface - only expose what's needed externally

// Application building
pub use app::{build_app, build_app_with_config, AppBuildError};

// Configuration
pub use config::{AppConfig, DiagnosticsConfig, WindowConfig};

// Performance monitoring
pub use diagnostics::{display_fps, DiagnosticsPlugin};

// Version information
pub use version::version_string;

// Core world types (commonly used in external code/tests)
pub use world::{Province, ProvinceId, TerrainType, ClimateZone};

// Nation types (for external access if needed)
pub use nations::{Nation, NationId};

// Game states (already in prelude, but explicit re-export for clarity)
pub use states::{GameState, MenuState};

// Resources (commonly accessed)
pub use resources::{WorldSeed, MapMode};

// Components (if needed externally)
// Note: Position and Size components have been removed/moved to bevy_ui
