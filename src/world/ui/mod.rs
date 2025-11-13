//! World UI gateway module
//!
//! This module contains all user interface elements specific to world configuration
//! and interaction. It uses the reusable UI tools from src/ui/ to build
//! world-specific interfaces.
//!
//! This is a GATEWAY file - it only controls access to UI implementations.
//!
//! # Architecture
//!
//! This module follows the feature-centric pattern where feature-specific UI
//! lives with the feature, while reusable UI tools remain in src/ui/.
//!
//! # Gateway Pattern
//!
//! This mod.rs file is a pure gateway controlling access to UI implementations.
//! The actual UI code is in private submodules.
//!
//! # Contents
//!
//! - **config**: World generation configuration screen
//!   - Parameter selection (size, climate, resources)
//!   - Preview and seed management
//!   - Advanced settings panel
//!
//! # Usage
//!
//! ```ignore
//! use crate::world::ui::{WorldConfigPlugin, WorldGenerationSettings};
//!
//! app.add_plugins(WorldConfigPlugin);
//! ```ignore
//!
//! # Design Philosophy
//!
//! Feature-specific UI (like world configuration) lives with its feature module,
//! while the src/ui/ directory contains only reusable UI tools and components
//! that can be used across multiple features.

// PRIVATE MODULES - UI implementation details

mod config; // World configuration screen (1800+ lines of UI code)

// SELECTIVE PUBLIC EXPORTS - Controlled UI API

// Re-export configuration types and plugin
pub use config::{
    AggressionLevel,
    ClimateType,
    IslandFrequency,
    MineralDistribution,
    MountainDensity,
    ResourceAbundance,
    TradePropensity,
    WorldConfigPlugin,

    WorldGenerationSettings,
    // Enums for world configuration
    WorldPreset,
};
