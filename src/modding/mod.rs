//! Modding system for Living Worlds
//!
//! This module provides comprehensive mod support including:
//! - Configuration externalization
//! - Mod discovery and loading
//! - Hot-reload support
//! - Steam Workshop integration
//!
//! # Gateway Architecture
//! This module follows the Gateway Architecture pattern where mod.rs is the ONLY
//! entry/exit point. All submodules are private and only controlled exports are
//! exposed through this file.
//!
//! **CRITICAL: This file contains ZERO implementation logic - it is a pure gateway.**

// INTERNAL MODULES - ALL PRIVATE
mod examples;
mod handlers;
mod loader;
mod manager;
mod plugin;
mod types;
mod ui;  // Now a subfolder with gateway architecture!

// CONTROLLED PUBLIC EXPORTS - Gateway Interface

// Main plugin (the primary entry point for external code)
pub use plugin::ModdingPlugin;

// Events that external code needs access to

// UI Events that need to be accessible from menus
pub use ui::OpenModBrowserEvent;

// Utility functions that external code may need

// Types that external systems need to understand

// Manager access for systems that need to query mod state

// PURE GATEWAY - Zero Implementation Logic
//
// This gateway file contains ABSOLUTELY ZERO business logic, implementations, or plugins.
// It serves only as a controlled access point to internal modules.
//
// All actual implementations are in their respective focused modules:
// - Plugin orchestration logic is in plugin.rs
// - Event handling logic is in handlers.rs
// - Mod management logic is in manager.rs
// - Config loading logic is in loader.rs
// - UI logic is in ui.rs
// - Type definitions are in types.rs
// - Example generation is in examples.rs
//
// This ensures perfect separation of concerns and maintainable architecture.
