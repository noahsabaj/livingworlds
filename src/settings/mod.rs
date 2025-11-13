//! Settings Module - Gateway Architecture
//!
//! Pure gateway module for settings functionality. Provides controlled API
//! for settings management, persistence, and UI interactions.
//!
//! Follows the gateway architecture pattern established by the ui/ module.


// PRIVATE MODULES - All implementation hidden behind gateway
mod components;
mod navigation;
mod persistence;
mod resolution;
mod setting_builder; // NEW: Declarative settings automation system
mod ui;
mod types;

// CONTROLLED EXPORTS - Minimal public API

// Essential types for external use

// Essential components for external queries (minimal exposure)

// Core functionality

// Settings UI builders (eating our own dog food)

// NEW: Declarative settings automation system

// Main plugin - Clean architecture with no legacy wrappers
pub use ui::SettingsUIPlugin;

