//! HUD (Heads-Up Display) Module - Pure Gateway
//!
//! Orchestrates HUD elements including time display, speed indicators,
//! and control hints. This is a pure gateway module that only coordinates
//! submodules without containing any implementation logic.

use bevy::prelude::*;

// Submodules - all private, exposed through plugin
mod control_hints;
mod plugin;
mod setup;
mod speed_display;
mod time_display;

// CONTROLLED EXPORTS - Gateway Interface

/// Marker component for the HUD root entity
#[derive(Component)]
pub struct HudRoot;

/// Plugin that manages all HUD elements (implementation in plugin.rs)
pub use plugin::HudPlugin;
