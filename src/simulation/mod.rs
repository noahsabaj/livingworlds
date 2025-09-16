//! Simulation Module - Gateway Architecture
//!
//! This is the ONLY entry point to the simulation module. All external code
//! must access simulation functionality through this gateway. Direct imports
//! from submodules (e.g., `simulation::time::systems`) are forbidden.
//!
//! # Architecture
//!
//! The simulation module is organized into three main domains:
//! - `time/` - Game time management and speed control
//! - `input/` - User input handling for simulation controls
//! - `tension/` - World tension tracking and calculations
//!
//! Each submodule has its own gateway (mod.rs) that controls its public API.
//! This creates a hierarchical gateway system ensuring clean module boundaries.

// PRIVATE modules - internal implementation details
mod input;
mod plugin;
mod tension;
mod time;

// CONTROLLED PUBLIC EXPORTS
// Only expose what external code needs, nothing more

// Main plugin for Bevy app integration
pub use plugin::SimulationPlugin;

// World tension system
pub use tension::WorldTension;

// Time-related exports that other systems need
pub use time::{GameTime, NewYearEvent, SimulationSpeedChanged};

// Note: Input handling is internal only - no public exports needed
// Note: Time systems are internal only - exposed through plugin
