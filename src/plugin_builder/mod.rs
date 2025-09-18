//! Plugin Builder - Internal Development Utilities
//!
//! This module contains Living Worlds-specific development utilities that
//! are not yet published in the external bevy-plugin-builder crate.
//!
//! **NOTE**: The main `define_plugin!` macro has been migrated to the
//! external `bevy-plugin-builder` crate. Use `use bevy_plugin_builder::define_plugin;`
//! instead of importing from this module.
//!
//! **NOTE**: Interaction handler automation has been moved to the UI module.
//! Use `use crate::ui::interaction::*;` for interaction automation features.
//!
//! This module only contains experimental features not yet ready for publication:
//! - Internal validation utilities
//! - Development helper types

// Private implementation modules for development utilities
// mod macros;        // MIGRATED: Use bevy-plugin-builder crate instead
// mod registration;  // MIGRATED: Use bevy-plugin-builder crate instead
// mod validation;    // MIGRATED: Use bevy-plugin-builder crate instead

// Public API exports for unreleased features only
// Currently empty - all major features have been migrated