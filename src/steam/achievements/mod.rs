//! Steam achievements gateway module
//!
//! This module provides all achievement functionality through a clean gateway
//! architecture. Achievement constants, display names, and trigger logic are
//! separated but accessible through this single entry point.

// PRIVATE MODULES - Achievement implementation details
mod constants;
mod display;
mod triggers;

// SELECTIVE PUBLIC EXPORTS - Controlled achievement API

// Export achievement constants
pub use constants::*;

// Export display functionality
pub use display::get_achievement_display_name;

// Export trigger system
pub use triggers::{handle_achievement_triggers, unlock_achievement};

// PURE GATEWAY - No Implementation Logic
// All actual implementations are in their respective files:
// - Achievement IDs are in constants.rs
// - Display names are in display.rs
// - Trigger logic is in triggers.rs