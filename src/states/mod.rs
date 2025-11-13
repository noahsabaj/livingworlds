//! Game State Management System
//!
//! This module provides comprehensive state management for Living Worlds,
//! controlling the overall flow of the application through different game states.
//!
//! ## Gateway Architecture
//!
//! This module follows Living Worlds' gateway architecture pattern. All submodules
//! are private, and external access is controlled through this mod.rs file.
//!
//! ## State Flow
//! ```text
//! Loading → MainMenu → WorldConfiguration → WorldGeneration → LoadingWorld → InGame ⇄ Paused
//!             ↑                   ↑                              ↑            ↓
//!             └───────────────────┴──────────────────────────────┴────────────┘
//! ```ignore
//!
//! ## Core Systems
//!
//! - **Definitions**: Core state enums, resources, events, and components
//! - **Transitions**: State flow validation and event handling logic
//! - **Lifecycle**: Enter/exit systems for state management and entity cleanup
//! - **Utils**: Helper functions for state checking and transition requests
//! - **Plugin**: Bevy integration for automatic system registration
//!
//! ## Usage
//!
//! ```ignore
//! use crate::states::{StatesPlugin, GameState, request_transition};
//!
//! // Add to Bevy app
//! app.add_plugins(StatesPlugin);
//!
//! // Request state transitions
//! request_transition(GameState::MainMenu, GameState::InGame, &mut writer);
//! ```ignore

#![allow(elided_lifetimes_in_paths)]

// Private submodules - all access controlled through this gateway
mod definitions;
mod development;
mod lifecycle;
mod plugin;
mod transitions;
mod utils;

// Public exports - controlled API surface

// Plugin export
pub use plugin::StatesPlugin;

// Core state enums and types
pub use definitions::{CurrentSettingsTab, GameState, MenuState, SettingsTab, WorldMeshEntity};

// Resource types
pub use definitions::{PendingWorldGeneration, SavedWorldExists, WorldGenerationInProgress};

// Event types
pub use definitions::{MenuEvent, RequestStateTransition, StartWorldGeneration};

// Utility functions
pub use utils::{can_generate_world, is_gameplay_active, is_in_menu, request_transition};

// Development functions
pub use development::setup_development_world;
