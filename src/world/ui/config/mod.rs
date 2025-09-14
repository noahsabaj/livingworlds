//! World configuration UI gateway module
//!
//! This module provides the user interface for configuring world generation
//! parameters. It follows gateway architecture - all internal modules are
//! private and only specific types are exposed through controlled exports.
//!
//! # Architecture
//!
//! - **types**: Configuration data structures and enums
//! - **components**: UI marker components for identifying elements
//! - **plugin**: Main plugin that registers all systems
//! - **layout**: UI construction using our builder patterns
//! - **handlers**: Event handling systems
//!
//! # Gateway Pattern
//!
//! This module acts as the sole entry point for world configuration UI.
//! External code can only access what we explicitly export here.

// PRIVATE MODULES - Implementation details
mod components;
mod handlers;
mod layout;
mod plugin;
mod types;

// CONTROLLED PUBLIC EXPORTS

// Plugin - the main integration point
pub use plugin::WorldConfigPlugin;

// Types - configuration data structures
pub use types::{
    AggressionLevel, ClimateType, IslandFrequency, MineralDistribution, MountainDensity,
    ResourceAbundance, TradePropensity, WorldGenerationSettings, WorldPreset,
};

// Note: We do NOT export:
// - Component markers (internal UI implementation)
// - Handler systems (internal logic)
// - Layout functions (internal UI construction)
// These remain private implementation details!
