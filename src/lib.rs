//! Living Worlds - Core Game Library
//!
//! This library contains all game systems, components, and logic for the
//! Living Worlds civilization observer simulator. It can be used by multiple
//! binaries, testing frameworks, and tooling.

// Suppress Bevy-specific lifetime warnings that aren't actionable
// Bevy's system parameters (Res, Query, etc.) have hidden lifetimes that are managed by the framework
#![allow(elided_lifetimes_in_paths)]

// === Module Declarations ===
// Core systems
pub mod components;
pub mod constants;
pub mod performance; // Rayon performance monitoring
pub mod resources;
pub mod safety;
pub mod states;
pub mod version; // Parallel safety validation

// Infrastructure systems
pub mod app; // Application building and plugin management
pub mod cli; // Command-line interface management
pub mod config; // Configuration management and settings
pub mod diagnostics; // Performance monitoring and FPS display
pub mod infrastructure; // System-level configuration and resource management

// World generation and representation
pub mod math; // Single source of truth for spatial math and noise
pub mod world; // World representation and rendering

// Gameplay systems
pub mod nations;
pub mod simulation;

// Content creation and viral moments
pub mod content_creation;

// UI and menus
pub mod loading;
pub mod menus;
pub mod settings;
pub mod ui;

// Utility systems
pub mod camera;
pub mod modding;
pub mod name_generator;
pub mod parallel; // Parallel processing infrastructure (single source of truth for Rayon)
pub mod plugin_builder; // Internal development utilities (main functionality moved to bevy-plugin-builder crate)
pub mod relationships; // Entity relationship system (Bevy 0.16)
pub mod save_load;

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
// Application building (formerly defined in this file)
pub use app::{build_app, build_app_with_config, AppBuildError};

// Configuration management (formerly defined in this file)
pub use config::{AppConfig, DiagnosticsConfig, WindowConfig};

// Performance monitoring (formerly defined in this file)
pub use diagnostics::{display_fps, DiagnosticsPlugin};
